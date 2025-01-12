mod application;
mod config;
mod core;
mod infrastructure;
mod presentation;

use crate::config::database_config::DatabaseType::Postgres;
use crate::config::{env::load_enviroment, settings::Settings};
use crate::presentation::controllers::user_controller::configure_user_routes;
use actix_web::{web, App, HttpServer};
use std::io::Write;
use std::process::Command;
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_enviroment();

    let settings = Settings::new();
    let port = settings.port;

    println!("Settings:");
    println!("environment: {}", settings.environment);
    println!("port.......: {}", port);
    println!("database...: {:?}", settings.database_type);

    if settings.database_type == Postgres {
        start_postgres();
        check_postgres();
    }

    println!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(settings.clone()))
            .configure(configure_user_routes)
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
    let output = Command::new("docker")
        .arg("exec")
        .arg("user-api-db")
        .arg("pg_isready")
        .arg("--host")
        .arg("localhost")
        .output()
        .unwrap();

    loop {
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
