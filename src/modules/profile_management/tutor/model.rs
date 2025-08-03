use sqlx::{FromRow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tutor {
    pub tutor_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_available: Option<bool>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub year: Option<i32>,
    pub bio: Option<String>,
    pub average: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct PartialTutor { // will be used as input type for update request
    pub tutor_id: String,
    pub is_available: Option<bool>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub year: Option<i32>,
    pub bio: Option<String>,
    pub average: Option<f32>,
}