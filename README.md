# Scratchplate

A Kubernetes platform to experiment security features including

* Istio Service Mesh
* Request-Level Authentication

## Prerequisites

* [kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
* [istioctl](https://istio.io/latest/docs/ops/diagnostic-tools/istioctl)
* [helm](https://helm.sh/docs/intro/install/)
* [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)

## Installation

Run the scripts

* `1-create-kind-cluster.sh`
* `2-create-keycloak-realm.sh`
* `3-deploy-apps.sh`

## Uninstallation

```bash
kind delete cluster --name scratchplate
```