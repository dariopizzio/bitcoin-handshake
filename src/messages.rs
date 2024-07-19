use std::io::Write;

use sha2::{Digest, Sha256};

use crate::types::{
    BigEndian, FromBytes, HeaderCommand, LittleEndian, MagicBytes, NodeInformation, ToBytes, UInt,
};

#[derive(Debug)]
pub struct MessageHeader {
    magic_bytes: MagicBytes,
    command: HeaderCommand,
    pub size: UInt<LittleEndian, 4>,
    checksum: UInt<BigEndian, 4>,
}

impl MessageHeader {
    pub fn new(
        magic_bytes: MagicBytes,
        command: HeaderCommand,
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
        message.write_all(&self.magic_bytes.to_bytes()).ok();
        message.write_all(&self.command.to_bytes()).ok();
        message.write_all(&self.size.to_bytes()).ok();
        message.write_all(&self.checksum.to_bytes()).ok();

        message
    }

    pub fn from_bytes(message: [u8; 24]) -> Self {
        Self {
            // TODO expect
            magic_bytes: MagicBytes::from_bytes(message[0..4].try_into().expect("Don't fail")),
            command: HeaderCommand::from_bytes(message[4..16].try_into().expect("Don't fail")),
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
    #[allow(clippy::too_many_arguments)]
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
        message.write_all(&self.protocol_version.to_bytes()).ok();
        message.write_all(&self.services).ok();
        message.write_all(&self.timestamp.to_bytes()).ok();
        message
            .write_all(&self.recv_node_information.services.to_bytes())
            .ok();
        message
            .write_all(&self.recv_node_information.ip_address.to_bytes())
            .ok();
        message
            .write_all(&self.recv_node_information.port.to_bytes())
            .ok();
        message
            .write_all(&self.trans_node_information.services.to_bytes())
            .ok();
        message
            .write_all(&self.trans_node_information.ip_address.to_bytes())
            .ok();
        message
            .write_all(&self.trans_node_information.port.to_bytes())
            .ok();
        message.write_all(&self.nonce.to_bytes()).ok();
        message.write_all(&self.user_agent_size).ok();

        if self.user_agent_size[0] > 0 {
            message.write_all(self.user_agent.as_bytes()).ok();
        }
        message.write_all(&self.start_height.to_bytes()).ok();

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

/// Given a payload returns its checksum
///
/// To get the checksum, you need to double hash the payload
/// and take the first 4 bytes
pub fn get_checksum(payload: &Vec<u8>) -> UInt<BigEndian, 4> {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let checksum: [u8; 4] = hasher.finalize()[..4].try_into().unwrap(); // first 4 bytes
    UInt::<BigEndian, 4>::from_be_bytes(checksum) // TODO check BigEndian vs LittleEndian
}
