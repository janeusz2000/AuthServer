pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_DATABASE_ADDRESS: &str = "127.0.0.1";

#[derive(Clone)]
pub struct EnvironmentConstants {
    pub address: String,
    pub port: String,
    pub database_address: String,
    pub database_port: String,
    pub database_name: String,
    pub database_user: String,
    pub database_password: String,
    pub request_throttling_limit: usize,
    pub connection_timeout: u64,
    pub client_timeout: u64,
    pub client_disconnect_timeout: u64,
}

pub fn get_environment_constants() -> EnvironmentConstants {
    let address =
        std::env::var("AUTH_SERVER_ADDRESS").unwrap_or_else(|_| DEFAULT_SERVER_ADDRESS.to_string());
    let port = std::env::var("AUTH_SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let database_address =
        std::env::var("DATABASE_ADDRESS").unwrap_or_else(|_| DEFAULT_DATABASE_ADDRESS.to_string());
    let database_user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "Jasiu".to_string());
    let database_password =
        std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "doopajasiu".to_string());
    let database_port = std::env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
    let database_name =
        std::env::var("DATABASE_NAME").unwrap_or_else(|_| "user_database".to_string());

    // Rate limiting / Synchronous request prevention
    let request_throttling_limit: usize = std::env::var("REQUEST_THROTTLING_LIMIT")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);

    let connection_timeout: u64 = std::env::var("CONNECTION_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);
    let client_timeout: u64 = std::env::var("CLIENT_TIMEOUT")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    let client_disconnect_timeout: u64 = std::env::var("CLIENT_DISCONNECT_TIMEOUT")
        .unwrap_or_else(|_| "500".to_string())
        .parse()
        .unwrap_or(500);

    EnvironmentConstants {
        address,
        port,
        database_address,
        database_port,
        database_name,
        database_user,
        database_password,
        request_throttling_limit,
        connection_timeout,
        client_timeout,
        client_disconnect_timeout,
    }
}
