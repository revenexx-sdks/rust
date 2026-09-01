use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Pages service
pub struct Pages {
    client: Client,
}

impl Pages {
    pub fn new(client: Client) -> Self {
        Pages { client }
    }
    /// The pool an editor picks a reusable block from. A library item is ONE block
    /// subtree that many pages share BY REFERENCE — edit the item and every page
    /// using it changes — which is what separates it from a template, the other
    /// reusable thing here, which copies instead and is at `GET /pages/templates`.
    /// So the two filters are the two questions the picker asks: `bundles` narrows
    /// to the block types that fit the field being filled, `text` matches the
    /// label a person gave the item.
    pub async fn pages_library_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, bundles: Option<String>, text: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/library".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &bundles {
            api_params.insert("bundles".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &text {
            api_params.insert("text".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Retires a reusable block. It leaves the picker and every list, but the
    /// blocks pointing at it keep their `library_item_id` — the FK's `set null`
    /// belongs to a hard delete, and this writes a tombstone. Delivery then skips
    /// the expansion for a struck item rather than failing on it, so a page that
    /// used it falls back to the block content stored in its own published
    /// revision: nothing breaks, but the pages quietly stop tracking each other.
    /// Nothing here tells you which pages those are, so establish that before
    /// striking it.
    pub async fn pages_library_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The stored subtree behind one reusable block, so a picker can preview what
    /// dropping it into a page would produce. Because delivery expands the
    /// reference against THIS row at read time, what comes back is also what every
    /// page already using the item is currently rendering — which makes this the
    /// call to make before editing one.
    pub async fn pages_library_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The one write in this app whose blast radius is not a single page. Delivery
    /// expands a library reference against this row every time it serves, so
    /// replacing `tree` re-renders every page that points at the item —
    /// published ones included — without any of them being edited, republished
    /// or even touched. Nothing warns you first and no revision records it,
    /// because the pages did not change; the item did. Changing `label` or
    /// `bundle` only moves the item around the picker. Detaching one page from the
    /// item, so it keeps a copy of its own, is an editor mutation and not this
    /// route.
    pub async fn pages_library_update(&self, id: String, bundle: Option<String>, label: Option<String>, tree: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tree {
            api_params.insert("tree".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The management view of the menus a tenant keeps — `main`, `footer`,
    /// `account` and whatever else the theme asks for, each with the key it is
    /// looked up by. This route reads no filter at all — a `?menu_key=` is
    /// ignored, which the empty `filter` echo shows — so fetch a page and pick,
    /// or address one by id.
    pub async fn pages_menus_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/menus".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Writes a menu by its KEY rather than by its id, which is what makes theme
    /// seeding safe to repeat: a key the tenant already has has its label and
    /// items replaced in place, a key it does not have is created. `items` is
    /// replaced wholesale and never merged, so sending an empty list empties the
    /// navigation. One caveat worth reading before you rely on the idempotence:
    /// the key's uniqueness is this route's doing and not the database's —
    /// `menu_key` carries an index but no unique constraint — so a duplicate key
    /// created any other way leaves this route updating whichever row it finds
    /// first.
    pub async fn pages_menus_upsert(&self, label: String, menu_key: String, items: Option<Vec<crate::models::PageMenuItem>>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/menus".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("label".to_string(), serde_json::to_value(&label)?);
        api_params.insert("menuKey".to_string(), serde_json::to_value(&menu_key)?);
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Writes the tombstone. The menu drops out of the management list and out of
    /// `GET /pages/delivery/menus` in the same moment, so a theme that reads its
    /// key gets nothing back and renders nothing — there is no fallback and no
    /// error a storefront could act on. The key is free immediately, which means
    /// re-seeding the theme is the way back. Check what reads the key before
    /// striking it.
    pub async fn pages_menus_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One menu and its whole item tree — the ordered links a theme renders as
    /// its header, footer or account navigation. `items` is nested, not one level,
    /// so this is the entire navigation for that key in a single read. Addressed
    /// by ROW ID here; the key a theme knows it by is `menu_key` on the body, and
    /// the route that works by key is the upsert.
    pub async fn pages_menus_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The same write as the upsert, for a caller that already holds the row id
    /// — use this when editing a menu a person picked from a list, and the
    /// upsert when reconciling a theme's defaults. `menu_key` is deliberately not
    /// editable here: the key is the handle every theme reads the menu by, so
    /// changing it would empty whatever is rendering that key without anything
    /// reporting an error.
    pub async fn pages_menus_update(&self, id: String, items: Option<Vec<crate::models::PageMenuItem>>, label: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The EDITORIAL index — every live page of the tenant, whatever its status,
    /// newest change first. This is the list the Cockpit shows a person: drafts
    /// and archived pages are in it, and a row here says nothing about whether a
    /// visitor can see the page, because a published status without a published
    /// revision still delivers nothing. A storefront wants `GET
    /// /pages/delivery/pages` instead, which answers only what is actually
    /// servable. Soft-deleted pages are never returned and the predicate is this
    /// route's own, not something a caller can switch off.
    pub async fn pages_pages_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, bundle: Option<String>, status: Option<String>, q: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/pages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &q {
            api_params.insert("q".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Writes two rows, not one: the page itself and the translation row for its
    /// source language, so a page is never without the language it was authored in
    /// and `GET /pages/delivery/page?slug=` can match a localized URL from the
    /// first moment. Everything the caller leaves out comes from the tenant's
    /// settings, not from a literal in this app: `bundle` from
    /// default_page_bundle, `sourceLanguage` from default_source_language
    /// (resolved for the request's market), and the status of both the page and
    /// its source translation from default_page_status (draft | published).
    pub async fn pages_pages_create(&self, title: String, bundle: Option<String>, host_options: Option<serde_json::Value>, meta: Option<serde_json::Value>, slug: Option<String>, source_language: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/pages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &host_options {
            api_params.insert("hostOptions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &meta {
            api_params.insert("meta".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &slug {
            api_params.insert("slug".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source_language {
            api_params.insert("sourceLanguage".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Writes a tombstone. The page leaves every list, every read and all delivery
    /// at once, and its slug is immediately free for another page — the unique
    /// index counts live rows only. Nothing is erased: the translations, blocks,
    /// edit state, revisions, comments and preview grants that hang off the page
    /// all keep their rows, because their `on delete cascade` belongs to a hard
    /// delete and this is not one. So a page can be brought back intact by
    /// clearing `deleted_at` — but not through this app, which publishes no
    /// route that does it.
    pub async fn pages_pages_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One page RECORD: what it is called, where it routes, what type it is, which
    /// revision is live. Not its content — the blocks are not on this row and no
    /// expansion here returns them. The editor reads them with `GET
    /// /pages/editor/{page_id}/state`, a renderer with `GET /pages/delivery/page`.
    /// A soft-deleted page answers 404 exactly like one that never existed, so
    /// this is also the check for whether an id is still good.
    pub async fn pages_pages_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Corrects the page RECORD — the five fields an editor changes without
    /// opening the visual editor, which are `title`, `slug`, `status`, `meta` and
    /// `bundle`, and no others. Anything else in the body is dropped rather than
    /// refused, and the block tree is unreachable from here by design: content
    /// moves only through the editor's mutation log, so a caller cannot half-edit
    /// a page behind the undo history's back. Two consequences worth knowing
    /// before you call it: a slug is unique among live pages, so claiming one that
    /// is held answers 409; and setting `status` to published does NOT put
    /// anything in front of a visitor — delivery needs a revision, which only
    /// `POST /pages/editor/{page_id}/publish` writes.
    pub async fn pages_pages_update(&self, id: String, bundle: Option<String>, meta: Option<serde_json::Value>, slug: Option<String>, status: Option<String>, title: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &meta {
            api_params.insert("meta".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &slug {
            api_params.insert("slug".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One entry per publication, newest first, which is the order a history is
    /// read in and the one this route sorts by unless `order` says otherwise. The
    /// `snapshot` — the whole published page, in every language — is
    /// deliberately not in the index: it is page-sized, and nothing that renders a
    /// history needs it.
    pub async fn pages_pages_revisions(&self, id: String, limit: Option<i64>, offset: Option<i64>, order: Option<String>, label: Option<String>, created_by: Option<String>, created_by_name: Option<String>, created_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/pages/{id}/revisions".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_by {
            api_params.insert("created_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_by_name {
            api_params.insert("created_by_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The target of a theme activation hook: hand it the theme's default pages
    /// and menus and it creates whatever is missing. Idempotent by `slug` and by
    /// menu key — a slug or a key the tenant already holds is skipped rather
    /// than rewritten, so re-running after a theme update adds only the new ones
    /// and never overwrites what an editor has since changed. A seeded page is
    /// published on the spot, immediately servable by delivery: the
    /// default_page_status setting deliberately does not apply, because a theme
    /// that activates with invisible pages looks broken.
    pub async fn pages_seed(&self, menus: Option<Vec<serde_json::Value>>, pages: Option<Vec<serde_json::Value>>) -> Result<crate::models::SeedResult, Error> {
        let api_path = "/v1/pages/seed".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &menus {
            api_params.insert("menus".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pages {
            api_params.insert("pages".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every column of a template is an exact-match filter here:
    /// `?page_bundle=standard&field_name=content` is how a picker asks for the
    /// templates offered in one place, and `?is_default=true` is how a "new page"
    /// flow finds the one to start from.
    pub async fn pages_templates_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, label: Option<String>, description: Option<String>, page_bundle: Option<String>, field_name: Option<String>, is_default: Option<bool>, created_by: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/templates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page_bundle {
            api_params.insert("page_bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &field_name {
            api_params.insert("field_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_by {
            api_params.insert("created_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Removes the template row outright. This is the one delete in the app that
    /// is not a tombstone — `templates` carries no `deleted_at` — so it cannot
    /// be undone and the id will not come back. Nothing else breaks by it: pages
    /// built from the template hold their own copy of the blocks and never
    /// referenced the row.
    pub async fn pages_templates_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The blocks a page would START from if an editor picked this template —
    /// read it to preview the insert. A template is a COPY source, the opposite of
    /// a library item: nothing links back from the pages already built from it, so
    /// this tells you what future pages get and nothing about existing ones.
    pub async fn pages_templates_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Edits what a future page will start from. Because templates copy rather
    /// than share, this reaches nothing that already exists — pages built from
    /// it keep the blocks they were handed, which is exactly the property that
    /// makes a template safe to edit and a library item dangerous. `is_default` is
    /// the one field with an effect past the picker: it decides what a new page of
    /// `page_bundle` starts with, and nothing here stops two templates of the same
    /// bundle from both claiming it, so which one wins is left to whoever reads
    /// the list.
    pub async fn pages_templates_update(&self, id: String, description: Option<String>, field_name: Option<String>, is_default: Option<bool>, label: Option<String>, page_bundle: Option<String>, tree: Option<Vec<crate::models::PageBlockTree>>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &field_name {
            api_params.insert("field_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page_bundle {
            api_params.insert("page_bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tree {
            api_params.insert("tree".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Discovery for the vocabulary routes: the enums this app publishes, each
    /// with its name, its title and what it is for, and none of them unpacked —
    /// the permitted values are not on this route, only on the one that serves a
    /// single vocabulary. Names: edit-state-statuses, page-statuses,
    /// translation-statuses. Fetch one with GET /pages/vocabularies/{name}; a
    /// client holding the qualified pair 'pages.<name>' builds that URL from the
    /// pair alone.
    pub async fn pages_vocabularies_list(&self) -> Result<crate::models::PagesVocabularyIndex, Error> {
        let api_path = "/v1/pages/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One vocabulary unpacked: every value the column permits, each with the
    /// title to show for it, the sentence explaining it and the badge tone to
    /// render it in — everything a select or a status pill needs, so nothing
    /// downstream keeps its own copy of the labels. The values are read out of the
    /// column's CHECK constraint, so the served set IS the enforced set and the
    /// two cannot drift — a value added to the constraint appears here even
    /// before anyone labels it, titled from its own key. Values come back in
    /// constraint order, which is the order a select should offer. 'closed' says
    /// the set is exhaustive, so a value outside it is stale data rather than a
    /// missing label. Names: edit-state-statuses, page-statuses,
    /// translation-statuses.
    pub async fn pages_vocabularies_get(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
