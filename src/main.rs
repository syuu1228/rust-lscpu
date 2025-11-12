use std::fs;
use std::collections::HashMap;
use std::env;
extern crate num_cpus;

#[derive(Debug,PartialEq)]
enum CpuByteOrder {
    LittleEndian,
    BigEndian,
}

struct CpuInfo {
    arch: String,
    bit32: bool,
    bit64: bool,
    byteorder: CpuByteOrder,
    processor: String,
    vendor_id: String,
    cpu_family: i32,
    model: u32,
    model_name: String,
    stepping: i32,
    microcode: u32,
    mhz: String,
    cache_size: u32,
    physical_id: i32,
    siblings: i32,
    core_id: i32,
    cpu_cores: i32,
    apicid: i32,
    initial_apicid: i32,
    fpu: bool,
    fpu_exception: bool,
    cpuid_level: i32,
    wp: bool,
    flags: Vec<String>,
    bugs: Vec<String>,
    bogomips: String,
    tlb_size: i32,
    clflush_size: u32,
    cache_alignment: i32,
    address_sizes: String,
    power_management: Vec<String>,
}

impl CpuInfo {
    fn new() -> Self {
        let (arch, bit32, bit64) = CpuInfo::parse_architecture();
        let byteorder = CpuInfo::parse_byteorder();
        let cpuinfo = CpuInfo::parse_cpuinfo();
        return CpuInfo {
            arch: arch,
            bit32: bit32,
            bit64: bit64,
            byteorder: byteorder,
            processor: cpuinfo[0].get("processor").unwrap().to_string(),
            vendor_id: cpuinfo[0].get("vendor_id").unwrap().to_string(),
            cpu_family: cpuinfo[0].get("cpu family").unwrap().parse().unwrap(),
            model: cpuinfo[0].get("model").unwrap().parse().unwrap(),
            model_name: cpuinfo[0].get("model name").unwrap().to_string(),
            stepping: cpuinfo[0].get("stepping").unwrap().parse().unwrap(),
            microcode: CpuInfo::cpuinfo_parse_hex(cpuinfo[0].get("microcode").unwrap()),
            mhz: cpuinfo[0].get("cpu MHz").unwrap().to_string(),
            cache_size: CpuInfo::cpuinfo_parse_cache_size(cpuinfo[0].get("cache size").unwrap()),
            physical_id: cpuinfo[0].get("physical id").unwrap().parse().unwrap(),
            siblings: cpuinfo[0].get("siblings").unwrap().parse().unwrap(),
            core_id: cpuinfo[0].get("core id").unwrap().parse().unwrap(),
            cpu_cores: cpuinfo[0].get("cpu cores").unwrap().parse().unwrap(),
            apicid: cpuinfo[0].get("apicid").unwrap().parse().unwrap(),
            initial_apicid: cpuinfo[0].get("initial apicid").unwrap().parse().unwrap(),
            fpu: CpuInfo::cpuinfo_parse_bool(cpuinfo[0].get("fpu").unwrap()),
            fpu_exception: CpuInfo::cpuinfo_parse_bool(cpuinfo[0].get("fpu_exception").unwrap()),
            cpuid_level: cpuinfo[0].get("cpuid level").unwrap().parse().unwrap(),
            wp: CpuInfo::cpuinfo_parse_bool(cpuinfo[0].get("wp").unwrap()),
            flags: CpuInfo::cpuinfo_parse_list(cpuinfo[0].get("flags").unwrap()),
            bugs: CpuInfo::cpuinfo_parse_list(cpuinfo[0].get("bugs").unwrap()),
            bogomips: cpuinfo[0].get("bogomips").unwrap().to_string(),
            tlb_size: CpuInfo::cpuinfo_parse_tlb_size(cpuinfo[0].get("TLB size").unwrap()),
            clflush_size: cpuinfo[0].get("clflush size").unwrap().parse().unwrap(),
            cache_alignment: cpuinfo[0].get("cache_alignment").unwrap().parse().unwrap(),
            address_sizes: cpuinfo[0].get("address sizes").unwrap().to_string(),
            power_management: CpuInfo::cpuinfo_parse_list(cpuinfo[0].get("power management").unwrap()),
        }
    }

