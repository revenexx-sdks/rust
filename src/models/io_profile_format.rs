use serde::{Deserialize, Serialize};

/// Profile source/sink format. `bmecat` is profile-only — the ad-hoc
/// `/io/imports` and `/io/exports` endpoints do not accept it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfileFormat {
}
