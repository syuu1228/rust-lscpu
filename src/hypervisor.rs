use std::fmt;
use crate::cpuid;

#[derive(Clone,Copy)]
pub enum HypervisorVendor {
    QEMU,
    KVM,
    HyperV,
    VMware,
    Xen,
    Parallels,
    VirtualBox,
    Bhyve,
    QNX,
    Unknown,
}

impl fmt::Display for HypervisorVendor {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            HypervisorVendor::QEMU => write!(f, "QEMU"),
            HypervisorVendor::KVM => write!(f, "KVM"),
            HypervisorVendor::HyperV => write!(f, "Microsoft"),
            HypervisorVendor::VMware => write!(f, "VMware"),
            HypervisorVendor::Xen => write!(f, "Xen"),
            HypervisorVendor::Parallels => write!(f, "Parallels"),
            HypervisorVendor::VirtualBox => write!(f, "VirtualBox"),
            HypervisorVendor::Bhyve => write!(f, "bhyve"),
            HypervisorVendor::QNX => write!(f, "QNX"),
            HypervisorVendor::Unknown => write!(f, "Unknown"),
        }
    }
}


pub struct Hypervisor {
}

impl Hypervisor {
    pub fn hypervisor_vendor() -> Option<HypervisorVendor> {
        if cpuid::Cpuid::hypervisor_bit() {
            let vendor_str = cpuid::Cpuid::hypervisor_vendor();
            let vendor = match vendor_str.as_str() {
                "TCGTCGTCGTCG" => HypervisorVendor::QEMU,
                " KVMKVMKVM  " => HypervisorVendor::KVM,
                "VMwareVMware" => HypervisorVendor::VMware,
                "VboxVboxVbox" => HypervisorVendor::VirtualBox,
                "XenVMMXenVMM" => HypervisorVendor::Xen,
                "Microsoft Hv" => HypervisorVendor::HyperV,
                " prl hyperv " => HypervisorVendor::Parallels,
                " lrpepyh vr " => HypervisorVendor::Parallels,
                "bhyve bhyve " => HypervisorVendor::Bhyve,
                " QNXQVMBSQG " => HypervisorVendor::QNX,
                _ => HypervisorVendor::Unknown,
            };
            return Some(vendor);
        } else {
            return None;
        }
    }
}