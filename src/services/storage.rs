use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Storage service
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(client: Client) -> Self {
        Storage { client }
    }
    /// List the media assets in this tenant, newest first. Narrow the list with
    /// `filter[folder_id]`, `filter[kind]`, `filter[status]` and a
    /// `filter[created_at][gte]`/`[lte]` range; search original names, display
    /// names, alt text and descriptions with `search`; order by `created_at`,
    /// `size_bytes` or `original_name` (prefix with `-` to reverse). One page is
    /// returned, 50 records by default and 200 at most.
    /// 
    /// Records only: no file content is returned — fetch bytes with
    /// `GET /assets/{id}/download` or hand out a link with
    /// `POST /assets/{id}/sign`. Deleted assets are not listed.
    pub async fn asset_index(&self, search: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &search {
            api_params.insert("search".to_string(), serde_json::to_value(value)?);
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
    /// Upload one file into this tenant's media library. The file is checked
    /// against the tenant's single-file limit and its remaining storage quota,
    /// its media type is sniffed from the content rather than trusted from the
    /// request, and it is virus-scanned before anything is written. The stored
    /// asset comes back with status `pending_processing`; metadata extraction
    /// finishes asynchronously and moves it to `available`. `folder_id`,
    /// `visibility`, `alt_text`, `description`, `display_name` and `tags` are
    /// applied on the way in; set `unpack` to also queue an uploaded archive's
    /// members for ingestion.
    /// 
    /// Every call creates a new asset — this never replaces the content of an
    /// existing one — and it takes exactly one file. Use `POST /assets/bulk` for
    /// several.
    pub async fn asset_store(&self, file: crate::input_file::InputFile, alt_text: Option<String>, description: Option<String>, display_name: Option<String>, folder_id: Option<String>, keep_archive: Option<bool>, tags: Option<Vec<String>>, unpack: Option<bool>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &alt_text {
            api_params.insert("alt_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &display_name {
            api_params.insert("display_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &keep_archive {
            api_params.insert("keep_archive".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tags {
            api_params.insert("tags".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unpack {
            api_params.insert("unpack".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let api_response = self
            .client
            .file_upload(&api_path, api_headers, api_params, "file", file)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Upload a batch of files in one request under `files`, each ingested
    /// exactly as `POST /assets` ingests a single file. The batch is rejected as
    /// a whole when it carries no files, more files than one request may carry,
    /// or too many bytes in total. Past that point every file is attempted
    /// independently and the call answers 207 with a `results` entry per file:
    /// either the created asset or the error that rejected it. A partial failure
    /// is therefore a successful call, not an error status — read `results`.
    /// 
    /// Only `folder_id` and `visibility` apply, and they apply to the whole
    /// batch; per-file metadata is not accepted here. Set it afterwards with
    /// `PATCH /assets/{id}`.
    pub async fn asset_bulk(&self, folder_id: Option<String>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/bulk".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
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
    /// Soft-delete an asset: it stops being listed and served, its status
    /// becomes `soft_deleted`, and it is scheduled for permanent deletion once
    /// the retention window has passed. Until then `POST /assets/{id}/restore`
    /// brings it back.
    /// 
    /// The stored file is not erased at this point and its bytes still count
    /// against the tenant's storage quota — use `DELETE /assets/{id}/permanent`
    /// to erase it and free the quota immediately.
    pub async fn asset_destroy(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Fetch one asset's record by id: name, folder, media type, size, status,
    /// tags, the extracted metadata and the delivery URL (null for a private
    /// asset, which is reachable only through a signed URL). Metadata only — the
    /// bytes are served by `GET /assets/{id}/download`. A deleted asset is not
    /// visible here until `POST /assets/{id}/restore` brings it back.
    pub async fn asset_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Change an asset's metadata: `display_name`, `alt_text`, `description`,
    /// `visibility` and `tags`. Sending `folder_id` moves it and sending `name`
    /// renames it; either re-derives the asset's public delivery path, so links
    /// built from the old path stop resolving. Only the fields present in the
    /// request are touched.
    /// 
    /// The stored file itself is never modified here — to change the content,
    /// upload a new asset.
    pub async fn asset_update(&self, id: String, alt_text: Option<String>, description: Option<String>, display_name: Option<String>, folder_id: Option<String>, name: Option<String>, tags: Option<Vec<String>>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &alt_text {
            api_params.insert("alt_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &display_name {
            api_params.insert("display_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tags {
            api_params.insert("tags".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Stream the asset's original file back as an attachment, named after the
    /// asset. This is the authenticated read path — every call carries the
    /// caller's credentials — and the bytes are the ones that were uploaded: no
    /// resizing, re-encoding or other transformation is applied.
    /// 
    /// To let a browser, an email or a third party fetch the file without an API
    /// credential, mint a link with `POST /assets/{id}/sign` instead.
    pub async fn asset_download(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/download".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Erase an asset and its stored file for good and credit its bytes back to
    /// the tenant's used storage. Works on live and soft-deleted assets alike.
    /// 
    /// This cannot be undone: there is no restore afterwards, and links to the
    /// asset stop resolving at once. Use `DELETE /assets/{id}` for the
    /// reversible variant. Requires the elevated (admin) tier.
    pub async fn asset_permanent(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/permanent".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Re-run post-upload processing for one asset. It returns to
    /// `pending_processing` and the job re-extracts its metadata — and, for a 3D
    /// model, re-renders the preview and mesh derivatives — before marking it
    /// `available` again. The usual reason is an asset stuck in
    /// `processing_failed`.
    /// 
    /// The stored file is neither re-uploaded nor altered, and no thumbnails are
    /// produced: delivery transforms are applied on the fly when the asset is
    /// served, not here.
    pub async fn asset_reprocess(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/reprocess".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Bring a soft-deleted asset back: the scheduled permanent deletion is
    /// cleared and the asset returns to `available`, listed and served again
    /// under its original path. Only works while the asset is still inside its
    /// retention window — once it has been erased, by
    /// `DELETE /assets/{id}/permanent` or by the retention sweep, there is
    /// nothing left to restore.
    pub async fn asset_restore(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/restore".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Mint a time-limited URL that serves this asset without an API credential
    /// — the way to hand a private asset to a browser, an email or a third
    /// party. `ttl_seconds` sets the lifetime: one hour by default, seven days
    /// at most. The response carries the URL and the lifetime it was issued
    /// with.
    /// 
    /// The signature is checked at the delivery edge. A link cannot be revoked
    /// before it expires, so keep the lifetime short. A public asset already
    /// carries an unsigned delivery URL on its record and does not need this.
    pub async fn asset_sign(&self, id: String, ttl_seconds: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/sign".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &ttl_seconds {
            api_params.insert("ttl_seconds".to_string(), serde_json::to_value(value)?);
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
    /// Ingest the members of an already-uploaded archive as individual assets.
    /// They land in a folder named after the archive, created under
    /// `target_folder_id` or, when that is omitted, under the archive's own
    /// folder, and the archive's internal directory structure is mirrored
    /// beneath it. Each member goes through the same pipeline as an upload —
    /// media-type sniff, virus scan, quota — and a member that fails is skipped
    /// rather than failing the run. `keep_archive` (true by default) decides
    /// whether the archive asset itself survives.
    /// 
    /// Asynchronous: this answers 202 as soon as the work is queued, so poll the
    /// folder or asset list for the results. Only an asset that is an archive of
    /// a supported type can be unpacked; an upload can ask for the same thing
    /// inline with `unpack`.
    pub async fn asset_unpack(&self, id: String, keep_archive: Option<bool>, target_folder_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/unpack".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &keep_archive {
            api_params.insert("keep_archive".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_folder_id {
            api_params.insert("target_folder_id".to_string(), serde_json::to_value(value)?);
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
    /// Return every folder in this tenant as one flat list ordered by path, each
    /// record carrying its `parent_id` and its materialized `path`, so a client
    /// can rebuild the tree without walking it. Not paginated and not filtered.
    /// 
    /// Folders hold no file content of their own — list a folder's assets with
    /// `GET /assets` and `filter[folder_id]`.
    pub async fn folder_index(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders".to_string();

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
    /// Create a folder under `parent_id`, or at the library root when it is
    /// omitted. The `name` is slugged into a path segment and appended to the
    /// parent's path; that path is what the public delivery URL of every asset
    /// inside it is built from, so two siblings may not slug to the same
    /// segment.
    /// 
    /// Creating a folder moves nothing into it — assign assets with
    /// `folder_id` on upload or with `PATCH /assets/{id}`.
    pub async fn folder_store(&self, name: String, parent_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
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
    /// Delete a folder. By default it has to be empty: a folder that still holds
    /// folders or assets is refused, so pass `recursive=true` to delete it
    /// together with everything beneath it.
    /// 
    /// A recursive delete soft-deletes the assets it takes with it — their files
    /// are not erased and their bytes still count against the tenant's storage
    /// quota, and each remains restorable through `POST /assets/{id}/restore`.
    /// System folders cannot be deleted.
    pub async fn folder_destroy(&self, id: String, recursive: Option<bool>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &recursive {
            api_params.insert("recursive".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Fetch one folder's record by id: its name, its parent, the materialized
    /// path assets inside it are delivered under, and whether it is a system
    /// folder (system folders cannot be renamed, moved or deleted).
    /// 
    /// Its contents are not included — list them with `GET /assets` and
    /// `filter[folder_id]`, and its child folders with `GET /folders`.
    pub async fn folder_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Rename a folder with `name`, move it under a different parent with
    /// `parent_id` (null for the root), or both at once. Either rewrites the
    /// folder's materialized path and the path of every folder beneath it, which
    /// changes the public delivery URL of every asset they hold — existing links
    /// built from the old path stop resolving.
    /// 
    /// Nothing else about the assets changes; they are not moved, re-uploaded or
    /// reprocessed. A system folder cannot be changed, a folder cannot be moved
    /// inside its own subtree, and the new name has to slug to a segment free
    /// among its new siblings.
    pub async fn folder_update(&self, id: String, name: Option<String>, parent_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Return this tenant's SFTP sync rules, newest first, each with the account
    /// and remote path it pulls from, the folder it imports into, its cron
    /// schedule, whether it is enabled and when it last ran. Not paginated and
    /// not filtered.
    /// 
    /// These are the rules themselves, not what they moved: for the files a rule
    /// has actually transferred, see `GET /sftp/sync-history`.
    pub async fn sync_rule_index(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules".to_string();

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
    /// Schedule a recurring one-way pull from a directory on the tenant's SFTP
    /// storage box into this media library. `sftp_account_id` selects the
    /// account, `source_path` the remote directory, `target_folder_id` the
    /// folder imported assets land in, and `schedule` a cron expression (every
    /// five minutes when omitted) at which the rule falls due. `options` carries
    /// the per-rule knobs: recursion, include/exclude and size filters, how long
    /// a remote file has to have stopped changing before it is taken, and
    /// whether it is deleted from the remote after a successful transfer.
    /// 
    /// Each run ingests every matching remote file exactly as an upload would,
    /// quota, media-type and virus checks included, and records one history
    /// entry per file. Creating the rule transfers nothing: the first run
    /// happens when the schedule next falls due, or immediately if you call
    /// `POST /sftp/rules/{id}/run`. Nothing is ever pushed back to the remote,
    /// beyond the optional delete after a successful transfer. Requires the
    /// elevated (admin) tier.
    pub async fn sync_rule_store(&self, sftp_account_id: String, source_path: String, enabled: Option<bool>, options: Option<Vec<String>>, schedule: Option<String>, target_folder_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("sftp_account_id".to_string(), serde_json::to_value(&sftp_account_id)?);
        api_params.insert("source_path".to_string(), serde_json::to_value(&source_path)?);
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &schedule {
            api_params.insert("schedule".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_folder_id {
            api_params.insert("target_folder_id".to_string(), serde_json::to_value(value)?);
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
    /// Delete a sync rule so it is never scheduled again. The assets it already
    /// imported stay exactly where they are, its recorded run history is kept,
    /// and nothing on the remote is touched.
    /// 
    /// To stop a rule only for a while, set `enabled` to false with
    /// `PATCH /sftp/rules/{id}` instead — a deleted rule cannot be restored.
    /// Requires the elevated (admin) tier.
    pub async fn sync_rule_destroy(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Fetch one sync rule's configuration by id: the account and remote path it
    /// pulls from, its target folder, its cron schedule, its `options` and
    /// `last_run_at`.
    /// 
    /// Configuration only, and `last_run_at` says when a run was last attempted,
    /// not whether it succeeded. What a run did is in
    /// `GET /sftp/rules/{id}/runs/{runId}` and `GET /sftp/sync-history`.
    pub async fn sync_rule_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Change a sync rule in place: its account, remote path, target folder,
    /// schedule or options, or `enabled` to pause and resume it without deleting
    /// it. Only the fields present in the request are touched, but `options` is
    /// replaced wholesale rather than merged — send the whole object.
    /// 
    /// A change takes effect from the next run; a run already in flight is not
    /// affected, and nothing a previous run imported is revisited or undone.
    /// Requires the elevated (admin) tier.
    pub async fn sync_rule_update(&self, id: String, enabled: Option<bool>, options: Option<Vec<String>>, schedule: Option<String>, sftp_account_id: Option<String>, source_path: Option<String>, target_folder_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &schedule {
            api_params.insert("schedule".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sftp_account_id {
            api_params.insert("sftp_account_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source_path {
            api_params.insert("source_path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_folder_id {
            api_params.insert("target_folder_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Queue a run of this rule straight away, outside its schedule. Answers 202
    /// with the rule id as soon as the job is queued — it does not wait for the
    /// transfer and it does not hand back a run id, so follow the outcome in
    /// `GET /sftp/sync-history`.
    /// 
    /// The rule's own schedule is untouched, and this does not enable a disabled
    /// rule: the job is queued but does nothing when it picks a disabled rule
    /// up. Requires the elevated (admin) tier.
    pub async fn sync_rule_run(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}/run".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    /// Return the per-file protocol of one run of one sync rule: every entry the
    /// run recorded, oldest first, with the remote source path, the asset it
    /// produced, the bytes transferred, the duration and the error where one
    /// applies — plus a `summary` counting those entries by status (`success`,
    /// `skipped`, `failed`, `quarantined`).
    /// 
    /// Use it to find out what one run actually did. It is not paginated, and it
    /// does not list a rule's runs: take the `run_id` from
    /// `GET /sftp/sync-history`. An unknown `runId` under a rule that does exist
    /// is an empty protocol, not a 404.
    pub async fn sync_rule_run_protocol(&self, id: String, run_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}/runs/{runId}".replace("{id}", &id.to_string()).replace("{runId}", &run_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("runId".to_string(), serde_json::to_value(&run_id)?);

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
    /// Page through this tenant's per-file sync records across every rule,
    /// newest first. Each entry names the run it belongs to, the rule, the
    /// remote source path, the asset it produced where there is one, the
    /// outcome — `success`, `skipped`, `failed` or `quarantined` — the bytes
    /// transferred and how long it took. Narrow it with `rule_id` and a
    /// `from`/`to` range on when the entry was recorded; one page is returned,
    /// 50 entries by default and 200 at most.
    /// 
    /// This is the audit trail of what SFTP sync has brought in: every file
    /// taken, skipped and rejected leaves an entry, and a run that matched
    /// nothing leaves one too. To read a single run whole instead, group by
    /// `run_id` and call `GET /sftp/rules/{id}/runs/{runId}`.
    pub async fn sync_rule_history(&self, rule_id: Option<String>, from: Option<String>, to: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/sync-history".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &rule_id {
            api_params.insert("rule_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &to {
            api_params.insert("to".to_string(), serde_json::to_value(value)?);
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
    /// Break this tenant's library down by asset kind — `image`, `video`,
    /// `audio`, `pdf`, `document`, `archive`, `model3d`, `other` — with a count
    /// and a byte total for each kind that has at least one asset, alongside the
    /// tenant-wide totals.
    /// 
    /// A dashboard figure, not a listing: no asset is named, and nothing here
    /// can be filtered. The tenant-wide byte total is the same running figure
    /// `GET /tenant/usage` reports, so soft-deleted assets are counted in it.
    pub async fn tenant_stats(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/tenant/stats".to_string();

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
    /// Report this tenant's storage consumption: the bytes in use, the byte
    /// quota in force (null when the tenant is uncapped) and how many assets it
    /// holds. This is the figure the quota check on upload compares against — it
    /// is maintained as a running total on every upload and permanent delete
    /// rather than summed on read.
    /// 
    /// Soft-deleted assets are still counted, because their files are still
    /// stored; their bytes come back only once they are permanently deleted. For
    /// the breakdown by asset kind, see `GET /tenant/stats`.
    pub async fn tenant_usage(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/tenant/usage".to_string();

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
}
