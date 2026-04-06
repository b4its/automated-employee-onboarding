use super::entities::{Onboarding, User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
}

#[async_trait]
pub trait OnboardingRepository: Send + Sync {
    async fn create(&self, onboarding: Onboarding) -> Result<Onboarding, sqlx::Error>;
    async fn get_active(&self) -> Result<Vec<Onboarding>, sqlx::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Onboarding>, sqlx::Error>;
    
    // Fungsi untuk ONB-04: Menambahkan dokumen dan mengubah status
    async fn add_document_and_update_status(
        &self, 
        onboarding_id: Uuid, 
        doc_type: String, 
        file_path: String
    ) -> Result<(), sqlx::Error>;
}