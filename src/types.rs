use std::{marker::PhantomData, net::Ipv4Addr, str::FromStr};

enum MagicBytes {
    MAINNET = 0xF9BEB4D9,
}

trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

impl ToBytes<4> for MagicBytes {
    fn to_bytes(&self) -> [u8; 4] {
        match self {
            MagicBytes::MAINNET => hex_to_bytes(MagicBytes::MAINNET),
        }
    }
}

fn hex_to_bytes(num: MagicBytes) -> [u8; 4] {
    let num = num as u32;
    let (b1, b2, b3, b4): (u8, u8, u8, u8) = (
        (num & 0x000000FF) as u8,
        (num >> 8 & 0x000000FF) as u8,
        (num >> 16 & 0x000000FF) as u8,
        (num >> 24 & 0x000000FF) as u8,
    );

    [b1, b2, b3, b4]
}

#[derive(Debug)]
pub struct NodeInformation {
    pub services: UInt<LittleEndian, 8>,
    pub ip_address: UInt<BigEndian, 16>,
    pub port: UInt<BigEndian, 2>,
}

impl NodeInformation {
    pub fn new(services: u64, ip_address: &str, port: u16) -> Self {
        let ip_address = Ipv4Addr::from_str(ip_address)
            .unwrap()
            .to_ipv6_mapped()
            .octets(); // TODO .octets().map(|b| b.to_be());
        Self {
            services: UInt::<LittleEndian, 8>::new(services),
            ip_address: UInt::<BigEndian, 16>::from_be_bytes(ip_address),
            port: UInt::<BigEndian, 2>::new(port),
        }
    }
    pub fn from_bytes(
        services: UInt<LittleEndian, 8>,
        ip_address: UInt<BigEndian, 16>,
        port: UInt<BigEndian, 2>,
    ) -> Self {
        Self {
            services,
            ip_address,
            port,
        }
    }
}

pub trait Endianess {}

#[derive(Debug)]
pub struct BigEndian {}
#[derive(Debug)]
pub struct LittleEndian {}

impl Endianess for BigEndian {}
impl Endianess for LittleEndian {}

#[derive(Debug)]
pub struct UInt<E: Endianess, const N: usize>([u8; N], PhantomData<E>);

macro_rules! impl_uint {
    ($size: literal, $int_arg: ty, BigEndian) => {
        impl UInt<BigEndian, $size> {
            pub fn new(value: $int_arg) -> Self {
                Self(value.to_be_bytes(), PhantomData)
            }
            pub fn from_be_bytes(value: [u8; $size]) -> Self {
                Self(value, PhantomData)
            }
            pub fn to_bytes(&self) -> [u8; $size] {
                self.0
            }
        }
    };
    ($size: literal, $int_arg: ty, LittleEndian) => {
        impl UInt<LittleEndian, $size> {
            pub fn new(value: $int_arg) -> Self {
                Self(value.to_le_bytes(), PhantomData)
            }
            pub fn from_le_bytes(value: [u8; $size]) -> Self {
                Self(value, PhantomData)
            }
            pub fn to_bytes(&self) -> [u8; $size] {
                self.0
            }
        }
    };
}

impl_uint!(2, u16, BigEndian);
impl_uint!(4, u32, LittleEndian);
impl_uint!(4, u32, BigEndian);
impl_uint!(8, u64, LittleEndian);
impl_uint!(16, u128, BigEndian);
