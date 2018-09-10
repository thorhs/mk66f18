#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"]
    pub sc: SC,
    #[doc = "0x04 - Modulus register"]
    pub mod_: MOD,
    #[doc = "0x08 - Counter register"]
    pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay register"]
    pub idly: IDLY,
    #[doc = "0x10 - Channel n Control register 1"]
    pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status register"]
    pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 register"]
    pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 register"]
    pub ch0dly1: CHDLY1,
    _reserved0: [u8; 24usize],
    #[doc = "0x38 - Channel n Control register 1"]
    pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status register"]
    pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 register"]
    pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 register"]
    pub ch1dly1: CHDLY1,
    _reserved1: [u8; 264usize],
    #[doc = "0x150 - DAC Interval Trigger n Control register"]
    pub dacintc0: DACINTC,
    #[doc = "0x154 - DAC Interval n register"]
    pub dacint0: DACINT,
    #[doc = "0x158 - DAC Interval Trigger n Control register"]
    pub dacintc1: DACINTC,
    #[doc = "0x15c - DAC Interval n register"]
    pub dacint1: DACINT,
    _reserved2: [u8; 48usize],
    #[doc = "0x190 - Pulse-Out n Enable register"]
    pub poen: POEN,
    #[doc = "0x194 - Pulse-Out n Delay register"]
    pub podly: [PODLY; 4],
}
#[doc = "Status and Control register"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "Modulus register"]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "Counter register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Interrupt Delay register"]
pub struct IDLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "Channel n Control register 1"]
pub struct CHC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "Channel n Status register"]
pub struct CHS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "Channel n Delay 0 register"]
pub struct CHDLY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "Channel n Delay 1 register"]
pub struct CHDLY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "DAC Interval Trigger n Control register"]
pub struct DACINTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Interval Trigger n Control register"]
pub mod dacintc;
#[doc = "DAC Interval n register"]
pub struct DACINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Interval n register"]
pub mod dacint;
#[doc = "Pulse-Out n Enable register"]
pub struct POEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "Pulse-Out n Delay register"]
pub struct PODLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse-Out n Delay register"]
pub mod podly;
