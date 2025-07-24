// handlers.rs - logika registrace uživatele

use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{PgPool, Row};
use bcrypt::{hash, DEFAULT_COST};
use validator::Validate;

use crate::models::{RegisterInput, UserOutput};

pub async fn register_user(
    State(db): State<PgPool>,
    Json(payload): Json<RegisterInput>,
) -> impl IntoResponse {
    // Validace vstupu
    if let Err(e) = payload.validate() {
        let msg = e.field_errors()
            .iter()
            .map(|(field, errs)| format!("{}: {}", field, errs[0].message.clone().unwrap_or_default()))
            .collect::<Vec<_>>()
            .join(", ");
        return (StatusCode::BAD_REQUEST, msg).into_response();
    }

    // Hash hesla
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Chyba při hashování hesla").into_response(),
    };

    // Vložení do DB
    let result = sqlx::query(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, created_at
        "#
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&db)
    .await;

    match result {
        Ok(row) => {
            let output = UserOutput {
                id: row.get("id"),
                username: row.get("username"),
                created_at: row.get("created_at"),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Uživatel {} zaregistrován!", output.username)).into_response()
        }
        Err(e) => {
            eprintln!("❌ DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Chyba databáze").into_response()
        }
    }
}
