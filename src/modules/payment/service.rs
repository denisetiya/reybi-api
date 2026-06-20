//! Midtrans Snap API client + webhook signature verification.
//!
//! Sandbox base:  https://app.sandbox.midtrans.com
//! Production:    https://app.midtrans.com
//!
//! Snap endpoint: POST /snap/v1/transactions
//! Auth:          Basic base64(server_key:)

use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha512};

use crate::config::AppConfig;
use crate::errors::{AppError, AppResult};
use crate::modules::payment::dto::{CreateSnapRequest, SnapResponse};

#[derive(Clone)]
pub struct MidtransClient {
    http: Client,
    base: String,
    server_key: String,
}

impl MidtransClient {
    pub fn from_config(cfg: &AppConfig) -> Self {
        // default to sandbox unless MIDTRANS_ENV=production
        let sandbox = std::env::var("MIDTRANS_ENV")
            .map(|v| v != "production")
            .unwrap_or(true);
        let base = if sandbox {
            "https://app.sandbox.midtrans.com".to_string()
        } else {
            "https://app.midtrans.com".to_string()
        };
        Self {
            http: Client::new(),
            base,
            server_key: cfg.midtrans_server_key.clone(),
        }
    }

    pub async fn create_snap(&self, req: CreateSnapRequest) -> AppResult<SnapResponse> {
        let expiry = req.expiry_minutes.unwrap_or(60);
        let body = json!({
            "transaction_details": {
                "order_id": req.order_id,
                "gross_amount": req.gross_amount,
            },
            "credit_card": { "secure": true },
            "expiry": {
                "duration": expiry,
                "unit": "minute"
            },
        });

        let url = format!("{}/snap/v1/transactions", self.base);
        let token = general_purpose::STANDARD
            .encode(format!("{}:", self.server_key));

        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Basic {}", token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        if !resp.status().is_success() {
            let txt = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!(
                "midtrans snap error: {txt}"
            )));
        }

        let raw: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(SnapResponse {
            token: raw
                .get("token")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            redirect_url: raw
                .get("redirect_url")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        })
    }

    /// Verify Midtrans webhook signature:
    ///   SHA-512(order_id + status_code + gross_amount + server_key) hex == signature_key
    pub fn verify_webhook(
        server_key: &str,
        order_id: &str,
        status_code: &str,
        gross_amount: &str,
        signature_key: &str,
    ) -> bool {
        let payload = format!(
            "{}{}{}{}",
            order_id, status_code, gross_amount, server_key
        );
        let mut hasher = Sha512::new();
        hasher.update(payload.as_bytes());
        let got = hex::encode(hasher.finalize());
        got.eq_ignore_ascii_case(signature_key)
    }

    /// Map Midtrans transaction_status → our internal status.
    pub fn map_status(transaction_status: &str, fraud_status: Option<&str>) -> &'static str {
        match (transaction_status, fraud_status) {
            ("capture", Some("accept")) => "paid",
            ("settlement", _) => "paid",
            ("pending", _) => "pending",
            ("deny", _) | ("cancel", _) | ("expire", _) => "failed",
            _ => "pending",
        }
    }
}