    fn cpuinfo_parse_cache_size(value: &str) -> u32 {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        return tokens[0].parse().unwrap();
    }

    fn cpuinfo_parse_tlb_size(value: &str) -> i32 {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        return tokens[0].parse().unwrap();
    }

    fn cpuinfo_parse_list(value: &str) -> Vec<String> {
        return value.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    }

    fn cpuinfo_parse_bool(value: &str) -> bool {
        if value == "yes" {
            return true;
        } else {
            return false;
        }
    }

    fn cpuinfo_parse_hex(value: &str) -> u32 {
        return u32::from_str_radix(value.strip_prefix("0x").unwrap(), 16).unwrap();
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

    fn parse_byteorder() -> CpuByteOrder {
        let cpu_byteorder_path = "/sys/kernel/cpu_byteorder";
        match fs::read_to_string(cpu_byteorder_path) {
            Ok(content) => {
                if content.trim() == "little" {
                    return CpuByteOrder::LittleEndian;
                } else {
                    return CpuByteOrder::BigEndian;
                }
            }
            Err(e) => {
                panic!("Unable to read {}: {}", cpu_byteorder_path, e);
            }
        }
    }

    fn parse_cpuinfo() -> Vec<HashMap<String, String>> {
        let cpuinfo_path = "/proc/cpuinfo";
        let mut cpuinfo = Vec::new();
        match fs::read_to_string(cpuinfo_path) {
            Ok(content) => {
                let mut cpu_params = HashMap::new();
                for line in content.lines() {
                    if line.len() == 0 {
                        cpuinfo.push(cpu_params);
                        cpu_params = HashMap::new();
                    } else {
                        let tokens: Vec<&str> = line.split(':').collect();
                        let key = tokens[0].trim();
                        let value = tokens[1].trim();
                        cpu_params.insert(key.to_string(), value.to_string());
                    }
                }
            }
            Err(e) => {
                panic!("Unable to read {}: {}", cpuinfo_path, e);
            }
        }
        return cpuinfo;
    }

    fn print_summary(&self) {
        println!("Architecture:\t\t\t{}", self.arch);
        let mut op_mode_string: &str;
        if self.bit32 && self.bit64 {
            op_mode_string = "32bit, 64bit";
        } else if self.bit32 {
            op_mode_string = "32bit";
        } else {
            op_mode_string = "64bit";
        }
        {
        println!("CPU op-mode(s):\t\t\t{}", op_mode_string);
        println!("Address sizes:\t\t\t{}", self.address_sizes);
        if self.byteorder == CpuByteOrder::LittleEndian {
            println!("Byte Order:\t\t\tLittle Endian");
        } else {
            println!("Byte Order:\t\t\tBig Endian");
        }
        println!("CPU(s):\t\t\t\t{}", num_cpus::get());
        println!("Vendor ID:\t\t\t{}", self.vendor_id);
        println!("  Model name:\t\t\t{}", self.model_name);
        println!("    CPU family:\t\t\t{}", self.cpu_family);
        println!("    Model:\t\t\t{}", self.model);
        println!("    Stepping:\t\t\t{}", self.stepping);
        println!("    BogoMIPS:\t\t\t{}", self.bogomips);
        println!("    Flags:\t\t\t{}", self.flags.join(" "));
        println!("Virtualization features:");
        if self.flags.iter().any(|f| f == "svm") {
            println!("  Virtualization: AMD-V");
        }
        if self.flags.iter().any(|f| f == "vmx") {
            println!("  Virtualization: VT-x");
        }
    }
    
}
}

fn main() {
    let cpuinfo = CpuInfo::new();
    cpuinfo.print_summary();
}
