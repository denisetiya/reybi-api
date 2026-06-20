use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use std::time::Duration;

use crate::config::AppState;
use crate::errors::{AppError, AppResult};
use crate::utils::cache::keys;

use super::dto::{CreateSnapRequest, MidtransWebhook};
use super::service::MidtransClient;

/// POST /v1/payments/snap  – create Snap transaction
pub async fn create_snap(
    State(state): State<AppState>,
    Json(req): Json<CreateSnapRequest>,
) -> AppResult<Json<Value>> {
    let client = MidtransClient::from_config(&state.config);
    let snap = client.create_snap(req).await?;
    Ok(Json(json!({ "data": snap })))
}

/// POST /v1/payments/midtrans/webhook  – Midtrans notification callback.
///
/// Idempotency: we SETEX `reybi:v1:webhook:idem:{transaction_id}` with 24h TTL
/// after a successful process.  Duplicate webhooks (Midtrans retries) hit the
/// early-return path and never re-process the same transaction.
pub async fn webhook(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> AppResult<Json<Value>> {
    let payload: MidtransWebhook = serde_json::from_slice(&body)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid webhook payload: {e}")))?;

    // ---- idempotency check ----
    let idem_key = keys::webhook_idem(&payload.transaction_id);
    if state.cache.get_raw(&idem_key).await.is_some() {
        tracing::info!(transaction_id = %payload.transaction_id, "↻ webhook duplicate — skipped");
        return Ok(Json(json!({ "ok": true, "duplicate": true })));
    }

    // ---- signature verification ----
    let status_code = payload.status_code.as_deref().unwrap_or("200");
    let ok = MidtransClient::verify_webhook(
        &state.config.midtrans_server_key,
        &payload.order_id,
        status_code,
        &payload.gross_amount,
        &payload.signature_key,
    );
    if !ok {
        return Err(AppError::Forbidden("invalid signature".into()));
    }

    let new_status =
        MidtransClient::map_status(&payload.transaction_status, payload.fraud_status.as_deref());

    sqlx::query(
        r#"
        INSERT INTO "PaymentHistory"
            (id, "orderId", method, type, amount, "midtransId", status, "createdAt", "updatedAt")
        VALUES
            (gen_random_uuid()::text, $1, $2, $3, $4, $5, $6, now(), now())
        ON CONFLICT ("midtransId") DO UPDATE
            SET status = EXCLUDED.status,
                "updatedAt" = now()
        "#,
    )
    .bind(&payload.order_id)
    .bind(&payload.payment_type)
    .bind(serde_json::Value::String(payload.payment_type.clone()))
    .bind(payload.gross_amount.parse::<f64>().unwrap_or(0.0))
    .bind(&payload.transaction_id)
    .bind(new_status)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    // mark idempotency key (24h — Midtrans retries within minutes, not days)
    state
        .cache
        .set_ex(&idem_key, "1", Duration::from_secs(86_400))
        .await;

    tracing::info!(
        order_id = %payload.order_id,
        status   = %new_status,
        "midtrans webhook processed"
    );

    Ok(Json(json!({ "ok": true })))
}
