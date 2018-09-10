#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    pub pdor: PDOR,
    #[doc = "0x04 - Port Set Output Register"]
    pub psor: PSOR,
    #[doc = "0x08 - Port Clear Output Register"]
    pub pcor: PCOR,
    #[doc = "0x0c - Port Toggle Output Register"]
    pub ptor: PTOR,
    #[doc = "0x10 - Port Data Input Register"]
    pub pdir: PDIR,
    #[doc = "0x14 - Port Data Direction Register"]
    pub pddr: PDDR,
}
#[doc = "Port Data Output Register"]
pub struct PDOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "Port Set Output Register"]
pub struct PSOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "Port Clear Output Register"]
pub struct PCOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "Port Toggle Output Register"]
pub struct PTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "Port Data Input Register"]
pub struct PDIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "Port Data Direction Register"]
pub struct PDDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Direction Register"]
pub mod pddr;
