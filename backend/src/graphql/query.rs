use crate::models::{actor::{Actor, RelatedActor}, common::*, event::Event, location::{Location, RelatedLocation}, source::{Source, RelatedSource}, auth::{User, UserRole, Claims}};
use async_graphql::{Context, Object, ComplexObject, Result, Error};
use neo4rs::{query as neo_query, Graph};
use sqlx::PgPool;
use uuid::Uuid;

fn row_to_source(r: &sqlx::postgres::PgRow) -> Result<Source> {
    use sqlx::Row;
    use bigdecimal::ToPrimitive;

    let media_links_str: Option<String> = r.try_get("media_links").unwrap_or(None);
    let media_links: Option<Vec<MediaLink>> = media_links_str.and_then(|s| serde_json::from_str(&s).ok());
    let bd: Option<bigdecimal::BigDecimal> = r.try_get("reliability_score").unwrap_or(None);

    Ok(Source {
        source_id: r.get("source_id"),
        domain: r.get("domain"),
        title: r.try_get("title").unwrap_or(None),
        author: r.try_get("author").unwrap_or(None),
        publication_era: r.try_get("publication_era").unwrap_or(None),
        reference_text: r.get("reference"),
        interpretation_method: r.try_get("interpretation_method").unwrap_or(None),
        reliability_score: bd.and_then(|b| b.to_f64()),
        reliability_assessment: r.try_get("reliability_assessment").unwrap_or(None),
        media_links,
        sub_references: None,
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    })
}

#[derive(async_graphql::SimpleObject)]
pub struct SearchResultItem {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub entity_type: String, // "event" | "actor" | "location" | "source"
}

