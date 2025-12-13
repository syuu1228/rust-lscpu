use std::fs;
use std::collections::HashMap;

pub struct CpuInfo {
    pub processor: String,
    pub vendor_id: String,
    pub cpu_family: i32,
    pub model: u32,
    pub model_name: String,
    pub stepping: i32,
    pub microcode: u32,
    pub mhz: String,
    pub cache_size: u32,
    pub physical_id: i32,
    pub siblings: i32,
    pub core_id: i32,
    pub cpu_cores: i32,
    pub apicid: i32,
    pub initial_apicid: i32,
    pub fpu: bool,
    pub fpu_exception: bool,
    pub cpuid_level: i32,
    pub wp: bool,
    pub flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: String,
    pub tlb_size: i32,
    pub clflush_size: u32,
    pub cache_alignment: i32,
    pub address_sizes: String,
    pub power_management: Vec<String>,
    pub ncpus: usize,
}

impl CpuInfo {
    pub fn new() -> Self {
        let cpuinfo = Self::parse_cpuinfo();
        return Self {
            processor: cpuinfo[0].get("processor").unwrap().to_string(),
            vendor_id: cpuinfo[0].get("vendor_id").unwrap().to_string(),
            cpu_family: cpuinfo[0].get("cpu family").unwrap().parse().unwrap(),
            model: cpuinfo[0].get("model").unwrap().parse().unwrap(),
            model_name: cpuinfo[0].get("model name").unwrap().to_string(),
            stepping: cpuinfo[0].get("stepping").unwrap().parse().unwrap(),
            microcode: Self::cpuinfo_parse_hex(cpuinfo[0].get("microcode").unwrap()),
            mhz: cpuinfo[0].get("cpu MHz").unwrap().to_string(),
            cache_size: Self::cpuinfo_parse_cache_size(cpuinfo[0].get("cache size").unwrap()),
            physical_id: cpuinfo[0].get("physical id").unwrap().parse().unwrap(),
            siblings: cpuinfo[0].get("siblings").unwrap().parse().unwrap(),
            core_id: cpuinfo[0].get("core id").unwrap().parse().unwrap(),
            cpu_cores: cpuinfo[0].get("cpu cores").unwrap().parse().unwrap(),
            apicid: cpuinfo[0].get("apicid").unwrap().parse().unwrap(),
            initial_apicid: cpuinfo[0].get("initial apicid").unwrap().parse().unwrap(),
            fpu: Self::cpuinfo_parse_bool(cpuinfo[0].get("fpu").unwrap()),
            fpu_exception: Self::cpuinfo_parse_bool(cpuinfo[0].get("fpu_exception").unwrap()),
            cpuid_level: cpuinfo[0].get("cpuid level").unwrap().parse().unwrap(),
            wp: Self::cpuinfo_parse_bool(cpuinfo[0].get("wp").unwrap()),
            flags: Self::cpuinfo_parse_list(cpuinfo[0].get("flags").unwrap()),
            bugs: Self::cpuinfo_parse_list(cpuinfo[0].get("bugs").unwrap()),
            bogomips: cpuinfo[0].get("bogomips").unwrap().to_string(),
            tlb_size: Self::cpuinfo_parse_tlb_size(cpuinfo[0].get("TLB size").unwrap()),
            clflush_size: cpuinfo[0].get("clflush size").unwrap().parse().unwrap(),
            cache_alignment: cpuinfo[0].get("cache_alignment").unwrap().parse().unwrap(),
            address_sizes: cpuinfo[0].get("address sizes").unwrap().to_string(),
            power_management: Self::cpuinfo_parse_list(cpuinfo[0].get("power management").unwrap()),
            ncpus: cpuinfo.len(),
        }
    }

    fn cpuinfo_parse_cache_size(value: &str) -> u32 {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        tokens[0].parse().unwrap()
    }

    fn cpuinfo_parse_tlb_size(value: &str) -> i32 {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        tokens[0].parse().unwrap()
    }

    fn cpuinfo_parse_list(value: &str) -> Vec<String> {
        value.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    fn cpuinfo_parse_bool(value: &str) -> bool {
        if value == "yes" {
            true
        } else {
            false
        }
    }

    fn cpuinfo_parse_hex(value: &str) -> u32 {
        u32::from_str_radix(value.strip_prefix("0x").unwrap(), 16).unwrap()
    }

    fn parse_cpuinfo() -> Vec<HashMap<String, String>> {
        let mut cpuinfo = Vec::new();
        let content = fs::read_to_string("/proc/cpuinfo").unwrap();
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
        cpuinfo
    }
}
