#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Select Address Register"]
    pub csar0: CSAR,
    #[doc = "0x04 - Chip Select Mask Register"]
    pub csmr0: CSMR,
    #[doc = "0x08 - Chip Select Control Register"]
    pub cscr0: CSCR,
    #[doc = "0x0c - Chip Select Address Register"]
    pub csar1: CSAR,
    #[doc = "0x10 - Chip Select Mask Register"]
    pub csmr1: CSMR,
    #[doc = "0x14 - Chip Select Control Register"]
    pub cscr1: CSCR,
    #[doc = "0x18 - Chip Select Address Register"]
    pub csar2: CSAR,
    #[doc = "0x1c - Chip Select Mask Register"]
    pub csmr2: CSMR,
    #[doc = "0x20 - Chip Select Control Register"]
    pub cscr2: CSCR,
    #[doc = "0x24 - Chip Select Address Register"]
    pub csar3: CSAR,
    #[doc = "0x28 - Chip Select Mask Register"]
    pub csmr3: CSMR,
    #[doc = "0x2c - Chip Select Control Register"]
    pub cscr3: CSCR,
    #[doc = "0x30 - Chip Select Address Register"]
    pub csar4: CSAR,
    #[doc = "0x34 - Chip Select Mask Register"]
    pub csmr4: CSMR,
    #[doc = "0x38 - Chip Select Control Register"]
    pub cscr4: CSCR,
    #[doc = "0x3c - Chip Select Address Register"]
    pub csar5: CSAR,
    #[doc = "0x40 - Chip Select Mask Register"]
    pub csmr5: CSMR,
    #[doc = "0x44 - Chip Select Control Register"]
    pub cscr5: CSCR,
    _reserved0: [u8; 24usize],
    #[doc = "0x60 - Chip Select port Multiplexing Control Register"]
    pub cspmcr: CSPMCR,
}
#[doc = "Chip Select Address Register"]
pub struct CSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Address Register"]
pub mod csar;
#[doc = "Chip Select Mask Register"]
pub struct CSMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Mask Register"]
pub mod csmr;
#[doc = "Chip Select Control Register"]
pub struct CSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Control Register"]
pub mod cscr;
#[doc = "Chip Select port Multiplexing Control Register"]
pub struct CSPMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select port Multiplexing Control Register"]
pub mod cspmcr;
