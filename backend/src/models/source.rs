use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::common::{MediaLink, MediaLinkInput};

// ---------------------------------------------------------------------------
// Source — Dimension 4: anchored in PostgreSQL
// ---------------------------------------------------------------------------

/// A scholarly source / reference stored in PostgreSQL.
#[derive(Debug, Clone, SimpleObject)]
pub struct Source {
    pub source_id: Uuid,
    pub domain: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publication_era: Option<String>,
    /// The actual reference text (mapped from `reference` column in PG).
    pub reference_text: String,
    pub interpretation_method: Option<String>,
    pub reliability_score: Option<f64>,
    pub reliability_assessment: Option<String>,
    pub media_links: Option<Vec<MediaLink>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

#[derive(Debug, InputObject)]
pub struct CreateSourceInput {
    pub domain: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publication_era: Option<String>,
    pub reference_text: String,
    pub interpretation_method: Option<String>,
    pub reliability_score: Option<f64>,
    pub reliability_assessment: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
}

#[derive(Debug, InputObject)]
pub struct UpdateSourceInput {
    pub domain: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publication_era: Option<String>,
    pub reference_text: Option<String>,
    pub interpretation_method: Option<String>,
    pub reliability_score: Option<f64>,
    pub reliability_assessment: Option<String>,
    pub media_links: Option<Vec<MediaLinkInput>>,
}
