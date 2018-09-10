#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address register"]
    pub dsaddr: DSADDR,
    #[doc = "0x04 - Block Attributes register"]
    pub blkattr: BLKATTR,
    #[doc = "0x08 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x0c - Transfer Type register"]
    pub xfertyp: XFERTYP,
    #[doc = "0x10 - Command Response 0"]
    pub cmdrsp0: CMDRSP0,
    #[doc = "0x14 - Command Response 1"]
    pub cmdrsp1: CMDRSP1,
    #[doc = "0x18 - Command Response 2"]
    pub cmdrsp2: CMDRSP2,
    #[doc = "0x1c - Command Response 3"]
    pub cmdrsp3: CMDRSP3,
    #[doc = "0x20 - Buffer Data Port register"]
    pub datport: DATPORT,
    #[doc = "0x24 - Present State register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Protocol Control register"]
    pub proctl: PROCTL,
    #[doc = "0x2c - System Control register"]
    pub sysctl: SYSCTL,
    #[doc = "0x30 - Interrupt Status register"]
    pub irqstat: IRQSTAT,
    #[doc = "0x34 - Interrupt Status Enable register"]
    pub irqstaten: IRQSTATEN,
    #[doc = "0x38 - Interrupt Signal Enable register"]
    pub irqsigen: IRQSIGEN,
    #[doc = "0x3c - Auto CMD12 Error Status Register"]
    pub ac12err: AC12ERR,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub htcapblt: HTCAPBLT,
    #[doc = "0x44 - Watermark Level Register"]
    pub wml: WML,
    _reserved0: [u8; 8usize],
    #[doc = "0x50 - Force Event register"]
    pub fevt: FEVT,
    #[doc = "0x54 - ADMA Error Status register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Addressregister"]
    pub adsaddr: ADSADDR,
    _reserved1: [u8; 100usize],
    #[doc = "0xc0 - Vendor Specific register"]
    pub vendor: VENDOR,
    #[doc = "0xc4 - MMC Boot register"]
    pub mmcboot: MMCBOOT,
    _reserved2: [u8; 52usize],
    #[doc = "0xfc - Host Controller Version"]
    pub hostver: HOSTVER,
}
#[doc = "DMA System Address register"]
pub struct DSADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA System Address register"]
pub mod dsaddr;
#[doc = "Block Attributes register"]
pub struct BLKATTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Attributes register"]
pub mod blkattr;
#[doc = "Command Argument register"]
pub struct CMDARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "Transfer Type register"]
pub struct XFERTYP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Type register"]
pub mod xfertyp;
#[doc = "Command Response 0"]
pub struct CMDRSP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response 0"]
pub mod cmdrsp0;
#[doc = "Command Response 1"]
pub struct CMDRSP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response 1"]
pub mod cmdrsp1;
#[doc = "Command Response 2"]
pub struct CMDRSP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response 2"]
pub mod cmdrsp2;
#[doc = "Command Response 3"]
pub struct CMDRSP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response 3"]
pub mod cmdrsp3;
#[doc = "Buffer Data Port register"]
pub struct DATPORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Data Port register"]
pub mod datport;
#[doc = "Present State register"]
pub struct PRSSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Present State register"]
pub mod prsstat;
#[doc = "Protocol Control register"]
pub struct PROCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control register"]
pub mod proctl;
#[doc = "System Control register"]
pub struct SYSCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control register"]
pub mod sysctl;
#[doc = "Interrupt Status register"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status register"]
pub mod irqstat;
#[doc = "Interrupt Status Enable register"]
pub struct IRQSTATEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Enable register"]
pub mod irqstaten;
#[doc = "Interrupt Signal Enable register"]
pub struct IRQSIGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Signal Enable register"]
pub mod irqsigen;
#[doc = "Auto CMD12 Error Status Register"]
pub struct AC12ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto CMD12 Error Status Register"]
pub mod ac12err;
#[doc = "Host Controller Capabilities"]
pub struct HTCAPBLT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Capabilities"]
pub mod htcapblt;
#[doc = "Watermark Level Register"]
pub struct WML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watermark Level Register"]
pub mod wml;
#[doc = "Force Event register"]
pub struct FEVT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force Event register"]
pub mod fevt;
#[doc = "ADMA Error Status register"]
pub struct ADMAES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA Error Status register"]
pub mod admaes;
#[doc = "ADMA System Addressregister"]
pub struct ADSADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA System Addressregister"]
pub mod adsaddr;
#[doc = "Vendor Specific register"]
pub struct VENDOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vendor Specific register"]
pub mod vendor;
#[doc = "MMC Boot register"]
pub struct MMCBOOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Boot register"]
pub mod mmcboot;
#[doc = "Host Controller Version"]
pub struct HOSTVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Version"]
pub mod hostver;
