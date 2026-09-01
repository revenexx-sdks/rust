use serde::{Deserialize, Serialize};

/// Replace ALL positions of the list (set semantics).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListItemsReplaceRequest {
    /// The new full set of positions, in the order they should carry. An empty
    /// array empties the list. Every existing position is deleted and rewritten,
    /// so ids are NOT preserved. The array order is the DEFAULT and not an
    /// override: an entry that names no `position` takes its index, one that names
    /// its own keeps it — so a replace does not by itself renumber the list from
    /// zero.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::OrderListItemInput>,
}