#[derive(async_graphql::SimpleObject)]
pub struct AuditLogItem {
    pub id: i64,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub performed_by: String,
    pub performed_at: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct SyncStatus {
    pub postgres_connected: bool,
    pub neo4j_connected: bool,
    pub postgres_records_count: i64,
    pub neo4j_nodes_count: i64,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // -------------------------------------------------------------------------
    // User / Auth Resolvers
    // -------------------------------------------------------------------------
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        let pool = ctx.data::<PgPool>()?;
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at FROM users WHERE id = $1"#,
            claims.sub
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can view all users"));
        }
        let pool = ctx.data::<PgPool>()?;
        let users = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at FROM users ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await?;
        Ok(users)
    }

    // -------------------------------------------------------------------------
    // Event Resolvers
    // -------------------------------------------------------------------------

    /// Get a single event by its UUID.
    async fn event(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<Option<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (e:Event {uuid: $uuid}) RETURN 
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
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links",
                )
                .param("uuid", uuid.to_string()),
            )
            .await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_event(&row)?));
        }

        Ok(None)
    }

    /// List historical events.
    async fn events(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);

        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (e:Event) RETURN 
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
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links 
                ORDER BY e.hijri_year ASC, e.hijri_month ASC, e.hijri_day ASC 
                SKIP $offset LIMIT $limit",
                )
                .param("offset", offset as i64)
                .param("limit", limit as i64),
            )
            .await?;

        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }

        Ok(events)
    }

    /// Search events using fulltext index.
    async fn search_events(
        &self,
        ctx: &Context<'_>,
        query: String,
        limit: Option<i32>,
    ) -> Result<Vec<Event>> {
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
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links 
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
        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (a:Actor {uuid: $uuid}) RETURN 
                a.uuid AS uuid, 
                a.name AS name, 
                a.actor_type AS actor_type, 
                a.cultural_sphere AS cultural_sphere, 
                a.birth_year AS birth_year, 
                a.death_year AS death_year, 
                a.curation_tier AS curation_tier, 
                a.is_connected_to_global AS is_connected_to_global, 
                a.global_pivot_category AS global_pivot_category,
                a.works AS works,
                a.roles AS roles,
                a.description AS description,
                a.media_links AS media_links",
                )
                .param("uuid", uuid.to_string()),
            )
            .await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_actor(&row)?));
        }

        Ok(None)
    }

    /// List historical actors.
    async fn actors(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Actor>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);

        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (a:Actor) RETURN 
                a.uuid AS uuid, 
                a.name AS name, 
                a.actor_type AS actor_type, 
                a.cultural_sphere AS cultural_sphere, 
                a.birth_year AS birth_year, 
                a.death_year AS death_year, 
                a.curation_tier AS curation_tier, 
                a.is_connected_to_global AS is_connected_to_global, 
                a.global_pivot_category AS global_pivot_category,
                a.works AS works,
                a.roles AS roles,
                a.description AS description,
                a.media_links AS media_links
                SKIP $offset LIMIT $limit",
                )
                .param("offset", offset as i64)
                .param("limit", limit as i64),
            )
            .await?;

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
        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (l:Location {uuid: $uuid}) RETURN 
                l.uuid AS uuid, 
                l.name AS name, 
                l.lat AS lat, 
                l.lng AS lng, 
                l.precision AS precision, 
                l.is_transcendental AS is_transcendental, 
                l.curation_tier AS curation_tier, 
                l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category,
                l.geography_climate AS geography_climate,
                l.demographics AS demographics,
                l.socio_cultural AS socio_cultural,
                l.historical_role AS historical_role,
                l.media_links AS media_links",
                )
                .param("uuid", uuid.to_string()),
            )
            .await?;

        if let Some(row) = result.next().await? {
            return Ok(Some(row_to_location(&row)?));
        }

        Ok(None)
    }

    /// List historical locations.
    async fn locations(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Location>> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);

        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (l:Location) RETURN 
                l.uuid AS uuid, 
                l.name AS name, 
                l.lat AS lat, 
                l.lng AS lng, 
                l.precision AS precision, 
                l.is_transcendental AS is_transcendental, 
                l.curation_tier AS curation_tier, 
                l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category,
                l.geography_climate AS geography_climate,
                l.demographics AS demographics,
                l.socio_cultural AS socio_cultural,
                l.historical_role AS historical_role,
                l.media_links AS media_links
                SKIP $offset LIMIT $limit",
                )
                .param("offset", offset as i64)
                .param("limit", limit as i64),
            )
            .await?;

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
        max_lng: f64,
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
                l.global_pivot_category AS global_pivot_category,
                l.geography_climate AS geography_climate,
                l.demographics AS demographics,
                l.socio_cultural AS socio_cultural,
                l.historical_role AS historical_role,
                l.media_links AS media_links")
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

        let row = sqlx::query(
            "SELECT source_id, domain, title, author, publication_era, reference, interpretation_method, reliability_score, reliability_assessment, media_links, created_at, updated_at \
             FROM sources WHERE source_id = $1"
        )
        .bind(source_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            return Ok(Some(row_to_source(&r)?));
        }

        Ok(None)
    }

    /// List sources.
    async fn sources(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Source>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(20) as i64;
        let offset = offset.unwrap_or(0) as i64;

        let rows = sqlx::query(
            "SELECT source_id, domain, title, author, publication_era, reference, interpretation_method, reliability_score, reliability_assessment, media_links, created_at, updated_at \
             FROM sources \
             ORDER BY created_at DESC \
             LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let mut sources = Vec::new();
        for r in rows {
            sources.push(row_to_source(&r)?);
        }

        Ok(sources)
    }

    #[graphql(name = "searchAll")]
    async fn search_all(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> Result<Vec<SearchResultItem>> {
        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;

        let search_pattern = format!("(?i).*{}.*", query);
        let mut results = Vec::new();

        // 1. Search Neo4j for Events, Actors, Locations
        let mut neo_res = graph.execute(
            neo_query("MATCH (n) WHERE (n:Event OR n:Actor OR n:Location) AND (
                (n.title IS NOT NULL AND n.title =~ $pattern) OR 
                (n.name IS NOT NULL AND n.name =~ $pattern) OR 
                (n.description IS NOT NULL AND n.description =~ $pattern)
            ) RETURN n.uuid AS uuid, n.title AS title, n.name AS name, n.description AS description, labels(n)[0] AS label LIMIT 30")
                .param("pattern", search_pattern)
        ).await?;

        while let Some(row) = neo_res.next().await? {
            let uuid: String = row.get("uuid").unwrap_or_default();
            let mut title: String = row.get("title").unwrap_or_default();
            if title.is_empty() {
                title = row.get("name").unwrap_or_default();
            }
            let description: Option<String> = row.get("description").ok();
            let label: String = row.get("label").unwrap_or_default();
            
            results.push(SearchResultItem {
                uuid,
                title,
                description,
                entity_type: label.to_lowercase(),
            });
        }

        // 2. Search Postgres for Sources
        let pg_pattern = format!("%{}%", query);
        let rows = sqlx::query(
            "SELECT source_id, title, author, reference FROM sources \
             WHERE title ILIKE $1 OR author ILIKE $1 OR reference ILIKE $1 \
             LIMIT 15"
        )
        .bind(&pg_pattern)
        .fetch_all(pool)
        .await?;

        for r in rows {
            use sqlx::Row;
            let uuid_val: uuid::Uuid = r.get("source_id");
            let title_opt: Option<String> = r.get("title");
            let author_opt: Option<String> = r.get("author");
            let reference_text: String = r.get("reference");

            let title = title_opt.unwrap_or_else(|| format!("Sumber oleh {}", author_opt.as_deref().unwrap_or("Anonim")));
            
            results.push(SearchResultItem {
                uuid: uuid_val.to_string(),
                title,
                description: Some(reference_text),
                entity_type: "source".to_string(),
            });
        }

        Ok(results)
    }

    #[graphql(name = "auditLogs")]
    async fn audit_logs(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<AuditLogItem>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(50) as i64;

        let rows = sqlx::query(
            "SELECT id, entity_type, entity_id, action, performed_by, performed_at \
             FROM audit_log \
             ORDER BY performed_at DESC \
             LIMIT $1"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let mut logs = Vec::new();
        for r in rows {
            use sqlx::Row;
            let id: i64 = r.get("id");
            let entity_type: String = r.get("entity_type");
            let entity_id_uuid: uuid::Uuid = r.get("entity_id");
            let action: String = r.get("action");
            let performed_by: String = r.get("performed_by");
            let performed_at_time: chrono::DateTime<chrono::Utc> = r.get("performed_at");

            logs.push(AuditLogItem {
                id,
                entity_type,
                entity_id: entity_id_uuid.to_string(),
                action,
                performed_by,
                performed_at: performed_at_time.to_rfc3339(),
            });
        }

        Ok(logs)
    }

    #[graphql(name = "syncStatus")]
    async fn sync_status(&self, ctx: &Context<'_>) -> Result<SyncStatus> {
        let pool = ctx.data::<PgPool>()?;
        let graph = ctx.data::<Graph>()?;

        // 1. Check PG connection & count
        let pg_count_res: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sources")
            .fetch_one(pool)
            .await
            .unwrap_or((0,));

        // 2. Check Neo4j connection & count
        let mut neo_res = graph.execute(
            neo_query("MATCH (n) RETURN COUNT(n) AS node_count")
        ).await?;
        
        let neo4j_nodes_count = if let Some(row) = neo_res.next().await? {
            let count: i64 = row.get("node_count")?;
            count
        } else {
            0
        };

        Ok(SyncStatus {
            postgres_connected: true,
            neo4j_connected: true,
            postgres_records_count: pg_count_res.0,
            neo4j_nodes_count,
        })
    }

    // -------------------------------------------------------------------------
    // Graph Traversal Resolvers
    // -------------------------------------------------------------------------

    /// Fetch the entire graph (or a limited subgraph) for 2D/3D visualization.
    /// Returns a JSON structure compatible with Cytoscape.js.
    #[graphql(name = "fullGraph")]
    async fn full_graph(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<serde_json::Value> {
        let graph = ctx.data::<Graph>()?;
        let limit = limit.unwrap_or(300) as i64;

        // Fetch nodes
        let mut nodes_result = graph.execute(
            neo_query("MATCH (n) WHERE n:Event OR n:Actor OR n:Location RETURN n.uuid AS id, n.title AS title, n.name AS name, labels(n)[0] AS type, n.curation_tier AS tier LIMIT $limit")
                .param("limit", limit)
        ).await?;

        let mut nodes = Vec::new();
        while let Some(row) = nodes_result.next().await? {
            let id: String = row.get("id").unwrap_or_default();
            let mut label: String = row.get("title").unwrap_or_default();
            if label.is_empty() {
                label = row.get("name").unwrap_or_default();
            }
            let mut node_type: String = row.get("type").unwrap_or_default();
            node_type = node_type.to_lowercase();
            let tier: String = row.get("tier").unwrap_or_else(|_| "draft".to_string()).to_lowercase();
            
            nodes.push(serde_json::json!({
                "data": {
                    "id": id,
                    "label": label,
                    "type": node_type,
                    "tier": tier
                }
            }));
        }

        // Fetch edges
        let mut edges_result = graph.execute(
            neo_query("MATCH (n)-[r]->(m) WHERE (n:Event OR n:Actor OR n:Location) AND (m:Event OR m:Actor OR m:Location) RETURN n.uuid AS source, m.uuid AS target, type(r) AS relationship LIMIT $limit")
                .param("limit", limit * 2)
        ).await?;

        let mut edges = Vec::new();
        while let Some(row) = edges_result.next().await? {
            let source: String = row.get("source").unwrap_or_default();
            let target: String = row.get("target").unwrap_or_default();
            let rel: String = row.get("relationship").unwrap_or_default();
            
            if !source.is_empty() && !target.is_empty() {
                edges.push(serde_json::json!({
                    "data": {
                        "source": source,
                        "target": target,
                        "relationship": rel
                    }
                }));
            }
        }

        Ok(serde_json::json!({
            "nodes": nodes,
            "edges": edges
        }))
    }

    /// Events in a specific year range based on Hijri calendar.
    async fn timeline(
        &self,
        ctx: &Context<'_>,
        from_year: i32,
        to_year: i32,
    ) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;

        let mut result = graph
            .execute(
                neo_query(
                    "MATCH (e:Event) 
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
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links 
                ORDER BY e.hijri_year ASC",
                )
                .param("from_year", from_year as i64)
                .param("to_year", to_year as i64),
            )
            .await?;

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

    let tier_str: String = row
        .get("curation_tier")
        .unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    let media_links_str: Option<String> = row.get("media_links").ok();
    let media_links: Option<Vec<MediaLink>> = media_links_str.and_then(|s| serde_json::from_str(&s).ok());

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
        media_links,
    })
}

fn row_to_actor(row: &neo4rs::Row) -> Result<Actor> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let name: String = row.get("name")?;
    let type_str: String = row
        .get("actor_type")
        .unwrap_or_else(|_| "Individual".to_string());

    let actor_type = match type_str.as_str() {
        "Individual" => ActorType::Individual,
        "Group" => ActorType::Group,
        _ => ActorType::Individual,
    };

    let cultural_sphere: Option<String> = row.get("cultural_sphere").ok();
    let birth_year: Option<i32> = row.get("birth_year").ok();
    let death_year: Option<i32> = row.get("death_year").ok();

    let tier_str: String = row
        .get("curation_tier")
        .unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    let works: Option<Vec<String>> = row.get("works").ok();
    let roles: Option<Vec<String>> = row.get("roles").ok();
    let description: Option<String> = row.get("description").ok();

    let media_links_str: Option<String> = row.get("media_links").ok();
    let media_links: Option<Vec<MediaLink>> = media_links_str.and_then(|s| serde_json::from_str(&s).ok());

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
        works,
        roles,
        description,
        media_links,
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

    let tier_str: String = row
        .get("curation_tier")
        .unwrap_or_else(|_| "Draft".to_string());
    let curation_tier = match tier_str.as_str() {
        "Draft" => CurationTier::Draft,
        "Verified" => CurationTier::Verified,
        "Reviewed" => CurationTier::Reviewed,
        "Canonical" => CurationTier::Canonical,
        _ => CurationTier::Draft,
    };

    let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
    let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

    let geography_climate: Option<String> = row.get("geography_climate").ok();
    let demographics: Option<String> = row.get("demographics").ok();
    let socio_cultural: Option<String> = row.get("socio_cultural").ok();
    let historical_role: Option<String> = row.get("historical_role").ok();

    let media_links_str: Option<String> = row.get("media_links").ok();
    let media_links: Option<Vec<MediaLink>> = media_links_str.and_then(|s| serde_json::from_str(&s).ok());

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
        geography_climate,
        demographics,
        socio_cultural,
        historical_role,
        media_links,
    })
}

