#!/bin/bash
set -euo pipefail

# ========= CONFIG =========
KEYCLOAK_URL=http://localhost:32000
KEYCLOAK_REALM="demo"
KEYCLOAK_ADMIN_USER="admin"
KEYCLOAK_ADMIN_PASS="admin"

CLIENT_ID="oauth2-proxy"
APP_DOMAIN="localhost"
REDIRECT_URI="https://${APP_DOMAIN}/oauth2/callback"

NAMESPACE="oauth2-proxy"
OAUTH2_PROXY_VERSION="7.6.0"

# ========= OBTAIN ADMIN TOKEN =========
echo "🔑 Getting Keycloak admin token..."
KC_TOKEN=$(curl -s \
  -d "client_id=admin-cli" \
  -d "username=${KEYCLOAK_ADMIN_USER}" \
  -d "password=${KEYCLOAK_ADMIN_PASS}" \
  -d "grant_type=password" \
  "${KEYCLOAK_URL}/realms/master/protocol/openid-connect/token" | jq -r .access_token)

if [[ "$KC_TOKEN" == "null" || -z "$KC_TOKEN" ]]; then
  echo "❌ Failed to get admin token. Check credentials."
  exit 1
fi

# ========= CHECK IF CLIENT EXISTS =========
CLIENT_EXISTS=$(curl -s -H "Authorization: Bearer ${KC_TOKEN}" \
  "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/clients?clientId=${CLIENT_ID}" | jq 'length')

if [[ "$CLIENT_EXISTS" -eq 0 ]]; then
  echo "🚀 Creating Keycloak client '${CLIENT_ID}'..."
  curl -s -X POST "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/clients" \
    -H "Authorization: Bearer ${KC_TOKEN}" \
    -H "Content-Type: application/json" \
    -d "{
      \"clientId\": \"${CLIENT_ID}\",
      \"enabled\": true,
      \"protocol\": \"openid-connect\",
      \"publicClient\": false,
      \"redirectUris\": [\"${REDIRECT_URI}\"],
      \"baseUrl\": \"https://${APP_DOMAIN}\",
      \"adminUrl\": \"https://${APP_DOMAIN}\",
      \"webOrigins\": [\"*\"],
      \"directAccessGrantsEnabled\": true,
      \"serviceAccountsEnabled\": true,
      \"standardFlowEnabled\": true,
      \"attributes\": {
        \"post.logout.redirect.uris\": \"${APP_DOMAIN}\"
      }
    }"
else
  echo "ℹ️ Keycloak client '${CLIENT_ID}' already exists."
fi

# ========= GET CLIENT UUID =========
CLIENT_UUID=$(curl -s -H "Authorization: Bearer ${KC_TOKEN}" \
  "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/clients?clientId=${CLIENT_ID}" | jq -r '.[0].id')

# ========= GET CLIENT SECRET =========
CLIENT_SECRET=$(curl -s -H "Authorization: Bearer ${KC_TOKEN}" \
  "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/clients/${CLIENT_UUID}/client-secret" | jq -r .value)

if [[ -z "$CLIENT_SECRET" || "$CLIENT_SECRET" == "null" ]]; then
  echo "❌ Failed to get client secret."
  exit 1
fi

echo "✅ Keycloak client '${CLIENT_ID}' created/verified."
echo "🔐 Client Secret: ${CLIENT_SECRET}"

# ========= INSTALL OAUTH2 PROXY =========
COOKIE_SECRET=$(head -c 16 /dev/urandom | base64 | tr -d '=+/')

kubectl create namespace $NAMESPACE || true

helm repo add oauth2-proxy https://oauth2-proxy.github.io/manifests
helm repo update

echo "🚀 Installing OAuth2 Proxy..."
helm upgrade --install oauth2-proxy oauth2-proxy/oauth2-proxy \
  --namespace $NAMESPACE \
  --version $OAUTH2_PROXY_VERSION \
  --set config.clientID="$CLIENT_ID" \
  --set config.clientSecret="$CLIENT_SECRET" \
  --set config.cookieSecret="$COOKIE_SECRET" \
  --set config.provider="oidc" \
  --set config.oidcIssuerURL="${KEYCLOAK_URL}/realms/${KEYCLOAK_REALM}" \
  --set config.redirectURL="$REDIRECT_URI" \
  --set config.emailDomains="*" \
  --set ingress.enabled=false \
  --set service.type=ClusterIP \
  --set extraArgs.provider-display-name="Keycloak" \
  --set extraArgs.scope="openid email profile" \
  --set extraArgs.set-authorization-header=true \
  --set extraArgs.pass-authorization-header=true

# ========= ISTIO CONFIGURATION =========
cat <<EOF | kubectl apply -f -
apiVersion: networking.istio.io/v1beta1
kind: Gateway
metadata:
  name: app-gateway
  namespace: $NAMESPACE
spec:
  selector:
    istio: ingressgateway
  servers:
  - port:
      number: 443
      name: https
      protocol: HTTPS
    tls:
      mode: SIMPLE
      credentialName: app-tls-secret
    hosts:
    - "*"
---
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: app-virtualservice
  namespace: $NAMESPACE
spec:
  hosts:
  - "$APP_DOMAIN"
  gateways:
  - app-gateway
  http:
  - match:
    - uri:
        prefix: /oauth2/
    route:
    - destination:
        host: oauth2-proxy.$NAMESPACE.svc.cluster.local
        port:
          number: 4180
EOF

echo "🎉 Deployment complete!"
echo "Visit https://${APP_DOMAIN} and login via Keycloak."
