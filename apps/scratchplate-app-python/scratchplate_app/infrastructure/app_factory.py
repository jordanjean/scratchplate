from __future__ import annotations

import asyncio
import contextlib
import logging
import signal
import sys
from typing import Any

import uvicorn
from fastapi import FastAPI, Header
from fastapi.responses import Response

from scratchplate_app.adapter.in_.web.auth import (
    AuthenticatedRequest,
    _forbidden,
    _unauthorized,
    authenticate_request,
)
from scratchplate_app.adapter.in_.web.exception_handlers import register_exception_handlers
from scratchplate_app.adapter.in_.web.routes import create_api_router, create_denial_router, create_management_router
from scratchplate_app.adapter.out.config import ConfigurationWelcomeTemplateAdapter
from scratchplate_app.application.service import GreetUserService
from scratchplate_app.domain import UserIdentity
from scratchplate_app.infrastructure.security import InvalidTokenError, JwksSigningKeyProvider, TokenVerifier
from scratchplate_app.infrastructure.settings import Settings

LOGGER = logging.getLogger(__name__)


class _UnauthorizedResponse(Exception):
    pass


class _ForbiddenResponse(Exception):
    pass


def _register_security_exception_handlers(app: FastAPI) -> None:
    @app.exception_handler(_UnauthorizedResponse)
    async def handle_unauthorized(*_: Any) -> Response:
        return _unauthorized()

    @app.exception_handler(_ForbiddenResponse)
    async def handle_forbidden(*_: Any) -> Response:
        return _forbidden()



def _build_token_verifier(settings: Settings, token_verifier: TokenVerifier | None) -> TokenVerifier:
    if token_verifier is not None:
        return token_verifier
    return TokenVerifier(
        issuer=settings.jwt_issuer_uri,
        signing_key_provider=JwksSigningKeyProvider(settings.jwt_jwk_set_uri),
    )


def create_app(settings: Settings | None = None, token_verifier: TokenVerifier | None = None) -> FastAPI:
    resolved_settings = settings or Settings.from_env()
    verifier = _build_token_verifier(resolved_settings, token_verifier)
    greet_user = GreetUserService(ConfigurationWelcomeTemplateAdapter(resolved_settings))

    app = FastAPI(docs_url=None, redoc_url=None, openapi_url=None)
    register_exception_handlers(app)
    _register_security_exception_handlers(app)

    async def authenticated_request_dependency(
        authorization: str | None = Header(default=None),
    ) -> AuthenticatedRequest:
        try:
            return authenticate_request(verifier, authorization)
        except InvalidTokenError as exc:
            raise _UnauthorizedResponse() from exc

    async def authorized_user_dependency(authorization: str | None = Header(default=None)) -> UserIdentity:
        authenticated = await authenticated_request_dependency(authorization)
        if not authenticated.user.roles:
            raise _ForbiddenResponse()
        return authenticated.user

    if resolved_settings.management_server_port == resolved_settings.server_port:
        app.include_router(create_management_router())
    app.include_router(create_api_router(greet_user, authorized_user_dependency))
    app.include_router(create_denial_router(authenticated_request_dependency))
    return app


def create_management_app() -> FastAPI:
    app = FastAPI(docs_url=None, redoc_url=None, openapi_url=None)
    app.include_router(create_management_router())
    return app


def build_apps(settings: Settings | None = None, token_verifier: TokenVerifier | None = None) -> tuple[Settings, FastAPI, FastAPI | None]:
    resolved_settings = settings or Settings.from_env()
    verifier = _build_token_verifier(resolved_settings, token_verifier)
    main_app = create_app(resolved_settings, verifier)
    management_app = None
    if resolved_settings.management_server_port != resolved_settings.server_port:
        management_app = create_management_app()
    return resolved_settings, main_app, management_app


def _build_server(app: FastAPI, port: int) -> uvicorn.Server:
    config = uvicorn.Config(app=app, host="0.0.0.0", port=port, log_level="info", access_log=True)
    server = uvicorn.Server(config)
    server.install_signal_handlers = lambda: None
    return server


async def run_servers(settings: Settings | None = None) -> None:
    resolved_settings, main_app, management_app = build_apps(settings)

    logging.basicConfig(level=logging.INFO, stream=sys.stdout)
    servers = [_build_server(main_app, resolved_settings.server_port)]
    if management_app is not None:
        servers.append(_build_server(management_app, resolved_settings.management_server_port))

    stop_event = asyncio.Event()

    def request_shutdown() -> None:
        LOGGER.info("Shutdown signal received")
        for server in servers:
            server.should_exit = True
        stop_event.set()

    loop = asyncio.get_running_loop()
    for signame in (signal.SIGINT, signal.SIGTERM):
        try:
            loop.add_signal_handler(signame, request_shutdown)
        except NotImplementedError:
            signal.signal(signame, lambda *_: request_shutdown())

    tasks = [asyncio.create_task(server.serve()) for server in servers]
    wait_task = asyncio.create_task(stop_event.wait())
    waiters = tasks + [wait_task]
    done, pending = await asyncio.wait(waiters, return_when=asyncio.FIRST_COMPLETED)
    _ = done

    if not stop_event.is_set():
        request_shutdown()

    await asyncio.gather(*tasks, return_exceptions=True)

    for task in pending:
        task.cancel()
        with contextlib.suppress(asyncio.CancelledError):
            await task
