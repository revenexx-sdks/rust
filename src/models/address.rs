use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "company", default)]
    pub company: String,
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "region", default)]
    pub region: String,
    #[serde(rename = "street", default)]
    pub street: String,
    #[serde(rename = "street2", default)]
    pub street2: String,
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "zip", default)]
    pub zip: String,
}
