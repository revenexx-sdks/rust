use serde::{Deserialize, Serialize};

/// Site
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Site {
    /// Site creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Site ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Site update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Site framework adapter.
    #[serde(rename = "adapter", default)]
    pub adapter: String,
    /// The build command used to build the site.
    #[serde(rename = "buildCommand", default)]
    pub build_command: String,
    /// Site build runtime.
    #[serde(rename = "buildRuntime", default)]
    pub build_runtime: String,
    /// Active deployment creation date in ISO 8601 format.
    #[serde(rename = "deploymentCreatedAt", default)]
    pub deployment_created_at: String,
    /// Site's active deployment ID.
    #[serde(rename = "deploymentId", default)]
    pub deployment_id: String,
    /// Screenshot of active deployment with dark theme preference file ID.
    #[serde(rename = "deploymentScreenshotDark", default)]
    pub deployment_screenshot_dark: String,
    /// Screenshot of active deployment with light theme preference file ID.
    #[serde(rename = "deploymentScreenshotLight", default)]
    pub deployment_screenshot_light: String,
    /// Site enabled.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Name of fallback file to use instead of 404 page. If null, Appwrite 404
    /// page will be displayed.
    #[serde(rename = "fallbackFile", default)]
    pub fallback_file: String,
    /// Site framework.
    #[serde(rename = "framework", default)]
    pub framework: String,
    /// The install command used to install the site dependencies.
    #[serde(rename = "installCommand", default)]
    pub install_command: String,
    /// Site VCS (Version Control System) installation id.
    #[serde(rename = "installationId", default)]
    pub installation_id: String,
    /// Latest deployment creation date in ISO 8601 format.
    #[serde(rename = "latestDeploymentCreatedAt", default)]
    pub latest_deployment_created_at: String,
    /// Site's latest deployment ID.
    #[serde(rename = "latestDeploymentId", default)]
    pub latest_deployment_id: String,
    /// Status of latest deployment. Possible values are "waiting", "processing",
    /// "building", "ready", and "failed".
    #[serde(rename = "latestDeploymentStatus", default)]
    pub latest_deployment_status: String,
    /// Is the site deployed with the latest configuration? This is set to false if
    /// you've changed an environment variables, entrypoint, commands, or other
    /// settings that needs redeploy to be applied. When the value is false,
    /// redeploy the site to update it with the latest configuration.
    #[serde(rename = "live", default)]
    pub live: bool,
    /// When disabled, request logs will exclude logs and errors, and site
    /// responses will be slightly faster.
    #[serde(rename = "logging", default)]
    pub logging: bool,
    /// Site name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The directory where the site build output is located.
    #[serde(rename = "outputDirectory", default)]
    pub output_directory: String,
    /// VCS (Version Control System) branch name
    #[serde(rename = "providerBranch", default)]
    pub provider_branch: String,
    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId", default)]
    pub provider_repository_id: String,
    /// Path to site in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory", default)]
    pub provider_root_directory: String,
    /// Is VCS (Version Control System) connection is in silent mode? When in
    /// silence mode, no comments will be posted on the repository pull or merge
    /// requests
    #[serde(rename = "providerSilentMode", default)]
    pub provider_silent_mode: bool,
    /// Machine specification for builds and executions.
    #[serde(rename = "specification", default)]
    pub specification: String,
    /// Site request timeout in seconds.
    #[serde(rename = "timeout", default)]
    pub timeout: i64,
    /// Site variables.
    #[serde(rename = "vars", default)]
    pub vars: Vec<crate::models::Variable>,
}
