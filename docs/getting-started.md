# Getting started

## Prerequisites

Required for deployment (Ansible playbook)

* [kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
* [istioctl](https://istio.io/latest/docs/ops/diagnostic-tools/istioctl)
* [helm](https://helm.sh/docs/intro/install/)
* [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)
* [ansible](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html)
* [curl](https://curl.se/download.html)
* [Maven](https://maven.apache.org/install.html) and a JDK 25+ (Spring app)
* Docker (builds the Rust and Python images, and runs kind)

Optional

* [jq](https://jqlang.org/download/) (needed for the token/query helper scripts in `utils/`)
* Node.js 20+ (to preview the [architecture diagrams](architecture/README.md))

## Installation

```bash
cd infra/ansible
ansible-galaxy collection install -r collections/requirements.yml
ansible-playbook playbooks/deploy.yml
```

Individual stages can be run with tags: `prereqs`, `kind_cluster`, `istio`, `keycloak`, `keycloak_bootstrap`, `apps_deploy`, `validation`.

```bash
ansible-playbook playbooks/deploy.yml --tags apps_deploy
```

The applications to deploy are listed in the `apps` variable of `infra/ansible/group_vars/all.yml`.

## Try it

Three implementations of the same API are deployed, each in its own namespace and behind its own route:

| App | Route | Stack |
|---|---|---|
| `scratchplate-app` | `/scratchplate/` | Spring Boot |
| `scratchplate-app-rust` | `/scratchplate-rust/` | Rust, axum |
| `scratchplate-app-python` | `/scratchplate-python/` | Python, FastAPI |

```bash
bash utils/query-welcome.sh admin admin           # all apps, administrator
bash utils/query-welcome.sh user user rust        # regular user, Rust app only (spring | rust | python | all)
```

A call without a token returns `401`: the API requires a JWT issued by Keycloak (realm `scratchplate`).

## Uninstallation

```bash
kind delete cluster --name scratchplate
```
