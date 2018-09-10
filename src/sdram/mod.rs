#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 66usize],
    #[doc = "0x42 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x48 - Address and Control Register"]
    pub ac0: AC,
    #[doc = "0x4c - Control Mask"]
    pub cm0: CM,
    #[doc = "0x50 - Address and Control Register"]
    pub ac1: AC,
    #[doc = "0x54 - Control Mask"]
    pub cm1: CM,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Address and Control Register"]
pub struct AC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address and Control Register"]
pub mod ac;
#[doc = "Control Mask"]
pub struct CM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Mask"]
pub mod cm;
