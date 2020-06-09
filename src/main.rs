extern crate tokio;

use rcon::{Connection, Error};
use serde::{Deserialize, Serialize};
use dotenv;
use warp::Filter;
use std::env;


mod rcon;


// Types
#[derive(Serialize, Deserialize, Debug)]
struct Command {
    from: String,
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: i32,
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let command = warp::post()
        // .and(warp::path("command"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|command: Command| {

            let _ = tokio::spawn(async move {
                let rcon_address = env::var("RCON_ADDRESS").unwrap();
                let rcon_password = env::var("RCON_PASSWORD").unwrap();
                // print!("{}:{}",rcon_address,rcon_password);
                let mut connection = Connection::connect(rcon_address, &rcon_password).await.unwrap();
                let result = send(&mut connection, &command.command).await.unwrap();
                return result
            });
            
            warp::reply::json(&{})

        });

    warp::serve(command).run(([0, 0, 0, 0], 3030)).await

}

async fn send(conn: &mut Connection, cmd: &str) -> Result<String, Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(resp)
}