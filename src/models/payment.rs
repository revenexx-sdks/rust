use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "amount", default)]
    pub amount: f64,
    #[serde(rename = "authorized_at", default)]
    pub authorized_at: String,
    #[serde(rename = "captured_at", default)]
    pub captured_at: String,
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "error_message", default)]
    pub error_message: String,
    #[serde(rename = "failed_at", default)]
    pub failed_at: String,
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "idempotency_key", default)]
    pub idempotency_key: String,
    #[serde(rename = "kind", default)]
    pub kind: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "method_code", default)]
    pub method_code: String,
    #[serde(rename = "next_action", default)]
    pub next_action: serde_json::Value,
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    #[serde(rename = "provider", default)]
    pub provider: String,
    #[serde(rename = "psp_payment_id", default)]
    pub psp_payment_id: String,
    #[serde(rename = "refunded_at", default)]
    pub refunded_at: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
