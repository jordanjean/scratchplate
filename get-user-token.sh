#!/bin/bash

set -euo pipefail

KEYCLOAK_URL=http://localhost:32000

#echo "Get user JWT token from: ${KEYCLOAK_URL}/realms/demo/protocol/openid-connect/token"
curl -X POST -sS ${KEYCLOAK_URL}/realms/demo/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=demo-client" \
  -d "grant_type=password" \
  -d "username=user" \
  -d "password=user" \
  | jq -r .access_token