#!/bin/bash

set -euo pipefail

CLUSTER_NAME="scratchplate"
KEYCLOAK_NS="keycloak"

echo "🌐 Create Kind cluster..."
cat <<EOF | kind create cluster --name ${CLUSTER_NAME} --config=-
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 32000
    hostPort: 32000
  - containerPort: 30080
    hostPort: 30080
  - containerPort: 30443
    hostPort: 30443
EOF

echo "🌐 Deploy Istio..."
istioctl install \
  --set profile=demo \
  --set values.gateways.istio-ingressgateway.type=NodePort \
  -f resources/istio-ingressgateway-nodeport.yaml -y
kubectl label namespace default istio-injection=enabled

kubectl create namespace $KEYCLOAK_NS
echo "🌐 Deploy Keycloak 26.3.1..."
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm install keycloak bitnami/keycloak \
  --version=24.7.7 \
  --namespace=${KEYCLOAK_NS} \
  --set auth.adminUser=admin \
  --set auth.adminPassword=admin \
  --set service.type=NodePort \
  --set service.nodePorts.http=32000

echo "⏳ Waiting for Keycloak to be ready..."
kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=keycloak -n $KEYCLOAK_NS --timeout=240s
echo "✅ Done. Keycloak available at http://localhost:32000"
