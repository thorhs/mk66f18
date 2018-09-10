#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Control Register"]
    pub cr: CR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub iscr: ISCR,
    #[doc = "0x14 - ETB Counter Control register"]
    pub etbcc: ETBCC,
    #[doc = "0x18 - ETB Reload register"]
    pub etbrl: ETBRL,
    #[doc = "0x1c - ETB Counter Value register"]
    pub etbcnt: ETBCNT,
    #[doc = "0x20 - Fault address register"]
    pub fadr: FADR,
    #[doc = "0x24 - Fault attributes register"]
    pub fatr: FATR,
    #[doc = "0x28 - Fault data register"]
    pub fdr: FDR,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Process ID register"]
    pub pid: PID,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub struct PLASC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub struct PLAMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Interrupt Status Register"]
pub struct ISCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod iscr;
#[doc = "ETB Counter Control register"]
pub struct ETBCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETB Counter Control register"]
pub mod etbcc;
#[doc = "ETB Reload register"]
pub struct ETBRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETB Reload register"]
pub mod etbrl;
#[doc = "ETB Counter Value register"]
pub struct ETBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETB Counter Value register"]
pub mod etbcnt;
#[doc = "Fault address register"]
pub struct FADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault address register"]
pub mod fadr;
#[doc = "Fault attributes register"]
pub struct FATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault attributes register"]
pub mod fatr;
#[doc = "Fault data register"]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault data register"]
pub mod fdr;
#[doc = "Process ID register"]
pub struct PID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Process ID register"]
pub mod pid;
#[doc = "Compute Operation Control Register"]
pub struct CPO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
