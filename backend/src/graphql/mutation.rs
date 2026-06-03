use crate::models::{
    actor::{Actor, CreateActorInput, UpdateActorInput},
    common::*,
    event::{CreateEventInput, Event, UpdateEventInput},
    location::{CreateLocationInput, UpdateLocationInput, Location},
    source::{CreateSourceInput, UpdateSourceInput, Source},
    auth::{User, UserRole, Claims, AuthToken, RegisterInput, LoginInput, UpdateProfileInput, UpdateUserRoleInput, AdminCreateUserInput, AdminUpdateUserInput, create_jwt},
};
use async_graphql::{Context, Object, Result, Error};
use neo4rs::{query as neo_query, Graph};
use sqlx::PgPool;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // -------------------------------------------------------------------------
    // Auth Mutations
    // -------------------------------------------------------------------------

    pub async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthToken> {
        let pool = ctx.data::<PgPool>()?;
        
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(input.password.as_bytes(), &salt)
            .map_err(|_| Error::new("Failed to hash password"))?
            .to_string();
            
        // For demonstration, if username is 'admin', set role to Admin, else Visitor
        let role = if input.username.to_lowercase() == "admin" || input.username.to_lowercase() == "super_admin" {
            UserRole::Admin
        } else {
            UserRole::Visitor
        };

        let row = sqlx::query!(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) RETURNING id, username, role as \"role: UserRole\", created_at, updated_at",
            input.username,
            password_hash,
            role as UserRole
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::new(format!("Database error or username taken: {}", e)))?;

        let user = User {
            id: row.id,
            username: row.username,
            email: None,
            full_name: None,
            avatar_url: None,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        let token = create_jwt(&user)?;

        Ok(AuthToken { token, user })
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthToken> {
        let pool = ctx.data::<PgPool>()?;

        let row = sqlx::query!(
            "SELECT id, username, password_hash, role as \"role: UserRole\", created_at, updated_at FROM users WHERE username = $1",
            input.username
        )
        .fetch_optional(pool)
        .await?;

        let row = row.ok_or_else(|| Error::new("Invalid username or password"))?;

        let parsed_hash = PasswordHash::new(&row.password_hash)
            .map_err(|_| Error::new("Invalid hash format in db"))?;
            
        Argon2::default()
            .verify_password(input.password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::new("Invalid username or password"))?;

        let user = User {
            id: row.id,
            username: row.username,
            email: None, // We don't fetch these for the token generation right now
            full_name: None,
            avatar_url: None,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        let token = create_jwt(&user)?;

        Ok(AuthToken { token, user })
    }

    pub async fn update_profile(&self, ctx: &Context<'_>, input: UpdateProfileInput) -> Result<User> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        let pool = ctx.data::<PgPool>()?;

        if let Some(f) = input.full_name {
            sqlx::query!("UPDATE users SET full_name = $1 WHERE id = $2", f, claims.sub).execute(pool).await?;
        }
        if let Some(e) = input.email {
            sqlx::query!("UPDATE users SET email = $1 WHERE id = $2", e, claims.sub).execute(pool).await?;
        }
        if let Some(a) = input.avatar_url {
            sqlx::query!("UPDATE users SET avatar_url = $1 WHERE id = $2", a, claims.sub).execute(pool).await?;
        }
        if let Some(p) = input.password {
            if !p.is_empty() {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                let password_hash = argon2
                    .hash_password(p.as_bytes(), &salt)
                    .map_err(|_| Error::new("Failed to hash password"))?
                    .to_string();
                sqlx::query!("UPDATE users SET password_hash = $1 WHERE id = $2", password_hash, claims.sub).execute(pool).await?;
            }
        }

        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at FROM users WHERE id = $1"#,
            claims.sub
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user_role(&self, ctx: &Context<'_>, user_id: Uuid, input: UpdateUserRoleInput) -> Result<User> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can update roles"));
        }
        let pool = ctx.data::<PgPool>()?;

        sqlx::query!("UPDATE users SET role = $1 WHERE id = $2", input.role as UserRole, user_id).execute(pool).await?;

        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at FROM users WHERE id = $1"#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn admin_create_user(&self, ctx: &Context<'_>, input: AdminCreateUserInput) -> Result<User> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can create users directly"));
        }
        let pool = ctx.data::<PgPool>()?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(input.password.as_bytes(), &salt)
            .map_err(|_| Error::new("Failed to hash password"))?
            .to_string();

        let row = sqlx::query!(
            r#"INSERT INTO users (username, password_hash, email, full_name, role) 
               VALUES ($1, $2, $3, $4, $5) 
               RETURNING id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at"#,
            input.username,
            password_hash,
            input.email,
            input.full_name,
            input.role as UserRole
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::new(format!("Database error or username taken: {}", e)))?;

        Ok(User {
            id: row.id,
            username: row.username,
            email: row.email,
            full_name: row.full_name,
            avatar_url: row.avatar_url,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn admin_update_user(&self, ctx: &Context<'_>, user_id: Uuid, input: AdminUpdateUserInput) -> Result<User> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can update other users"));
        }
        let pool = ctx.data::<PgPool>()?;

        if let Some(u) = input.username {
            sqlx::query!("UPDATE users SET username = $1 WHERE id = $2", u, user_id).execute(pool).await?;
        }
        if let Some(f) = input.full_name {
            sqlx::query!("UPDATE users SET full_name = $1 WHERE id = $2", f, user_id).execute(pool).await?;
        }
        if let Some(e) = input.email {
            sqlx::query!("UPDATE users SET email = $1 WHERE id = $2", e, user_id).execute(pool).await?;
        }
        if let Some(r) = input.role {
            sqlx::query!("UPDATE users SET role = $1 WHERE id = $2", r as UserRole, user_id).execute(pool).await?;
        }
        if let Some(p) = input.password {
            if !p.is_empty() {
                let salt = SaltString::generate(&mut OsRng);
                let argon2 = Argon2::default();
                let password_hash = argon2
                    .hash_password(p.as_bytes(), &salt)
                    .map_err(|_| Error::new("Failed to hash password"))?
                    .to_string();
                sqlx::query!("UPDATE users SET password_hash = $1 WHERE id = $2", password_hash, user_id).execute(pool).await?;
            }
        }

        let user = sqlx::query_as!(
            User,
            r#"SELECT id, username, email, full_name, avatar_url, role as "role: UserRole", created_at, updated_at FROM users WHERE id = $1"#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can delete users"));
        }
        
        // Prevent deleting yourself
        if claims.sub == user_id {
            return Err(Error::new("Forbidden: You cannot delete your own account"));
        }

        let pool = ctx.data::<PgPool>()?;
        let res = sqlx::query!("DELETE FROM users WHERE id = $1", user_id).execute(pool).await?;

        if res.rows_affected() > 0 {
            Ok(true)
        } else {
            Err(Error::new("User not found or already deleted"))
        }
    }

    // -------------------------------------------------------------------------
    // Event Mutations
    // -------------------------------------------------------------------------

    pub async fn create_event(&self, ctx: &Context<'_>, input: CreateEventInput) -> Result<Event> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to create events"));
        }

        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let precision_str = format!("{:?}", input.precision);
        let default_tier = format!("{:?}", CurationTier::Draft);
        let media_links_json = serde_json::to_string(&input.media_links.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

        graph
            .run(
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
                    global_pivot_category: $pivot_cat,
                    media_links: $media_links
                })",
                )
                .param("uuid", uuid.to_string())
                .param("title", input.title.clone())
                .param("description", input.description.clone())
                .param("iso_8601", input.iso_8601.clone())
                .param("hijri_year", input.islamic_date.year as i64)
                .param("hijri_month", input.islamic_date.month.map(|v| v as i64))
                .param("hijri_day", input.islamic_date.day.map(|v| v as i64))
                .param("gregorian_year", input.gregorian_date.year as i64)
                .param(
                    "gregorian_month",
                    input.gregorian_date.month.map(|v| v as i64),
                )
                .param("gregorian_day", input.gregorian_date.day.map(|v| v as i64))
                .param("precision", precision_str)
                .param("curation_tier", default_tier)
                .param(
                    "is_connected",
                    input.is_connected_to_global.unwrap_or(false),
                )
                .param("pivot_cat", input.global_pivot_category.clone())
                .param("media_links", media_links_json),
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
            media_links: input.media_links.map(|v| {
                v.into_iter()
                    .map(|item| MediaLink {
                        media_type: item.media_type,
                        url: item.url,
                        title: item.title,
                    })
                    .collect()
            }),
        })
    }

    pub async fn update_event(
        &self,
        ctx: &Context<'_>,
        uuid: Uuid,
        input: UpdateEventInput,
    ) -> Result<Event> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to edit events"));
        }

        let graph = ctx.data::<Graph>()?;

        // Find existing event to merge/retrieve
        let mut check = graph
            .execute(
                neo_query(
                    "MATCH (e:Event {uuid: $uuid}) RETURN 
                e.title AS title, 
                e.description AS description, 
                e.iso_8601 AS iso_8601,
                e.hijri_year AS hijri_year,
                e.gregorian_year AS gregorian_year,
                e.precision AS precision,
                e.curation_tier AS curation_tier",
                )
                .param("uuid", uuid.to_string()),
            )
            .await?;

        if check.next().await?.is_none() {
            return Err(async_graphql::Error::new("Event not found"));
        }

        // Simplistic Cypher update logic
        if let Some(t) = input.title {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.title = $t")
                        .param("uuid", uuid.to_string())
                        .param("t", t),
                )
                .await?;
        }
        if let Some(d) = input.description {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.description = $d")
                        .param("uuid", uuid.to_string())
                        .param("d", d),
                )
                .await?;
        }
        if let Some(iso) = input.iso_8601 {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.iso_8601 = $iso")
                        .param("uuid", uuid.to_string())
                        .param("iso", iso),
                )
                .await?;
        }
        if let Some(isl) = input.islamic_date {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.hijri_year = $year")
                        .param("uuid", uuid.to_string())
                        .param("year", isl.year),
                )
                .await?;
        }
        if let Some(greg) = input.gregorian_date {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.gregorian_year = $year")
                        .param("uuid", uuid.to_string())
                        .param("year", greg.year),
                )
                .await?;
        }
        if let Some(prec) = input.precision {
            let prec_str = match prec {
                TimePrecision::Exact => "Exact",
                TimePrecision::Year => "Year",
                TimePrecision::Decade => "Decade",
                TimePrecision::Century => "Century",
                TimePrecision::Approximate => "Approximate",
            };
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.precision = $prec")
                        .param("uuid", uuid.to_string())
                        .param("prec", prec_str),
                )
                .await?;
        }
        if let Some(is_conn) = input.is_connected_to_global {
            graph
                .run(
                    neo_query(
                        "MATCH (e:Event {uuid: $uuid}) SET e.is_connected_to_global = $is_conn",
                    )
                    .param("uuid", uuid.to_string())
                    .param("is_conn", is_conn),
                )
                .await?;
        }
        if let Some(pivot) = input.global_pivot_category {
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.global_pivot_category = $pivot")
                        .param("uuid", uuid.to_string())
                        .param("pivot", pivot),
                )
                .await?;
        }
        if let Some(media_links) = input.media_links {
            let media_links_json = serde_json::to_string(&media_links).unwrap_or_else(|_| "[]".to_string());
            graph
                .run(
                    neo_query("MATCH (e:Event {uuid: $uuid}) SET e.media_links = $ml")
                        .param("uuid", uuid.to_string())
                        .param("ml", media_links_json),
                )
                .await?;
        }

        // Just returning a dummy constructed event or refetching
        // We'll refetch it so we return accurate updated data
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
                e.global_pivot_category AS global_pivot_category,
                e.media_links AS media_links",
                )
                .param("uuid", uuid.to_string()),
            )
            .await?;

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
            let media_links_str: Option<String> = row.get("media_links").ok();
            let media_links: Option<Vec<MediaLink>> = media_links_str.and_then(|s| serde_json::from_str(&s).ok());

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
                media_links,
            });
        }

        Err(async_graphql::Error::new("Failed to refetch updated event"))
    }

    // -------------------------------------------------------------------------
    // Actor Mutations
    // -------------------------------------------------------------------------

    pub async fn create_actor(&self, ctx: &Context<'_>, input: CreateActorInput) -> Result<Actor> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to create actors"));
        }

        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let actor_type_str = format!("{:?}", input.actor_type);
        let default_tier = format!("{:?}", CurationTier::Draft);
        let media_links_json = serde_json::to_string(&input.media_links.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

        graph
            .run(
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
                    global_pivot_category: $pivot_cat,
                    works: $works,
                    roles: $roles,
                    description: $description,
                    media_links: $media_links
                })",
                )
                .param("uuid", uuid.to_string())
                .param("name", input.name.clone())
                .param("actor_type", actor_type_str)
                .param("cultural_sphere", input.cultural_sphere.clone())
                .param("birth_year", input.birth_year.map(|v| v as i64))
                .param("death_year", input.death_year.map(|v| v as i64))
                .param("curation_tier", default_tier)
                .param(
                    "is_connected",
                    input.is_connected_to_global.unwrap_or(false),
                )
                .param("pivot_cat", input.global_pivot_category.clone())
                .param("works", input.works.clone().unwrap_or_default())
                .param("roles", input.roles.clone().unwrap_or_default())
                .param("description", input.description.clone())
                .param("media_links", media_links_json),
            )
            .await?;

        let media_links_out: Option<Vec<MediaLink>> = input.media_links.map(|v| {
            v.into_iter()
                .map(|item| MediaLink {
                    media_type: item.media_type,
                    url: item.url,
                    title: item.title,
                })
                .collect()
        });

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
            works: input.works,
            roles: input.roles,
            description: input.description,
            media_links: media_links_out,
        })
    }

    pub async fn update_actor(
        &self,
        ctx: &Context<'_>,
        uuid: Uuid,
        input: UpdateActorInput,
    ) -> Result<Actor> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to edit actors"));
        }

        let graph = ctx.data::<Graph>()?;

        if let Some(n) = input.name {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.name = $n")
                        .param("uuid", uuid.to_string())
                        .param("n", n),
                )
                .await?;
        }
        if let Some(cs) = input.cultural_sphere {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.cultural_sphere = $cs")
                        .param("uuid", uuid.to_string())
                        .param("cs", cs),
                )
                .await?;
        }
        if let Some(at) = input.actor_type {
            let at_str = format!("{:?}", at);
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.actor_type = $at")
                        .param("uuid", uuid.to_string())
                        .param("at", at_str),
                )
                .await?;
        }
        if let Some(by) = input.birth_year {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.birth_year = $by")
                        .param("uuid", uuid.to_string())
                        .param("by", by as i64),
                )
                .await?;
        }
        if let Some(dy) = input.death_year {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.death_year = $dy")
                        .param("uuid", uuid.to_string())
                        .param("dy", dy as i64),
                )
                .await?;
        }
        if let Some(ic) = input.is_connected_to_global {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.is_connected_to_global = $ic")
                        .param("uuid", uuid.to_string())
                        .param("ic", ic),
                )
                .await?;
        }
        if let Some(pc) = input.global_pivot_category {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.global_pivot_category = $pc")
                        .param("uuid", uuid.to_string())
                        .param("pc", pc),
                )
                .await?;
        }
        if let Some(works) = input.works {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.works = $works")
                        .param("uuid", uuid.to_string())
                        .param("works", works),
                )
                .await?;
        }
        if let Some(roles) = input.roles {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.roles = $roles")
                        .param("uuid", uuid.to_string())
                        .param("roles", roles),
                )
                .await?;
        }
        if let Some(desc) = input.description {
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.description = $desc")
                        .param("uuid", uuid.to_string())
                        .param("desc", desc),
                )
                .await?;
        }
        if let Some(media_links) = input.media_links {
            let media_links_json = serde_json::to_string(&media_links).unwrap_or_else(|_| "[]".to_string());
            graph
                .run(
                    neo_query("MATCH (a:Actor {uuid: $uuid}) SET a.media_links = $ml")
                        .param("uuid", uuid.to_string())
                        .param("ml", media_links_json),
                )
                .await?;
        }

        // Fetch the updated node to return it
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
            // Helper function logic from row_to_actor:
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
        } else {
            Err(Error::new("Actor not found after update"))
        }
    }

    // -------------------------------------------------------------------------
    // Location Mutations
    // -------------------------------------------------------------------------

    pub async fn create_location(
        &self,
        ctx: &Context<'_>,
        input: CreateLocationInput,
    ) -> Result<Location> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to create locations"));
        }

        let graph = ctx.data::<Graph>()?;
        let uuid = Uuid::new_v4();

        let precision_str = format!("{:?}", input.precision);
        let default_tier = format!("{:?}", CurationTier::Draft);
        let media_links_json = serde_json::to_string(&input.media_links.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

        graph
            .run(
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
                    global_pivot_category: $pivot_cat,
                    geography_climate: $geography_climate,
                    demographics: $demographics,
                    socio_cultural: $socio_cultural,
                    historical_role: $historical_role,
                    media_links: $media_links
                })",
                )
                .param("uuid", uuid.to_string())
                .param("name", input.name.clone())
                .param("lat", input.lat)
                .param("lng", input.lng)
                .param("precision", precision_str)
                .param(
                    "is_transcendental",
                    input.is_transcendental.unwrap_or(false),
                )
                .param("curation_tier", default_tier)
                .param(
                    "is_connected",
                    input.is_connected_to_global.unwrap_or(false),
                )
                .param("pivot_cat", input.global_pivot_category.clone())
                .param("geography_climate", input.geography_climate.clone())
                .param("demographics", input.demographics.clone())
                .param("socio_cultural", input.socio_cultural.clone())
                .param("historical_role", input.historical_role.clone())
                .param("media_links", media_links_json),
            )
            .await?;

        let media_links_out: Option<Vec<MediaLink>> = input.media_links.map(|v| {
            v.into_iter()
                .map(|item| MediaLink {
                    media_type: item.media_type,
                    url: item.url,
                    title: item.title,
                })
                .collect()
        });

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
            geography_climate: input.geography_climate,
            demographics: input.demographics,
            socio_cultural: input.socio_cultural,
            historical_role: input.historical_role,
            media_links: media_links_out,
        })
    }

    pub async fn update_location(
        &self,
        ctx: &Context<'_>,
        uuid: Uuid,
        input: UpdateLocationInput,
    ) -> Result<Location> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to edit locations"));
        }

        let graph = ctx.data::<Graph>()?;

        if let Some(n) = input.name {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.name = $n")
                        .param("uuid", uuid.to_string())
                        .param("n", n),
                )
                .await?;
        }
        if let Some(lat) = input.lat {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.lat = $lat")
                        .param("uuid", uuid.to_string())
                        .param("lat", lat),
                )
                .await?;
        }
        if let Some(lng) = input.lng {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.lng = $lng")
                        .param("uuid", uuid.to_string())
                        .param("lng", lng),
                )
                .await?;
        }
        if let Some(p) = input.precision {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.precision = $p")
                        .param("uuid", uuid.to_string())
                        .param("p", format!("{:?}", p)),
                )
                .await?;
        }
        if let Some(t) = input.is_transcendental {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.is_transcendental = $t")
                        .param("uuid", uuid.to_string())
                        .param("t", t),
                )
                .await?;
        }
        if let Some(ic) = input.is_connected_to_global {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.is_connected_to_global = $ic")
                        .param("uuid", uuid.to_string())
                        .param("ic", ic),
                )
                .await?;
        }
        if let Some(pc) = input.global_pivot_category {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.global_pivot_category = $pc")
                        .param("uuid", uuid.to_string())
                        .param("pc", pc),
                )
                .await?;
        }
        if let Some(gc) = input.geography_climate {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.geography_climate = $gc")
                        .param("uuid", uuid.to_string())
                        .param("gc", gc),
                )
                .await?;
        }
        if let Some(d) = input.demographics {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.demographics = $d")
                        .param("uuid", uuid.to_string())
                        .param("d", d),
                )
                .await?;
        }
        if let Some(sc) = input.socio_cultural {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.socio_cultural = $sc")
                        .param("uuid", uuid.to_string())
                        .param("sc", sc),
                )
                .await?;
        }
        if let Some(hr) = input.historical_role {
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.historical_role = $hr")
                        .param("uuid", uuid.to_string())
                        .param("hr", hr),
                )
                .await?;
        }
        if let Some(media_links) = input.media_links {
            let media_links_json = serde_json::to_string(&media_links).unwrap_or_else(|_| "[]".to_string());
            graph
                .run(
                    neo_query("MATCH (l:Location {uuid: $uuid}) SET l.media_links = $ml")
                        .param("uuid", uuid.to_string())
                        .param("ml", media_links_json),
                )
                .await?;
        }

        // Fetch updated location
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
        } else {
            Err(Error::new("Location not found after update"))
        }
    }

    // -------------------------------------------------------------------------
    // Source Mutations
    // -------------------------------------------------------------------------

    pub async fn create_source(
        &self,
        ctx: &Context<'_>,
        input: CreateSourceInput,
    ) -> Result<Source> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to create sources"));
        }

        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::new_v4();
        
        use bigdecimal::BigDecimal;
        use std::str::FromStr;
        use sqlx::Row;

        let score = input.reliability_score.map(|f| BigDecimal::from_str(&f.to_string()).unwrap_or_default());
        let media_links_json = serde_json::to_string(&input.media_links.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

        let row = sqlx::query(
            "INSERT INTO sources (source_id, domain, title, author, publication_era, reference, interpretation_method, reliability_score, reliability_assessment, media_links, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW()) \
             RETURNING created_at, updated_at"
        )
        .bind(uuid)
        .bind(&input.domain)
        .bind(&input.title)
        .bind(&input.author)
        .bind(&input.publication_era)
        .bind(&input.reference_text)
        .bind(&input.interpretation_method)
        .bind(score)
        .bind(&input.reliability_assessment)
        .bind(&media_links_json)
        .fetch_one(pool)
        .await?;

        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        // 2. Insert/create corresponding node in Neo4j so it can be matched for relationships
        let graph = ctx.data::<Graph>()?;
        graph.run(
            neo_query("CREATE (s:Source {uuid: $uuid})")
                .param("uuid", uuid.to_string())
        ).await?;

        Ok(Source {
            source_id: uuid,
            domain: input.domain,
            title: input.title,
            author: input.author,
            publication_era: input.publication_era,
            reference_text: input.reference_text,
            interpretation_method: input.interpretation_method,
            reliability_score: input.reliability_score,
            reliability_assessment: input.reliability_assessment,
            media_links: input.media_links.map(|v| v.into_iter().map(|m| MediaLink {
                media_type: m.media_type,
                url: m.url,
                title: m.title,
            }).collect()),
            sub_references: None,
            created_at,
            updated_at,
        })
    }

    pub async fn update_source(
        &self,
        ctx: &Context<'_>,
        source_id: Uuid,
        input: UpdateSourceInput,
    ) -> Result<Source> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to edit sources"));
        }

        let pool = ctx.data::<PgPool>()?;
        use bigdecimal::BigDecimal;
        use std::str::FromStr;
        
        if let Some(d) = &input.domain {
            sqlx::query("UPDATE sources SET domain = $1, updated_at = NOW() WHERE source_id = $2").bind(d).bind(source_id).execute(pool).await?;
        }
        if let Some(t) = &input.title {
            sqlx::query("UPDATE sources SET title = $1, updated_at = NOW() WHERE source_id = $2").bind(t).bind(source_id).execute(pool).await?;
        }
        if let Some(a) = &input.author {
            sqlx::query("UPDATE sources SET author = $1, updated_at = NOW() WHERE source_id = $2").bind(a).bind(source_id).execute(pool).await?;
        }
        if let Some(pe) = &input.publication_era {
            sqlx::query("UPDATE sources SET publication_era = $1, updated_at = NOW() WHERE source_id = $2").bind(pe).bind(source_id).execute(pool).await?;
        }
        if let Some(r) = &input.reference_text {
            sqlx::query("UPDATE sources SET reference = $1, updated_at = NOW() WHERE source_id = $2").bind(r).bind(source_id).execute(pool).await?;
        }
        if let Some(i) = &input.interpretation_method {
            sqlx::query("UPDATE sources SET interpretation_method = $1, updated_at = NOW() WHERE source_id = $2").bind(i).bind(source_id).execute(pool).await?;
        }
        if let Some(s) = input.reliability_score {
            let score = BigDecimal::from_str(&s.to_string()).ok();
            sqlx::query("UPDATE sources SET reliability_score = $1, updated_at = NOW() WHERE source_id = $2").bind(score).bind(source_id).execute(pool).await?;
        }
        if let Some(ra) = &input.reliability_assessment {
            sqlx::query("UPDATE sources SET reliability_assessment = $1, updated_at = NOW() WHERE source_id = $2").bind(ra).bind(source_id).execute(pool).await?;
        }
        if let Some(ml) = &input.media_links {
            let media_links_json = serde_json::to_string(ml).unwrap_or_else(|_| "[]".to_string());
            sqlx::query("UPDATE sources SET media_links = $1, updated_at = NOW() WHERE source_id = $2").bind(media_links_json).bind(source_id).execute(pool).await?;
        }

        let row = sqlx::query(
            "SELECT source_id, domain, title, author, publication_era, reference, interpretation_method, reliability_score, reliability_assessment, media_links, created_at, updated_at \
             FROM sources WHERE source_id = $1"
        )
        .bind(source_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            use sqlx::Row;
            use bigdecimal::ToPrimitive;

            let media_links_str: Option<String> = r.try_get("media_links").unwrap_or(None);
            let media_links = media_links_str.and_then(|s| serde_json::from_str(&s).ok());
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
        } else {
            Err(Error::new("Source not found after update"))
        }
    }

    // -------------------------------------------------------------------------
    // Relationship Mutations
    // -------------------------------------------------------------------------

    pub async fn clear_event_relations(
        &self,
        ctx: &Context<'_>,
        event_uuid: Uuid,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to clear relations"));
        }
        let graph = ctx.data::<Graph>()?;
        
        // Delete relationships for Actors, Locations, and Sources
        graph.run(
            neo_query(
                "MATCH (e:Event {uuid: $uuid})-[r:PARTICIPATED_IN|OCCURRED_AT|SOURCED_FROM]-()
                 DELETE r"
            ).param("uuid", event_uuid.to_string())
        ).await?;

        Ok(true)
    }

    pub async fn link_actor_to_event(
        &self,
        ctx: &Context<'_>,
        actor_uuid: Uuid,
        event_uuid: Uuid,
        role: Option<String>,
    ) -> Result<bool> {
        let graph = ctx.data::<Graph>()?;

        graph
            .run(
                neo_query(
                    "MATCH (a:Actor {uuid: $actor_uuid}), (e:Event {uuid: $event_uuid})
                       MERGE (a)-[r:PARTICIPATED_IN]->(e)
                       SET r.role = $role",
                )
                .param("actor_uuid", actor_uuid.to_string())
                .param("event_uuid", event_uuid.to_string())
                .param("role", role.unwrap_or_else(|| "Participant".to_string())),
            )
            .await?;

        Ok(true)
    }

    pub async fn link_event_to_location(
        &self,
        ctx: &Context<'_>,
        event_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to link entities"));
        }

        let graph = ctx.data::<Graph>()?;

        graph
            .run(
                neo_query(
                    "MATCH (e:Event {uuid: $event_uuid}), (l:Location {uuid: $location_uuid})
                       MERGE (e)-[r:OCCURRED_AT]->(l)",
                )
                .param("event_uuid", event_uuid.to_string())
                .param("location_uuid", location_uuid.to_string()),
            )
            .await?;

        Ok(true)
    }

    pub async fn link_events(
        &self,
        ctx: &Context<'_>,
        from_uuid: Uuid,
        to_uuid: Uuid,
        relation_type: EventRelation,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to link entities"));
        }

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

        graph
            .run(
                neo_query(&query_str)
                    .param("from_uuid", from_uuid.to_string())
                    .param("to_uuid", to_uuid.to_string()),
            )
            .await?;

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Source Attribution
    // -------------------------------------------------------------------------

    pub async fn link_event_to_source(
        &self,
        ctx: &Context<'_>,
        event_uuid: Uuid,
        source_id: Uuid,
        sub_references: Option<String>,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to link sources"));
        }
        let graph = ctx.data::<Graph>()?;
        graph.run(
            neo_query("MATCH (e:Event {uuid: $event_uuid}), (s:Source {uuid: $source_id})
                       MERGE (e)-[r:SOURCED_FROM]->(s)
                       SET r.source_id = $source_id, r.sub_references = $sub_ref")
                .param("event_uuid", event_uuid.to_string())
                .param("source_id", source_id.to_string())
                .param("sub_ref", sub_references.unwrap_or_default())
        ).await?;
        Ok(true)
    }

    pub async fn add_source_to_event(
        &self,
        ctx: &Context<'_>,
        event_uuid: Uuid,
        input: CreateSourceInput,
    ) -> Result<Source> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_edit() {
            return Err(Error::new("Forbidden: You do not have permission to add sources"));
        }

        use sqlx::Row;
        let pool = ctx.data::<PgPool>()?;
        let graph = ctx.data::<Graph>()?;

        // 1. Insert into PostgreSQL
        let reliability_dec = input
            .reliability_score
            .map(|v| BigDecimal::from_f64(v).unwrap_or_default());

        let row = sqlx::query(
            "INSERT INTO sources (domain, reference, interpretation_method, reliability_score) \
             VALUES ($1, $2, $3, $4) \
             RETURNING source_id, created_at, updated_at",
        )
        .bind(input.domain.clone())
        .bind(input.reference_text.clone())
        .bind(input.interpretation_method.clone())
        .bind(reliability_dec)
        .fetch_one(pool)
        .await?;

        let source_id: Uuid = row.get("source_id");
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        // 2. Link in Neo4j using SOURCED_FROM with source_id
        graph.run(
            neo_query("MATCH (e:Event {uuid: $event_uuid})
                       CREATE (e)-[r:SOURCED_FROM {source_id: $source_id}]->(:Source {uuid: $source_id})")
                .param("event_uuid", event_uuid.to_string())
                .param("source_id", source_id.to_string())
        ).await?;

        Ok(Source {
            source_id,
            domain: input.domain,
            title: None,
            author: None,
            publication_era: None,
            reference_text: input.reference_text,
            interpretation_method: input.interpretation_method,
            reliability_score: input.reliability_score,
            reliability_assessment: None,
            media_links: None,
            sub_references: None,
            created_at,
            updated_at,
        })
    }

    // -------------------------------------------------------------------------
    // Governance
    // -------------------------------------------------------------------------

    pub async fn promote_entity(
        &self,
        ctx: &Context<'_>,
        uuid: Uuid,
        new_tier: CurationTier,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_promote() {
            return Err(Error::new("Forbidden: Only Reviewers and Administrators can approve and promote data tiers"));
        }

        let graph = ctx.data::<Graph>()?;
        let pool = ctx.data::<PgPool>()?;

        let tier_name = format!("{:?}", new_tier);
        let mut res = graph
            .execute(
                neo_query("MATCH (n) WHERE n.uuid = $uuid SET n.curation_tier = $new_tier RETURN labels(n)[0] AS label")
                    .param("uuid", uuid.to_string())
                    .param("new_tier", tier_name),
            )
            .await?;

        let label = if let Some(row) = res.next().await? {
            let lbl: String = row.get("label")?;
            lbl
        } else {
            return Err(async_graphql::Error::new("Entity not found in Graph with the given UUID"));
        };

        // Log this action to PostgreSQL Audit Log
        sqlx::query(
            "INSERT INTO audit_log (entity_type, entity_id, action, performed_by) \
             VALUES ($1, $2, 'promote', 'Curator (BozzQ)')",
        )
        .bind(&label)
        .bind(uuid)
        .execute(pool)
        .await?;

        Ok(true)
    }

    pub async fn promote_tier(
        &self,
        ctx: &Context<'_>,
        entity_type: String,
        uuid: Uuid,
        new_tier: CurationTier,
    ) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.can_promote() {
            return Err(Error::new("Forbidden: Only Reviewers and Administrators can approve and promote data tiers"));
        }

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
        let mut res = graph
            .execute(
                neo_query(&query_str)
                    .param("uuid", uuid.to_string())
                    .param("new_tier", tier_name),
            )
            .await?;

        if res.next().await?.is_none() {
            return Err(async_graphql::Error::new("Entity not found in Graph"));
        }

        // Log this action to PostgreSQL Audit Log
        sqlx::query(
            "INSERT INTO audit_log (entity_type, entity_id, action, performed_by) \
             VALUES ($1, $2, 'promote', 'Curator (BozzQ)')",
        )
        .bind(label)
        .bind(uuid)
        .execute(pool)
        .await?;

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Delete / Purge Mutations (Admin Only)
    // -------------------------------------------------------------------------

    pub async fn delete_event(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can delete records"));
        }
        let graph = ctx.data::<Graph>()?;
        graph.run(neo_query("MATCH (e:Event {uuid: $uuid}) DETACH DELETE e").param("uuid", uuid.to_string())).await?;
        Ok(true)
    }

    pub async fn delete_actor(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can delete records"));
        }
        let graph = ctx.data::<Graph>()?;
        graph.run(neo_query("MATCH (a:Actor {uuid: $uuid}) DETACH DELETE a").param("uuid", uuid.to_string())).await?;
        Ok(true)
    }

    pub async fn delete_location(&self, ctx: &Context<'_>, uuid: Uuid) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can delete records"));
        }
        let graph = ctx.data::<Graph>()?;
        graph.run(neo_query("MATCH (l:Location {uuid: $uuid}) DETACH DELETE l").param("uuid", uuid.to_string())).await?;
        Ok(true)
    }

    pub async fn delete_source(&self, ctx: &Context<'_>, source_id: Uuid) -> Result<bool> {
        let claims = ctx.data_opt::<Claims>().ok_or_else(|| Error::new("Unauthorized: Please log in"))?;
        if !claims.role.is_admin() {
            return Err(Error::new("Forbidden: Only administrators can delete records"));
        }
        let pool = ctx.data::<PgPool>()?;
        let graph = ctx.data::<Graph>()?;

        sqlx::query!("DELETE FROM sources WHERE source_id = $1", source_id).execute(pool).await?;
        graph.run(neo_query("MATCH (s:Source {uuid: $uuid}) DETACH DELETE s").param("uuid", source_id.to_string())).await?;
        
        Ok(true)
    }
}

// Minimal helper to resolve BigDecimal type conversion
use num_traits::FromPrimitive;
use sqlx::types::BigDecimal;
