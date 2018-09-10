#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC Control Register"]
    pub cr: CR,
    _reserved0: [u8; 1usize],
    #[doc = "0x02 - OSC_DIV"]
    pub div: DIV,
}
#[doc = "OSC Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OSC Control Register"]
pub mod cr;
#[doc = "OSC_DIV"]
pub struct DIV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OSC_DIV"]
pub mod div;
