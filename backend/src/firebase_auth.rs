use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};

#[derive(Clone, Debug, Deserialize)]
pub struct FirebaseAuth {
    pub project_id: String,
}

impl FirebaseAuth {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
    pub async fn verify(&self, token: &str) -> Result<Claims> {
        let header = decode_header(token)?;
        tracing::info!("decoded header: {:?}", header);
        let kid = match header.kid {
            Some(kid) => kid,
            None => return Err(anyhow::anyhow!("kid not found")),
        };
        tracing::info!("kid: {}", kid);
        let res = reqwest::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com").await.map_err(|e| anyhow::anyhow!(e))?;
        tracing::info!("metadata response: {:?}", res);

        // https://github.com/firebase/firebase-admin-node/blob/bd8a11106c4af22d2fd46c84158d1a80e7d4828a/src/utils/jwt.ts#L257
        let keys = res.json::<HashMap<String, String>>().await?;
        tracing::info!("key keys: {:?}", keys);

        let key = keys
            .get(&kid)
            .ok_or(anyhow::anyhow!("public key not found"))?;

        let certificate = openssl::x509::X509::from_pem(key.as_bytes())?;
        let public_key_bytes = certificate.public_key()?.rsa()?.public_key_to_pem()?;
        let public_key = DecodingKey::from_rsa_pem(&public_key_bytes)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.project_id.clone()]);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]);

        let token =
            decode::<Claims>(token, &public_key, &validation).map_err(|e| anyhow::anyhow!(e))?;
        let timestamp = jsonwebtoken::get_current_timestamp();
        tracing::info!("timestamp: {}", timestamp);
        tracing::info!("token exp: {}", token.claims.exp);
        tracing::info!("token iat: {}", token.claims.iat);

        // exp check
        if token.claims.exp < timestamp {
            return Err(anyhow::anyhow!("token expired"));
        }

        // iat check
        if token.claims.iat > timestamp {
            return Err(anyhow::anyhow!("invalid iat"));
        }

        tracing::info!("token: {:?}", token);

        Ok(token.claims)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: Option<String>,
}
