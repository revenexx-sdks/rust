use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, echoed with the
/// values as they arrived. A query parameter that is not a filterable column
/// of this entity is DROPPED rather than refused, and is simply missing here
/// — so an empty object next to a query string that had a filter in it means
/// the filter was misspelled, and is the only way to tell that from a filter
/// that matched nothing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionListFilter {
    /// The `created_at` filter, verbatim as the query string carried it. A string
    /// here whatever the column's own type.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The `form_id` filter, verbatim as the query string carried it. A string
    /// here whatever the column's own type.
    #[serde(rename = "form_id", default)]
    pub form_id: String,
    /// The `form_slug` filter, verbatim as the query string carried it. A string
    /// here whatever the column's own type.
    #[serde(rename = "form_slug", default)]
    pub form_slug: String,
    /// The `id` filter, verbatim as the query string carried it. A string here
    /// whatever the column's own type.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The `source` filter, verbatim as the query string carried it. A string here
    /// whatever the column's own type.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The `status` filter, verbatim as the query string carried it. A string here
    /// whatever the column's own type — and NOT necessarily one of the permitted
    /// values: `?status=zzz` is echoed back unchanged and matches nothing, which
    /// is the point of the echo.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The `updated_at` filter, verbatim as the query string carried it. A string
    /// here whatever the column's own type.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
