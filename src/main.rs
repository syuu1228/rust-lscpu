mod cpuinfo;
mod cpuid;
mod hypervisor;
mod cpubyteorder;
mod architecture;

fn print_summary() {
    let cpuinfo = cpuinfo::CpuInfo::new();
    let byteorder = cpubyteorder::CpuByteOrder::new();
    let architecture = architecture::Architecture::new();
    println!("Architecture:\t\t\t{}", architecture.arch);
    let mut op_mode_string: &str;
    if architecture.bit32 && architecture.bit64 {
        op_mode_string = "32bit, 64bit";
    } else if architecture.bit32 {
        op_mode_string = "32bit";
    } else {
        op_mode_string = "64bit";
    }
    println!("CPU op-mode(s):\t\t\t{}", op_mode_string);
    println!("Address sizes:\t\t\t{}", cpuinfo.address_sizes);
    println!("Byte Order:\t\t\t{}", byteorder.byteorder);
    println!("CPU(s):\t\t\t\t{}", cpuinfo.ncpus);
    println!("Vendor ID:\t\t\t{}", cpuinfo.vendor_id);
    println!("  Model name:\t\t\t{}", cpuinfo.model_name);
    println!("    CPU family:\t\t\t{}", cpuinfo.cpu_family);
    println!("    Model:\t\t\t{}", cpuinfo.model);
    println!("    Stepping:\t\t\t{}", cpuinfo.stepping);
    println!("    BogoMIPS:\t\t\t{}", cpuinfo.bogomips);
    println!("    Flags:\t\t\t{}", cpuinfo.flags.join(" "));
    println!("Virtualization features:");
    let mut virtualization_string = "";
    if cpuinfo.flags.iter().any(|f| f == "svm") {
        virtualization_string = "AMD-V";
    }
    if cpuinfo.flags.iter().any(|f| f == "vmx") {
        virtualization_string = "VT-x";
    }
    println!("  Virtualization: {}", virtualization_string);
    let hypervisor_vendor = hypervisor::Hypervisor::hypervisor_vendor();
    if let Some(result) = hypervisor_vendor {
        println!("Hypervisor vendor:\t\t{}", result)
    }
}

fn main() {
    print_summary();
}
