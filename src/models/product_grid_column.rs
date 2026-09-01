use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductGridColumn {
    /// The key to read out of a row: a column name for the fixed columns, an
    /// attribute code for the rest (then it is a key of the row's `attributes`
    /// object).
    #[serde(rename = "code", default)]
    pub code: String,
    /// The attribute's i18n labels, or a plain title for the fixed columns.
    #[serde(rename = "label", default)]
    pub label: serde_json::Value,
    /// Where the value comes from: 'column' is a plain products column,
    /// 'attribute' a key inside `attribute_values`, 'resolved' something this
    /// route computed (the display name).
    #[serde(rename = "source", default)]
    pub source: String,
    /// Which control renders the cell — the same widget vocabulary `GET
    /// /products/attribute-schema` uses, so one renderer serves both.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
