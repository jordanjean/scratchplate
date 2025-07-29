#!/bin/bash

set -euo pipefail

INGRESS_HOST=localhost
INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway \
  -o jsonpath='{.spec.ports[?(@.port==80)].nodePort}')
TOKEN=${1:-$(bash get-user-token.sh)}

echo
echo "Try to reach http://${INGRESS_HOST}:${INGRESS_PORT}/headers with bearer token: ${TOKEN}"
echo
RESPONSE=$(curl -sS http://"${INGRESS_HOST}":"${INGRESS_PORT}/headers" \
  -H "Authorization: Bearer ${TOKEN}")
echo "Response: ${RESPONSE}"