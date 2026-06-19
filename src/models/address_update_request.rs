use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddressUpdateRequest {
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "company", default)]
    pub company: String,
    /// Owning contact (personal address).
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 3166-1 alpha-2 code.
    #[serde(rename = "country", default)]
    pub country: String,
    /// The default address of its owner and type.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Recipient name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Owning organization (company address).
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
    /// Default 'shipping'.
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "zip", default)]
    pub zip: String,
}
