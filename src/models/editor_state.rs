use serde::{Deserialize, Serialize};

/// Everything the blökkli editor runs on, for one page in one language,
/// materialized at the current point of the undo history. The theme adapter
/// maps it 1:1 onto blökkli's MappedState.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorState {
    /// Whether the caller may write. False means every write answers 409 until
    /// `POST …/take-ownership` — so the editor should go read-only rather than
    /// let someone type into a refusal.
    #[serde(rename = "currentUserIsOwner", default)]
    pub current_user_is_owner: bool,
    /// Every entity-reference field of every block — the fields an editor drags
    /// a product or a media item into.
    #[serde(rename = "droppableFieldValues", default)]
    pub droppable_field_values: Vec<serde_json::Value>,
    /// The open working copy, or `null` when nobody has started editing — in
    /// which case the state shown is simply the published one.
    #[serde(rename = "editState", default)]
    pub edit_state: serde_json::Value,
    /// What the tenant's settings allow, so a client hides a control instead of
    /// discovering the refusal.
    #[serde(rename = "features", default)]
    pub features: serde_json::Value,
    /// The block tree, flattened into one entry per (host, field) pair. This is
    /// the list the editor renders and drops into.
    #[serde(rename = "fields", default)]
    pub fields: Vec<serde_json::Value>,
    /// Analyze findings that were dismissed for this page, so the editor stops
    /// reporting them.
    #[serde(rename = "ignoredAnalyzeIdentifiers", default)]
    pub ignored_analyze_identifiers: Vec<String>,
    /// The language this whole state was resolved for — the `?langcode` that was
    /// applied, or the page's source language.
    #[serde(rename = "langcode", default)]
    pub langcode: String,
    /// The page-level field values the edit state changed, merged
    /// source-then-language — `{ "title": …, "slug": …, "meta": … }`.
    /// Empty when nobody edited the page itself, only its blocks.
    #[serde(rename = "mutatedEntity", default)]
    pub mutated_entity: serde_json::Value,
    /// The PAGE-level display options after the unpublished changes, as a flat
    /// `option key → value` map. Theme-defined.
    #[serde(rename = "mutatedHostOptions", default)]
    pub mutated_host_options: serde_json::Value,
    /// Every block's display options after the unpublished changes, keyed by block
    /// uuid: `{ "<uuid>": { "background": "grey" } }`.
    #[serde(rename = "mutatedOptions", default)]
    pub mutated_options: serde_json::Value,
    /// The undo/redo history, oldest first. Its length and
    /// `editState.currentIndex` are what an undo button and a history sidebar are
    /// drawn from.
    #[serde(rename = "mutations", default)]
    pub mutations: Vec<serde_json::Value>,
    /// The page itself, with the unpublished edits already applied — so the
    /// title here is what publishing would store, not what is stored now.
    #[serde(rename = "page", default)]
    pub page: serde_json::Value,
    /// Every string field of every block, flattened. It is what the translation
    /// view and the CSV export are built on — one row per translatable string.
    #[serde(rename = "textFieldValues", default)]
    pub text_field_values: Vec<serde_json::Value>,
    /// Every language this page exists in, so the editor can offer a language
    /// switcher that shows what is missing.
    #[serde(rename = "translations", default)]
    pub translations: Vec<serde_json::Value>,
    /// Why publishing would be refused right now. Empty means `POST …/publish`
    /// succeeds without `force`.
    #[serde(rename = "violations", default)]
    pub violations: Vec<serde_json::Value>,
}
