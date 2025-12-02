use std::fmt;
use std::fs;
use crate::cpuid;

#[derive(Clone,Copy,PartialEq)]
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

pub enum HypervisorType {
    Para,
    Full,
    Container,
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

impl fmt::Display for HypervisorType {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            HypervisorType::Para => write!(f, "para"),
            HypervisorType::Full => write!(f, "full"),
            HypervisorType::Container => write!(f, "container"),
        }
    }
}

pub struct Hypervisor {
    pub hypervisor_vendor: Option<HypervisorVendor>,
    pub hypervisor_type: Option<HypervisorType>,
}

impl Hypervisor {
    const XENFEAT_supervisor_mode_kernel:u32 = 3;
    const XENFEAT_mmu_pt_update_preserve_ad:u32 = 5;
    const XENFEAT_hvm_callback_vector:u32 = 8;

    const XEN_FEATURES_PV_MASK:u32 = (1 << Self::XENFEAT_mmu_pt_update_preserve_ad);
    const XEN_FEATURES_PVH_MASK:u32 = ((1 << Self::XENFEAT_supervisor_mode_kernel) | (1 << Self::XENFEAT_hvm_callback_vector));

    pub fn new() -> Self {
        let (hypervisor_vendor, hypervisor_type) = Self::detect_hypervisor();
        return Hypervisor {
            hypervisor_vendor: hypervisor_vendor,
            hypervisor_type: hypervisor_type,
        }
    }

    fn detect_hypervisor() -> (Option<HypervisorVendor>, Option<HypervisorType>) {
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
            let mut hypervisor_type = HypervisorType::Full;

            if vendor == HypervisorVendor::Xen {
                let features_str = fs::read_to_string("/sys/hypervisor/properties/features").unwrap();
                let features = u32::from_str_radix(&features_str, 16).unwrap();
                if (features & Self::XEN_FEATURES_PV_MASK) != 0 {
                    hypervisor_type = HypervisorType::Para;
                } else if (features & Self::XEN_FEATURES_PVH_MASK) == Self::XEN_FEATURES_PVH_MASK {
                    hypervisor_type = HypervisorType::Para;
                }
            }
            return (Some(vendor), Some(hypervisor_type));
        } else {
            return (None, None);
        }
    }
}