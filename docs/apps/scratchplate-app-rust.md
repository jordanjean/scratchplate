# scratchplate-app-rust

Rust / Axum re-implementation of `scratchplate-app`, preserving the Spring Boot service's API, configuration contract, role handling, JWT validation, and actuator-compatible management endpoints.

```
GET /api/v1/welcome   (Authorization: Bearer <JWT>)

{"message":"Welcome admin! You are signed in as an administrator: ...","username":"admin","role":"admin"}
```

## Architecture (hexagonal / ports & adapters)

```
src
├── domain                          # Pure business model, framework-free
│   ├── role.rs                     # Role (admin/user, privilege ordering)
│   ├── user_identity.rs            # Authenticated caller in domain terms
│   ├── welcome_message.rs          # Personalised welcome response model
│   └── missing_role_error.rs       # Domain error for unsupported roles
├── application                     # Use cases and ports, framework-free
│   ├── port_in.rs                  # GreetUserUseCase
│   ├── port_out.rs                 # WelcomeTemplatePort
│   └── service.rs                  # GreetUserService
├── adapter
│   ├── inbound/web.rs              # Axum handlers, JWT claims -> UserIdentity,
│   │                               # auth/authorization/error HTTP mapping
│   └── outbound/config.rs          # Welcome templates from environment config
├── infrastructure
│   ├── config.rs                   # Environment configuration
│   ├── security.rs                 # JWT verifier + KeySource + JWKS cache
│   └── app.rs                      # Composition root / listener startup
├── lib.rs                          # Library crate exposing routers/runtime
└── main.rs                         # Thin binary entry point
```

The domain and application layers stay independent of Axum, Tokio, Reqwest, and JSON Web Token handling. Infrastructure wires the concrete adapters together at startup.

## API and behaviour

- `GET /api/v1/welcome`
  - Requires `Authorization: Bearer <JWT>`.
  - Returns `200 OK` with JSON fields `message`, `username`, `role`.
  - Roles come from `realm_access.roles` (case-insensitive); `admin` wins over `user` when both are present.
  - Username comes from `preferred_username`, falling back to `sub` when missing/blank.
- Invalid or missing bearer token returns `401` with empty body and `WWW-Authenticate: Bearer`.
- Authenticated callers without a supported `admin`/`user` role get `403` with empty body.
- Non-`/api/**` paths are denied (`401` anonymous, `403` authenticated), except actuator endpoints.
- Domain `MissingRoleError` maps to `403 application/problem+json` with title `Missing role`.

## Security

- Stateless JWT validation inside the service (in addition to Istio):
  - signature via JWKS (`RS256` in production),
  - `exp` validation,
  - `iss == JWT_ISSUER_URI`,
  - no audience check.
- Production key lookup uses a `KeySource` backed by JWKS fetched from `JWT_JWK_SET_URI`.
- JWKS keys are cached in-memory and re-fetched when an unknown `kid` appears, with a refresh rate limit.
- Tests use a separate static `HS256` key source; no private RSA keys are stored in the repository.
- Logs are written to stdout via `tracing`.
- The runtime handles `SIGTERM` for graceful shutdown.

## Actuator-compatible endpoints

- `/actuator/health` -> `{"status":"UP"}`
- `/actuator/health/liveness` -> `{"status":"UP"}`
- `/actuator/health/readiness` -> `{"status":"UP"}`
- `/actuator/info` -> `{}`
- Other `/actuator/**` paths are denied.
- When `MANAGEMENT_SERVER_PORT == SERVER_PORT`, actuator endpoints are served on the main listener.
- When the ports differ, a dedicated management listener serves actuator only.

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `SERVER_PORT` | `8080` | Main HTTP listener |
| `MANAGEMENT_SERVER_PORT` | same as server | Actuator listener; set to `8081` in Kubernetes |
| `JWT_ISSUER_URI` | `http://localhost:32000/realms/scratchplate` | Expected `iss` claim |
| `JWT_JWK_SET_URI` | `http://localhost:32000/realms/scratchplate/protocol/openid-connect/certs` | JWKS endpoint used for `RS256` verification |
| `SCRATCHPLATE_WELCOME_ADMIN_TEMPLATE` | `Welcome {username}! You are signed in as an administrator: you have full access to the platform.` | Admin greeting template |
| `SCRATCHPLATE_WELCOME_USER_TEMPLATE` | `Welcome {username}! You are signed in as a regular user.` | User greeting template |

## Build and image

A multi-stage Dockerfile builds, tests, and packages the service.

```bash
cd apps/scratchplate-app-rust
./build-image.sh
./build-image.sh my-registry/scratchplate-app-rust:dev
SKIP_TESTS=true ./build-image.sh
```

`build-image.sh`:
- runs `docker build --target test` first unless `SKIP_TESTS=true`,
- then builds the final runtime image into the local Docker daemon,
- defaults to image tag `scratchplate-app-rust:0.1.0`.

Image characteristics:
- Rust dependencies are cached in an intermediate build stage,
- final image uses `gcr.io/distroless/cc-debian12:nonroot`,
- runs as numeric UID/GID `10001`,
- compatible with `readOnlyRootFilesystem: true`,
- exposes `8080` and `8081`.

## Helm values

`apps/scratchplate-app-rust/helm/values.yaml` is modelled on the Spring app's values file and targets the generic chart in `apps/helm`.

Notable values:
- `image.repository: scratchplate-app-rust`
- `image.tag: "0.1.0"`
- `service.targetPort: 8080`
- `managementPort: 8081`
- same JWT env vars as the Spring app
- `istio.virtualService.pathPrefix: /scratchplate-rust/`
