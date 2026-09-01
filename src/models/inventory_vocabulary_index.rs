use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryVocabularyIndex {
    /// This app's name — the part before the dot in a qualified vocabulary id
    /// such as `inventories.movement-types`.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, WITHOUT its values — the index a
    /// client reads to discover them. Fetch the values with GET
    /// /inventories/vocabularies/{name}.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<serde_json::Value>,
}
