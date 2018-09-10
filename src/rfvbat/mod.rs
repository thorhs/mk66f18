#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VBAT register file register"]
    pub reg: [REG; 8],
}
#[doc = "VBAT register file register"]
pub struct REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VBAT register file register"]
pub mod reg;
