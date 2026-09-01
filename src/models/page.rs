use serde::{Deserialize, Serialize};

/// One addressable page of the storefront: its metadata and publish pointer.
/// Its CONTENT is not here — blocks live behind the editor and delivery
/// routes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page {
    /// Identifiers of findings the blökkli analyze feature was told to stop
    /// reporting for this page. Written by the `set_ignored_analyze` mutation and
    /// carried through publish, so dismissing a finding survives the next edit.
    #[serde(rename = "analyze_ignored", default)]
    pub analyze_ignored: Vec<String>,
    /// The page TYPE, e.g. `standard` or a landing-page type the theme defines. It
    /// decides which fields the editor offers and which template the theme
    /// renders; the value set belongs to the active theme, not to this app.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// When the page was created.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The user id that created the page.
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    /// The tombstone. A soft-deleted page is never listed, never delivered and
    /// answers 404 — and it drops out of the unique slug index at once, so
    /// deleting a page frees its slug immediately.
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    /// Page-level blökkli display options, as a flat `option key → value` map
    /// — the options that belong to the PAGE rather than to a block (background,
    /// width, whether the header is shown). The keys are defined by the theme;
    /// this app stores whatever the `update_host_options` mutation set.
    #[serde(rename = "host_options", default)]
    pub host_options: serde_json::Value,
    /// The page id. Every editor and delivery route addresses a page by it, and it
    /// never changes — publishing replaces a page's blocks, never the page.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The page's free-form metadata bag — SEO fields, social preview data,
    /// whatever the theme asks the editor for. Nothing in this app reads a key of
    /// it: it is stored, versioned into revisions and handed back to the renderer
    /// untouched, so the theme owns its shape.
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    /// The revision the storefront is currently serving. `null` means nothing has
    /// ever been published, and delivery answers 404 for the page even when
    /// `status` says `published`.
    #[serde(rename = "published_revision_id", default)]
    pub published_revision_id: String,
    /// The path segment the storefront routes this page under, without a leading
    /// slash. Unique per tenant among live pages, and `null` for a page that is
    /// only ever reached by id. `GET /pages/delivery/page?slug=` matches it first
    /// and the translations second.
    #[serde(rename = "slug", default)]
    pub slug: String,
    /// The language the page was authored in. It is the fallback for every field a
    /// translation leaves empty, so a page never renders as a hole.
    #[serde(rename = "source_language", default)]
    pub source_language: String,
    /// Where the page sits in the editorial lifecycle. Only `published` is ever
    /// delivered, and only together with a `published_revision_id`.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The page title as an editor typed it, in the page's source language.
    /// Publishing overwrites it with the title the edit state carries, so this is
    /// always the last published (or last saved) wording.
    #[serde(rename = "title", default)]
    pub title: String,
    /// When the page last changed. The default sort of `GET /pages/pages` is this
    /// column descending, because "what did we touch last" is the question an
    /// editorial list is opened with.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// The user id that last changed the page — set by an update, a soft delete
    /// and by publishing.
    #[serde(rename = "updated_by", default)]
    pub updated_by: String,
}
