#!/bin/bash

set -euo pipefail

APP1_NS="httpbin"
APP2_NS="demo-app"
INGRESS_HOST="localhost"

INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway \
  -o jsonpath='{.spec.ports[?(@.port==80)].nodePort}')

echo "🌐 Deploy httpbin app..."
kubectl create namespace $APP1_NS
kubectl label namespace $APP1_NS istio-injection=enabled
kubectl apply -n $APP1_NS -f resources/istio-1.26.2/samples/httpbin/httpbin.yaml
echo "🌐 Create Istio Gateway and VirtualService..."
kubectl apply -n $APP1_NS -f resources/istio-1.26.2/samples/httpbin/httpbin-gateway.yaml
echo "🌐 Setup Istio request authentication..."
kubectl apply -f resources/request-authentication.yaml
kubectl apply -f resources/authorization-policy.yaml

echo "🌐 Deploy demo app..."
kubectl create namespace $APP2_NS
kubectl label namespace $APP2_NS istio-injection=enabled
kubectl apply -n $APP2_NS -f resources/demo-app

kubectl wait --for=condition=ready pod -l app=httpbin -n $APP1_NS --timeout=180s
echo "✅ httpbin app available here: http://${INGRESS_HOST}:${INGRESS_PORT}/headers"
kubectl wait --for=condition=ready pod -l app=demo-app -n $APP2_NS --timeout=180s
echo "✅ Demo app available here: http://${INGRESS_HOST}:${INGRESS_PORT}/demo"