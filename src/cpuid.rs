use std::arch::asm;

pub struct Cpuid {
}

impl Cpuid {
    const CPUID_FEAT_ECX_HYPERVISOR:u32 = 1 << 31;
    const HYPERVISOR_INFO_LEAF:u32 = 0x40000000;

    fn register_to_str(reg: u32) -> String {
        let c0:char = (reg as u8) as char;
        let c1:char = ((reg >> 8) as u8) as char;
        let c2:char = ((reg >> 16) as u8) as char;
        let c3:char = ((reg >> 24) as u8) as char;
        return format!("{c0}{c1}{c2}{c3}");
    }

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
        if ecx & Cpuid::CPUID_FEAT_ECX_HYPERVISOR == Cpuid::CPUID_FEAT_ECX_HYPERVISOR {
            return true;
        } else {
            return false;
        }
    }

    pub fn hypervisor_vendor() -> String {
        let eax:u32 = Cpuid::HYPERVISOR_INFO_LEAF;
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
        let str_ebx = Cpuid::register_to_str(ebx);
        let str_ecx = Cpuid::register_to_str(ecx);
        let str_edx = Cpuid::register_to_str(edx);
        return format!("{str_ebx}{str_ecx}{str_edx}");
    }
}