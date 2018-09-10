#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Voltage Detect Status And Control 1 register"]
    pub lvdsc1: LVDSC1,
    #[doc = "0x01 - Low Voltage Detect Status And Control 2 register"]
    pub lvdsc2: LVDSC2,
    #[doc = "0x02 - Regulator Status And Control register"]
    pub regsc: REGSC,
}
#[doc = "Low Voltage Detect Status And Control 1 register"]
pub struct LVDSC1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Low Voltage Detect Status And Control 1 register"]
pub mod lvdsc1;
#[doc = "Low Voltage Detect Status And Control 2 register"]
pub struct LVDSC2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Low Voltage Detect Status And Control 2 register"]
pub mod lvdsc2;
#[doc = "Regulator Status And Control register"]
pub struct REGSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Regulator Status And Control register"]
pub mod regsc;
