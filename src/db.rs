use sqlx::postgres::PgPoolOptions;

use uuid::Uuid;

use crate::auth::user::UserInfo;
use crate::logging::log::{log_error, log_warn};

pub const MAX_DB_CONNECTIONS: u32 = 7;
pub const SECRET_ACCESS_KEY_TABLE: &str = "secret_access_keys";
pub const SECRET_REFRESH_KEY_TABLE: &str = "refresh_access_keys";
pub const USERS_TABLE: &str = "users";
pub const SESSION_TABLE: &str = "session_table";
pub const SUBSCRIPTION_LEVEL_TYPE: &str = "subscription_level";

pub static mut ENVIRONMENT_CONSTANTS: Option<
    crate::startup::environment_constants::EnvironmentConstants,
> = None;

fn quote_string_value(value: &str) -> String {
    format!("\'{}\'", value)
}

pub async fn check_database_connection() -> Result<(), Box<dyn std::error::Error>> {
    match crate::db::create_pool().await {
        Ok(_) => Ok(()),
        _ => {
            log_error("Cannot connect to the database");
            std::process::abort();
        }
    }
}

async fn create_secret_access_key_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS {} (
user_id BIGINT PRIMARY KEY,
secret_key VARCHAR(255) NOT NULL)",
        SECRET_ACCESS_KEY_TABLE
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn create_secret_refresh_key_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS {} (
user_id BIGINT PRIMARY KEY,
secret_refresh_key VARCHAR(255) NOT NULL)",
        SECRET_REFRESH_KEY_TABLE
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn create_session_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS {} (
session_uuid varchar(36) PRIMARY KEY,
user_id BIGINT NOT NULL, 
session_id BIGSERIAL UNIQUE,
log_date TIMESTAMP NOT NULL DEFAULT current_timestamp)",
        SESSION_TABLE
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn create_user_level_enum_type(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        r"
        DO $$ BEGIN
            CREATE TYPE {} AS ENUM (
            'non-premium',
            'basic-premium',
            'normal-premium',
            'enterprise-premium');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;",
        SUBSCRIPTION_LEVEL_TYPE
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn create_user_backend(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    create_user_level_enum_type(pool).await?;
    create_user_table(pool).await?;
    Ok(())
}

async fn create_user_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS {} (
        user_id BIGSERIAL PRIMARY KEY, 
        username VARCHAR(50) NOT NULL UNIQUE,
        password VARCHAR(256) NOT NULL,
        email VARCHAR(50) NOT NULL,
        subscription subscription_level NOT NULL DEFAULT 'non-premium'
        );",
        USERS_TABLE
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

// TODO: [SPIR-101] this function will be used in this task
#[allow(dead_code)]
pub async fn set_user_subscription_level(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i64,
    level: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        "UPDATE {} SET subscription = $1 WHERE user_id = $2",
        USERS_TABLE
    );

    sqlx::query(&query)
        .bind(level)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn drop_user_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DROP TABLE IF EXISTS {}", USERS_TABLE);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn drop_level_subscription_type(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DROP TYPE IF EXISTS {}", SUBSCRIPTION_LEVEL_TYPE);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn drop_user_backend(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    drop_user_table(pool).await?;
    drop_level_subscription_type(pool).await?;
    Ok(())
}

async fn drop_secret_access_key_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DROP TABLE IF EXISTS {}", SECRET_ACCESS_KEY_TABLE);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn drop_session_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DROP TABLE IF EXISTS {}", SESSION_TABLE);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

async fn drop_secret_refresh_key_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DROP TABLE IF EXISTS {}", SECRET_REFRESH_KEY_TABLE);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn clear_database() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    log_warn("starting database clean up...");
    drop_user_backend(&pool).await?;
    create_user_backend(&pool).await?;
    drop_secret_access_key_table(&pool).await?;
    create_secret_access_key_table(&pool).await?;
    drop_secret_refresh_key_table(&pool).await?;
    create_secret_refresh_key_table(&pool).await?;
    drop_session_table(&pool).await?;
    create_session_table(&pool).await?;
    log_warn("database clean up done.");

    Ok(())
}

pub async fn does_user_id_exists(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT COUNT(*) FROM {} WHERE user_id={}",
        USERS_TABLE, user_id
    );
    let row: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;

    Ok(row.0 == 1)
}

pub async fn does_username_exists(
    pool: &sqlx::Pool<sqlx::Postgres>,
    username: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT COUNT(*) FROM {} WHERE username={}",
        USERS_TABLE,
        quote_string_value(username)
    );
    let row: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;
    Ok(row.0 == 1)
}

pub async fn get_secret_access_key(
    user_id: i64,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT secret_key FROM {} WHERE user_id={}",
        SECRET_ACCESS_KEY_TABLE, user_id
    );
    let row: (String,) = sqlx::query_as(&query).fetch_one(pool).await?;
    Ok(row.0)
}

