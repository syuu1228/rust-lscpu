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
        return CpuByteOrder {
            byteorder: CpuByteOrder::parse_byteorder(),
        }
    }

    fn parse_byteorder() -> Endian {
        let cpu_byteorder_path = "/sys/kernel/cpu_byteorder";
        match fs::read_to_string(cpu_byteorder_path) {
            Ok(content) => {
                if content.trim() == "little" {
                    return Endian::LittleEndian;
                } else {
                    return Endian::BigEndian;
                }
            }
            Err(e) => {
                panic!("Unable to read {}: {}", cpu_byteorder_path, e);
            }
        }
    }
}