use serde::{Deserialize, Serialize};

/// The product as the buyer was shown it when this line was added — the
/// cart's own copy, so it stays honest when the catalogue moves underneath it.
/// Free-form apart from the price: conversion reads `unit_price` (or `price`
/// as a fallback) and nothing else. A snapshot without a readable price leaves
/// the line alone in both price modes, which is deliberate — a missing
/// snapshot must never be read as "free".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItemSnapshot {
    /// The older spelling of the same thing, read only when `unit_price` is
    /// absent.
    #[serde(rename = "price", default)]
    pub price: f64,
    /// The net unit price the buyer was shown. This is what carts.order books the
    /// line on under price_snapshot_mode = snapshot, and what it rewrites under =
    /// live.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
}
