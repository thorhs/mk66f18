#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAI Transmit Control Register"]
    pub tcsr: TCSR,
    #[doc = "0x04 - SAI Transmit Configuration 1 Register"]
    pub tcr1: TCR1,
    #[doc = "0x08 - SAI Transmit Configuration 2 Register"]
    pub tcr2: TCR2,
    #[doc = "0x0c - SAI Transmit Configuration 3 Register"]
    pub tcr3: TCR3,
    #[doc = "0x10 - SAI Transmit Configuration 4 Register"]
    pub tcr4: TCR4,
    #[doc = "0x14 - SAI Transmit Configuration 5 Register"]
    pub tcr5: TCR5,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - SAI Transmit Data Register"]
    pub tdr: [TDR; 2],
    _reserved1: [u8; 24usize],
    #[doc = "0x40 - SAI Transmit FIFO Register"]
    pub tfr: [TFR; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x60 - SAI Transmit Mask Register"]
    pub tmr: TMR,
    _reserved3: [u8; 28usize],
    #[doc = "0x80 - SAI Receive Control Register"]
    pub rcsr: RCSR,
    #[doc = "0x84 - SAI Receive Configuration 1 Register"]
    pub rcr1: RCR1,
    #[doc = "0x88 - SAI Receive Configuration 2 Register"]
    pub rcr2: RCR2,
    #[doc = "0x8c - SAI Receive Configuration 3 Register"]
    pub rcr3: RCR3,
    #[doc = "0x90 - SAI Receive Configuration 4 Register"]
    pub rcr4: RCR4,
    #[doc = "0x94 - SAI Receive Configuration 5 Register"]
    pub rcr5: RCR5,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - SAI Receive Data Register"]
    pub rdr: [RDR; 2],
    _reserved5: [u8; 24usize],
    #[doc = "0xc0 - SAI Receive FIFO Register"]
    pub rfr: [RFR; 2],
    _reserved6: [u8; 24usize],
    #[doc = "0xe0 - SAI Receive Mask Register"]
    pub rmr: RMR,
    _reserved7: [u8; 28usize],
    #[doc = "0x100 - SAI MCLK Control Register"]
    pub mcr: MCR,
    #[doc = "0x104 - SAI MCLK Divide Register"]
    pub mdr: MDR,
}
#[doc = "SAI Transmit Control Register"]
pub struct TCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Control Register"]
pub mod tcsr;
#[doc = "SAI Transmit Configuration 1 Register"]
pub struct TCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Configuration 1 Register"]
pub mod tcr1;
#[doc = "SAI Transmit Configuration 2 Register"]
pub struct TCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod tcr2;
#[doc = "SAI Transmit Configuration 3 Register"]
pub struct TCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod tcr3;
#[doc = "SAI Transmit Configuration 4 Register"]
pub struct TCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod tcr4;
#[doc = "SAI Transmit Configuration 5 Register"]
pub struct TCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod tcr5;
#[doc = "SAI Transmit Data Register"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Data Register"]
pub mod tdr;
#[doc = "SAI Transmit FIFO Register"]
pub struct TFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit FIFO Register"]
pub mod tfr;
#[doc = "SAI Transmit Mask Register"]
pub struct TMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Transmit Mask Register"]
pub mod tmr;
#[doc = "SAI Receive Control Register"]
pub struct RCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Control Register"]
pub mod rcsr;
#[doc = "SAI Receive Configuration 1 Register"]
pub struct RCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Configuration 1 Register"]
pub mod rcr1;
#[doc = "SAI Receive Configuration 2 Register"]
pub struct RCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Configuration 2 Register"]
pub mod rcr2;
#[doc = "SAI Receive Configuration 3 Register"]
pub struct RCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Configuration 3 Register"]
pub mod rcr3;
#[doc = "SAI Receive Configuration 4 Register"]
pub struct RCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Configuration 4 Register"]
pub mod rcr4;
#[doc = "SAI Receive Configuration 5 Register"]
pub struct RCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Configuration 5 Register"]
pub mod rcr5;
#[doc = "SAI Receive Data Register"]
pub struct RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Data Register"]
pub mod rdr;
#[doc = "SAI Receive FIFO Register"]
pub struct RFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive FIFO Register"]
pub mod rfr;
#[doc = "SAI Receive Mask Register"]
pub struct RMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI Receive Mask Register"]
pub mod rmr;
#[doc = "SAI MCLK Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI MCLK Control Register"]
pub mod mcr;
#[doc = "SAI MCLK Divide Register"]
pub struct MDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI MCLK Divide Register"]
pub mod mdr;
