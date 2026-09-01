use serde::{Deserialize, Serialize};

/// No body. Everything this needs is the path id and what the catalog already
/// holds; send `{}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCompletenessRequest {
}
