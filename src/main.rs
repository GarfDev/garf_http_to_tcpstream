use rcon::{Connection, Error};

mod rcon;

/*
    This example expects a Minecraft with rcon enabled on port 25575
    and the rcon password "test"
*/

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = "0.0.0.0:3333";
    let mut conn = Connection::connect(address, "test").await?;

    demo(&mut conn, "list").await?;
    demo(&mut conn, "say Rust lang rocks! ;P").await?;
    //demo(&mut conn, "stop");
    Ok(())
}

async fn demo(conn: &mut Connection, cmd: &str) -> Result<(), Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(())
}