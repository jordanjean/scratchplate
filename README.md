# Scratchplate

A Kubernetes platform to experiment security features including

* Istio Service Mesh
* Request-Level Authentication
* Keycloak IDP
* Spring Security

Supported targets

* Kind

## Prerequisites

Required for deployment (Ansible playbook)

* [kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
* [istioctl](https://istio.io/latest/docs/ops/diagnostic-tools/istioctl)
* [helm](https://helm.sh/docs/intro/install/)
* [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)
* [ansible](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html)
* [curl](https://curl.se/download.html)

Optional (needed for token/query helper scripts)

* [jq](https://jqlang.org/download/)

## Installation

Preferred (single command)

```bash
cd infra/ansible
ansible-galaxy collection install -r collections/requirements.yml
ansible-playbook playbooks/deploy.yml
```

Individual stages can be run with tags, e.g. `ansible-playbook playbooks/deploy.yml --tags apps_deploy`.

## Scratchplate App

`apps/scratchplate-app` is a Spring Boot service (hexagonal architecture, Spring Security resource server). It greets the caller according to the Keycloak realm role in their token. See its [README](apps/scratchplate-app/README.md).

```bash
bash utils/query-welcome.sh admin admin   # {"message":"Welcome admin! You are signed in as an administrator: ...","role":"admin",...}
bash utils/query-welcome.sh user user     # {"message":"Welcome user! You are signed in as a regular user.","role":"user",...}
```

## Generic Helm chart

`apps/helm` is a generic chart (see its README); install the scratchplate app directly with:

```bash
helm upgrade --install scratchplate-app apps/helm -f apps/scratchplate-app/helm/values.yaml \
  --namespace scratchplate-app \
  --create-namespace
```

## Uninstallation

```bash
kind delete cluster --name scratchplate
```
