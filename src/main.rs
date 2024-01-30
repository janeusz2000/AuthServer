use actix_session::SessionExt;
use actix_web::http::KeepAlive;
use actix_web::{dev::ServiceRequest, middleware::Logger, web, App, HttpServer};

use auth_server::auth::api_requests::{login, logout, ping, refresh_token, register};
use auth_server::logging::log::{log_error, log_info};
use auth_server::startup::environment_constants::EnvironmentConstants;
use auth_server::xml_request;
use clap::{App as ClapApp, Arg};
use colored::*;
use std::io::Write;

fn initialize_logger() {
    // Initalize env_logger
    env_logger::Builder::new()
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Cyan,
                log::Level::Trace => Color::Magenta,
            };

            std::writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .color(Color::Cyan),
                record.level().to_string().color(level_color),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}

async fn handle_flag_arguments() {
    let flag_matches = ClapApp::new("Authentication server")
        .arg(
            Arg::with_name("clear_database")
                .short('c')
                .long("clear_database")
                .help("clears the database"),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("registers admin user in the database"),
        )
        .get_matches();

    log_info("Gathered initial flags:");
    let clear_database_flag = flag_matches.is_present("clear_database");
    let debug_flag = flag_matches.is_present("debug");
    log_info(&format!("clear database:\t{}", clear_database_flag));

    // Flag execution
    if clear_database_flag {
        match auth_server::db::clear_database().await {
            Ok(_) => {
                log_info("Database clear SUCCESS");
            }
            Err(e) => {
                log_error(&format!("Could not clear the database.\nReason {}", e));
            }
        }
    }

    if debug_flag {
        const ADMIN_LOGIN: &str = "Zenek";
        const ADMIN_PASSWORD: &str = "Super";
        const ADMIN_EMAIL: &str = "lol@kekw.com";
        match register::create_user(ADMIN_LOGIN, ADMIN_PASSWORD, ADMIN_EMAIL).await {
            Ok(_) => {
                log_info(&format!(
                    "User {} was created with password: {}",
                    ADMIN_LOGIN, ADMIN_PASSWORD
                ));
            }
            Err(e) => {
                log_error(&format!("Debug user could not be crated. Reason: {}", e));
                std::process::abort();
            }
        }
    }
}

fn send_server_is_ready_event(env_constants: &EnvironmentConstants) {
    println!("Server is ready for handling requests!");
    log_info(&format!(
        "You can access server at: http://{}:{}",
        env_constants.address, env_constants.port
    ));
}

async fn perform_startup_sequence(env_constants: &EnvironmentConstants) {
    initialize_logger();
    auth_server::db::check_database_connection().await.unwrap();
    handle_flag_arguments().await;
    send_server_is_ready_event(env_constants);
}

async fn index() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let environment_constants: EnvironmentConstants =
        auth_server::startup::environment_constants::get_environment_constants();

    unsafe {
        auth_server::db::ENVIRONMENT_CONSTANTS = Some(environment_constants.clone());
    }

    perform_startup_sequence(&environment_constants).await;

    let limiter = web::Data::new(
        actix_limitation::Limiter::builder("redis://127.0.0.1")
            .key_by(|req: &ServiceRequest| {
                req.get_session()
                    .get("session-id")
                    .unwrap_or_else(|_| req.cookie("rate-api-id").map(|c| c.to_string()))
            })
            .limit(environment_constants.request_throttling_limit)
            .period(std::time::Duration::from_secs(60))
            .build()
            .unwrap(),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("request acquired:\tIP:\t\t%a\nREQUEST TYPE:\t%r\nPROCESS TIME:\t%T\nURL:\t\t%U\nRESPONSE:\t%s\nAGENT:\t\t%{User-Agent}i"))
            .wrap(actix_limitation::RateLimiter::default())
            .app_data(limiter.clone())
            .route("/ping", web::get().to(ping::ping))
            .route("/", web::get().to(index))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register::register))
                    .route("/login", web::post().to(login::login))
                    .route("/logout", web::get().to(logout::logout))
                    .route("/refresh", web::post().to(refresh_token::refresh_token)),
            )
            .service(web::scope("/xml-api").route(
                "/send_xml",
                web::post().to(xml_request::xml_private_api::send_xml),
            ))
    })
    .keep_alive(KeepAlive::Timeout(std::time::Duration::from_secs(
        environment_constants.connection_timeout,
    )))
    .client_request_timeout(std::time::Duration::from_secs(
        environment_constants.client_timeout,
    ))
    .client_disconnect_timeout(std::time::Duration::from_secs(
        environment_constants.client_disconnect_timeout,
    ))
    .bind(std::format!(
        "{}:{}",
        environment_constants.address,
        environment_constants.port
    ))?
    .run()
    .await
}
