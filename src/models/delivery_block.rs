use serde::{Deserialize, Serialize};

/// One block, ready to render: props resolved for the requested language,
/// library references already expanded, scheduled blocks already filtered out.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryBlock {
    /// The block type. This is what a theme switches its component on.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// Nested blocks keyed by the field they sit in — `{ "columns": [...] }`.
    /// Empty object on a leaf block.
    #[serde(rename = "children", default)]
    pub children: serde_json::Value,
    /// The theme fragment to render instead of a props-driven component.
    /// Theme-defined, like a bundle.
    #[serde(rename = "fragmentName", default)]
    pub fragment_name: String,
    /// The library item this block came from, or `null`. Its content is already
    /// inlined above — this is for cache invalidation and editor links, not for
    /// a second fetch.
    #[serde(rename = "libraryItemId", default)]
    pub library_item_id: String,
    /// Display options for this block, as a flat `option key → value` map.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// The block's field values for the requested language, source values already
    /// overlaid with that language's overrides. Theme-defined keys.
    #[serde(rename = "props", default)]
    pub props: serde_json::Value,
    /// The block uuid — stable across publishes, so it is safe to use as a
    /// render key or an anchor.
    #[serde(rename = "uuid", default)]
    pub uuid: String,
}
