use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PagesEditor service
pub struct PagesEditor {
    client: Client,
}

impl PagesEditor {
    pub fn new(client: Client) -> Self {
        PagesEditor { client }
    }
    /// The drafts overview — the "what is unpublished right now" list, across
    /// every page: who holds it, since when, and whether it is parked for a date.
    /// Always newest-first — this route does not read `order`. An edit state
    /// whose page has been deleted is dropped from `items` but still counted in
    /// `total`.
    pub async fn pages_editor_edit_states(&self, status: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/edit-states".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
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
    /// The translation is the tenant's provider's, not this app's, and a tenant
    /// that has configured none gets no translation at all. The endpoint comes
    /// from the tenant setting `translate_endpoint` (PAGES_TRANSLATE_ENDPOINT
    /// remains a fallback). The bearer token does NOT: the gateway masks every
    /// setting flagged `sensitive`, so a key stored as one could never be read
    /// back — it stays the PAGES_TRANSLATE_KEY function secret. This app does
    /// not translate anything itself; it forwards `items` and hands the answer
    /// back.
    pub async fn pages_editor_translate(&self, items: Option<Vec<serde_json::Value>>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/translate".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
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
    /// Per-user editor preferences — one row per user, scoped to this app. Not
    /// tenant configuration: nothing here changes what the API does, only how one
    /// person's editor looks.
    pub async fn pages_editor_user_settings_get(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/user-settings".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

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
    /// Replaces the caller's preferences wholesale — this is not a merge, so
    /// send the whole bag.
    pub async fn pages_editor_user_settings_put(&self, settings: Option<serde_json::Value>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/user-settings".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &settings {
            api_params.insert("settings".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Undo and redo. The pointer is the edit state's `current_index`, the
    /// position in the mutation log the page is materialized at, and this route is
    /// the only thing that moves it — `GET …/state?index=` looks at another
    /// position without going there. The log itself is never rewritten — only
    /// the pointer moves — so redo stays available until the next change is
    /// appended.
    pub async fn pages_editor_history(&self, page_id: String, index: i64, langcode: Option<String>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/history".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("index".to_string(), serde_json::to_value(&index)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The cheap poll behind "someone else is editing this page": one integer, the
    /// moment the open edit state last moved, in epoch seconds rather than as a
    /// timestamp so a comparison is a subtraction. Compare it with the `updatedAt`
    /// you last saw and re-fetch the state only when it moved.
    pub async fn pages_editor_last_changed(&self, page_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/last-changed".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

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
    /// Take one change out of the replay without deleting it — "what would the
    /// page look like without this edit". The entry stays in the history and can
    /// be switched back on.
    pub async fn pages_editor_mutation_status(&self, page_id: String, enabled: bool, index: i64, langcode: Option<String>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/mutation-status".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("enabled".to_string(), serde_json::to_value(&enabled)?);
        api_params.insert("index".to_string(), serde_json::to_value(&index)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The one way page CONTENT changes. Each call appends one entry to the
    /// append-only log and answers the whole re-materialized state, so a client
    /// never re-fetches. A page nobody has opened yet needs no separate call to
    /// open it: the first mutation creates the edit state and takes ownership of
    /// it, and every later one asks for that ownership, so a second person editing
    /// the same page is refused until they take it over. Appending while the
    /// pointer sits mid-history discards the redo branch, exactly as an editor
    /// expects.
    pub async fn pages_editor_mutate(&self, page_id: String, plugin: String, langcode: Option<String>, payload: Option<serde_json::Value>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/mutations".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("plugin".to_string(), serde_json::to_value(&plugin)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payload {
            api_params.insert("payload".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Mints a link that shows this page's current edit state — the UNPUBLISHED
    /// one — to somebody without an editor account. The token is the whole
    /// credential — anyone holding it sees the page — so it expires, and a new
    /// one is cheap.
    pub async fn pages_editor_preview_grant(&self, page_id: String, ttl_hours: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/preview-grant".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        if let Some(value) = &ttl_hours {
            api_params.insert("ttlHours".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Four things in one call: the mutation log is replayed into a finished block
    /// tree, that tree is snapshotted into a new revision, the page's canonical
    /// blocks are replaced by it, and the edit state is archived — so the page
    /// comes out of this with nothing unpublished and the working copy behind it
    /// closed rather than deleted. The revision is written FIRST and the canonical
    /// blocks replaced after, so a failure mid-way leaves the page recoverable.
    /// Block uuids survive, which is why comments anchored to a block outlive the
    /// publish.
    pub async fn pages_editor_publish(&self, page_id: String, force: Option<bool>, label: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/publish".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        if let Some(value) = &force {
            api_params.insert("force".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Throws the whole working copy away: the edit state row is deleted and its
    /// mutation log with it, so the history goes too — this is not an undo and
    /// cannot itself be undone. Unlike publishing, which archives the edit state,
    /// nothing of it survives to be reopened. The published page is untouched.
    pub async fn pages_editor_revert(&self, page_id: String) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/revert".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Gated on the tenant setting `enable_scheduled_publishing`, which is off by
    /// default: nothing in the platform publishes a scheduled edit state yet, so a
    /// date accepted here would be a promise the app cannot keep. Every editor
    /// state carries `features.scheduledPublishing` so the control can be hidden
    /// rather than the refusal discovered.
    pub async fn pages_editor_schedule(&self, page_id: String, scheduled_at: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/schedule".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("scheduledAt".to_string(), serde_json::to_value(&scheduled_at)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The one call the visual editor boots on, and the only place the UNPUBLISHED
    /// page can be seen whole: the canonical blocks with every enabled mutation of
    /// the log replayed over them, the resulting field lists, the mutation history
    /// itself, who owns the edit state and where the undo pointer sits, and the
    /// tenant's editor feature flags. `langcode` decides which language the props
    /// resolve in, falling back to the page's source language. `index` replays the
    /// log up to a given position instead of the current one, which is how the
    /// editor previews an undo without performing it — it changes nothing, so it
    /// is safe to call at any position. Reading this creates nothing either: a
    /// page nobody has opened answers with a null `editState`, an empty history,
    /// and the published blocks as they stand.
    pub async fn pages_editor_state(&self, page_id: String, langcode: Option<String>, index: Option<i64>) -> Result<crate::models::EditorState, Error> {
        let api_path = "/v1/pages/editor/{page_id}/state".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &index {
            api_params.insert("index".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One page has one writer. This is how the second person gets the pen — the
    /// previous owner is notified rather than silently locked out.
    pub async fn pages_editor_take_ownership(&self, page_id: String) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/take-ownership".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Freezes a selection into a reusable starting point. The blocks are read out
    /// of the page's CURRENT edit state rather than out of what is published, so a
    /// template can be cut from work in progress and the uuids you send are the
    /// ones the editor is showing. Unlike making a block reusable, this COPIES:
    /// pages later made from the template are independent of it and of each other.
    pub async fn pages_editor_templates_create(&self, page_id: String, label: String, uuids: Vec<String>, description: Option<String>, field_name: Option<String>, is_default: Option<bool>, page_bundle: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/templates".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("label".to_string(), serde_json::to_value(&label)?);
        api_params.insert("uuids".to_string(), serde_json::to_value(&uuids)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &field_name {
            api_params.insert("fieldName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("isDefault".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page_bundle {
            api_params.insert("pageBundle".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a parked edit state back to `active` and clears its date, so the
    /// scheduled publication simply does not happen. The work is not touched —
    /// the mutation log, the undo position and the owner all stay as they were —
    /// and the page can then be published by hand or scheduled again for a
    /// different date. Like every other write to an edit state it asks for
    /// ownership, and a page with no open edit state answers 404 rather than
    /// pretending to have cancelled something.
    pub async fn pages_editor_unschedule(&self, page_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/unschedule".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
}
