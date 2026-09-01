use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Suppression {
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "address_hash", default)]
    pub address_hash: String,
    #[serde(rename = "channel", default)]
    pub channel: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "note", default)]
    pub note: String,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "scope", default)]
    pub scope: String,
    #[serde(rename = "source", default)]
    pub source: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
