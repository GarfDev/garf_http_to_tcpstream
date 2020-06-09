extern crate tokio;

use rcon::{Connection, Error};
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, Result};
use dotenv;
use std::env;

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
    let mut connection = Connection::connect(rcon_address, &rcon_password).await.unwrap();
    let result = send(&mut connection, &command.command).await.unwrap();

    Ok(format!("{}", result))

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| App::new().route("/", web::post().to(call_command)))
        .bind("0.0.0.0:3030")?
        .run()
        .await
}

