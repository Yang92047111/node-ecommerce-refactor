use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;

pub struct TestDatabase<'a> {
    _container: Container<'a, Postgres>,
    pub pool: PgPool,
    pub connection_string: String,
}

impl<'a> TestDatabase<'a> {
    pub async fn new(docker: &'a Cli) -> Self {
        // Start PostgreSQL container
        let container = docker.run(Postgres::default());
        let host_port = container.get_host_port_ipv4(5432);
        
        // Build connection string
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            host_port
        );

        // Create connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .expect("Failed to create test database pool");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations on test database");

        Self {
            _container: container,
            pool,
            connection_string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_setup() {
        let docker = Cli::default();
        let test_db = TestDatabase::new(&docker).await;
        
        // Verify connection works
        let result = sqlx::query("SELECT 1")
            .fetch_one(&test_db.pool)
            .await;
        
        assert!(result.is_ok());
    }
}
