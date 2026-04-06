use crate::domain::{
    entities::{Onboarding, User},
    repositories::{OnboardingRepository, UserRepository},
};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        let rec = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, name, email, role, password)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, email, role, password
            "#,
            user.id,
            user.name,
            user.email,
            user.role,
            user.password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let rec = sqlx::query_as!(
            User,
            r#"SELECT id, name, email, role, password FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }
}

#[async_trait]
impl OnboardingRepository for PostgresRepository {
    async fn create(&self, onboarding: Onboarding) -> Result<Onboarding, sqlx::Error> {
        let rec = sqlx::query_as!(
            Onboarding,
            r#"
            INSERT INTO onboardings (id, first_name, last_name, personal_email, department, start_date, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, first_name, last_name, personal_email, department, start_date, status as "status: _", created_at, updated_at
            "#,
            onboarding.id,
            onboarding.first_name,
            onboarding.last_name,
            onboarding.personal_email,
            onboarding.department,
            onboarding.start_date,
            onboarding.status as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn get_active(&self) -> Result<Vec<Onboarding>, sqlx::Error> {
        let recs = sqlx::query_as!(
            Onboarding,
            r#"
            SELECT id, first_name, last_name, personal_email, department, start_date, status as "status: _", created_at, updated_at
            FROM onboardings
            WHERE status != 'COMPLETED'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(recs)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Onboarding>, sqlx::Error> {
        let rec = sqlx::query_as!(
            Onboarding,
            r#"
            SELECT id, first_name, last_name, personal_email, department, start_date, status as "status: _", created_at, updated_at
            FROM onboardings
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn add_document_and_update_status(
        &self,
        onboarding_id: Uuid,
        doc_type: String,
        file_path: String,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"INSERT INTO onboarding_documents (onboarding_id, document_type, file_path) VALUES ($1, $2, $3)"#,
            onboarding_id, doc_type, file_path
        )
        .execute(&mut *tx).await?;

        sqlx::query!(
            r#"UPDATE onboardings SET status = 'PENDING_REVIEW', updated_at = NOW() WHERE id = $1"#,
            onboarding_id
        )
        .execute(&mut *tx).await?;

        sqlx::query!(
            r#"INSERT INTO onboarding_history (onboarding_id, new_status, notes) VALUES ($1, 'PENDING_REVIEW', 'Document uploaded')"#,
            onboarding_id
        )
        .execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }
}