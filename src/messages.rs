use std::io::Write;

use crate::types::{BigEndian, LittleEndian, UInt};

#[derive(Debug)]
pub struct MessageHeader {
    magic_bytes: [u8; 4],
    command: [u8; 12],
    pub size: UInt<LittleEndian, 4>,
    checksum: UInt<BigEndian, 4>,
}

impl MessageHeader {
    pub fn new(
        magic_bytes: [u8; 4],
        command: [u8; 12],
        size: UInt<LittleEndian, 4>,
        checksum: UInt<BigEndian, 4>,
    ) -> Self {
        Self {
            magic_bytes,
            command,
            size,
            checksum,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut message: Vec<u8> = vec![];
        // TODO ok()
        message.write(&self.magic_bytes).ok();
        message.write(&self.command).ok();
        message.write(&self.size.to_bytes()).ok();
        message.write(&self.checksum.to_bytes()).ok();

        message
    }

    pub fn from_bytes(message: [u8; 24]) -> Self {
        Self {
            // TODO expect
            magic_bytes: message[0..4].try_into().expect("Don't fail"),
            command: message[4..16].try_into().expect("Don't fail"),
            size: UInt::<LittleEndian, 4>::from_le_bytes(
                message[16..20].try_into().expect("Don't fail"),
            ),
            checksum: UInt::<BigEndian, 4>::from_be_bytes(
                message[20..24].try_into().expect("Don't fail"),
            ),
        }
    }
}

#[derive(Debug)]
pub struct VersionMessagePayload {
    protocol_version: UInt<LittleEndian, 4>,
    services: [u8; 8],
    timestamp: [u8; 8],
    addr_recv_services: [u8; 8],
    addr_recv_ip_address: [u8; 16],
    addr_recv_ip_port: [u8; 2],
    addr_trans_services: [u8; 8],
    addr_trans_ip_address: [u8; 16],
    addr_trans_ip_port: [u8; 2],
    nonce: UInt<LittleEndian, 8>,
    user_agent_size: [u8; 1],
    user_agent: String,
    start_height: UInt<LittleEndian, 4>,
}

impl VersionMessagePayload {
    pub fn new(
        protocol_version: UInt<LittleEndian, 4>,
        services: [u8; 8],
        timestamp: [u8; 8],
        addr_recv_services: [u8; 8],
        addr_recv_ip_address: [u8; 16],
        addr_recv_ip_port: [u8; 2],
        addr_trans_services: [u8; 8],
        addr_trans_ip_address: [u8; 16],
        addr_trans_ip_port: [u8; 2],
        nonce: UInt<LittleEndian, 8>,
        user_agent_size: [u8; 1],
        user_agent: String,
        start_height: UInt<LittleEndian, 4>,
    ) -> Self {
        Self {
            protocol_version,
            services,
            timestamp,
            addr_recv_services,
            addr_recv_ip_address,
            addr_recv_ip_port,
            addr_trans_services,
            addr_trans_ip_address,
            addr_trans_ip_port,
            nonce,
            user_agent_size,
            user_agent,
            start_height,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut message: Vec<u8> = vec![];
        // TODO ok()

        message.write(&self.protocol_version.to_bytes()).ok();
        message.write(&self.services).ok();
        message.write(&self.timestamp).ok();
        message.write(&self.addr_recv_services).ok();
        message.write(&self.addr_recv_ip_address).ok();
        message.write(&self.addr_recv_ip_port).ok();
        message.write(&self.addr_trans_services).ok();
        message.write(&self.addr_trans_ip_address).ok();
        message.write(&self.addr_trans_ip_port).ok();
        message.write(&self.nonce.to_bytes()).ok();
        message.write(&self.user_agent_size).ok();

        if (self.user_agent_size[0] as u8) > 0 {
            message.write(self.user_agent.as_bytes()).ok();
        }
        message.write(&self.start_height.to_bytes()).ok();

        message
    }

    pub fn from_bytes(message: Vec<u8>) -> Self {
        let user_agent_size: [u8; 1] = message[80..81].try_into().expect("Don't fail");
        let size = i8::from_le_bytes(user_agent_size);
        let end_user_agent = 81 + size as usize;
        Self {
            // TODO expect
            protocol_version: UInt::<LittleEndian, 4>::from_le_bytes(
                message[0..4].try_into().expect("Don't fail"),
            ),
            services: message[4..12].try_into().expect("Don't fail"),
            timestamp: message[12..20].try_into().expect("Don't fail"),
            addr_recv_services: message[20..28].try_into().expect("Don't fail"),
            addr_recv_ip_address: message[28..44].try_into().expect("Don't fail"),
            addr_recv_ip_port: message[44..46].try_into().expect("Don't fail"),
            addr_trans_services: message[46..54].try_into().expect("Don't fail"),
            addr_trans_ip_address: message[54..70].try_into().expect("Don't fail"),
            addr_trans_ip_port: message[70..72].try_into().expect("Don't fail"),
            nonce: UInt::<LittleEndian, 8>::from_le_bytes(
                message[72..80].try_into().expect("Don't fail"),
            ),
            user_agent_size,
            user_agent: String::from_utf8(message[81..end_user_agent].to_vec())
                .expect("Don't fail"),
            start_height: UInt::<LittleEndian, 4>::from_le_bytes(
                message[end_user_agent..end_user_agent + 4]
                    .try_into()
                    .expect("Don't fail"),
            ),
            //add relay field as optional
        }
    }
}
