#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Register file register"]
    pub reg: [REG; 8],
}
#[doc = "Register file register"]
pub struct REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register file register"]
pub mod reg;
