use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- User Entities ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    #[serde(skip_serializing)] 
    pub password: String,
}

// --- Onboarding Entities ---
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "onboarding_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OnboardingStatus {
    Initiated,
    PendingReview,
    ResubmissionRequired,
    ItProvisioning,
    Completed,
}

#[derive(Debug, Serialize, Clone)]
pub struct Onboarding {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub personal_email: String,
    pub department: String,
    pub start_date: NaiveDate,
    pub status: OnboardingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}