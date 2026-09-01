use serde::{Deserialize, Serialize};

/// One reusable block. Every page that references it renders THIS tree, so
/// editing the item changes every placement at once.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibraryItem {
    /// The block type this item instantiates. The library picker filters by it, so
    /// an item only ever appears where its bundle is allowed. Theme-defined.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// When the item entered the library.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The user id that made the block reusable.
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    /// The tombstone. A soft-deleted item is never listed or handed out, and a
    /// block still referencing it keeps rendering its own last state rather than
    /// breaking.
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    /// The library item id. A block references it to become an instance of the
    /// item rather than a copy.
    #[serde(rename = "id", default)]
    pub id: String,
    /// What the item is called in the library picker. This is the only thing an
    /// editor sees before inserting it, so it carries the whole description.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The block and everything under it, serialized. This is the payload: every
    /// page that references the item renders THIS tree, so editing it here changes
    /// every placement at once.
    #[serde(rename = "tree", default)]
    pub tree: crate::models::PageBlockTree,
    /// When the item last changed — i.e. when every page referencing it last
    /// changed with it.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
