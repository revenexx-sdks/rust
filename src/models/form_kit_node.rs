use serde::{Deserialize, Serialize};

/// One node of a form definition.
/// 
/// A definition is a FLAT ARRAY of these, and the storefront hands each one to
/// `<FormKitSchema>` verbatim — it maps nothing, so every key FormKit
/// understands works here whether or not it is named below (`options`, `if`,
/// `rows`, `autocomplete`, `min`, `max`, `$cmp`, …). Three kinds of node
/// occur:
/// 
/// • an INPUT node (`$formkit`) collects a value and, if it carries a
/// `name`, contributes exactly one key to a submission's `data`;
/// • a CONTENT node (`$el`) renders markup — a paragraph of legal text, a
/// heading — and collects nothing;
/// • a STEP MARKER (`$rxStep`) is a Revenexx extension the storefront
/// consumes and strips before FormKit sees the node; it splits the flat array
/// into wizard steps.
/// 
/// Only the four keys `name`, `label`, `placeholder` and `help` are read by
/// Revenexx code at all (the last three are what the per-form i18n overlay
/// translates). Everything else is FormKit's business.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormKitNode {
    /// A CONTENT node instead of an input: a raw element name ('p', 'h2', 'div').
    /// It collects no value and contributes no key to `data`.
    #[serde(rename = "$el", default)]
    pub el: String,
    /// An INPUT node: the FormKit input type — 'text', 'email', 'textarea',
    /// 'number', 'select', 'checkbox', 'radio', 'date', 'group', 'list', … . The
    /// set is FormKit's, not this app's, which is why nothing here enforces it and
    /// no vocabulary is published for it; the storefront adds one input of its
    /// own, `datepicker`, and three validation rules (`zip`, `companyName`,
    /// `phoneNumber`).
    #[serde(rename = "$formkit", default)]
    pub formkit: String,
    /// A Revenexx step marker. The storefront cuts the flat array at each marker
    /// and renders the nodes that follow it as one wizard step, then removes the
    /// marker before FormKit renders anything. A definition with no marker is a
    /// single-step form.
    #[serde(rename = "$rxStep", default)]
    pub rx_step: crate::models::FormKitStepMarker,
    /// The content of an `$el` node: a string of text, or nested nodes.
    #[serde(rename = "children", default)]
    pub children: String,
    /// The hint under the input. Translatable.
    #[serde(rename = "help", default)]
    pub help: String,
    /// What the visitor reads above the input. Translatable: the per-form i18n
    /// overlay replaces it per locale.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The key this input writes into a submission's `data` — `{ "$formkit":
    /// "email", "name": "email" }` here is the `"email"` key there, and that
    /// correspondence is the whole contract between a form and its inbox. A node
    /// with a non-empty `name` is a FIELD: only fields count against the tenant's
    /// `max_form_fields`, so a form with twenty paragraphs of legal text and three
    /// inputs is a three-field form. A `group` or `list` input nests, and its
    /// `name` keys the nested object or array.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Placeholder text inside the input. Translatable.
    #[serde(rename = "placeholder", default)]
    pub placeholder: String,
    /// A Revenexx hint about where the value comes from rather than what it looks
    /// like. 'product' means the storefront prefills this input from the page
    /// context or the query string (`?sku=…`) and renders it read-only — how a
    /// price request knows which article it is about. Stripped before FormKit
    /// renders the node.
    #[serde(rename = "rxKind", default)]
    pub rx_kind: String,
    /// FormKit validation, in either notation FormKit accepts: the pipe string
    /// 'required|email', or the array form. It is enforced in the browser by
    /// FormKit — this API stores whatever `data` it is sent, so a server-side
    /// integration must not treat it as a guarantee.
    #[serde(rename = "validation", default)]
    pub validation: String,
}
