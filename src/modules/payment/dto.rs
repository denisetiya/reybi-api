use serde::{Deserialize, Serialize};
use validator::Validate;

/// Body for creating a Snap transaction (QRIS / VA / etc.).
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSnapRequest {
    /// Your internal order id (must be unique per attempt).
    pub order_id: String,
    pub gross_amount: i64,
    /// Optional: "qris" | "bank_transfer" | "gopay" | etc.
    #[serde(default)]
    pub payment_type: Option<String>,
    /// Optional expiry for the Snap token.
    #[serde(default)]
    pub expiry_minutes: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SnapResponse {
    pub token: String,
    pub redirect_url: String,
}

/// Midtrans webhook payload (subset we care about).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MidtransWebhook {
    pub transaction_id: String,
    pub order_id: String,
    pub gross_amount: String,
    pub payment_type: String,
    pub transaction_time: String,
    pub transaction_status: String,
    pub fraud_status: Option<String>,
    /// SHA-512(order_id+status_code+gross_amount+server_key)
    pub signature_key: String,
    #[serde(default)]
    pub status_code: Option<String>,
}
