# scratchplate-app-python

FastAPI / Python 3.13 re-implementation of `apps/scratchplate-app` with the same API, JWT validation rules, environment variables, actuator-compatible probes, and hexagonal architecture.

```
GET /api/v1/welcome   (Authorization: Bearer <JWT>)

{"message":"Welcome admin! You are signed in as an administrator: ...","username":"admin","role":"admin"}
```

## Architecture (hexagonal / ports & adapters)

```
scratchplate_app
├── domain
│   ├── missing_role_error.py
│   ├── role.py
│   ├── user_identity.py
│   └── welcome_message.py
├── application
│   ├── port_in/greet_user.py
│   ├── port_out/welcome_template.py
│   └── service/greet_user_service.py
├── adapter
│   ├── in_/web
│   │   ├── auth.py
│   │   ├── exception_handlers.py
│   │   ├── models.py
│   │   └── routes.py
│   └── out/config/welcome_templates.py
└── infrastructure
    ├── app_factory.py
    ├── main.py
    ├── security.py
    └── settings.py
```

The domain and application layers stay framework-free. `tests/unit/test_architecture.py` enforces that they do not import FastAPI, JWT libraries, adapters, or infrastructure code.

## Security

- Stateless bearer-token validation inside the app, even though Istio also validates JWTs.
- Signature verification uses RS256 keys from `JWT_JWK_SET_URI`, cached in-process and refreshed when an unknown `kid` appears (rate-limited to one refresh window per 30 seconds).
- The app validates `exp` and `iss` (`JWT_ISSUER_URI`) and intentionally does **not** validate `aud`.
- Keycloak realm roles come from `realm_access.roles`, are mapped case-insensitively to `admin` / `user`, and the most privileged role wins.
- `/api/**` requires a valid bearer token; `/api/v1/welcome` also requires a supported role. Other main-app paths return `401` for anonymous callers and `403` for authenticated callers. Actuator exposes only `health` and `info`.
- `/docs`, `/redoc`, and `/openapi.json` are disabled.

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `SERVER_PORT` | `8080` | Main HTTP listener |
| `MANAGEMENT_SERVER_PORT` | same as `SERVER_PORT` | Actuator listener; when different, a second Uvicorn server starts in the same process |
| `JWT_ISSUER_URI` | `http://localhost:32000/realms/scratchplate` | Expected `iss` claim |
| `JWT_JWK_SET_URI` | `http://localhost:32000/realms/scratchplate/protocol/openid-connect/certs` | RS256 JWKS endpoint |
| `SCRATCHPLATE_WELCOME_ADMIN_TEMPLATE` | `Welcome {username}! You are signed in as an administrator: you have full access to the platform.` | Admin welcome template |
| `SCRATCHPLATE_WELCOME_USER_TEMPLATE` | `Welcome {username}! You are signed in as a regular user.` | User welcome template |

## API and actuator

- `GET /api/v1/welcome` with `Authorization: Bearer <JWT>` returns `200` and a JSON body `{message, username, role}`.
- Missing or invalid tokens return `401` with `WWW-Authenticate: Bearer` and an empty body.
- Authenticated callers without a supported role receive `403` with an empty body.
- `MissingRoleError` is mapped to RFC 9457 `application/problem+json` with title `Missing role`.
- Actuator-compatible endpoints:
  - `GET /actuator/health`
  - `GET /actuator/health/liveness`
  - `GET /actuator/health/readiness`
  - `GET /actuator/info`

## Build

```bash
cd apps/scratchplate-app-python
./build-image.sh
./build-image.sh my-registry/scratchplate-app-python:dev
SKIP_TESTS=true ./build-image.sh
```

- `Dockerfile` is multi-stage (`builder` → `test` → `final`) on `python:3.13-slim`.
- The final image runs as numeric non-root UID `10001`, sets `PYTHONDONTWRITEBYTECODE=1` and `PYTHONUNBUFFERED=1`, exposes ports `8080` and `8081`, and works with a read-only root filesystem.
- The script reads the version from `apps/scratchplate-app-python/VERSION`, runs the `test` stage unless `SKIP_TESTS=true`, then builds the final image into the local Docker daemon.
