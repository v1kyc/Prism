use uuid::Uuid;
use chrono::{DateTime, Utc};
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}