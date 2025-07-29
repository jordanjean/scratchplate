#!/bin/bash

set -euo pipefail

KEYCLOAK_URL=http://localhost:32000
ADMIN_USERNAME="admin"
ADMIN_PW="admin"

echo "🌐 Configure Keycloak realm via API..."
ACCESS_TOKEN=$(curl -s \
  -d "client_id=admin-cli" \
  -d "username=$ADMIN_USERNAME" \
  -d "password=$ADMIN_PW" \
  -d "grant_type=password" \
  "${KEYCLOAK_URL}/realms/master/protocol/openid-connect/token" \
  | jq -r .access_token)

curl -s -X POST "${KEYCLOAK_URL}/admin/realms" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "realm": "demo",
    "enabled": true
}'

echo "🌐 Create public client..."
curl -s -X POST "${KEYCLOAK_URL}/admin/realms/demo/clients" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "clientId": "demo-client",
    "publicClient": true,
    "redirectUris": ["*"],
    "protocol": "openid-connect",
    "enabled": true
}'

echo "👤 Create user in realm..."
curl -X POST http://localhost:32000/admin/realms/demo/users \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user",
    "email": "user@demo.net",
    "firstName": "user",
    "lastName": "user",
    "enabled": true,
    "credentials": [{
      "type": "password",
      "value": "user",
      "temporary": false
    }]
}'

echo "✅ Done."
