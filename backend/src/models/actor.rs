use async_graphql::{InputObject, SimpleObject};

use uuid::Uuid;

use super::common::*;

// ---------------------------------------------------------------------------
// Actor — Dimension 2 of the Knowledge Graph
// ---------------------------------------------------------------------------

/// A historical actor (individual or group) stored in Neo4j.
#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Actor {
    pub uuid: Uuid,
    pub name: String,
    pub actor_type: ActorType,
    pub cultural_sphere: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub curation_tier: CurationTier,
    pub global_hook: GlobalHook,
    pub works: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub description: Option<String>,
    pub media_links: Option<Vec<MediaLink>>,
}

/// Actor with a specific relationship label/type.
#[derive(Debug, Clone, SimpleObject)]
pub struct RelatedActor {
    pub actor: Actor,
    pub relationship_type: String,
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
    pub works: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub description: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
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
    pub works: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub description: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
}
