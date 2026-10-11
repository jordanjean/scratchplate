# Getting started

## Prerequisites

Required for deployment (Ansible playbook)

* [kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
* [istioctl](https://istio.io/latest/docs/ops/diagnostic-tools/istioctl)
* [helm](https://helm.sh/docs/intro/install/)
* [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)
* [ansible](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html)
* [curl](https://curl.se/download.html)
* [Maven](https://maven.apache.org/install.html) and a JDK 25+
* Docker

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

## Try it

```bash
bash utils/query-welcome.sh admin admin   # {"message":"Welcome admin! You are signed in as an administrator: ...","role":"admin",...}
bash utils/query-welcome.sh user user     # {"message":"Welcome user! You are signed in as a regular user.","role":"user",...}
```

A call without a token returns `401`: the API requires a JWT issued by Keycloak (realm `scratchplate`).

## Uninstallation

```bash
kind delete cluster --name scratchplate
```
