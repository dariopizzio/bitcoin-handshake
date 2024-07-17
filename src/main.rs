use core::time;
use std::io::{BufRead, BufReader};
use std::thread;
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpStream},
    str::FromStr,
};

use chrono::Utc;
use messages::{MessageHeader, VersionMessagePayload};
use sha2::{Digest, Sha256};
use types::{BigEndian, LittleEndian, UInt};

mod messages;
mod types;

const HEADER_MAGIC_BYTES: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
const HEADER_COMMAND_VERSION: &str = "version";
const PROTOCOL_VERSION: u32 = 70015;

fn main() {
    let remote_ip = "52.15.138.15";
    let mut tcp_stream = TcpStream::connect(format!("{}:8333", remote_ip)).unwrap();

    let payload_message = get_payload_message_header(remote_ip);
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

    // Double hashing
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&hash1);
    let checksum: [u8; 4] = hasher.finalize()[..4].try_into().unwrap(); // first 4 bytes
    let checksum = UInt::<BigEndian, 4>::from_be_bytes(checksum); // TODO check BigEndian vs LittleEndian

    MessageHeader::new(HEADER_MAGIC_BYTES, command, size, checksum)
}

fn get_payload_message_header(remote_ip: &str) -> VersionMessagePayload {
    let protocol_version = UInt::<LittleEndian, 4>::new(PROTOCOL_VERSION);

    let services: [u8; 8] = [0; 8];

    let epoch_time = Utc::now().timestamp();
    let time = epoch_time.to_le_bytes();

    let remote_service: u32 = 0;
    let remote_service = remote_service.to_le_bytes();
    let remote_service = get_field::<8>(&remote_service);

    let remote_ip = Ipv4Addr::from_str(remote_ip).expect("Error parsing IP");
    let remote_ip = remote_ip.to_ipv6_mapped();
    let remote_ip = remote_ip.octets().map(|b| b.to_be());

    let remote_port: u16 = 8333;
    let remote_port = remote_port.to_be_bytes();

    let local_service: [u8; 8] = [0; 8];

    let local_ip = Ipv4Addr::from_str("127.0.0.1").expect("Error parsing IP");
    let local_ip = local_ip.to_ipv6_mapped();
    let local_ip = local_ip.octets().map(|b| b.to_be());

    let local_port: u16 = 8333;
    let local_port = local_port.to_be_bytes();

    let nonce = UInt::<LittleEndian, 8>::new(0);

    let user_agent: [u8; 1] = [0; 1];

    let last_block = UInt::<LittleEndian, 4>::new(0);

    VersionMessagePayload::new(
        protocol_version,
        services,
        time,
        remote_service,
        remote_ip,
        remote_port,
        local_service,
        local_ip,
        local_port,
        nonce,
        user_agent,
        "".to_string(),
        last_block,
    )
}

pub fn get_field<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut field: [u8; N] = [0; N];
    field[0..bytes.len()].copy_from_slice(bytes);
    field
}
