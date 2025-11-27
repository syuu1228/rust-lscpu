use std::arch::asm;
use std::fmt;

#[derive(Clone,Copy)]
pub enum HypervisorVendor {
    KVM,
    HyperV,
    VMware,
    Xen,
    Parallels,
    VirtualBox,
    Unknown,
}

impl fmt::Display for HypervisorVendor {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            HypervisorVendor::KVM => write!(f, "KVM"),
            HypervisorVendor::HyperV => write!(f, "Hyper-V"),
            HypervisorVendor::VMware => write!(f, "VMware"),
            HypervisorVendor::Xen => write!(f, "Xen"),
            HypervisorVendor::Parallels => write!(f, "Parallels"),
            HypervisorVendor::VirtualBox => write!(f, "VirtualBox"),
            HypervisorVendor::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct Cpuid {
}

impl Cpuid {
    pub fn hypervisor_bit() -> bool {
        let eax: u32 = 0x01;
        let ecx: u32;
        unsafe {
            asm!{
                "cpuid",
                in("eax") eax,
                out("ecx") ecx
            }
        }
        if ecx & (1 << 31) == (1 << 31) {
            return true;
        } else {
            return false;
        }
    }


    pub fn hypervisor_vendor() -> Option<HypervisorVendor> {
        fn register_to_str(reg: u32) -> String {
            let c0:char = (reg as u8) as char;
            let c1:char = ((reg >> 8) as u8) as char;
            let c2:char = ((reg >> 16) as u8) as char;
            let c3:char = ((reg >> 24) as u8) as char;
            return format!("{c0}{c1}{c2}{c3}");
        }
    
        if !Cpuid::hypervisor_bit() {
            return None;
        }
        let eax:u32 = 0x40000000;
        let ebx:u32;
        let ecx:u32;
        let edx:u32;
        unsafe {
            asm!{
                "cpuid",
                "mov edi, ebx",
                in("eax") eax,
                out("edi") ebx,
                out("ecx") ecx,
                out("edx") edx
            }
        }
        let str_ebx = register_to_str(ebx);
        let str_ecx = register_to_str(ecx);
        let str_edx = register_to_str(edx);
        let vendor_str = format!("{str_ebx}{str_ecx}{str_edx}");
        let vendor = match *&(vendor_str.trim()) {
            "KVMKVMKVM" => HypervisorVendor::KVM,
            "Microsoft Hv" => HypervisorVendor::HyperV,
            "VMwareVMware" => HypervisorVendor::VMware,
            "XenVMMXenVMM" => HypervisorVendor::Xen,
            "prl hyperv" => HypervisorVendor::Parallels,
            "VboxVboxVbox" => HypervisorVendor::VirtualBox,
            _ => HypervisorVendor::Unknown,
        };
        return Some(vendor);
    }
}