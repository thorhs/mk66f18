#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Configuration register"]
    pub chcfg: [CHCFG; 32],
}
#[doc = "Channel Configuration register"]
pub struct CHCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel Configuration register"]
pub mod chcfg;
