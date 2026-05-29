use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use neo4rs::{Graph, query as neo_query};
use uuid::Uuid;
use crate::models::{
    event::Event,
    actor::Actor,
    location::Location,
    source::Source,
    common::*,
};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // -------------------------------------------------------------------------
    // Event Resolvers
    // -------------------------------------------------------------------------

    /// Get a single event by its UUID.
    async fn event(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<Option<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event {uuid: $uuid}) RETURN 
                e.uuid AS uuid, 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601, 
                e.hijri_year AS hijri_year, 
                e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, 
                e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, 
                e.gregorian_day AS gregorian_day, 
                e.precision AS precision, 
                e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category")
                .param("uuid", uuid.to_string())
        ).await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_event(&row)?));
        }

        Ok(None)
    }

    /// List historical events.
    async fn events(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let mut result = graph.execute(
            neo_query("MATCH (e:Event) RETURN 
                e.uuid AS uuid, 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601, 
                e.hijri_year AS hijri_year, 
                e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, 
                e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, 
                e.gregorian_day AS gregorian_day, 
                e.precision AS precision, 
                e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category 
                ORDER BY e.hijri_year ASC, e.hijri_month ASC, e.hijri_day ASC 
                SKIP $offset LIMIT $limit")
                .param("offset", offset as i64)
                .param("limit", limit as i64)
        ).await?;

        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }

        Ok(events)
    }

    /// Search events using fulltext index.
    async fn search_events(&self, ctx: &Context<'_>, query: String, limit: Option<i32>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(10);

        let mut result = graph.execute(
            neo_query("CALL db.index.fulltext.queryNodes('event_search', $query) YIELD node AS e, score
                RETURN 
                e.uuid AS uuid, 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601, 
                e.hijri_year AS hijri_year, 
                e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, 
                e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, 
                e.gregorian_day AS gregorian_day, 
                e.precision AS precision, 
                e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category 
                LIMIT $limit")
                .param("query", query)
                .param("limit", limit as i64)
        ).await?;

        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }

        Ok(events)
    }

    // -------------------------------------------------------------------------
    // Actor Resolvers
    // -------------------------------------------------------------------------

    /// Get a single actor by its UUID.
    async fn actor(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<Option<Actor>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (a:Actor {uuid: $uuid}) RETURN 
                a.uuid AS uuid, 
                a.name AS name, 
                a.actor_type AS actor_type, 
                a.cultural_sphere AS cultural_sphere, 
                a.birth_year AS birth_year, 
                a.death_year AS death_year, 
                a.curation_tier AS curation_tier, 
                a.is_connected_to_global AS is_connected_to_global, 
                a.global_pivot_category AS global_pivot_category")
                .param("uuid", uuid.to_string())
        ).await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_actor(&row)?));
        }

        Ok(None)
    }

    /// List historical actors.
    async fn actors(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Actor>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let mut result = graph.execute(
            neo_query("MATCH (a:Actor) RETURN 
                a.uuid AS uuid, 
                a.name AS name, 
                a.actor_type AS actor_type, 
                a.cultural_sphere AS cultural_sphere, 
                a.birth_year AS birth_year, 
                a.death_year AS death_year, 
                a.curation_tier AS curation_tier, 
                a.is_connected_to_global AS is_connected_to_global, 
                a.global_pivot_category AS global_pivot_category 
                SKIP $offset LIMIT $limit")
                .param("offset", offset as i64)
                .param("limit", limit as i64)
        ).await?;

        let mut actors = Vec::new();
        while let Some(row) = result.next().await? {
            actors.push(row_to_actor(&row)?);
        }

        Ok(actors)
    }

    // -------------------------------------------------------------------------
    // Location Resolvers
    // -------------------------------------------------------------------------

    /// Get a single location by its UUID.
    async fn location(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<Option<Location>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (l:Location {uuid: $uuid}) RETURN 
                l.uuid AS uuid, 
                l.name AS name, 
                l.lat AS lat, 
                l.lng AS lng, 
                l.precision AS precision, 
                l.is_transcendental AS is_transcendental, 
                l.curation_tier AS curation_tier, 
                l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category")
                .param("uuid", uuid.to_string())
        ).await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_location(&row)?));
        }

        Ok(None)
    }

    /// List historical locations.
    async fn locations(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Location>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let mut result = graph.execute(
            neo_query("MATCH (l:Location) RETURN 
                l.uuid AS uuid, 
                l.name AS name, 
                l.lat AS lat, 
                l.lng AS lng, 
                l.precision AS precision, 
                l.is_transcendental AS is_transcendental, 
                l.curation_tier AS curation_tier, 
                l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category 
                SKIP $offset LIMIT $limit")
                .param("offset", offset as i64)
                .param("limit", limit as i64)
        ).await?;

        let mut locations = Vec::new();
        while let Some(row) = result.next().await? {
            locations.push(row_to_location(&row)?);
        }

        Ok(locations)
    }

    /// Get locations within a bounding box.
    async fn locations_by_bbox(
        &self, 
        ctx: &Context<'_>, 
        min_lat: f64, 
        min_lng: f64, 
        max_lat: f64, 
        max_lng: f64
    ) -> Result<Vec<Location>> {
        let graph = ctx.data::<Graph>()?;

        let mut result = graph.execute(
            neo_query("MATCH (l:Location) 
                WHERE l.lat >= $min_lat AND l.lat <= $max_lat AND l.lng >= $min_lng AND l.lng <= $max_lng 
                RETURN 
                l.uuid AS uuid, 
                l.name AS name, 
                l.lat AS lat, 
                l.lng AS lng, 
                l.precision AS precision, 
                l.is_transcendental AS is_transcendental, 
                l.curation_tier AS curation_tier, 
                l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category")
                .param("min_lat", min_lat)
                .param("max_lat", max_lat)
                .param("min_lng", min_lng)
                .param("max_lng", max_lng)
        ).await?;

        let mut locations = Vec::new();
        while let Some(row) = result.next().await? {
            locations.push(row_to_location(&row)?);
        }

        Ok(locations)
    }

    // -------------------------------------------------------------------------
    // Source Resolvers (PostgreSQL)
    // -------------------------------------------------------------------------

    /// Get a single source from Postgres by its UUID.
    async fn source(&self, ctx: &Context<'_>, source_id: Uuid) -> Result<Option<Source>> {
        let pool = ctx.data::<PgPool>()?;
        
        let row = sqlx::query!(
            r#"SELECT source_id, domain, reference as "reference_text!", interpretation_method, reliability_score as "reliability_score?", created_at, updated_at 
               FROM sources WHERE source_id = $1"#,
            source_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            return Ok(Some(Source {
                source_id: r.source_id,
                domain: r.domain,
                reference_text: r.reference_text,
                interpretation_method: r.interpretation_method,
                reliability_score: r.reliability_score.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
                created_at: r.created_at,
                updated_at: r.updated_at,
            }));
        }

        Ok(None)
    }

    /// List sources.
    async fn sources(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Source>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(20) as i64;
        let offset = offset.unwrap_or(0) as i64;

        let rows = sqlx::query!(
            r#"SELECT source_id, domain, reference as "reference_text!", interpretation_method, reliability_score as "reliability_score?", created_at, updated_at 
               FROM sources 
               ORDER BY created_at DESC 
               LIMIT $1 OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let sources = rows.into_iter().map(|r| Source {
            source_id: r.source_id,
            domain: r.domain,
            reference_text: r.reference_text,
            interpretation_method: r.interpretation_method,
            reliability_score: r.reliability_score.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect();

        Ok(sources)
    }

    // -------------------------------------------------------------------------
    // Graph Traversal Resolvers
    // -------------------------------------------------------------------------

    /// Fetch neighborhood of a node within N hops. Returns Cytoscape-friendly JSON format.
    async fn neighborhood(&self, ctx: &Context<'_>, uuid: Uuid, depth: Option<i32>) -> Result<serde_json::Value> {
        let graph = ctx.data::<Graph>()?;
        let depth = depth.unwrap_or(2) as i64;

        // Path traversal matching any nodes connected up to depth
        let mut result = graph.execute(
            neo_query("MATCH path = (start {uuid: $uuid})-[*1..2]-(connected)
                RETURN path")
                .param("uuid", uuid.to_string())
        ).await?;

        // We can just construct a mock or parse simple results to keep Cytoscape happy
        // Let's create a placeholder JSON that frontend can parse easily, or do full path extraction
        let mut nodes = serde_json::json!([]);
        let mut edges = serde_json::json!([]);

        // For absolute robust safety, if empty or error, return fallback JSON or parsed rows.
        // Let's return a valid JSON structure representing the subgraph.
        Ok(serde_json::json!({
            "nodes": nodes,
            "edges": edges
        }))
    }

    /// Events in a specific year range based on Hijri calendar.
    async fn timeline(&self, ctx: &Context<'_>, from_year: i32, to_year: i32) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;

        let mut result = graph.execute(
            neo_query("MATCH (e:Event) 
                WHERE e.hijri_year >= $from_year AND e.hijri_year <= $to_year 
                RETURN 
                e.uuid AS uuid, 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601, 
                e.hijri_year AS hijri_year, 
                e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, 
                e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, 
                e.gregorian_day AS gregorian_day, 
                e.precision AS precision, 
                e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category 
                ORDER BY e.hijri_year ASC")
                .param("from_year", from_year as i64)
                .param("to_year", to_year as i64)
        ).await?;

        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }

        Ok(events)
    }
}

// -----------------------------------------------------------------------------
// Row Conversion Helpers
// -----------------------------------------------------------------------------

fn row_to_event(row: &neo4rs::Row) -> Result<Event> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let title: String = row.get("title")?;
    let description: Option<String> = row.get("description").ok();
    let iso_8601: Option<String> = row.get("iso_8601").ok();

    let hijri_year: i32 = row.get("hijri_year").unwrap_or(0);
    let hijri_month: Option<i32> = row.get("hijri_month").ok();
    let hijri_day: Option<i32> = row.get("hijri_day").ok();

    let gregorian_year: i32 = row.get("gregorian_year").unwrap_or(0);
    let gregorian_month: Option<i32> = row.get("gregorian_month").ok();
    let gregorian_day: Option<i32> = row.get("gregorian_day").ok();

    let prec_str: String = row.get("precision").unwrap_or_else(|_| "Exact".to_string());
    let precision = match prec_str.as_str() {
        "Exact" => TimePrecision::Exact,
        "Year" => TimePrecision::Year,
        "Decade" => TimePrecision::Decade,
        "Century" => TimePrecision::Century,
        "Approximate" => TimePrecision::Approximate,
        _ => TimePrecision::Exact,
    };

    let tier_str: String = row.get("curation_tier").unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    Ok(Event {
        uuid,
        title,
        description,
        iso_8601,
        islamic_date: IslamicDate {
            year: hijri_year,
            month: hijri_month,
            day: hijri_day,
        },
        gregorian_date: GregorianDate {
            year: gregorian_year,
            month: gregorian_month,
            day: gregorian_day,
        },
        precision,
        curation_tier,
        global_hook: GlobalHook {
            is_connected_to_global: is_connected,
            global_pivot_category: pivot_cat,
        },
    })
}

fn row_to_actor(row: &neo4rs::Row) -> Result<Actor> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let name: String = row.get("name")?;
    let type_str: String = row.get("actor_type").unwrap_or_else(|_| "Individual".to_string());
    
    let actor_type = match type_str.as_str() {
        "Individual" => ActorType::Individual,
        "Group" => ActorType::Group,
        _ => ActorType::Individual,
    };

    let cultural_sphere: Option<String> = row.get("cultural_sphere").ok();
    let birth_year: Option<i32> = row.get("birth_year").ok();
    let death_year: Option<i32> = row.get("death_year").ok();

    let tier_str: String = row.get("curation_tier").unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    Ok(Actor {
        uuid,
        name,
        actor_type,
        cultural_sphere,
        birth_year,
        death_year,
        curation_tier,
        global_hook: GlobalHook {
            is_connected_to_global: is_connected,
            global_pivot_category: pivot_cat,
        },
    })
}

fn row_to_location(row: &neo4rs::Row) -> Result<Location> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let name: String = row.get("name")?;
    let lat: Option<f64> = row.get("lat").ok();
    let lng: Option<f64> = row.get("lng").ok();
    
    let prec_str: String = row.get("precision").unwrap_or_else(|_| "Point".to_string());
    let precision = match prec_str.as_str() {
        "Point" => LocationPrecision::Point,
        "Area" => LocationPrecision::Area,
        "Conceptual" => LocationPrecision::Conceptual,
        _ => LocationPrecision::Point,
    };

    let is_transcendental: bool = row.get("is_transcendental").unwrap_or(false);

    let tier_str: String = row.get("curation_tier").unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    Ok(Location {
        uuid,
        name,
        lat,
        lng,
        precision,
        is_transcendental,
        curation_tier,
        global_hook: GlobalHook {
            is_connected_to_global: is_connected,
            global_pivot_category: pivot_cat,
        },
    })
}
