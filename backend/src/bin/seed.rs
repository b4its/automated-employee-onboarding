use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load environment variables dari .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diset di file .env");

    // 2. Buat koneksi pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("🌱 Memulai proses seeding...");

    // 3. Data yang akan dimasukkan
    // Tips: Di dunia nyata, pastikan password sudah di-hash!
    let users = vec![
        ("Nur Hikari", "admin@address.com", "admin", "admin123"),
    ];

    for (name, email, role, password) in users {
        let hashed_password = hash(password, DEFAULT_COST)?;
        let res = sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, role, password)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (email) DO NOTHING
            "#,
            Uuid::new_v4(),
            name,
            email,
            role,
            hashed_password
        )
        .execute(&pool)
        .await?;

        if res.rows_affected() > 0 {
            println!("✅ Berhasil membuat user: {}", email);
        } else {
            println!("⚠️ User {} sudah ada, dilewati.", email);
        }
    }

    println!("✨ Seeding selesai!");
    Ok(())
}