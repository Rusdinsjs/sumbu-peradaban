use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

use super::common::*;

// ---------------------------------------------------------------------------
// Location — Dimension 3 of the Knowledge Graph
// ---------------------------------------------------------------------------

/// A geographic or conceptual location stored in Neo4j.
#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Location {
    pub uuid: Uuid,
    pub name: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub precision: LocationPrecision,
    /// Whether this location exists beyond physical geography
    /// (e.g. Sidratul Muntaha, Isra' Mi'raj route).
    pub is_transcendental: bool,
    pub curation_tier: CurationTier,
    pub global_hook: GlobalHook,
    pub geography_climate: Option<String>,
    pub demographics: Option<String>,
    pub socio_cultural: Option<String>,
    pub historical_role: Option<String>,
    pub media_links: Option<Vec<MediaLink>>,
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

#[derive(Debug, InputObject)]
pub struct CreateLocationInput {
    pub name: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub precision: LocationPrecision,
    pub is_transcendental: Option<bool>,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
    pub geography_climate: Option<String>,
    pub demographics: Option<String>,
    pub socio_cultural: Option<String>,
    pub historical_role: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
}

#[allow(dead_code)]
#[derive(Debug, InputObject)]
pub struct UpdateLocationInput {
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub precision: Option<LocationPrecision>,
    pub is_transcendental: Option<bool>,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
    pub geography_climate: Option<String>,
    pub demographics: Option<String>,
    pub socio_cultural: Option<String>,
    pub historical_role: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
}