pub async fn get_secret_refresh_key(
    user_id: i64,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT secret_refresh_key FROM {} WHERE user_id={}",
        SECRET_REFRESH_KEY_TABLE, user_id
    );
    let row: (String,) = sqlx::query_as(&query).fetch_one(pool).await?;
    Ok(row.0)
}

pub async fn get_user_from_db(
    username: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT user_id, password, email FROM {} WHERE username={}",
        USERS_TABLE,
        quote_string_value(username)
    );
    let row: (i64, String, String) = sqlx::query_as(&query).fetch_one(pool).await?;

    let user = UserInfo {
        user_id: row.0,
        username: username.to_string(),
        password: row.1,
        email: row.2,
    };
    Ok(user)
}

pub async fn get_user_from_db_with_user_id(
    user_id: i64,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT username, password, email FROM {} WHERE user_id={}",
        USERS_TABLE, user_id
    );

    let row: (String, String, String) = sqlx::query_as(&query).fetch_one(pool).await?;

    let user = UserInfo {
        user_id,
        username: row.0,
        password: row.1,
        email: row.2,
    };
    Ok(user)
}

pub async fn create_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    unsafe {
        match &ENVIRONMENT_CONSTANTS {
            Some(constants) => {
                // TODO: [SPIR-63] Database address should be evaluated only once
                let database_url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    constants.database_user.clone(),
                    constants.database_password.clone(),
                    constants.database_address.clone(),
                    constants.database_port.clone(),
                    constants.database_name.clone()
                );

                crate::logging::log::log_info(&format!(
                    "Connecting to database at address: {}",
                    database_url
                ));
                PgPoolOptions::new()
                    .max_connections(MAX_DB_CONNECTIONS)
                    .connect(&database_url)
                    .await
            }
            _ => {
                log_error("We lost access to environemnt variables");
                std::process::abort();
            }
        }
    }
}

pub async fn store_secret_access_key(
    user_id: i64,
    secret_key: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        "INSERT INTO {} (user_id, secret_key) VALUES ({}, {})",
        SECRET_ACCESS_KEY_TABLE,
        user_id,
        quote_string_value(secret_key)
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn store_secret_refresh_key(
    user_id: i64,
    secret_refresh_key: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        "INSERT INTO {} (user_id, secret_refresh_key) VALUES ({}, {})",
        SECRET_REFRESH_KEY_TABLE,
        user_id,
        quote_string_value(secret_refresh_key)
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

#[derive(Debug)]
pub struct SessionCreationError;

impl std::fmt::Display for SessionCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Session was not created!")
    }
}

impl std::error::Error for SessionCreationError {}

pub async fn create_session(
    user: &UserInfo,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, Box<dyn std::error::Error>> {
    let session_uuid = Uuid::new_v4();
    match store_session_info(user.user_id, &session_uuid, pool).await {
        Ok(_) => Ok(session_uuid.to_string()),
        Err(e) => {
            log_error(&format!(
                "There was an error with session creation.\nReason: {}",
                e
            ));
            Err(Box::new(SessionCreationError))
        }
    }
}

pub async fn get_session_uuid(
    user_id: i64,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT session_uuid FROM {} WHERE user_id={} ORDER BY session_id DESC",
        SESSION_TABLE, user_id
    );
    let row: (String,) = sqlx::query_as(&query).fetch_one(pool).await?;
    Ok(row.0)
}

pub async fn get_session_user_id(
    session_uuid: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT user_id FROM {} WHERE session_uuid = {}",
        SESSION_TABLE,
        quote_string_value(session_uuid)
    );

    let row: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;
    Ok(row.0)
}

pub async fn get_user_id_with_session_uuid(
    session_uuid: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = crate::db::create_pool().await?;
    get_session_user_id(session_uuid, &pool).await
}

pub async fn store_session_info(
    user_id: i64,
    session_uuid: &Uuid,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        "INSERT INTO {} (user_id, session_uuid) VALUES ({}, {})",
        SESSION_TABLE,
        user_id,
        quote_string_value(&session_uuid.to_string())
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn store_new_user(
    username: &str,
    password_hash: &str,
    email: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(
        "INSERT INTO {} (username, password, email) VALUES ({}, {}, {})",
        USERS_TABLE,
        quote_string_value(username),
        quote_string_value(password_hash),
        quote_string_value(email)
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn drop_session(session_uuid: &str) -> bool {
    let pool = crate::db::create_pool().await.unwrap();
    match proceed_drop_session(session_uuid, &pool).await {
        Ok(_) => true,
        Err(e) => {
            log_error(&e.to_string());
            false
        }
    }
}

pub async fn proceed_drop_session(
    session_uuid: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), sqlx::error::Error> {
    let query = format!(
        "DELETE FROM {} WHERE session_uuid={}",
        SESSION_TABLE,
        quote_string_value(session_uuid)
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}
