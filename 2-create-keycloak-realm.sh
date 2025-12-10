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

echo "🌐 Create realm role admin..."
curl -s -X POST "${KEYCLOAK_URL}/admin/realms/demo/roles" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "admin",
    "description": "Administrator role for the demo realm"
}'

echo "👤 Create admin user in realm..."
USER_ID=$(curl -s -X POST "http://localhost:32000/admin/realms/demo/users" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "email": "admin@demo.net",
    "firstName": "Admin",
    "lastName": "User",
    "enabled": true,
    "credentials": [{
      "type": "password",
      "value": "admin",
      "temporary": false
    }]
  }' \
  -D - | grep -i Location | awk -F '/' '{print $NF}' | tr -d '\r')

echo "🔍 Retrieve role 'admin'..."
ROLE=$(curl -s -X GET "${KEYCLOAK_URL}/admin/realms/demo/roles/admin" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "🔑 Assign realm role 'admin' to user..."
curl -s -X POST "http://localhost:32000/admin/realms/demo/users/$USER_ID/role-mappings/realm" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d "[$ROLE]"

echo "✅ Done."
