use std::io::{BufRead, BufReader, BufWriter};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use chrono::Utc;
use messages::{get_checksum, MessageHeader, VersionMessagePayload};
use types::{HeaderCommand, LittleEndian, MagicBytes, NodeInformation, UInt};

mod messages;
mod types;

const PROTOCOL_VERSION: u32 = 70015;

const REMOTE_IP: &str = "52.15.138.15";
const LOCAL_IP: &str = "127.0.0.1";
const MAINNET_PORT: u16 = 8333;

const HEADER_MESSAGE_LENGHT: usize = 24;

fn main() {
    let mut tcp_stream = TcpStream::connect(format!("{REMOTE_IP}:{MAINNET_PORT}")).unwrap();

    let payload_message = get_payload_message_header();
    let version_header =
        get_version_message_header(HeaderCommand::VERSION, &payload_message.to_bytes());

    println!("## SEND VERSION HEADER & PAYLOAD");
    write_bytes(&tcp_stream, &version_header.to_bytes());
    write_bytes(&tcp_stream, &payload_message.to_bytes());

    println!("## READ VERSION HEADER");
    let mut buffer = [0u8; HEADER_MESSAGE_LENGHT];
    read_bytes(&mut tcp_stream, &mut buffer);

    let received_message = MessageHeader::from_bytes(buffer);
    println!("parsed received message: {received_message:?}");

    println!("## READ VERSION PAYLOAD");
    let size = u32::from_le_bytes(received_message.size.to_bytes());

    let mut br = BufReader::new(&tcp_stream);
    let buffer = br.fill_buf().unwrap();
    println!("Size: {size} | Response: {buffer:?}");

    let received_message = VersionMessagePayload::from_bytes(buffer.to_vec());
    println!("parsed received message: {received_message:?}");

    // Consume the payload from the buffer
    br.consume(size as usize);

    println!("## READ VERACK HEADER");
    let mut buffer = [0u8; HEADER_MESSAGE_LENGHT];
    read_bytes(&mut tcp_stream, &mut buffer);

    let received_message = MessageHeader::from_bytes(buffer);
    println!("parsed received message: {received_message:?}");

    println!("## SEND VERACK HEADER");
    let version_header = get_version_message_header(HeaderCommand::VERACK, &vec![]);

    write_bytes(&tcp_stream, &version_header.to_bytes());

    println!("HANDSHAKE COMPLETED!");

    println!("## READ NEW HEADER MESSAGE");
    let mut buffer = [0u8; HEADER_MESSAGE_LENGHT];
    read_bytes(&mut tcp_stream, &mut buffer);

    let received_message = MessageHeader::from_bytes(buffer);
    println!("parsed received message: {received_message:?}");
}

fn read_bytes(tcp_stream: &mut TcpStream, buffer: &mut [u8; 24]) -> usize {
    // TODO use BufReader
    let size = tcp_stream.read(buffer).expect("expect");
    println!("Size: {size} | Response: {buffer:?}");
    size
}

fn write_bytes(tcp_stream: &TcpStream, bytes: &Vec<u8>) {
    let mut buf_writer = BufWriter::new(tcp_stream);
    buf_writer.write(bytes).expect("write");
}

fn get_version_message_header(command: HeaderCommand, payload: &Vec<u8>) -> MessageHeader {
    let size = payload.len() as u32;
    let size = UInt::<LittleEndian, 4>::new(size);
    let checksum = get_checksum(payload);

    MessageHeader::new(MagicBytes::MAINNET, command, size, checksum)
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
