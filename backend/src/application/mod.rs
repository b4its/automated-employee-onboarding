use crate::domain::{
    entities::{Onboarding, OnboardingStatus, User},
    repositories::{OnboardingRepository, UserRepository},
};
use chrono::{NaiveDate, Utc};
use std::sync::Arc;
use uuid::Uuid;

use bcrypt::{hash, verify, DEFAULT_COST};

pub struct AppService {
    user_repo: Arc<dyn UserRepository>,
    onboarding_repo: Arc<dyn OnboardingRepository>,
}

impl AppService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        onboarding_repo: Arc<dyn OnboardingRepository>,
    ) -> Self {
        Self {
            user_repo,
            onboarding_repo,
        }
    }

    // --- User Use Cases ---
    pub async fn register(&self, name: String, email: String, password: String) -> Result<User, String> {
        let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
        let user = User {
            id: Uuid::new_v4(),
            name,
            email,
            role: "anonymous".to_string(), // Default role
            password: hashed_password,
        };
        self.user_repo.create(user).await.map_err(|e| e.to_string())
    }

    pub async fn login(&self, email: String, password: String) -> Result<String, String> {
        let user = self.user_repo.find_by_email(&email).await.map_err(|e| e.to_string())?
            .ok_or_else(|| "User not found".to_string())?;

        let is_valid = verify(password, &user.password).unwrap_or(false);
        if is_valid {
            // TODO: Generate JWT token di sini
            Ok(format!("mock_jwt_token_for_{}", user.id))
        } else {
            Err("Invalid password".to_string())
        }
    }

    // --- Onboarding Use Cases ---
    pub async fn initiate_onboarding(
        &self,
        first_name: String,
        last_name: String,
        personal_email: String,
        department: String,
        start_date: NaiveDate,
    ) -> Result<Onboarding, String> {
        let ob = Onboarding {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            personal_email,
            department,
            start_date,
            status: OnboardingStatus::Initiated,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.onboarding_repo.create(ob).await.map_err(|e| e.to_string())
    }

    pub async fn get_active_onboardings(&self) -> Result<Vec<Onboarding>, String> {
        self.onboarding_repo.get_active().await.map_err(|e| e.to_string())
    }

    pub async fn get_onboarding_by_id(&self, id: Uuid) -> Result<Onboarding, String> {
        self.onboarding_repo.get_by_id(id).await.map_err(|e| e.to_string())?
            .ok_or_else(|| "Onboarding record not found".to_string())
    }

    pub async fn upload_document(
        &self,
        onboarding_id: Uuid,
        document_type: String,
        _file_name: String,
        _file_bytes: Vec<u8>,
    ) -> Result<(), String> {
        // Simulasi path (Requirement ONB-04) 
        let storage_path = format!("/tmp/{}", _file_name); 
        
        self.onboarding_repo
            .add_document_and_update_status(onboarding_id, document_type, storage_path)
            .await
            .map_err(|e| e.to_string())
    }
}