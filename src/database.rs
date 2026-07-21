use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;


pub async fn conectar_banco(
    database_url: &str
) -> PgPool {

    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Erro ao conectar ao banco de dados")
}
