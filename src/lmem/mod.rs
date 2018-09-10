#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub pcccr: PCCCR,
    #[doc = "0x04 - Cache line control register"]
    pub pcclcr: PCCLCR,
    #[doc = "0x08 - Cache search address register"]
    pub pccsar: PCCSAR,
    #[doc = "0x0c - Cache read/write value register"]
    pub pcccvr: PCCCVR,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: PCCRMR,
}
#[doc = "Cache control register"]
pub struct PCCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache control register"]
pub mod pcccr;
#[doc = "Cache line control register"]
pub struct PCCLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache line control register"]
pub mod pcclcr;
#[doc = "Cache search address register"]
pub struct PCCSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache search address register"]
pub mod pccsar;
#[doc = "Cache read/write value register"]
pub struct PCCCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache read/write value register"]
pub mod pcccvr;
#[doc = "Cache regions mode register"]
pub struct PCCRMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache regions mode register"]
pub mod pccrmr;
