use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

pub async fn crear_bbdd_memoria() -> Pool<Sqlite> {
    let datos = include_str!("datos.sql");

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .max_lifetime(None)
        .idle_timeout(None)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(datos)
        .execute(&pool)
        .await
        .expect("Fallo en datos");

    return pool;
}
