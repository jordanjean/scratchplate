#!/bin/bash
# Usage: bash query-welcome.sh [username] [password] [app]
#   bash query-welcome.sh admin admin          -> administrator welcome, all apps
#   bash query-welcome.sh user user rust       -> regular user welcome, Rust app only
# app: spring | rust | python | all (default)

set -euo pipefail

INGRESS_HOST=localhost
INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway \
  -o jsonpath='{.spec.ports[?(@.port==80)].nodePort}')
USERNAME=${1:-user}
PASSWORD=${2:-${USERNAME}}
TOKEN=$(bash "$(dirname "$0")/get-user-token.sh" "${USERNAME}" "${PASSWORD}")
APP=${3:-all}

case "${APP}" in
  spring) PREFIXES=(scratchplate) ;;
  rust) PREFIXES=(scratchplate-rust) ;;
  python) PREFIXES=(scratchplate-python) ;;
  all) PREFIXES=(scratchplate scratchplate-rust scratchplate-python) ;;
  *) echo "Unknown app '${APP}' (spring | rust | python | all)" >&2; exit 1 ;;
esac

for PREFIX in "${PREFIXES[@]}"; do
  URL="http://${INGRESS_HOST}:${INGRESS_PORT}/${PREFIX}/api/v1/welcome"
  echo "GET ${URL} as '${USERNAME}'"
  curl -sS "${URL}" -H "Authorization: Bearer ${TOKEN}" -w "\nHTTP %{http_code}\n"
done
