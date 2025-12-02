use std::env;

pub struct Architecture {
    pub arch: String,
    pub bit32: bool,
    pub bit64: bool,
}

impl Architecture {
    pub fn new() -> Self {
        let (arch, bit32, bit64) = Self::parse_architecture();
        return Self {
            arch: arch,
            bit32: bit32,
            bit64: bit64,
        }
    }

    fn parse_architecture() -> (String, bool, bool) {
        let arch: String = env::consts::ARCH.to_string();
        let bit32: bool;
        let bit64: bool;
        match env::consts::ARCH {
            "x86" => {
                bit32 = true;
                bit64 = false;
            }
            "x86_64" => {
                bit32 = true;
                bit64 = true;
            }
            &_ => todo!()
        }
        return (arch, bit32, bit64);
    }
}