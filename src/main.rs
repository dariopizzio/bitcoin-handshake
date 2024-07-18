use core::time;
use std::io::{BufRead, BufReader};
use std::thread;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use chrono::Utc;
use messages::{get_checksum, MessageHeader, VersionMessagePayload};
use types::{LittleEndian, MagicBytes, NodeInformation, ToBytes, UInt};

mod messages;
mod types;

const HEADER_COMMAND_VERSION: &str = "version";
const PROTOCOL_VERSION: u32 = 70015;

const REMOTE_IP: &str = "52.15.138.15";
const LOCAL_IP: &str = "127.0.0.1";
const MAINNET_PORT: u16 = 8333;

fn main() {
    let mut tcp_stream = TcpStream::connect(format!("{REMOTE_IP}:{MAINNET_PORT}")).unwrap();

    let payload_message = get_payload_message_header();
    println!("payload_message: {payload_message:?}");

    let version_header = get_version_message_header(&payload_message.to_bytes());
    println!("version_header: {version_header:?}");

    tcp_stream
        .write(&version_header.to_bytes())
        .expect("expect");
    tcp_stream
        .write(&payload_message.to_bytes())
        .expect("expect");

    thread::sleep(time::Duration::from_secs(2));
    println!("after sleep");

    // Read version message
    let mut buf_read = [0u8; 24];
    let size = tcp_stream.read(&mut buf_read).expect("expect");
    println!("resp size: {size}");
    println!("RESPONSE: {buf_read:?}");

    let received_message = MessageHeader::from_bytes(buf_read);
    println!("received message: {received_message:?}");

    // Read payload message
    let size = u32::from_le_bytes(received_message.size.to_bytes());
    println!("expected_size: {size}");

    let mut br = BufReader::new(tcp_stream);
    let buf_read = br.fill_buf().unwrap();
    println!("resp size: {size}");
    println!("RESPONSE: {buf_read:?}");

    let received_message = VersionMessagePayload::from_bytes(buf_read.to_vec());
    println!("received message: {received_message:?}");
}

fn get_version_message_header(payload: &Vec<u8>) -> MessageHeader {
    let mut command: [u8; 12] = [0; 12];
    command[0..7].copy_from_slice(HEADER_COMMAND_VERSION.as_bytes());

    let size = payload.len() as u32;
    let size = UInt::<LittleEndian, 4>::new(size);

    let checksum = get_checksum(payload);

    MessageHeader::new(MagicBytes::MAINNET.to_bytes(), command, size, checksum)
}

fn get_payload_message_header() -> VersionMessagePayload {
    let protocol_version = UInt::<LittleEndian, 4>::new(PROTOCOL_VERSION);
    let services: [u8; 8] = [0; 8];
    let epoch_time = Utc::now().timestamp();
    let time = UInt::<LittleEndian, 8>::new(epoch_time as u64);
    let recv_node_information = NodeInformation::new(0, REMOTE_IP, MAINNET_PORT);
    let trans_node_information = NodeInformation::new(0, LOCAL_IP, MAINNET_PORT);
    let nonce = UInt::<LittleEndian, 8>::new(0);
    let user_agent: [u8; 1] = [0; 1];
    let last_block = UInt::<LittleEndian, 4>::new(0);

    VersionMessagePayload::new(
        protocol_version,
        services,
        time,
        recv_node_information,
        trans_node_information,
        nonce,
        user_agent,
        "".to_string(),
        last_block,
    )
}

// TODO remove
pub fn get_field<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut field: [u8; N] = [0; N];
    field[0..bytes.len()].copy_from_slice(bytes);
    field
}
