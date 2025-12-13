use std::fs;
use std::fmt;

#[derive(Debug,PartialEq)]
pub enum Endian {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for Endian {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            Endian::LittleEndian => write!(f, "Little Endian"),
            Endian::BigEndian => write!(f, "Big Endian"),
        }
    }
}


pub struct CpuByteOrder {
    pub byteorder: Endian,
}

impl CpuByteOrder {
    pub fn new() -> Self {
        Self {
            byteorder: Self::parse_byteorder(),
        }
    }

    fn parse_byteorder() -> Endian {
        let content = fs::read_to_string("/sys/kernel/cpu_byteorder").unwrap();
        match content.trim() {
            "little" => Endian::LittleEndian,
            _ => Endian::BigEndian,
        }
    }
}