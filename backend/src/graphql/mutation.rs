use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use neo4rs::{Graph, query as neo_query};
use uuid::Uuid;
use crate::models::{
    event::{Event, CreateEventInput, UpdateEventInput},
    actor::{Actor, CreateActorInput, UpdateActorInput},
    location::{Location, CreateLocationInput, UpdateLocationInput},
    source::{Source, CreateSourceInput},
    common::*,
};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // -------------------------------------------------------------------------
    // Event Mutations
    // -------------------------------------------------------------------------

    async fn create_event(&self, ctx: &Context<'_>, input: CreateEventInput) -> Result<Event> {
        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let precision_str = format!("{:?}", input.precision);
        let default_tier = format!("{:?}", CurationTier::Draft);

        graph.run(
            neo_query(
                "CREATE (e:Event {
                    uuid: $uuid,
                    title: $title,
                    description: $description,
                    iso_8601: $iso_8601,
                    hijri_year: $hijri_year,
                    hijri_month: $hijri_month,
                    hijri_day: $hijri_day,
                    gregorian_year: $gregorian_year,
                    gregorian_month: $gregorian_month,
                    gregorian_day: $gregorian_day,
                    precision: $precision,
                    curation_tier: $curation_tier,
                    is_connected_to_global: $is_connected,
                    global_pivot_category: $pivot_cat
                })"
            )
            .param("uuid", uuid.to_string())
            .param("title", input.title.clone())
            .param("description", input.description.clone())
            .param("iso_8601", input.iso_8601.clone())
            .param("hijri_year", input.islamic_date.year as i64)
            .param("hijri_month", input.islamic_date.month.map(|v| v as i64))
            .param("hijri_day", input.islamic_date.day.map(|v| v as i64))
            .param("gregorian_year", input.gregorian_date.year as i64)
            .param("gregorian_month", input.gregorian_date.month.map(|v| v as i64))
            .param("gregorian_day", input.gregorian_date.day.map(|v| v as i64))
            .param("precision", precision_str)
            .param("curation_tier", default_tier)
            .param("is_connected", input.is_connected_to_global.unwrap_or(false))
            .param("pivot_cat", input.global_pivot_category.clone())
        )
        .await?;

        Ok(Event {
            uuid,
            title: input.title,
            description: input.description,
            iso_8601: input.iso_8601,
            islamic_date: IslamicDate {
                year: input.islamic_date.year,
                month: input.islamic_date.month,
                day: input.islamic_date.day,
            },
            gregorian_date: GregorianDate {
                year: input.gregorian_date.year,
                month: input.gregorian_date.month,
                day: input.gregorian_date.day,
            },
            precision: input.precision,
            curation_tier: CurationTier::Draft,
            global_hook: GlobalHook {
                is_connected_to_global: input.is_connected_to_global.unwrap_or(false),
                global_pivot_category: input.global_pivot_category,
            },
        })
    }

    async fn update_event(&self, ctx: &Context<'_>, uuid: Uuid, input: UpdateEventInput) -> Result<Event> {
        let graph = ctx.data::<Graph>()?;
        
        // Find existing event to merge/retrieve
        let mut check = graph.execute(
            neo_query("MATCH (e:Event {uuid: $uuid}) RETURN 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601,
                e.hijri_year AS hijri_year,
                e.gregorian_year AS gregorian_year,
                e.precision AS precision,
                e.curation_tier AS curation_tier")
                .param("uuid", uuid.to_string())
        ).await?;

        if check.next().await?.is_none() {
            return Err(async_graphql::Error::new("Event not found"));
        }

        // Simplistic Cypher update logic
        if let Some(t) = input.title {
            graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) SET e.title = $t").param("uuid", uuid.to_string()).param("t", t)).await?;
        }
        if let Some(d) = input.description {
            graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) SET e.description = $d").param("uuid", uuid.to_string()).param("d", d)).await?;
        }
        if let Some(iso) = input.iso_8601 {
            graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) SET e.iso_8601 = $iso").param("uuid", uuid.to_string()).param("iso", iso)).await?;
        }
        if let Some(is_conn) = input.is_connected_to_global {
            graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) SET e.is_connected_to_global = $is_conn").param("uuid", uuid.to_string()).param("is_conn", is_conn)).await?;
        }
        if let Some(pivot) = input.global_pivot_category {
            graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) SET e.global_pivot_category = $pivot").param("uuid", uuid.to_string()).param("pivot", pivot)).await?;
        }

        // Just returning a dummy constructed event or refetching
        // We'll refetch it so we return accurate updated data
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
            // Re-use helper logic
            let title: String = row.get("title")?;
            let description: Option<String> = row.get("description").ok();
            let iso_8601: Option<String> = row.get("iso_8601").ok();
            let hijri_year: i32 = row.get("hijri_year").unwrap_or(0);
            let hijri_month: Option<i32> = row.get("hijri_month").ok();
            let hijri_day: Option<i32> = row.get("hijri_day").ok();
            let gregorian_year: i32 = row.get("gregorian_year").unwrap_or(0);
            let gregorian_month: Option<i32> = row.get("gregorian_month").ok();
            let gregorian_day: Option<i32> = row.get("gregorian_day").ok();
            let is_connected: bool = row.get("is_connected_to_global").unwrap_or(false);
            let pivot_cat: Option<String> = row.get("global_pivot_category").ok();

            return Ok(Event {
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
                precision: TimePrecision::Exact,
                curation_tier: CurationTier::Draft,
                global_hook: GlobalHook {
                    is_connected_to_global: is_connected,
                    global_pivot_category: pivot_cat,
                },
            });
        }

        Err(async_graphql::Error::new("Failed to refetch updated event"))
    }

    // -------------------------------------------------------------------------
    // Actor Mutations
    // -------------------------------------------------------------------------

    async fn create_actor(&self, ctx: &Context<'_>, input: CreateActorInput) -> Result<Actor> {
        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let actor_type_str = format!("{:?}", input.actor_type);
        let default_tier = format!("{:?}", CurationTier::Draft);

        graph.run(
            neo_query(
                "CREATE (a:Actor {
                    uuid: $uuid,
                    name: $name,
                    actor_type: $actor_type,
                    cultural_sphere: $cultural_sphere,
                    birth_year: $birth_year,
                    death_year: $death_year,
                    curation_tier: $curation_tier,
                    is_connected_to_global: $is_connected,
                    global_pivot_category: $pivot_cat
                })"
            )
            .param("uuid", uuid.to_string())
            .param("name", input.name.clone())
            .param("actor_type", actor_type_str)
            .param("cultural_sphere", input.cultural_sphere.clone())
            .param("birth_year", input.birth_year.map(|v| v as i64))
            .param("death_year", input.death_year.map(|v| v as i64))
            .param("curation_tier", default_tier)
            .param("is_connected", input.is_connected_to_global.unwrap_or(false))
            .param("pivot_cat", input.global_pivot_category.clone())
        )
        .await?;

        Ok(Actor {
            uuid,
            name: input.name,
            actor_type: input.actor_type,
            cultural_sphere: input.cultural_sphere,
            birth_year: input.birth_year,
            death_year: input.death_year,
            curation_tier: CurationTier::Draft,
            global_hook: GlobalHook {
                is_connected_to_global: input.is_connected_to_global.unwrap_or(false),
                global_pivot_category: input.global_pivot_category,
            },
        })
    }

    async fn update_actor(&self, ctx: &Context<'_>, uuid: Uuid, input: UpdateActorInput) -> Result<Actor> {
        let graph = ctx.data::<Graph>()?;

        if let Some(n) = input.name {
            graph.run(neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.name = $n").param("uuid", uuid.to_string()).param("n", n)).await?;
        }
        if let Some(cs) = input.cultural_sphere {
            graph.run(neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.cultural_sphere = $cs").param("uuid", uuid.to_string()).param("cs", cs)).await?;
        }

        Ok(Actor {
            uuid,
            name: "Updated Actor".to_string(),
            actor_type: ActorType::Individual,
            cultural_sphere: None,
            birth_year: None,
            death_year: None,
            curation_tier: CurationTier::Draft,
            global_hook: GlobalHook {
                is_connected_to_global: false,
                global_pivot_category: None,
            },
        })
    }

    // -------------------------------------------------------------------------
    // Location Mutations
    // -------------------------------------------------------------------------

    async fn create_location(&self, ctx: &Context<'_>, input: CreateLocationInput) -> Result<Location> {
        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let precision_str = format!("{:?}", input.precision);
        let default_tier = format!("{:?}", CurationTier::Draft);

        graph.run(
            neo_query(
                "CREATE (l:Location {
                    uuid: $uuid,
                    name: $name,
                    lat: $lat,
                    lng: $lng,
                    precision: $precision,
                    is_transcendental: $is_transcendental,
                    curation_tier: $curation_tier,
                    is_connected_to_global: $is_connected,
                    global_pivot_category: $pivot_cat
                })"
            )
            .param("uuid", uuid.to_string())
            .param("name", input.name.clone())
            .param("lat", input.lat)
            .param("lng", input.lng)
            .param("precision", precision_str)
            .param("is_transcendental", input.is_transcendental.unwrap_or(false))
            .param("curation_tier", default_tier)
            .param("is_connected", input.is_connected_to_global.unwrap_or(false))
            .param("pivot_cat", input.global_pivot_category.clone())
        )
        .await?;

        Ok(Location {
            uuid,
            name: input.name,
            lat: input.lat,
            lng: input.lng,
            precision: input.precision,
            is_transcendental: input.is_transcendental.unwrap_or(false),
            curation_tier: CurationTier::Draft,
            global_hook: GlobalHook {
                is_connected_to_global: input.is_connected_to_global.unwrap_or(false),
                global_pivot_category: input.global_pivot_category,
            },
        })
    }

    // -------------------------------------------------------------------------
    // Relationship Mutations
    // -------------------------------------------------------------------------

    async fn link_actor_to_event(
        &self, 
        ctx: &Context<'_>, 
        actor_uuid: Uuid, 
        event_uuid: Uuid, 
        role: Option<String>
    ) -> Result<bool> {
        let graph = ctx.data::<Graph>()?;
        
        graph.run(
            neo_query("MATCH (a:Actor {uuid: $actor_uuid}), (e:Event {uuid: $event_uuid})
                       CREATE (a)-[r:PARTICIPATED_IN {role: $role}]->(e)")
                .param("actor_uuid", actor_uuid.to_string())
                .param("event_uuid", event_uuid.to_string())
                .param("role", role.unwrap_or_else(|| "Participant".to_string()))
        ).await?;

        Ok(true)
    }

    async fn link_event_to_location(
        &self, 
        ctx: &Context<'_>, 
        event_uuid: Uuid, 
        location_uuid: Uuid
    ) -> Result<bool> {
        let graph = ctx.data::<Graph>()?;
        
        graph.run(
            neo_query("MATCH (e:Event {uuid: $event_uuid}), (l:Location {uuid: $location_uuid})
                       CREATE (e)-[r:OCCURRED_AT]->(l)")
                .param("event_uuid", event_uuid.to_string())
                .param("location_uuid", location_uuid.to_string())
        ).await?;

        Ok(true)
    }

    async fn link_events(
        &self, 
        ctx: &Context<'_>, 
        from_uuid: Uuid, 
        to_uuid: Uuid, 
        relation_type: EventRelation
    ) -> Result<bool> {
        let graph = ctx.data::<Graph>()?;
        let rel_name = match relation_type {
            EventRelation::Caused => "CAUSED",
            EventRelation::Preceded => "PRECEDED",
            EventRelation::Influenced => "INFLUENCED",
            EventRelation::ContemporaryWith => "CONTEMPORARY_WITH",
        };

        // Standard Cypher MATCH + CREATE with dynamic relationship name
        let query_str = format!(
            "MATCH (a:Event {{uuid: $from_uuid}}), (b:Event {{uuid: $to_uuid}})
             CREATE (a)-[r:{}]->(b)", 
            rel_name
        );

        graph.run(
            neo_query(&query_str)
                .param("from_uuid", from_uuid.to_string())
                .param("to_uuid", to_uuid.to_string())
        ).await?;

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Source Attribution
    // -------------------------------------------------------------------------

    async fn add_source_to_event(
        &self, 
        ctx: &Context<'_>, 
        event_uuid: Uuid, 
        input: CreateSourceInput
    ) -> Result<Source> {
        let pool = ctx.data::<PgPool>()?;
        let graph = ctx.data::<Graph>()?;

        // 1. Insert into PostgreSQL
        let reliability_dec = input.reliability_score.map(|v| BigDecimal::from_f64(v).unwrap_or_default());
        
        let row = sqlx::query!(
            r#"INSERT INTO sources (domain, reference, interpretation_method, reliability_score)
               VALUES ($1, $2, $3, $4)
               RETURNING source_id, created_at, updated_at"#,
            input.domain,
            input.reference_text,
            input.interpretation_method,
            reliability_dec
        )
        .fetch_one(pool)
        .await?;

        // 2. Link in Neo4j using SOURCED_FROM with source_id
        graph.run(
            neo_query("MATCH (e:Event {uuid: $event_uuid})
                       CREATE (e)-[r:SOURCED_FROM {source_id: $source_id}]->(:Source {uuid: $source_id})")
                .param("event_uuid", event_uuid.to_string())
                .param("source_id", row.source_id.to_string())
        ).await?;

        Ok(Source {
            source_id: row.source_id,
            domain: input.domain,
            reference_text: input.reference_text,
            interpretation_method: input.interpretation_method,
            reliability_score: input.reliability_score,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    // -------------------------------------------------------------------------
    // Governance
    // -------------------------------------------------------------------------

    async fn promote_tier(
        &self, 
        ctx: &Context<'_>, 
        entity_type: String, 
        uuid: Uuid, 
        new_tier: CurationTier
    ) -> Result<bool> {
        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;

        let label = match entity_type.to_lowercase().as_str() {
            "event" => "Event",
            "actor" => "Actor",
            "location" => "Location",
            _ => return Err(async_graphql::Error::new("Invalid entity type")),
        };

        let query_str = format!(
            "MATCH (n:{} {{uuid: $uuid}}) SET n.curation_tier = $new_tier RETURN n.uuid",
            label
        );

        let tier_name = format!("{:?}", new_tier);
        let mut res = graph.execute(
            neo_query(&query_str)
                .param("uuid", uuid.to_string())
                .param("new_tier", tier_name)
        ).await?;

        if res.next().await?.is_none() {
            return Err(async_graphql::Error::new("Entity not found in Graph"));
        }

        // Log this action to PostgreSQL Audit Log
        sqlx::query!(
            "INSERT INTO audit_log (entity_type, entity_id, action, performed_by)
             VALUES ($1, $2, 'promote', 'Curator (BozzQ)')",
            label,
            uuid
        )
        .execute(pool)
        .await?;

        Ok(true)
    }
}

// Minimal helper to resolve BigDecimal type conversion
use sqlx::types::BigDecimal;
use num_traits::FromPrimitive;
