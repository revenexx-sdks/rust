use serde::{Deserialize, Serialize};

/// No payload — send {}. Which market is promoted comes from the path, and
/// there is nothing else to say.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketMakeDefaultRequest {
}
