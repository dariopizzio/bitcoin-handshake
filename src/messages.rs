use std::io::Write;

use crate::types::{BigEndian, LittleEndian, NodeInformation, UInt};

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
    timestamp: UInt<LittleEndian, 8>,
    recv_node_information: NodeInformation,
    trans_node_information: NodeInformation,
    nonce: UInt<LittleEndian, 8>,
    user_agent_size: [u8; 1],
    user_agent: String,
    start_height: UInt<LittleEndian, 4>,
}

impl VersionMessagePayload {
    pub fn new(
        protocol_version: UInt<LittleEndian, 4>,
        services: [u8; 8],
        timestamp: UInt<LittleEndian, 8>,
        recv_node_information: NodeInformation,
        trans_node_information: NodeInformation,
        nonce: UInt<LittleEndian, 8>,
        user_agent_size: [u8; 1],
        user_agent: String,
        start_height: UInt<LittleEndian, 4>,
    ) -> Self {
        Self {
            protocol_version,
            services,
            timestamp,
            recv_node_information,
            trans_node_information,
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
        message.write(&self.timestamp.to_bytes()).ok();
        message
            .write(&self.recv_node_information.services.to_bytes())
            .ok();
        message
            .write(&self.recv_node_information.ip_address.to_bytes())
            .ok();
        message
            .write(&self.recv_node_information.port.to_bytes())
            .ok();
        message
            .write(&self.trans_node_information.services.to_bytes())
            .ok();
        message
            .write(&self.trans_node_information.ip_address.to_bytes())
            .ok();
        message
            .write(&self.trans_node_information.port.to_bytes())
            .ok();
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

        let recv_node_information = NodeInformation::from_bytes(
            UInt::<LittleEndian, 8>::from_le_bytes(message[20..28].try_into().expect("Don't fail")),
            UInt::<BigEndian, 16>::from_be_bytes(message[28..44].try_into().expect("Don't fail")),
            UInt::<BigEndian, 2>::from_be_bytes(message[44..46].try_into().expect("Don't fail")),
        );
        let trans_node_information = NodeInformation::from_bytes(
            UInt::<LittleEndian, 8>::from_le_bytes(message[46..54].try_into().expect("Don't fail")),
            UInt::<BigEndian, 16>::from_be_bytes(message[54..70].try_into().expect("Don't fail")),
            UInt::<BigEndian, 2>::from_be_bytes(message[70..72].try_into().expect("Don't fail")),
        );
        Self {
            // TODO expect
            protocol_version: UInt::<LittleEndian, 4>::from_le_bytes(
                message[0..4].try_into().expect("Don't fail"),
            ),
            services: message[4..12].try_into().expect("Don't fail"),
            timestamp: UInt::<LittleEndian, 8>::from_le_bytes(
                message[12..20].try_into().expect("Don't fail"),
            ),
            recv_node_information,
            trans_node_information,
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
