use serde::{Deserialize, Serialize};

/// A Revenexx step marker. The storefront cuts the flat array at each marker
/// and renders the nodes that follow it as one wizard step, then removes the
/// marker before FormKit renders anything. A definition with no marker is a
/// single-step form.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormKitStepMarker {
    /// Stable id for the step, so a client can address it.
    #[serde(rename = "id", default)]
    pub id: String,
    /// What the step is: 'fields' for a normal step, 'thankyou' for the
    /// confirmation panel shown after a successful submit.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The step heading the visitor reads.
    #[serde(rename = "title", default)]
    pub title: String,
}
