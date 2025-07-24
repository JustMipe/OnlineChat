// vytvoření tabulek pro databázové připojení

use sqlx::{PgPool, Executor};

// vytvoření tabulky users (pokud neexistuje)
pub async fn inicializuj_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now()
        );
        "#,
    )
    .await?;

    println!("✅ Tabulka 'users' pro registraci uživatelů připravena");
    Ok(())
}
