use pumpkin_macros::packet;

use crate::{
    bytebuf::{ByteBuffer, DeserializerError},
    ConnectionState, ServerPacket, VarInt,
};

#[packet(0x00)]
pub struct SHandShake {
    pub protocol_version: VarInt,
    pub server_address: String, // 255
    pub server_port: u16,
    pub next_state: ConnectionState,
}

impl ServerPacket for SHandShake {
    fn read(bytebuf: &mut ByteBuffer) -> Result<Self, DeserializerError> {
        Ok(Self {
            protocol_version: bytebuf.get_var_int(),
            server_address: bytebuf.get_string_len(255).unwrap(),
            server_port: bytebuf.get_u16(),
            next_state: bytebuf.get_var_int().into(),
        })
    }
}