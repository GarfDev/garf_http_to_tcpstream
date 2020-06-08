use rcon::{Connection, Error};
use dotenv;
use std::env;

mod rcon;


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::from_filename(".env").ok();
    let rcon_address = env::var("RCON_ADDRESS").unwrap();
    let rcon_password = env::var("RCON_PASSWORD").unwrap();

    let mut conn = Connection::connect(rcon_address, &rcon_password).await?;

    demo(&mut conn, "list").await?;
    demo(&mut conn, "say Rust lang rocks! ;P").await?;
    Ok(())
}

async fn demo(conn: &mut Connection, cmd: &str) -> Result<(), Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(())
}