use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Curation Tier — tracks editorial confidence level
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum CurationTier {
    Draft,
    Verified,
    Reviewed,
    Canonical,
}

impl CurationTier {
    /// Map from the PostgreSQL enum literal to the Rust variant.
    pub fn from_pg(s: &str) -> Self {
        match s {
            "draft" => Self::Draft,
            "verified" => Self::Verified,
            "reviewed" => Self::Reviewed,
            "canonical" => Self::Canonical,
            _ => Self::Draft,
        }
    }

    /// Map to the PostgreSQL enum literal.
    pub fn as_pg(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Reviewed => "reviewed",
            Self::Canonical => "canonical",
        }
    }
}

// ---------------------------------------------------------------------------
// Time Precision — granularity of a date reference
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum TimePrecision {
    Exact,
    Year,
    Decade,
    Century,
    Approximate,
}

// ---------------------------------------------------------------------------
// Location Precision
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum LocationPrecision {
    Point,
    Area,
    Conceptual,
}

// ---------------------------------------------------------------------------
// Actor Type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum ActorType {
    Individual,
    Group,
}

// ---------------------------------------------------------------------------
// Event Relation — edge types between events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum EventRelation {
    Caused,
    Preceded,
    Influenced,
    ContemporaryWith,
}

// ---------------------------------------------------------------------------
// Islamic Date (output / query type)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct IslamicDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

/// Input variant of [`IslamicDate`] for mutations.
#[derive(Debug, Clone, InputObject)]
pub struct IslamicDateInput {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

// ---------------------------------------------------------------------------
// Gregorian Date (output / query type)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct GregorianDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

/// Input variant of [`GregorianDate`] for mutations.
#[derive(Debug, Clone, InputObject)]
pub struct GregorianDateInput {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

// ---------------------------------------------------------------------------
// Global Hook — link to world-history pivot points
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct GlobalHook {
    pub is_connected_to_global: bool,
    pub global_pivot_category: Option<String>,
}

/// Input variant of [`GlobalHook`] for mutations.
#[allow(dead_code)]
#[derive(Debug, Clone, InputObject)]
pub struct GlobalHookInput {
    pub is_connected_to_global: bool,
    pub global_pivot_category: Option<String>,
}

// ---------------------------------------------------------------------------
// Media Link Structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct MediaLink {
    pub media_type: String, // "text" | "audio" | "image" | "video"
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct MediaLinkInput {
    pub media_type: String, // "text" | "audio" | "image" | "video"
    pub url: String,
    pub title: Option<String>,
}
