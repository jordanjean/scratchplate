#!/bin/bash

set -euo pipefail

KEYCLOAK_URL=http://localhost:32000
USERNAME=${1:-user}
PASSWORD=${2:-user}

#echo "Get user JWT token from: ${KEYCLOAK_URL}/realms/demo/protocol/openid-connect/token"
curl -X POST -sS ${KEYCLOAK_URL}/realms/demo/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=demo-client" \
  -d "grant_type=password" \
  -d "username=${USERNAME}" \
  -d "password=${PASSWORD}" \
  | jq -r .access_token