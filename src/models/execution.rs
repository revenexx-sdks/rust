use serde::{Deserialize, Serialize};

/// Execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Execution {
    /// Execution creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Execution ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Execution roles.
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Execution update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Function's deployment ID used to create the execution.
    #[serde(rename = "deploymentId", default)]
    pub deployment_id: String,
    /// Resource(function/site) execution duration in seconds.
    #[serde(rename = "duration", default)]
    pub duration: f64,
    /// Function errors. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "errors", default)]
    pub errors: String,
    /// Function ID.
    #[serde(rename = "functionId", default)]
    pub function_id: String,
    /// Function logs. Includes the last 4,000 characters. This will return an
    /// empty string unless the response is returned using an API key or as part of
    /// a webhook payload.
    #[serde(rename = "logs", default)]
    pub logs: String,
    /// HTTP request headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "requestHeaders", default)]
    pub request_headers: Vec<crate::models::Headers>,
    /// HTTP request method type.
    #[serde(rename = "requestMethod", default)]
    pub request_method: String,
    /// HTTP request path and query.
    #[serde(rename = "requestPath", default)]
    pub request_path: String,
    /// HTTP response body. This will return empty unless execution is created as
    /// synchronous.
    #[serde(rename = "responseBody", default)]
    pub response_body: String,
    /// HTTP response headers as a key-value object. This will return only
    /// whitelisted headers. All headers are returned if execution is created as
    /// synchronous.
    #[serde(rename = "responseHeaders", default)]
    pub response_headers: Vec<crate::models::Headers>,
    /// HTTP response status code.
    #[serde(rename = "responseStatusCode", default)]
    pub response_status_code: i64,
    /// The scheduled time for execution. If left empty, execution will be queued
    /// immediately.
    #[serde(rename = "scheduledAt", default)]
    pub scheduled_at: String,
    /// The status of the function execution. Possible values can be: `waiting`,
    /// `processing`, `completed`, `failed`, or `scheduled`.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The trigger that caused the function to execute. Possible values can be:
    /// `http`, `schedule`, or `event`.
    #[serde(rename = "trigger", default)]
    pub trigger: String,
}
