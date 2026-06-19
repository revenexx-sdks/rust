use serde::{Deserialize, Serialize};

/// Deployment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Deployment {
    /// Deployment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Deployment ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Deployment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Whether the deployment should be automatically activated.
    #[serde(rename = "activate", default)]
    pub activate: bool,
    /// Raw billing.json bytes captured from the source archive at deploy time.
    /// Empty when no billing.json was shipped (private app).
    #[serde(rename = "billingJson", default)]
    pub billing_json: String,
    /// The current build time in seconds.
    #[serde(rename = "buildDuration", default)]
    pub build_duration: i64,
    /// The current build ID.
    #[serde(rename = "buildId", default)]
    pub build_id: String,
    /// The build logs.
    #[serde(rename = "buildLogs", default)]
    pub build_logs: String,
    /// The build output size in bytes.
    #[serde(rename = "buildSize", default)]
    pub build_size: i64,
    /// The entrypoint file to use to execute the deployment code.
    #[serde(rename = "entrypoint", default)]
    pub entrypoint: String,
    /// Raw manifest.json bytes captured from the source archive at deploy time.
    /// Empty for legacy Function/Site deployments without a manifest.
    #[serde(rename = "manifestJson", default)]
    pub manifest_json: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranch", default)]
    pub provider_branch: String,
    /// The branch of the vcs repository
    #[serde(rename = "providerBranchUrl", default)]
    pub provider_branch_url: String,
    /// The name of vcs commit author
    #[serde(rename = "providerCommitAuthor", default)]
    pub provider_commit_author: String,
    /// The url of vcs commit author
    #[serde(rename = "providerCommitAuthorUrl", default)]
    pub provider_commit_author_url: String,
    /// The commit hash of the vcs commit
    #[serde(rename = "providerCommitHash", default)]
    pub provider_commit_hash: String,
    /// The commit message
    #[serde(rename = "providerCommitMessage", default)]
    pub provider_commit_message: String,
    /// The url of the vcs commit
    #[serde(rename = "providerCommitUrl", default)]
    pub provider_commit_url: String,
    /// The name of the vcs provider repository
    #[serde(rename = "providerRepositoryName", default)]
    pub provider_repository_name: String,
    /// The name of the vcs provider repository owner
    #[serde(rename = "providerRepositoryOwner", default)]
    pub provider_repository_owner: String,
    /// The url of the vcs provider repository
    #[serde(rename = "providerRepositoryUrl", default)]
    pub provider_repository_url: String,
    /// Resource ID.
    #[serde(rename = "resourceId", default)]
    pub resource_id: String,
    /// Resource type.
    #[serde(rename = "resourceType", default)]
    pub resource_type: String,
    /// Screenshot with dark theme preference file ID.
    #[serde(rename = "screenshotDark", default)]
    pub screenshot_dark: String,
    /// Screenshot with light theme preference file ID.
    #[serde(rename = "screenshotLight", default)]
    pub screenshot_light: String,
    /// The code size in bytes.
    #[serde(rename = "sourceSize", default)]
    pub source_size: i64,
    /// The deployment status. Possible values are "waiting", "processing",
    /// "building", "ready", "canceled" and "failed".
    #[serde(rename = "status", default)]
    pub status: String,
    /// The total size in bytes (source and build output).
    #[serde(rename = "totalSize", default)]
    pub total_size: i64,
    /// Type of deployment.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
