# Generic application Helm chart

One chart for every app in `apps/`. It deploys a `ServiceAccount`, `Service`, hardened `Deployment`
(non-root, read-only root filesystem, probes), Istio `Gateway` + `VirtualService`, `RequestAuthentication`
and `AuthorizationPolicy` (JWT).

Resource names come from the **release name** (override with `fullnameOverride`), so the same chart can be
installed once per app. App-specific settings live in `apps/<app>/helm/values.yaml`
(image, ports, `env`, route prefix). `env` values are rendered with `tpl`, so they can reference other values
(e.g. `{{ .Values.security.requestAuthentication.issuer }}`).

## Install an app

```bash
helm upgrade --install scratchplate-app apps/helm \
  -f apps/scratchplate-app/helm/values.yaml \
  --namespace scratchplate-app --create-namespace
```

With kind, build and load the image first:

```bash
apps/scratchplate-app/build-image.sh
kind load docker-image scratchplate-app:0.1.0 --name scratchplate
```

## Add a new app

1. Create `apps/<app>/helm/values.yaml` with `image`, `service`, `env`, `istio.virtualService.pathPrefix`.
2. Install with `helm upgrade --install <app> apps/helm -f apps/<app>/helm/values.yaml -n <ns>`.

## Useful overrides

```bash
--set security.authorizationPolicy.adminOnly=true        # require the roles in allowedRoles
--set security.requestAuthentication.issuer=... --set security.requestAuthentication.jwksUri=...
--set probes.port=http                                   # probe the main port instead of the management one
```
