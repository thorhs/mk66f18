#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Control Register n"]
    pub pcr: [PCR; 32],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: GPCLR,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: GPCHR,
    _reserved0: [u8; 24usize],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: ISFR,
}
#[doc = "Pin Control Register n"]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "Global Pin Control Low Register"]
pub struct GPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "Global Pin Control High Register"]
pub struct GPCHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "Interrupt Status Flag Register"]
pub struct ISFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
