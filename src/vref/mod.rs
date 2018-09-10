#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF Trim Register"]
    pub trm: TRM,
    #[doc = "0x01 - VREF Status and Control Register"]
    pub sc: SC,
}
#[doc = "VREF Trim Register"]
pub struct TRM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "VREF Trim Register"]
pub mod trm;
#[doc = "VREF Status and Control Register"]
pub struct SC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "VREF Status and Control Register"]
pub mod sc;
