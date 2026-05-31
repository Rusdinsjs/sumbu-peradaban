use async_graphql::{InputObject, SimpleObject, Enum};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Visitor,
    Contributor,
    Editor,
    Reviewer,
    Admin,
}

impl UserRole {
    pub fn can_edit(&self) -> bool {
        matches!(self, UserRole::Contributor | UserRole::Editor | UserRole::Reviewer | UserRole::Admin)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_promote(&self) -> bool {
        matches!(self, UserRole::Reviewer | UserRole::Admin)
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Visitor
    }
}

// Stored in Postgres
#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(InputObject)]
pub struct UpdateProfileInput {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub password: Option<String>, // if they want to change password
}

#[derive(InputObject)]
pub struct UpdateUserRoleInput {
    pub role: UserRole,
}

#[derive(InputObject)]
pub struct AdminCreateUserInput {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub role: UserRole,
}

#[derive(InputObject)]
pub struct AdminUpdateUserInput {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub role: Option<UserRole>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub role: UserRole,
    pub exp: usize,
}

#[derive(InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct AuthToken {
    pub token: String,
    pub user: User,
}

const JWT_SECRET: &[u8] = b"SUMBU_PERADABAN_SUPER_SECRET_KEY";

pub fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        role: user.role,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
