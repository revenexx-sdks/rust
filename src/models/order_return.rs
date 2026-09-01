use serde::{Deserialize, Serialize};

/// Goods coming BACK, with their own lifecycle: registered → received →
/// completed | rejected. Only completing books anything onto the positions;
/// registering and receiving are announcements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturn {
    /// When the return was settled, stamped by the SERVER. Never taken from the
    /// body: a client clock records when a client thinks it acted, not when the
    /// goods were booked.
    #[serde(rename = "completed_at", default)]
    pub completed_at: String,
    /// When the return row was written.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the return. The {rid} segment of the return routes.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Free-form data for the caller — the returns portal's own reference.
    /// Stored and returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The RETURN number — drawn from the tenant's return range, unique per
    /// tenant, and a third series alongside orders and delivery notes. What the
    /// customer writes on the parcel.
    #[serde(rename = "number", default)]
    pub number: String,
    /// The order the goods are coming back from. A return of another order is a
    /// 404 on these routes, not a cross-order write.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// The positions and quantities this return covers, fixed when it was
    /// registered and guarded against the shipped-but-not-yet-returned quantity of
    /// each. Entries flagged restock are what the completion reports back for the
    /// inventories call.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderReturnedPosition>,
    /// Why the goods are coming back, free text as the customer or the desk stated
    /// it. Also what /reject stores when it is given no resolution out of the
    /// published set.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// When the goods physically arrived back. Null until POST …/receive — and
    /// null forever on a return that was completed straight out of registered,
    /// which is allowed.
    #[serde(rename = "received_at", default)]
    pub received_at: String,
    /// When the return was announced. Defaults to now.
    #[serde(rename = "registered_at", default)]
    pub registered_at: String,
    /// When the return was refused. Null unless it was.
    #[serde(rename = "rejected_at", default)]
    pub rejected_at: String,
    /// How it ended, in one of the words this app publishes — the settlement
    /// words on a completion (refund, partial_refund, replacement, repair,
    /// store_credit), the refusal words on a rejection (wear_and_tear,
    /// not_returnable); GET /orders/vocabularies/return-resolutions carries both
    /// sets with the stage that accepts each. The column carries no database
    /// constraint; the ROUTES enforce the set, which is what stopped a client
    /// settling returns with a word nobody else knew. On a rejection that named no
    /// resolution, the free-text reason is stored here instead — which is the
    /// one case a value outside the two sets appears.
    #[serde(rename = "resolution", default)]
    pub resolution: String,
    /// Where the return stands: 'registered' = announced, nothing booked;
    /// 'received' = the goods are back but not yet settled; 'completed' = settled,
    /// and the only transition that books quantity_returned; 'rejected' = refused,
    /// nothing booked. The last two are final.
    #[serde(rename = "status", default)]
    pub status: String,
    /// When the return last changed — each of its transitions writes it.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
