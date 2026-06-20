use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

pub mod etag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub key_server: String,
}

impl AuthConfig {
    pub fn new(access_secret: String, refresh_secret: String, key_server: String) -> Self {
        Self {
            access_secret,
            refresh_secret,
            key_server,
        }
    }
}

pub async fn jwt_auth(
    axum::extract::State(state): axum::extract::State<crate::config::AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    if path.starts_with("/v1/auth")
        || (method == axum::http::Method::GET
            && (path.starts_with("/v1/products")
                || path.starts_with("/v1/banners")
                || path.starts_with("/v1/articles")))
    {
        return Ok(next.run(req).await);
    }

    let token = crate::utils::helpers::extract_bearer_token(req.headers());
    if let Some(token) = token {
        let validation = Validation::default();
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(state.config.jwt_access_secret.as_bytes()),
            &validation,
        );
        if let Ok(token_data) = decoded {
            req.extensions_mut().insert(token_data.claims);
            return Ok(next.run(req).await);
        }
    }

    let refresh_token = req
        .headers()
        .get("x-refresh-token")
        .and_then(|v| v.to_str().ok());

    if let Some(refresh) = refresh_token {
        let validation = Validation::default();
        if let Ok(token_data) = decode::<Claims>(
            refresh,
            &DecodingKey::from_secret(state.config.jwt_refresh_secret.as_bytes()),
            &validation,
        ) {
            let new_claims = Claims {
                id: token_data.claims.id,
                email: token_data.claims.email,
                iat: chrono::Utc::now().timestamp() as usize,
                exp: (chrono::Utc::now() + chrono::Duration::hours(3)).timestamp() as usize,
            };

            let new_access = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &new_claims,
                &jsonwebtoken::EncodingKey::from_secret(state.config.jwt_access_secret.as_bytes()),
            )
            .map_err(|_| AppError::Internal(anyhow::anyhow!("token generation failed")))?;

            req.extensions_mut().insert(new_claims);
            let mut response = next.run(req).await;
            response.headers_mut().insert(
                "x-new-access-token"
                    .parse::<axum::http::HeaderName>()
                    .unwrap(),
                new_access.parse().unwrap(),
            );
            return Ok(response);
        }
    }

    Err(AppError::Unauthorized)
}
