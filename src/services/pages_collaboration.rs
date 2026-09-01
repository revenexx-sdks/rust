use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PagesCollaboration service
pub struct PagesCollaboration {
    client: Client,
}

impl PagesCollaboration {
    pub fn new(client: Client) -> Self {
        PagesCollaboration { client }
    }
    /// The caller's own notifications, newest first, 20 at a time. Paged by an
    /// opaque cursor rather than by offset, so new arrivals never shift a page
    /// under the reader. It is also the one read in this app that writes:
    /// `?markAsRead=true` flags the notifications on the page it just returned as
    /// read, which is how a feed that has been looked at empties its badge without
    /// a second call — leave it off and reading changes nothing.
    pub async fn pages_editor_notifications_list(&self, after: Option<String>, mark_as_read: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &after {
            api_params.insert("after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mark_as_read {
            api_params.insert("markAsRead".to_string(), serde_json::to_value(value)?);
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
    /// Empties the badge in one call. Every unread notification of the CURRENT
    /// user is flagged read — the user is the one the request's context token
    /// names and there is no body with which to name another. Nothing is deleted:
    /// `GET /pages/editor/notifications` still returns the same feed, just with
    /// `read` set. The answer is the new unread count, so a client can set the
    /// badge straight from it without a second read.
    pub async fn pages_editor_notifications_mark_all_read(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications/mark-all-read".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

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
    /// The cheap poll behind the badge.
    pub async fn pages_editor_notifications_unread_count(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications/unread-count".to_string();

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
    /// What the @mention picker is filled from. When the identity service cannot
    /// be reached this degrades to the authors who have already commented on this
    /// tenant's pages rather than answering an error — a mention list that is
    /// short is more useful than one that is missing.
    pub async fn pages_editor_users(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/users".to_string();

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
    /// Every comment on the page in one flat list, oldest first, roots and replies
    /// together and resolved threads included — there is no filter and no
    /// paging, because the editor nests and filters them itself from `parentUuid`
    /// and pins each root to its blocks with `blockUuids`. Comments hang off the
    /// PAGE, not off a revision or an edit state, so publishing and reverting
    /// leave them standing; that is what makes them usable as a review trail
    /// across several rounds of edits.
    pub async fn pages_editor_comments_list(&self, page_id: String) -> Result<crate::models::PageCommentList, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The same route writes both kinds, and which one you get is decided by the
    /// body: `blockUuids` starts a new thread pinned to those blocks, `parentUuid`
    /// hangs a reply under an existing root. Everyone named with an @mention in
    /// the body is notified, and on a reply so is everybody already in the thread
    /// — the actor never notifies themselves.
    pub async fn pages_editor_comments_create(&self, page_id: String, body: String, block_uuids: Option<Vec<String>>, parent_uuid: Option<String>) -> Result<crate::models::PageCommentList, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments".replace("{page_id}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("body".to_string(), serde_json::to_value(&body)?);
        if let Some(value) = &block_uuids {
            api_params.insert("blockUuids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_uuid {
            api_params.insert("parentUuid".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A hard delete, and deleting a root takes its replies with it.
    pub async fn pages_editor_comments_delete(&self, page_id: String, uuid: String) -> Result<crate::models::PageCommentList, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}".replace("{page_id}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Rewrites what a comment says, and only its author may — a comment carries
    /// an `author_id` and anybody else is refused with 403. Only the body moves:
    /// what the comment is pinned to, whether the thread is resolved and who wrote
    /// it are all fixed when it is created. Rewriting a body does NOT re-run the
    /// @mention notifications, so mentioning somebody new by editing will not
    /// reach them. Answers the page's whole comment list rather than the one row,
    /// so a client can re-render from the response.
    pub async fn pages_editor_comments_update(&self, page_id: String, uuid: String, body: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}".replace("{page_id}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);
        api_params.insert("body".to_string(), serde_json::to_value(&body)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Marks a thread handled, so the editor stops surfacing it on the block it is
    /// pinned to. Only a ROOT can be resolved — resolved-ness is a property of
    /// the thread and not of a message in it, so pointing this at a reply is
    /// refused with 400 rather than quietly resolving its parent. Nothing is
    /// deleted, nobody is notified, and the thread stays in the list;
    /// `.../unresolve` is the way back. Answers the page's whole comment list.
    pub async fn pages_editor_comments_resolve(&self, page_id: String, uuid: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/resolve".replace("{page_id}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A comment body may carry a task list. This flips one checkbox by rewriting
    /// the body's markup, and answers the single comment rather than the whole
    /// list. A `taskIndex` that names no checkbox is refused and nothing is
    /// written — the comment's `updated_at` is the editor's "edited" marker, so
    /// a call that changes nothing must not move it.
    pub async fn pages_editor_comments_toggle_task(&self, page_id: String, uuid: String, task_index: i64) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/toggle-task".replace("{page_id}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);
        api_params.insert("taskIndex".to_string(), serde_json::to_value(&task_index)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Clears the resolved flag and puts the thread back in front of whoever is
    /// editing — the mirror of `.../resolve` in every respect, including that
    /// only a root can be reopened and that a reply answers 400. A thread that was
    /// already open is accepted and stays open. Answers the page's whole comment
    /// list.
    pub async fn pages_editor_comments_unresolve(&self, page_id: String, uuid: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/unresolve".replace("{page_id}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
