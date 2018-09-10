#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TSI General Control and Status Register"]
    pub gencs: GENCS,
    #[doc = "0x04 - TSI DATA Register"]
    pub data: DATA,
    #[doc = "0x08 - TSI Threshold Register"]
    pub tshd: TSHD,
}
#[doc = "TSI General Control and Status Register"]
pub struct GENCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSI General Control and Status Register"]
pub mod gencs;
#[doc = "TSI DATA Register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSI DATA Register"]
pub mod data;
#[doc = "TSI Threshold Register"]
pub struct TSHD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSI Threshold Register"]
pub mod tshd;
