use err_derive::Error;
use packet::{Packet, PacketType};
use std::io;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::delay_for;

mod packet;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "authentication failed")]
    Auth,
    #[error(display = "command exceeds the maximum length")]
    CommandTooLong,
    #[error(display = "{}", _0)]
    Io(#[error(source)] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Connection {
    stream: TcpStream,
    next_packet_id: i32,
}

const INITIAL_PACKET_ID: i32 = 1;

const DELAY_TIME_MILLIS: u64 = 3;

impl Connection {
    pub async fn connect<T: ToSocketAddrs>(address: T, password: &str) -> Result<Connection> {
        let stream = TcpStream::connect(address).await?;
        let mut conn = Connection {
            stream,
            next_packet_id: INITIAL_PACKET_ID,
        };

        conn.auth(password).await?;

        Ok(conn)
    }

    pub async fn cmd(&mut self, cmd: &str) -> Result<String> {
        if cmd.len() > 1413 {
            return Err(Error::CommandTooLong);
        }

        self.send(PacketType::ExecCommand, cmd).await?;

        if cfg!(feature = "delay") {
            delay_for(Duration::from_millis(DELAY_TIME_MILLIS)).await;
        }

        let end_id = self.send(PacketType::ExecCommand, "").await?;

        let mut result = String::new();

        loop {
            let received_packet = self.recv().await?;

            if received_packet.get_id() == end_id {
                // This is the response to the end-marker packet
                break;
            }

            result += received_packet.get_body();
        }

        Ok(result)
    }

    async fn auth(&mut self, password: &str) -> Result<()> {
        self.send(PacketType::Auth, password).await?;
        let received_packet = loop {
            let received_packet = self.recv().await?;
            if received_packet.get_type() == PacketType::AuthResponse {
                break received_packet;
            }
        };

        if received_packet.is_error() {
            Err(Error::Auth)
        } else {
            Ok(())
        }
    }

    async fn send(&mut self, ptype: PacketType, body: &str) -> io::Result<i32> {
        let id = self.generate_packet_id();

        let packet = Packet::new(id, ptype, body.into());

        packet.serialize(&mut self.stream).await?;

        Ok(id)
    }

    async fn recv(&mut self) -> io::Result<Packet> {
        Packet::deserialize(&mut self.stream).await
    }

    fn generate_packet_id(&mut self) -> i32 {
        let id = self.next_packet_id;

        self.next_packet_id = self
            .next_packet_id
            .checked_add(1)
            .unwrap_or(INITIAL_PACKET_ID);

        id
    }
}
