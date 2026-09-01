use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value. Every page that
/// references this item renders the new tree the next time it is delivered,
/// which is the whole point of the library and the whole risk of editing one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageLibraryItemUpdateRequest {
    /// The block type this item instantiates. Changing it moves the item to a
    /// different part of the picker.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// What the item is called in the picker.
    #[serde(rename = "label", default)]
    pub label: String,
    /// A block and its whole subtree, serialized. Produced by the editor when a
    /// selection is made reusable or saved as a template, and instantiated back
    /// into real blocks when one is inserted.
    #[serde(rename = "tree", default)]
    pub tree: crate::models::PageBlockTree,
}
