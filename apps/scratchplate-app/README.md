# scratchplate-app

Spring Boot 4 / Java 25 service that greets the caller according to the Keycloak realm role (`admin` or `user`) carried in their JWT.

```
GET /api/v1/welcome   (Authorization: Bearer <token>)

{"message":"Welcome admin! You are signed in as an administrator: ...","username":"admin","role":"admin"}
```

## Architecture (hexagonal / ports & adapters)

```
io.scratchplate.app
├── domain                      # Pure business model, no framework
│   ├── model                   # Role, UserIdentity, WelcomeMessage
│   └── exception               # MissingRoleException
├── application                 # Use cases, no framework
│   ├── port.in                 # GreetUserUseCase        (driving port)
│   ├── port.out                # WelcomeTemplatePort     (driven port)
│   └── service                 # GreetUserService
├── adapter
│   ├── in.web                  # REST controller, JWT -> UserIdentity mapping, error handling
│   └── out.config              # Welcome templates from configuration properties
└── infrastructure              # Composition root
    ├── config                  # Wires the use cases as Spring beans
    └── security                # Spring Security resource server + Keycloak role converter
```

Boundaries are enforced by `ArchitectureTest` (ArchUnit): the domain and application layers can't depend on Spring, adapters, or infrastructure.

## Security

- Stateless OAuth2 resource server: Spring Security checks the JWT signature (JWKS), expiry, and issuer. Istio also checks the token at the mesh level, so tokens are verified twice.
- Keycloak `realm_access.roles` claims become `ROLE_*` authorities (`KeycloakRealmRoleConverter`).
- `/api/**` requires `ROLE_ADMIN` or `ROLE_USER`. Every other path is denied. Actuator only exposes `health` and `info`, on a separate management port.
- When a user has several roles, the most privileged one wins (`ADMIN` over `USER`).

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `JWT_ISSUER_URI` | `http://localhost:32000/realms/scratchplate` | Expected `iss` claim |
| `JWT_JWK_SET_URI` | `http://localhost:32000/realms/scratchplate/protocol/openid-connect/certs` | Signing keys endpoint |
| `MANAGEMENT_SERVER_PORT` | same as server | Actuator port (`8081` in Kubernetes) |
| `SCRATCHPLATE_WELCOME_ADMIN_TEMPLATE` / `..._USER_TEMPLATE` | see `application.yaml` | Message templates (`{username}` placeholder) |

## Build

The container image is built with [Jib](https://github.com/GoogleContainerTools/jib). There's no Dockerfile. The image uses the `eclipse-temurin:25-jre` base, runs as non-root UID `10001`, keeps dependencies, resources and classes in separate layers, and builds reproducibly.

```bash
./build-image.sh                               # builds scratchplate-app:0.1.0 into the local Docker daemon
./build-image.sh my-registry/scratchplate-app:dev
SKIP_TESTS=true ./build-image.sh
```

The script runs `./mvnw package jib:dockerBuild` when a local JDK 25 is available. Otherwise it runs `jib:buildTar` inside a `maven:3.9-eclipse-temurin-25` container and `docker load`s the result.

With a JDK you can also call Jib directly:

```bash
./mvnw verify                                  # tests only
./mvnw package jib:dockerBuild                 # image into local Docker
./mvnw package jib:build -Dimage.name=registry.example.com/scratchplate-app:0.1.0   # push to a registry, no Docker needed
```

The Ansible `apps_deploy` role runs `build-image.sh`, then `kind load docker-image`, then deploys the generic chart in `apps/helm` with `apps/scratchplate-app/helm/values.yaml`.


