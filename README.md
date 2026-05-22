# Scratchplate

A Kubernetes platform to experiment security features including

* Istio Service Mesh
* Request-Level Authentication
* Keycloak IDP

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
cd ansible
ansible-galaxy collection install -r collections/requirements.yml
ansible-playbook playbooks/deploy.yml
```

Legacy step-by-step wrappers (now backed by Ansible)

* `1-create-kind-cluster.sh`
* `2-create-keycloak-realm.sh`
* `3-deploy-apps.sh`

## Uninstallation

```bash
kind delete cluster --name scratchplate
```
