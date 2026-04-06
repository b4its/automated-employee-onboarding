use crate::application::AppService;
use axum::{
    extract::{Multipart, Path, State}, 
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<AppService>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Auth Routes
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        // Onboarding Routes
        .route("/api/v1/onboarding/initiate", post(initiate_handler))
        .route("/api/v1/onboarding/active", get(active_handler))
        .route("/api/v1/onboarding/:id", get(get_by_id_handler))
        // Document Upload Route 
        .route("/api/v1/onboarding/:id/documents", post(upload_document_handler))
        .with_state(state)
}

// --- Auth DTOs & Handlers ---
#[derive(Deserialize)]
pub struct RegisterReq { pub name: String, pub email: String, pub password: String }
#[derive(Deserialize)]
pub struct LoginReq { pub email: String, pub password: String }

async fn register_handler(State(state): State<AppState>, Json(payload): Json<RegisterReq>) -> impl IntoResponse {
    match state.service.register(payload.name, payload.email, payload.password).await {
        Ok(user) => (StatusCode::CREATED, Json(serde_json::json!({"id": user.id, "email": user.email}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn login_handler(State(state): State<AppState>, Json(payload): Json<LoginReq>) -> impl IntoResponse {
    match state.service.login(payload.email, payload.password).await {
        Ok(token) => (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// --- Onboarding DTOs & Handlers ---
#[derive(Deserialize)]
pub struct InitiateReq {
    #[serde(rename = "firstName")] pub first_name: String,
    #[serde(rename = "lastName")] pub last_name: String,
    #[serde(rename = "personalEmail")] pub personal_email: String,
    pub department: String,
    #[serde(rename = "startDate")] pub start_date: NaiveDate,
}

async fn initiate_handler(State(state): State<AppState>, Json(payload): Json<InitiateReq>) -> impl IntoResponse {
    match state.service.initiate_onboarding(payload.first_name, payload.last_name, payload.personal_email, payload.department, payload.start_date).await {
        Ok(ob) => {
            // [cite: 61, 62, 63]
            let resp = serde_json::json!({
                "onboardingId": format!("ob_{}", ob.id),
                "status": "INITIATED",
                "message": "Onboarding started. Welcome email sent to candidate."
            });
            (StatusCode::CREATED, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn active_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.service.get_active_onboardings().await {
        Ok(obs) => {
            // [cite: 68, 69, 70, 71, 72, 73, 74, 75, 76, 77]
            let data: Vec<_> = obs.into_iter().map(|o| {
                serde_json::json!({
                    "onboardingId": format!("ob_{}", o.id),
                    "candidateName": format!("{} {}", o.first_name, o.last_name),
                    "department": o.department,
                    "startDate": o.start_date.format("%Y-%m-%d").to_string(),
                    "status": format!("{:?}", o.status).to_uppercase()
                })
            }).collect();
            let total = data.len();
            (StatusCode::OK, Json(serde_json::json!({"data": data, "totalActive": total}))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn get_by_id_handler(State(state): State<AppState>, Path(id_str): Path<String>) -> impl IntoResponse {
    let clean_id = id_str.replace("ob_", "");
    let uuid = match Uuid::parse_str(&clean_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid ID format"}))).into_response(),
    };

    match state.service.get_onboarding_by_id(uuid).await {
        Ok(ob) => (StatusCode::OK, Json(serde_json::json!({
            "onboardingId": format!("ob_{}", ob.id),
            "candidateName": format!("{} {}", ob.first_name, ob.last_name),
            "status": format!("{:?}", ob.status).to_uppercase()
        }))).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// --- Handler Baru: Upload Document --- //
async fn upload_document_handler(
    State(state): State<AppState>,
    Path(id_str): Path<String>,
    mut multipart: Multipart, // Menerima multipart/form-data 
) -> impl IntoResponse {
    let clean_id = id_str.replace("ob_", "");
    let uuid = match Uuid::parse_str(&clean_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid ID format"}))).into_response(),
    };

    let mut document_type = String::new();
    let mut file_bytes = Vec::new();
    let mut file_name = String::new();

    // Membaca setiap field dari multipart payload [cite: 80, 81, 82]
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "documentType" {
            document_type = field.text().await.unwrap_or_default();
        } else if name == "file" {
            file_name = field.file_name().unwrap_or("unknown.pdf").to_string();
            file_bytes = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    if file_bytes.is_empty() || document_type.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Field 'file' dan 'documentType' wajib diisi"}))).into_response();
    }

    match state.service.upload_document(uuid, document_type, file_name, file_bytes).await {
        Ok(_) => {
            // Sesuai dengan API Contract Response dokumen [cite: 84, 85, 86, 87, 88]
            let resp = serde_json::json!({
                "onboardingId": format!("ob_{}", uuid),
                "status": "PENDING_REVIEW",
                "message": "Document encrypted and stored successfully."
            });
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}