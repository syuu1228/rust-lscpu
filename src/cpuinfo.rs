use std::fs;
use std::collections::HashMap;

pub struct CpuInfo {
    pub processor: String,
    pub vendor_id: String,
    pub cpu_family: String,
    pub model: String,
    pub model_name: String,
    pub stepping: String,
    pub microcode: String,
    pub mhz: String,
    pub cache_size: String,
    pub physical_id: String,
    pub siblings: String,
    pub core_id: String,
    pub cpu_cores: String,
    pub apicid: String,
    pub initial_apicid: String,
    pub fpu: String,
    pub fpu_exception: String,
    pub cpuid_level: String,
    pub wp: String,
    pub flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: String,
    pub tlb_size: String,
    pub clflush_size: String,
    pub cache_alignment: String,
    pub address_sizes: String,
    pub power_management: Vec<String>,
    pub ncpus: usize,
}

impl CpuInfo {
    pub fn new() -> Self {
        let cpuinfo = Self::parse_cpuinfo();
        return Self {
            processor: Self::get(&cpuinfo, "processor"),
            vendor_id: Self::get(&cpuinfo, "vendor_id"),
            cpu_family: Self::get(&cpuinfo, "cpu family"),
            model: Self::get(&cpuinfo, "model"),
            model_name: Self::get(&cpuinfo, "model name"),
            stepping: Self::get(&cpuinfo, "stepping"),
            microcode: Self::get(&cpuinfo, "microcode"),
            mhz: Self::get(&cpuinfo, "cpu MHz"),
            cache_size: Self::parse_cache_size(Self::get(&cpuinfo, "cache size")),
            physical_id: Self::get(&cpuinfo, "physical id"),
            siblings: Self::get(&cpuinfo, "siblings"),
            core_id: Self::get(&cpuinfo, "core id"),
            cpu_cores: Self::get(&cpuinfo, "cpu cores"),
            apicid: Self::get(&cpuinfo, "apicid"),
            initial_apicid: Self::get(&cpuinfo, "initial apicid"),
            fpu: Self::get(&cpuinfo, "fpu"),
            fpu_exception: Self::get(&cpuinfo, "fpu_exception"),
            cpuid_level: Self::get(&cpuinfo, "cpuid level"),
            wp: Self::get(&cpuinfo, "wp"),
            flags: Self::parse_list(Self::get(&cpuinfo, "flags")),
            bugs: Self::parse_list(Self::get(&cpuinfo, "bugs")),
            bogomips: Self::get(&cpuinfo, "bogomips"),
            tlb_size: Self::parse_tlb_size(Self::get(&cpuinfo, "TLB size")),
            clflush_size: Self::get(&cpuinfo, "clflush size"),
            cache_alignment: Self::get(&cpuinfo, "cache_alignment"),
            address_sizes: Self::get(&cpuinfo, "address sizes"),
            power_management: Self::parse_list(Self::get(&cpuinfo, "power management")),
            ncpus: cpuinfo.len(),
        }
    }

    fn get(cpuinfo: &Vec<HashMap<String, String>>, key: &str) -> String {
        cpuinfo[0].get(key).map_or("", String::as_str).to_string()
    }

    fn parse_cache_size(value: String) -> String {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        if tokens.len() > 0 {
            tokens[0].to_string()
        } else {
            "".to_string()
        }
    }

    fn parse_tlb_size(value: String) -> String {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        if tokens.len() > 0 {
            tokens[0].to_string()
        } else {
            "".to_string()
        }
    }

    fn parse_list(value: String) -> Vec<String> {
        value.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
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