#[ComplexObject]
impl Event {
    async fn actors(&self, ctx: &Context<'_>) -> Result<Vec<Actor>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event {uuid: $uuid})<-[:PARTICIPATED_IN]-(a:Actor) RETURN 
                a.uuid AS uuid, a.name AS name, a.actor_type AS actor_type, 
                a.cultural_sphere AS cultural_sphere, a.birth_year AS birth_year, 
                a.death_year AS death_year, a.curation_tier AS curation_tier, 
                a.is_connected_to_global AS is_connected_to_global, 
                a.global_pivot_category AS global_pivot_category")
            .param("uuid", self.uuid.to_string())
        ).await?;
        let mut actors = Vec::new();
        while let Some(row) = result.next().await? {
            actors.push(row_to_actor(&row)?);
        }
        Ok(actors)
    }

    async fn locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event {uuid: $uuid})-[:OCCURRED_AT]->(l:Location) RETURN 
                l.uuid AS uuid, l.name AS name, l.location_type AS location_type, 
                l.region AS region, l.latitude AS latitude, l.longitude AS longitude, 
                l.curation_tier AS curation_tier, l.is_connected_to_global AS is_connected_to_global, 
                l.global_pivot_category AS global_pivot_category")
            .param("uuid", self.uuid.to_string())
        ).await?;
        let mut locs = Vec::new();
        while let Some(row) = result.next().await? {
            locs.push(row_to_location(&row)?);
        }
        Ok(locs)
    }

    async fn sources(&self, ctx: &Context<'_>) -> Result<Vec<Source>> {

        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event {uuid: $uuid})-[r:SOURCED_FROM]->(s:Source) RETURN s.uuid AS source_id, r.sub_references AS sub_references")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut attributions = Vec::new();
        while let Some(row) = result.next().await? {
            let id: String = row.get("source_id").unwrap_or_default();
            let sub_ref: Option<String> = row.get("sub_references").ok();
            if let Ok(uid) = Uuid::parse_str(&id) {
                attributions.push((uid, sub_ref));
            }
        }
        
        let mut sources = Vec::new();
        for (id, sub_ref) in attributions {
            let row = sqlx::query("SELECT * FROM sources WHERE source_id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;
            if let Some(r) = row {
                let mut source = row_to_source(&r)?;
                source.sub_references = sub_ref;
                sources.push(source);
            }
        }
        Ok(sources)
    }
}

