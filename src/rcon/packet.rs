use std::io;
use tokio::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketType {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
    Unknown(i32),
}

impl PacketType {
    fn to_i32(self) -> i32 {
        match self {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::Unknown(n) => n,
        }
    }

    pub fn from_i32(n: i32, is_response: bool) -> PacketType {
        match n {
            3 => PacketType::Auth,
            2 if is_response => PacketType::AuthResponse,
            2 => PacketType::ExecCommand,
            0 => PacketType::ResponseValue,
            n => PacketType::Unknown(n),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    length: i32,
    id: i32,
    ptype: PacketType,
    body: String,
}

impl Packet {
    pub fn new(id: i32, ptype: PacketType, body: String) -> Packet {
        Packet {
            length: 10 + body.len() as i32,
            id,
            ptype,
            body,
        }
    }

    pub fn is_error(&self) -> bool {
        self.id < 0
    }

    pub async fn serialize<T: Unpin + AsyncWrite>(&self, w: &mut T) -> io::Result<()> {
        let mut buf = Vec::with_capacity(self.length as usize);

        io::Write::write(&mut buf, &self.length.to_le_bytes()).unwrap();
        io::Write::write(&mut buf, &self.id.to_le_bytes()).unwrap();
        io::Write::write(&mut buf, &self.ptype.to_i32().to_le_bytes()).unwrap();
        io::Write::write(&mut buf, self.body.as_bytes()).unwrap();
        io::Write::write(&mut buf, b"\x00\x00").unwrap();

        w.write(&buf).await?;

        Ok(())
    }

    pub async fn deserialize<T: Unpin + AsyncRead>(r: &mut T) -> io::Result<Packet> {
        let length = i32::from_be(r.read_i32().await?);
        let id = i32::from_be(r.read_i32().await?);
        let ptype = i32::from_be(r.read_i32().await?);
        let body_length = length - 10;
        let mut body_buffer = Vec::with_capacity(body_length as usize);
        r.take(body_length as u64)
            .read_to_end(&mut body_buffer)
            .await?;
        let body = String::from_utf8(body_buffer)
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        r.read_u16().await?;

        let packet = Packet {
            length,
            id,
            ptype: PacketType::from_i32(ptype, true),
            body,
        };

        Ok(packet)
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_type(&self) -> PacketType {
        self.ptype
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
