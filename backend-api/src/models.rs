//models.rs - vytvoření struktur pro databázové připojení
// 1. validace vstupních dat a struktura pro registraci uživatele

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

// validace vstupních dat pro registraci uživatele
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 2, message = "Uživatelské jméno je příliš krátké"))]
    pub username: String,

    #[validate(length(min = 6, message = "Heslo musí mít alespoň 6 znaků"))]
    pub password: String,
}

// struktura pro registraci uživatele
#[derive(Debug, Serialize)]
pub struct UserOutput {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}
