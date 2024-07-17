use std::marker::PhantomData;

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

struct NodeInformation {
    services: [u8; 8],
    ip_address: [u8; 16],
    ip_port: [u8; 2],
}

// ----------

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

impl_uint!(8, u64, LittleEndian);
impl_uint!(4, u32, LittleEndian);
impl_uint!(4, u32, BigEndian);
//impl_uint!(8, u64, LittleEndian);
//impl_uint!(2, u16, BigEndian);
//impl_uint!(8, u64, BigEndian);
