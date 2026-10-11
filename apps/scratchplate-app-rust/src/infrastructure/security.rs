use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub sub: String,
    pub preferred_username: Option<String>,
    pub exp: usize,
    #[serde(default)]
    pub realm_access: RealmAccessClaims,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RealmAccessClaims {
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug)]
pub enum AuthenticationError {
    InvalidToken(String),
    UnsupportedAlgorithm(String),
    MissingKeyId,
    KeyUnavailable(String),
    RemoteKeyFetch(String),
    InvalidKeyMaterial(String),
}

impl Display for AuthenticationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(message) => write!(formatter, "invalid token: {message}"),
            Self::UnsupportedAlgorithm(message) => write!(formatter, "unsupported algorithm: {message}"),
            Self::MissingKeyId => write!(formatter, "missing kid header"),
            Self::KeyUnavailable(message) => write!(formatter, "key unavailable: {message}"),
            Self::RemoteKeyFetch(message) => write!(formatter, "remote key fetch failed: {message}"),
            Self::InvalidKeyMaterial(message) => write!(formatter, "invalid key material: {message}"),
        }
    }
}

impl Error for AuthenticationError {}

#[async_trait]
pub trait KeySource: Send + Sync {
    fn algorithm(&self) -> Algorithm;
    async fn key_for(&self, header: &Header) -> Result<Arc<DecodingKey>, AuthenticationError>;
}

pub struct JwtVerifier {
    issuer: String,
    key_source: Arc<dyn KeySource>,
}

impl JwtVerifier {
    pub fn new(issuer: impl Into<String>, key_source: Arc<dyn KeySource>) -> Self {
        Self {
            issuer: issuer.into(),
            key_source,
        }
    }

    pub async fn verify(&self, token: &str) -> Result<JwtClaims, AuthenticationError> {
        let header = decode_header(token)
            .map_err(|error| AuthenticationError::InvalidToken(error.to_string()))?;

        if header.alg != self.key_source.algorithm() {
            return Err(AuthenticationError::UnsupportedAlgorithm(format!(
                "expected {:?}, got {:?}",
                self.key_source.algorithm(),
                header.alg
            )));
        }

        let key = self.key_source.key_for(&header).await?;
        let mut validation = Validation::new(self.key_source.algorithm());
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.validate_aud = false;
        validation.validate_exp = true;
        validation.leeway = 0;

        let token = decode::<JwtClaims>(token, &key, &validation)
            .map_err(|error| AuthenticationError::InvalidToken(error.to_string()))?;

        Ok(token.claims)
    }
}

pub struct StaticKeySource {
    algorithm: Algorithm,
    key: Arc<DecodingKey>,
}

impl StaticKeySource {
    pub fn new_hs256(secret: &[u8]) -> Self {
        Self {
            algorithm: Algorithm::HS256,
            key: Arc::new(DecodingKey::from_secret(secret)),
        }
    }
}

#[async_trait]
impl KeySource for StaticKeySource {
    fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    async fn key_for(&self, header: &Header) -> Result<Arc<DecodingKey>, AuthenticationError> {
        if header.alg != self.algorithm {
            return Err(AuthenticationError::UnsupportedAlgorithm(format!(
                "expected {:?}, got {:?}",
                self.algorithm, header.alg
            )));
        }
        Ok(self.key.clone())
    }
}

pub struct JwksKeySource {
    client: reqwest::Client,
    jwk_set_uri: String,
    min_refresh_interval: Duration,
    cache: RwLock<JwksCache>,
}

struct JwksCache {
    keys: HashMap<String, Arc<DecodingKey>>,
    last_refresh_attempt: Option<Instant>,
}

impl JwksKeySource {
    pub fn new(client: reqwest::Client, jwk_set_uri: impl Into<String>, min_refresh_interval: Duration) -> Self {
        Self {
            client,
            jwk_set_uri: jwk_set_uri.into(),
            min_refresh_interval,
            cache: RwLock::new(JwksCache {
                keys: HashMap::new(),
                last_refresh_attempt: None,
            }),
        }
    }

    async fn refresh(&self) -> Result<(), AuthenticationError> {
        {
            let mut cache = self.cache.write().await;
            cache.last_refresh_attempt = Some(Instant::now());
        }

        let response = self
            .client
            .get(&self.jwk_set_uri)
            .send()
            .await
            .map_err(|error| AuthenticationError::RemoteKeyFetch(error.to_string()))?
            .error_for_status()
            .map_err(|error| AuthenticationError::RemoteKeyFetch(error.to_string()))?;

        let jwks = response
            .json::<JwkSet>()
            .await
            .map_err(|error| AuthenticationError::RemoteKeyFetch(error.to_string()))?;

        let mut keys = HashMap::new();
        for jwk in jwks.keys {
            if jwk.kty != "RSA" {
                continue;
            }

            let kid = jwk.kid.ok_or_else(|| {
                AuthenticationError::InvalidKeyMaterial("RSA key missing kid".to_owned())
            })?;
            let modulus = jwk.n.ok_or_else(|| {
                AuthenticationError::InvalidKeyMaterial(format!("RSA key '{kid}' missing modulus"))
            })?;
            let exponent = jwk.e.ok_or_else(|| {
                AuthenticationError::InvalidKeyMaterial(format!("RSA key '{kid}' missing exponent"))
            })?;

            let key = DecodingKey::from_rsa_components(&modulus, &exponent)
                .map_err(|error| AuthenticationError::InvalidKeyMaterial(error.to_string()))?;
            keys.insert(kid, Arc::new(key));
        }

        let mut cache = self.cache.write().await;
        cache.keys = keys;
        cache.last_refresh_attempt = Some(Instant::now());
        Ok(())
    }
}

#[async_trait]
impl KeySource for JwksKeySource {
    fn algorithm(&self) -> Algorithm {
        Algorithm::RS256
    }

    async fn key_for(&self, header: &Header) -> Result<Arc<DecodingKey>, AuthenticationError> {
        if header.alg != Algorithm::RS256 {
            return Err(AuthenticationError::UnsupportedAlgorithm(format!(
                "expected {:?}, got {:?}",
                Algorithm::RS256,
                header.alg
            )));
        }

        let kid = header.kid.clone().ok_or(AuthenticationError::MissingKeyId)?;

        {
            let cache = self.cache.read().await;
            if let Some(key) = cache.keys.get(&kid) {
                return Ok(key.clone());
            }
        }

        let should_refresh = {
            let cache = self.cache.read().await;
            cache
                .last_refresh_attempt
                .is_none_or(|last| last.elapsed() >= self.min_refresh_interval)
        };

        if should_refresh {
            self.refresh().await?;
            let cache = self.cache.read().await;
            if let Some(key) = cache.keys.get(&kid) {
                return Ok(key.clone());
            }
        }

        Err(AuthenticationError::KeyUnavailable(format!(
            "no RSA key found for kid '{kid}'"
        )))
    }
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kty: String,
    kid: Option<String>,
    n: Option<String>,
    e: Option<String>,
}
