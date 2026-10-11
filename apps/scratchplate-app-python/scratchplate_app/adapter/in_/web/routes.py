from __future__ import annotations

from collections.abc import Callable

from fastapi import APIRouter, Depends
from fastapi.responses import Response

from scratchplate_app.application.port_in import GreetUserUseCase
from scratchplate_app.domain import UserIdentity

from .auth import AuthenticatedRequest, _forbidden
from .models import WelcomeResponse


def create_api_router(
    greet_user: GreetUserUseCase,
    authorized_user_dependency: Callable[..., UserIdentity],
) -> APIRouter:
    router = APIRouter()

    @router.get("/api/v1/welcome", response_model=WelcomeResponse)
    async def welcome(user: UserIdentity = Depends(authorized_user_dependency)) -> WelcomeResponse:
        return WelcomeResponse.from_domain(greet_user.greet(user))

    return router


def create_management_router() -> APIRouter:
    router = APIRouter()

    @router.get("/actuator/health")
    @router.get("/actuator/health/liveness")
    @router.get("/actuator/health/readiness")
    async def health() -> dict[str, str]:
        return {"status": "UP"}

    @router.get("/actuator/info")
    async def info() -> dict[str, object]:
        return {}

    @router.api_route("/actuator/{path:path}", methods=["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"])
    async def deny_other_actuator_paths(path: str) -> Response:
        _ = path
        return _forbidden()

    return router


def create_denial_router(authenticated_request_dependency: Callable[..., AuthenticatedRequest]) -> APIRouter:
    router = APIRouter()

    @router.api_route("/{path:path}", methods=["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"])
    async def deny_other_paths(authenticated: AuthenticatedRequest = Depends(authenticated_request_dependency)) -> Response:
        _ = authenticated
        return _forbidden()

    return router
