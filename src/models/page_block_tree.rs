use serde::{Deserialize, Serialize};

/// The block and everything under it, serialized. This is the payload: every
/// page that references the item renders THIS tree, so editing it here changes
/// every placement at once.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTree {
    /// The block type — `hero`, `text`, `teaser`, whatever the active theme
    /// defines. It decides which component renders it and which props it carries.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// Nested blocks, keyed by the field they sit in — `{ "content": [...],
    /// "buttons": [...] }`. Absent on a leaf block.
    #[serde(rename = "children", default)]
    pub children: serde_json::Value,
    /// The theme fragment this block renders instead of a props-driven component,
    /// or `null` for an ordinary block. Theme-defined, like a bundle.
    #[serde(rename = "fragment_name", default)]
    pub fragment_name: String,
    /// blökkli display options for this block, as a flat `option key → value`
    /// map (variant, spacing, background). Theme-defined, set by the
    /// `update_options` mutation.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// The block's field values in the page's SOURCE language, as a flat `field
    /// name → value` map. The field names are the theme's; this app stores and
    /// replays them without reading one.
    #[serde(rename = "props", default)]
    pub props: serde_json::Value,
    /// Per-language overrides of `props`, keyed by langcode: `{ "en": { "title":
    /// "About us" } }`. A field missing for a language falls back to `props`,
    /// which is why a half-translated page still renders.
    #[serde(rename = "props_i18n", default)]
    pub props_i18n: serde_json::Value,
}
