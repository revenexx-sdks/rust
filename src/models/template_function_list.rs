use serde::{Deserialize, Serialize};

/// Function Templates List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateFunctionList {
    /// List of templates.
    #[serde(rename = "templates", default)]
    pub templates: Vec<crate::models::TemplateFunction>,
    /// Total number of templates that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
