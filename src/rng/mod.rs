#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNGA Control Register"]
    pub cr: CR,
    #[doc = "0x04 - RNGA Status Register"]
    pub sr: SR,
    #[doc = "0x08 - RNGA Entropy Register"]
    pub er: ER,
    #[doc = "0x0c - RNGA Output Register"]
    pub or: OR,
}
#[doc = "RNGA Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNGA Control Register"]
pub mod cr;
#[doc = "RNGA Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNGA Status Register"]
pub mod sr;
#[doc = "RNGA Entropy Register"]
pub struct ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNGA Entropy Register"]
pub mod er;
#[doc = "RNGA Output Register"]
pub struct OR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNGA Output Register"]
pub mod or;
