use std::fmt;
use std::fs;
use std::path::Path;
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
    WSL,
    Unknown,
}

pub enum HypervisorType {
    Para,
    Full,
    Container,
    None,
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
            HypervisorVendor::WSL => write!(f, "Windows Subsystem for Linux"),
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
            HypervisorType::None => write!(f, "none"),
        }
    }
}

pub struct Hypervisor {
    pub hypervisor_vendor: Option<HypervisorVendor>,
    pub hypervisor_type: Option<HypervisorType>,
}

impl Hypervisor {
    const XENFEAT_SUPERVISOR_MODE_KERNEL:u32 = 3;
    const XENFEAT_MMU_PT_UPDATE_PRESERVE_AD:u32 = 5;
    const XENFEAT_HVM_CALLBACK_VECTOR:u32 = 8;

    const XEN_FEATURES_PV_MASK:u32 = (1 << Self::XENFEAT_MMU_PT_UPDATE_PRESERVE_AD);
    const XEN_FEATURES_PVH_MASK:u32 = ((1 << Self::XENFEAT_SUPERVISOR_MODE_KERNEL) | (1 << Self::XENFEAT_HVM_CALLBACK_VECTOR));

    pub fn new() -> Self {
        let (hypervisor_vendor, hypervisor_type) = Self::detect_hypervisor();
        Self {
            hypervisor_vendor: hypervisor_vendor,
            hypervisor_type: hypervisor_type,
        }
    }

    fn detect_hypervisor() -> (Option<HypervisorVendor>, Option<HypervisorType>) {
        if let Ok(os_release) = fs::read_to_string("/sys/kernel/osrelease")
            && os_release.contains("Microsoft") {
            let vendor = HypervisorVendor::WSL;
            let hypervisor_type = HypervisorType::Container;
            return (Some(vendor), Some(hypervisor_type));
        }

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
            (Some(vendor), Some(hypervisor_type))
        } else if Path::new("/proc/xen/").exists() {
            let xen_capabilities = fs::read_to_string("/proc/xen/capabilities").unwrap();
            let hypervisor_type = match xen_capabilities.as_str() {
                "control_d" => HypervisorType::None,
                _ => HypervisorType::Para,
            };
            let vendor = HypervisorVendor::Xen;
            (Some(vendor), Some(hypervisor_type))
        } else {
            (None, None)
        }
    }
}