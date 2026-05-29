use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

use super::common::*;

// ---------------------------------------------------------------------------
// Actor — Dimension 2 of the Knowledge Graph
// ---------------------------------------------------------------------------

/// A historical actor (individual or group) stored in Neo4j.
#[derive(Debug, Clone, SimpleObject)]
pub struct Actor {
    pub uuid: Uuid,
    pub name: String,
    pub actor_type: ActorType,
    pub cultural_sphere: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub curation_tier: CurationTier,
    pub global_hook: GlobalHook,
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

#[derive(Debug, InputObject)]
pub struct CreateActorInput {
    pub name: String,
    pub actor_type: ActorType,
    pub cultural_sphere: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdateActorInput {
    pub name: Option<String>,
    pub actor_type: Option<ActorType>,
    pub cultural_sphere: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub is_connected_to_global: Option<bool>,
    pub global_pivot_category: Option<String>,
}
