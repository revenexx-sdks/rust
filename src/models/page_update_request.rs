use serde::{Deserialize, Serialize};

/// Partial update — only title, slug, status, meta and bundle are applied;
/// other keys are ignored. The page's CONTENT is never edited here: blocks
/// change through the editor's mutation log.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageUpdateRequest {
    /// The page type. Changing it changes which template the theme renders.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// The page's metadata bag. Replaced wholesale, not merged.
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    /// The path segment the storefront routes it under. Sending a slug another
    /// live page holds answers 409; sending null makes the page unreachable by
    /// path.
    #[serde(rename = "slug", default)]
    pub slug: String,
    /// The lifecycle status. Setting `published` here does NOT publish content —
    /// delivery still needs a revision, which only `POST
    /// /pages/editor/{page_id}/publish` writes.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The page title in its source language.
    #[serde(rename = "title", default)]
    pub title: String,
}
