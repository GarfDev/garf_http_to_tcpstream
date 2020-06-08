use rcon::{Connection, Error};
use dotenv;
use std::env;


mod rcon;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let rcon_address = env::var("RCON_ADDRESS").unwrap();
    let rcon_password = env::var("RCON_PASSWORD").unwrap();

    let mut connection = Connection::connect(rcon_address, &rcon_password).await.unwrap();

}

async fn send(conn: &mut Connection, cmd: &str) -> Result<(), Error> {
    let resp = conn.cmd(cmd).await?;
    // println!("{}", resp);
    Ok(())
}