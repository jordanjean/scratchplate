#!/bin/bash
# Usage: bash query-welcome.sh [username] [password]
#   bash query-welcome.sh admin admin   -> administrator welcome
#   bash query-welcome.sh user user     -> regular user welcome

set -euo pipefail

INGRESS_HOST=localhost
INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway \
  -o jsonpath='{.spec.ports[?(@.port==80)].nodePort}')
USERNAME=${1:-user}
PASSWORD=${2:-${USERNAME}}
TOKEN=$(bash "$(dirname "$0")/get-user-token.sh" "${USERNAME}" "${PASSWORD}")
URL="http://${INGRESS_HOST}:${INGRESS_PORT}/scratchplate/api/v1/welcome"

echo "GET ${URL} as '${USERNAME}'"
curl -sS "${URL}" -H "Authorization: Bearer ${TOKEN}" -w "\nHTTP %{http_code}\n"

