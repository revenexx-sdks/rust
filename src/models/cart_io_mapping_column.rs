use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartIoMappingColumn {
    /// The cart or line field, spelled as this app spells it — one of the
    /// canonical column names.
    #[serde(rename = "from", default)]
    pub from: String,
    /// What that field is called on the outside: the CSV header, or the JSON key
    /// of the system on the other end.
    #[serde(rename = "to", default)]
    pub to: String,
}
