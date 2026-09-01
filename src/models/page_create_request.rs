use serde::{Deserialize, Serialize};

/// A new page. Only the title is yours to supply — everything else has a
/// tenant default behind it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCreateRequest {
    /// The page type. Omit to take the default_page_bundle setting.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// Page-level blökkli display options as a flat `option key → value` map.
    /// Theme-defined; usually left out and set later from the editor.
    #[serde(rename = "hostOptions", default)]
    pub host_options: serde_json::Value,
    /// The page's metadata bag (SEO and social fields). Stored and handed back
    /// untouched — this app reads no key of it, so the theme decides what goes
    /// in.
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    /// The path segment the storefront routes it under, without a leading slash.
    /// Unique per tenant among live pages; omit or send null for a page reached
    /// only by id. Nothing here derives one from the title.
    #[serde(rename = "slug", default)]
    pub slug: String,
    /// The language you are authoring in, and the fallback for every later
    /// translation. Omit to take the default_source_language setting for the
    /// request market.
    #[serde(rename = "sourceLanguage", default)]
    pub source_language: String,
    /// What the page is called, in its source language. Shown in the editorial
    /// list and searched by `?q=`.
    #[serde(rename = "title", default)]
    pub title: String,
}
