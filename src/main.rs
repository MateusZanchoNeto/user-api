mod application;
mod config;
mod core;
mod infrastructure;
mod presentation;
mod schema;
pub mod test_orchestrator;

use crate::config::database_config::DatabaseType::Postgres;
use crate::config::{env::load_enviroment, settings::Settings};
use crate::infrastructure::database::postgres::database_manager::DatabaseManager;
use crate::presentation::controllers::configure_routes;
use actix_web::{web, App, HttpServer};
use std::io::Write;
use std::process::Command;
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_enviroment();

    let mut settings = Settings::new();
    let port = settings.port;

    println!("Settings:");
    println!("environment: {}", settings.environment);
    println!("port.......: {}", port);
    println!("database...: {:?}", settings.database_config.database_type);

    if settings.database_config.database_type == Postgres {
        start_postgres();
        check_postgres();
        init_connection_pool(&mut settings);
    }

    println!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(settings.clone()))
            .configure(configure_routes)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

fn start_postgres() {
    let status = Command::new("docker")
        .arg("compose")
        .arg("up")
        .arg("-d")
        .status()
        .unwrap();

    if !status.success() {
        panic!("Failed to start docker-compose");
    }
}

fn check_postgres() {
    loop {
        let output = Command::new("docker")
            .arg("exec")
            .arg("user-api-db")
            .arg("pg_isready")
            .arg("--host")
            .arg("localhost")
            .output()
            .unwrap();

        if String::from_utf8_lossy(&output.stdout).contains("accepting connections") {
            break;
        }

        for i in 0..4 {
            print!(
                "\rWaiting for postgres to accept connections{}",
                " ".repeat(4)
            );
            print!(
                "\rWaiting for postgres to accept connections{}",
                ".".repeat(i)
            );
            std::io::stdout().flush().unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

fn init_connection_pool(settings: &mut Settings) {
    let database_manager = DatabaseManager::new(&format!(
        "postgres://{}:{}@{}/{}",
        settings.database_config.user,
        settings.database_config.password,
        settings.database_config.host,
        settings.database_config.database
    ));
    settings.connection_pool = Some(database_manager.get_pool());
}
