from __future__ import annotations

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from scratchplate_app.domain import MissingRoleError


def register_exception_handlers(app: FastAPI) -> None:
    @app.exception_handler(MissingRoleError)
    async def handle_missing_role(_: Request, exc: MissingRoleError) -> JSONResponse:
        return JSONResponse(
            status_code=403,
            content={
                "type": "about:blank",
                "title": "Missing role",
                "status": 403,
                "detail": str(exc),
            },
            media_type="application/problem+json",
        )
