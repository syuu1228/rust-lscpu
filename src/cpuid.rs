use std::arch::asm;

pub struct Cpuid {
}

impl Cpuid {
    pub fn hypervisor_bit() -> bool {
        let mut eax: u32 = 0x01;
        let ecx: u32;
        unsafe {
            asm!{
                "cpuid",
                inout("eax") eax,
                out("ecx") ecx
            }
        }
        return (ecx & (1 << 31)) != 0;
    }

    pub fn hypervisor_vendor() -> String {
        let mut eax = 0x40000000;
        let mut vendor_bytes = [0_u8; 4*3];
        unsafe {
            asm!{
                "push rbx",
                "cpuid",
                "mov [rdi], ebx",
                "mov [rdi + 4], ecx",
                "mov [rdi + 8], edx",
                "pop rbx",
                in("rdi") vendor_bytes.as_mut_ptr(),
                inout("eax") eax,
                out("ecx") _,
                out("edx") _,
            }
        }
        return String::from_utf8_lossy(&vendor_bytes).to_string();
    }
}