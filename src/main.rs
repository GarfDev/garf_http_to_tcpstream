extern crate tokio;

use chrono::prelude::*;
use colored::Colorize;
use rcon::{Connection, Error};
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, Result};
use std::env;
use dotenv;

mod rcon;

// TYPES

#[derive(Serialize, Deserialize)]
struct Command {
    from: String,
    command: String,
}

async fn send(conn: &mut Connection, cmd: &str) -> Result<String, Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(resp)
}

async fn call_command(command: web::Json<Command>) -> Result<String> {
    let rcon_address = env::var("RCON_ADDRESS").unwrap();
    let rcon_password = env::var("RCON_PASSWORD").unwrap();
    match Connection::connect(rcon_address, &rcon_password).await {
        Ok(mut conn) => {
            let result = send(&mut conn, &command.command).await.unwrap();
            let now = Utc::now();
            let (_is_common_era, year) = now.year_ce();
            let timestamp = format!("[{}-{:02}-{:02} {:02}:{:02}:{:02}]",year,
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
            println!(
                "{} Successed on execute command: {}",
                timestamp.green(),
                command.command
            );
            Ok(format!("{}", result))
        }
        Err(error) => {
            let now = Utc::now();
            let (_is_common_era, year) = now.year_ce();
            let timestamp = format!("[{}-{:02}-{:02} {:02}:{:02}:{:02}]",year,
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
            println!(
                "{} Failed on execute command: {}",
                timestamp.red(),
                command.command
            );
            Ok(format!("{}", error))
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| App::new().route("/", web::post().to(call_command)))
        .bind("0.0.0.0:3030")?
        .run()
        .await
}
