use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

use super::common::*;

// ---------------------------------------------------------------------------
// Event — Dimension 1 of the Knowledge Graph
// ---------------------------------------------------------------------------

/// A historical event node stored in Neo4j.
#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Event {
    pub uuid: Uuid,
    pub title: String,
    pub description: Option<String>,

    /// Optional ISO-8601 date string (e.g. "0622-07-16").
    pub iso_8601: Option<String>,

    pub islamic_date: IslamicDate,
    pub gregorian_date: GregorianDate,
    pub precision: TimePrecision,
    pub curation_tier: CurationTier,
    pub global_hook: GlobalHook,
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

#[derive(Debug, InputObject)]
pub struct CreateEventInput {
    pub title: String,
    pub description: Option<String>,
    pub iso_8601: Option<String>,
    pub islamic_date: IslamicDateInput,
    pub gregorian_date: GregorianDateInput,
    pub precision: TimePrecision,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdateEventInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub iso_8601: Option<String>,
    pub islamic_date: Option<IslamicDateInput>,
    pub gregorian_date: Option<GregorianDateInput>,
    pub precision: Option<TimePrecision>,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
}
