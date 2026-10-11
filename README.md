# Scratchplate

A Kubernetes platform to experiment security features including

* Istio Service Mesh
* Request-Level Authentication
* Keycloak IDP
* Spring Security

Supported targets

* Kind

## Quick start

```bash
cd infra/ansible
ansible-galaxy collection install -r collections/requirements.yml
ansible-playbook playbooks/deploy.yml
bash ../../utils/query-welcome.sh admin admin
```

## Repository layout

| Path | Content |
|---|---|
| `infra/` | Ansible and everything related to the Kubernetes cluster (kind, Istio, Keycloak) |
| `apps/` | The same app in Spring (`scratchplate-app`), Rust (`scratchplate-app-rust`) and Python (`scratchplate-app-python`), and the generic Helm chart (`apps/helm`) |
| `utils/` | Helper scripts for tests (token, queries) |
| `docs/` | All the documentation, including the LikeC4 architecture model |

## Documentation

See [docs/](docs/README.md): [getting started](docs/getting-started.md), [architecture](docs/architecture/README.md), [scratchplate-app](docs/apps/scratchplate-app.md) ([Rust](docs/apps/scratchplate-app-rust.md), [Python](docs/apps/scratchplate-app-python.md)), [Helm chart](docs/helm-chart.md).