#[ComplexObject]
impl Actor {
    /// 1. Timeline: Events the actor was involved in, sorted by their Gregorian date.
    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (a:Actor {uuid: $uuid})-[:PARTICIPATED_IN]->(e:Event) RETURN 
                e.uuid AS uuid, e.title AS title, e.description AS description, 
                e.iso_8601 AS iso_8601, e.hijri_year AS hijri_year, e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, e.gregorian_day AS gregorian_day, 
                e.precision AS precision, e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }
        
        // Sort by Gregorian date: year -> month -> day
        events.sort_by(|e1, e2| {
            let y1 = e1.gregorian_date.year;
            let y2 = e2.gregorian_date.year;
            if y1 != y2 {
                return y1.cmp(&y2);
            }
            let m1 = e1.gregorian_date.month.unwrap_or(1);
            let m2 = e2.gregorian_date.month.unwrap_or(1);
            if m1 != m2 {
                return m1.cmp(&m2);
            }
            let d1 = e1.gregorian_date.day.unwrap_or(1);
            let d2 = e2.gregorian_date.day.unwrap_or(1);
            d1.cmp(&d2)
        });
        
        Ok(events)
    }

    /// 2. Related Actors: Other actors linked to this actor dynamically.
    async fn related_actors(&self, ctx: &Context<'_>) -> Result<Vec<RelatedActor>> {
        let graph = ctx.data::<Graph>()?;
        // Match direct relationships AND co-participation in events
        let mut result = graph.execute(
            neo_query("
                MATCH (a:Actor {uuid: $uuid})
                OPTIONAL MATCH (a)-[r:RELATED_TO]-(other1:Actor)
                OPTIONAL MATCH (a)-[:PARTICIPATED_IN]->(e:Event)<-[:PARTICIPATED_IN]-(other2:Actor)
                WITH a, other1, r, other2, e
                WITH a, other1, coalesce(r.type, r.description, 'Kerabat/Kolega') AS rel1,
                     other2,
                     CASE
                       WHEN e.title =~ '(?i).*perang.*' OR e.title =~ '(?i).*pertempuran.*' OR e.title =~ '(?i).*penaklukan.*' THEN 'Seperjuangan di ' + e.title
                       WHEN e.title =~ '(?i).*belajar.*' OR e.title =~ '(?i).*kajian.*' OR e.title =~ '(?i).*majelis.*' OR e.title =~ '(?i).*madrasah.*' THEN 'Kolega di ' + e.title
                       ELSE 'Terlibat bersama di ' + e.title
                     END AS rel2
                WITH a, collect({node: other1, rel: rel1}) + collect({node: other2, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.node AS other, conn.rel AS rel
                WHERE other IS NOT NULL AND other.uuid <> $uuid
                RETURN DISTINCT
                    other.uuid AS uuid, other.name AS name, other.actor_type AS actor_type, 
                    other.cultural_sphere AS cultural_sphere, other.birth_year AS birth_year, 
                    other.death_year AS death_year, other.curation_tier AS curation_tier, 
                    other.is_connected_to_global AS is_connected_to_global, 
                    other.global_pivot_category AS global_pivot_category,
                    other.works AS works, other.roles AS roles, other.description AS description, other.media_links AS media_links,
                    collect(rel) AS relationships
            ")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut related_actors = Vec::new();
        while let Some(row) = result.next().await? {
            let actor = row_to_actor(&row)?;
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            related_actors.push(RelatedActor {
                actor,
                relationship_type,
            });
        }
        Ok(related_actors)
    }

    /// 3. Participated Events: Events involved in (unsorted).
    async fn participated_events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (a:Actor {uuid: $uuid})-[:PARTICIPATED_IN]->(e:Event) RETURN 
                e.uuid AS uuid, e.title AS title, e.description AS description, 
                e.iso_8601 AS iso_8601, e.hijri_year AS hijri_year, e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, e.gregorian_day AS gregorian_day, 
                e.precision AS precision, e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }
        Ok(events)
    }

    /// 4. Visited Locations: Locations related to this actor (visited or occurred events).
    async fn visited_locations(&self, ctx: &Context<'_>) -> Result<Vec<RelatedLocation>> {
        let graph = ctx.data::<Graph>()?;
        // Match direct visited locations AND locations of events this actor participated in
        let mut result = graph.execute(
            neo_query("
                MATCH (a:Actor {uuid: $uuid})
                OPTIONAL MATCH (a)-[:VISITED]->(l1:Location)
                OPTIONAL MATCH (a)-[:PARTICIPATED_IN]->(e:Event)-[:OCCURRED_AT]->(l2:Location)
                WITH a, l1, l2, e
                WITH a, l1, 'Pernah Disinggahi' AS rel1,
                     l2, 'Lokasi Peristiwa: ' + e.title AS rel2
                WITH a, collect({node: l1, rel: rel1}) + collect({node: l2, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.node AS loc, conn.rel AS rel
                WHERE loc IS NOT NULL
                RETURN DISTINCT
                    loc.uuid AS uuid, loc.name AS name, loc.lat AS lat, loc.lng AS lng, 
                    loc.precision AS precision, loc.is_transcendental AS is_transcendental, 
                    loc.curation_tier AS curation_tier, loc.is_connected_to_global AS is_connected_to_global, 
                    loc.global_pivot_category AS global_pivot_category,
                    loc.geography_climate AS geography_climate, loc.demographics AS demographics, 
                    loc.socio_cultural AS socio_cultural, loc.historical_role AS historical_role, 
                    loc.media_links AS media_links,
                    collect(rel) AS relationships
            ")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut related_locations = Vec::new();
        while let Some(row) = result.next().await? {
            let location = row_to_location(&row)?;
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            related_locations.push(RelatedLocation {
                location,
                relationship_type,
            });
        }
        Ok(related_locations)
    }

    /// 5. Sources: Historical sources related to this actor (directly or via events).
    async fn sources(&self, ctx: &Context<'_>) -> Result<Vec<RelatedSource>> {

        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;
        
        // Match direct sources AND sources of events this actor participated in
        let mut result = graph.execute(
            neo_query("
                MATCH (a:Actor {uuid: $uuid})
                OPTIONAL MATCH (a)-[:SOURCED_FROM]->(s1:Source)
                OPTIONAL MATCH (a)-[:PARTICIPATED_IN]->(e:Event)-[:SOURCED_FROM]->(s2:Source)
                WITH a, s1, s2, e
                WITH a, s1, 'Rujukan Riwayat Tokoh' AS rel1,
                     s2, 'Rujukan Peristiwa: ' + e.title AS rel2
                WITH a, collect({id: s1.uuid, rel: rel1}) + collect({id: s2.uuid, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.id AS source_id, conn.rel AS rel
                WHERE source_id IS NOT NULL
                RETURN DISTINCT source_id, collect(rel) AS relationships
            ")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut source_attributions = Vec::new();
        while let Some(row) = result.next().await? {
            let id: String = row.get("source_id").unwrap_or_default();
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            if let Ok(uid) = Uuid::parse_str(&id) {
                source_attributions.push((uid, relationship_type));
            }
        }
        
        let mut related_sources = Vec::new();
        for (id, relationship_type) in source_attributions {
            let row = sqlx::query("SELECT * FROM sources WHERE source_id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;
            if let Some(r) = row {
                let source = row_to_source(&r)?;
                related_sources.push(RelatedSource {
                    source,
                    relationship_type,
                });
            }
        }
        Ok(related_sources)
    }
}

#[ComplexObject]
impl Location {
    /// 1. Timeline: Events that occurred at this location, sorted chronologically.
    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event)-[:OCCURRED_AT]->(l:Location {uuid: $uuid}) RETURN 
                e.uuid AS uuid, e.title AS title, e.description AS description, 
                e.iso_8601 AS iso_8601, e.hijri_year AS hijri_year, e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, e.gregorian_day AS gregorian_day, 
                e.precision AS precision, e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }
        
        events.sort_by(|e1, e2| {
            let y1 = e1.gregorian_date.year;
            let y2 = e2.gregorian_date.year;
            if y1 != y2 {
                return y1.cmp(&y2);
            }
            let m1 = e1.gregorian_date.month.unwrap_or(1);
            let m2 = e2.gregorian_date.month.unwrap_or(1);
            if m1 != m2 {
                return m1.cmp(&m2);
            }
            let d1 = e1.gregorian_date.day.unwrap_or(1);
            let d2 = e2.gregorian_date.day.unwrap_or(1);
            d1.cmp(&d2)
        });
        
        Ok(events)
    }

    /// 2. Related Locations: Places linked with this place.
    async fn related_locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (l:Location {uuid: $uuid})-[:RELATED_TO]-(other:Location) RETURN DISTINCT
                other.uuid AS uuid, other.name AS name, other.lat AS lat, other.lng AS lng, 
                other.precision AS precision, other.is_transcendental AS is_transcendental, 
                other.curation_tier AS curation_tier, other.is_connected_to_global AS is_connected_to_global, 
                other.global_pivot_category AS global_pivot_category,
                other.geography_climate AS geography_climate,
                other.demographics AS demographics,
                other.socio_cultural AS socio_cultural,
                other.historical_role AS historical_role,
                other.media_links AS media_links")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut locations = Vec::new();
        while let Some(row) = result.next().await? {
            locations.push(row_to_location(&row)?);
        }
        Ok(locations)
    }

    /// 3. Events: Events occurred at this location (unsorted).
    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("MATCH (e:Event)-[:OCCURRED_AT]->(l:Location {uuid: $uuid}) RETURN 
                e.uuid AS uuid, e.title AS title, e.description AS description, 
                e.iso_8601 AS iso_8601, e.hijri_year AS hijri_year, e.hijri_month AS hijri_month, 
                e.hijri_day AS hijri_day, e.gregorian_year AS gregorian_year, 
                e.gregorian_month AS gregorian_month, e.gregorian_day AS gregorian_day, 
                e.precision AS precision, e.curation_tier AS curation_tier, 
                e.is_connected_to_global AS is_connected_to_global, 
                e.global_pivot_category AS global_pivot_category, e.media_links AS media_links")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }
        Ok(events)
    }

    /// 4. Actors: Actors related to this location (visited it or participated in events here).
    async fn actors(&self, ctx: &Context<'_>) -> Result<Vec<RelatedActor>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("
                MATCH (l:Location {uuid: $uuid})
                OPTIONAL MATCH (a1:Actor)-[:VISITED]->(l)
                OPTIONAL MATCH (a2:Actor)-[:PARTICIPATED_IN]->(e:Event)-[:OCCURRED_AT]->(l)
                WITH l, a1, a2, e
                WITH l, a1, 'Pernah Berkunjung' AS rel1,
                     a2, 'Hadir pada Peristiwa: ' + e.title AS rel2
                WITH l, collect({node: a1, rel: rel1}) + collect({node: a2, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.node AS other, conn.rel AS rel
                WHERE other IS NOT NULL
                RETURN DISTINCT
                    other.uuid AS uuid, other.name AS name, other.actor_type AS actor_type, 
                    other.cultural_sphere AS cultural_sphere, other.birth_year AS birth_year, 
                    other.death_year AS death_year, other.curation_tier AS curation_tier, 
                    other.is_connected_to_global AS is_connected_to_global, 
                    other.global_pivot_category AS global_pivot_category,
                    other.works AS works, other.roles AS roles, other.description AS description, other.media_links AS media_links,
                    collect(rel) AS relationships
            ")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut related_actors = Vec::new();
        while let Some(row) = result.next().await? {
            let actor = row_to_actor(&row)?;
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            related_actors.push(RelatedActor {
                actor,
                relationship_type,
            });
        }
        Ok(related_actors)
    }

    /// 5. Sources: Historical sources referring to this location (directly or via events).
    async fn sources(&self, ctx: &Context<'_>) -> Result<Vec<RelatedSource>> {

        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;
        let mut result = graph.execute(
            neo_query("
                MATCH (l:Location {uuid: $uuid})
                OPTIONAL MATCH (l)-[:SOURCED_FROM]->(s1:Source)
                OPTIONAL MATCH (l)<-[:OCCURRED_AT]-(e:Event)-[:SOURCED_FROM]->(s2:Source)
                WITH l, s1, s2, e
                WITH l, s1, 'Rujukan Riwayat Lokasi' AS rel1,
                     s2, 'Rujukan Peristiwa: ' + e.title AS rel2
                WITH l, collect({id: s1.uuid, rel: rel1}) + collect({id: s2.uuid, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.id AS source_id, conn.rel AS rel
                WHERE source_id IS NOT NULL
                RETURN DISTINCT source_id, collect(rel) AS relationships
            ")
            .param("uuid", self.uuid.to_string())
        ).await?;
        
        let mut source_attributions = Vec::new();
        while let Some(row) = result.next().await? {
            let id: String = row.get("source_id").unwrap_or_default();
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            if let Ok(uid) = Uuid::parse_str(&id) {
                source_attributions.push((uid, relationship_type));
            }
        }
        
        let mut related_sources = Vec::new();
        for (id, relationship_type) in source_attributions {
            let row = sqlx::query("SELECT * FROM sources WHERE source_id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;
            if let Some(r) = row {
                let source = row_to_source(&r)?;
                related_sources.push(RelatedSource {
                    source,
                    relationship_type,
                });
            }
        }
        Ok(related_sources)
    }
}

#[ComplexObject]
impl Source {
    /// 1. Actors: Actors referenced by this source (directly or via events).
    async fn actors(&self, ctx: &Context<'_>) -> Result<Vec<RelatedActor>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("
                MATCH (s:Source {uuid: $source_id})
                OPTIONAL MATCH (a1:Actor)-[:SOURCED_FROM]->(s)
                OPTIONAL MATCH (a2:Actor)-[:PARTICIPATED_IN]->(e:Event)-[:SOURCED_FROM]->(s)
                WITH s, a1, a2, e
                WITH s, a1, 'Ditulis langsung di Riwayat' AS rel1,
                     a2, 'Ditulis di Peristiwa: ' + e.title AS rel2
                WITH s, collect({node: a1, rel: rel1}) + collect({node: a2, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.node AS other, conn.rel AS rel
                WHERE other IS NOT NULL
                RETURN DISTINCT
                    other.uuid AS uuid, other.name AS name, other.actor_type AS actor_type, 
                    other.cultural_sphere AS cultural_sphere, other.birth_year AS birth_year, 
                    other.death_year AS death_year, other.curation_tier AS curation_tier, 
                    other.is_connected_to_global AS is_connected_to_global, 
                    other.global_pivot_category AS global_pivot_category,
                    other.works AS works, other.roles AS roles, other.description AS description, other.media_links AS media_links,
                    collect(rel) AS relationships
            ")
            .param("source_id", self.source_id.to_string())
        ).await?;
        
        let mut related_actors = Vec::new();
        while let Some(row) = result.next().await? {
            let actor = row_to_actor(&row)?;
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            related_actors.push(RelatedActor {
                actor,
                relationship_type,
            });
        }
        Ok(related_actors)
    }

    /// 2. Locations: Locations referenced by this source (directly or via events).
    async fn locations(&self, ctx: &Context<'_>) -> Result<Vec<RelatedLocation>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("
                MATCH (s:Source {uuid: $source_id})
                OPTIONAL MATCH (l1:Location)-[:SOURCED_FROM]->(s)
                OPTIONAL MATCH (l2:Location)<-[:OCCURRED_AT]-(e:Event)-[:SOURCED_FROM]->(s)
                WITH s, l1, l2, e
                WITH s, l1, 'Disebut langsung di Riwayat' AS rel1,
                     l2, 'Disebut di Peristiwa: ' + e.title AS rel2
                WITH s, collect({node: l1, rel: rel1}) + collect({node: l2, rel: rel2}) AS connections
                UNWIND connections AS conn
                WITH conn.node AS loc, conn.rel AS rel
                WHERE loc IS NOT NULL
                RETURN DISTINCT
                    loc.uuid AS uuid, loc.name AS name, loc.lat AS lat, loc.lng AS lng, 
                    loc.precision AS precision, loc.is_transcendental AS is_transcendental, 
                    loc.curation_tier AS curation_tier, loc.is_connected_to_global AS is_connected_to_global, 
                    loc.global_pivot_category AS global_pivot_category,
                    loc.geography_climate AS geography_climate, loc.demographics AS demographics, 
                    loc.socio_cultural AS socio_cultural, loc.historical_role AS historical_role, 
                    loc.media_links AS media_links,
                    collect(rel) AS relationships
            ")
            .param("source_id", self.source_id.to_string())
        ).await?;
        
        let mut related_locations = Vec::new();
        while let Some(row) = result.next().await? {
            let location = row_to_location(&row)?;
            let rels: Vec<String> = row.get("relationships").unwrap_or_default();
            let relationship_type = rels.join(", ");
            related_locations.push(RelatedLocation {
                location,
                relationship_type,
            });
        }
        Ok(related_locations)
    }

    /// 3. Events: Historical events referenced by this source.
    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let graph = ctx.data::<Graph>()?;
        let mut result = graph.execute(
            neo_query("
                MATCH (e:Event)-[:SOURCED_FROM]->(s:Source {uuid: $source_id})
                RETURN DISTINCT
                    e.uuid AS uuid, e.title AS title, e.description AS description, 
                    e.iso_8601 AS iso_8601, e.hijri_year AS hijri_year, e.hijri_month AS hijri_month, 
                    e.hijri_day AS hijri_day, e.gregorian_year AS gregorian_year, 
                    e.gregorian_month AS gregorian_month, e.gregorian_day AS gregorian_day, 
                    e.precision AS precision, e.curation_tier AS curation_tier, 
                    e.is_connected_to_global AS is_connected_to_global, 
                    e.global_pivot_category AS global_pivot_category, e.media_links AS media_links
            ")
            .param("source_id", self.source_id.to_string())
        ).await?;
        
        let mut events = Vec::new();
        while let Some(row) = result.next().await? {
            events.push(row_to_event(&row)?);
        }
        Ok(events)
    }
}
