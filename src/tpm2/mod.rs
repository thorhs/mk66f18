#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status and Control"]
    pub c0sc: CSC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: CV,
    #[doc = "0x14 - Channel (n) Status and Control"]
    pub c1sc: CSC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: CV,
    _reserved0: [u8; 52usize],
    #[doc = "0x50 - Capture and Compare Status"]
    pub status: STATUS,
    _reserved1: [u8; 16usize],
    #[doc = "0x64 - Combine Channel Register"]
    pub combine: COMBINE,
    _reserved2: [u8; 8usize],
    #[doc = "0x70 - Channel Polarity"]
    pub pol: POL,
    _reserved3: [u8; 4usize],
    #[doc = "0x78 - Filter Control"]
    pub filter: FILTER,
    _reserved4: [u8; 4usize],
    #[doc = "0x80 - Quadrature Decoder Control and Status"]
    pub qdctrl: QDCTRL,
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
}
#[doc = "Status and Control"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control"]
pub mod sc;
#[doc = "Counter"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo"]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Channel (n) Status and Control"]
pub struct CSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "Channel (n) Value"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "Capture and Compare Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "Combine Channel Register"]
pub struct COMBINE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combine Channel Register"]
pub mod combine;
#[doc = "Channel Polarity"]
pub struct POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Polarity"]
pub mod pol;
#[doc = "Filter Control"]
pub struct FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Control"]
pub mod filter;
#[doc = "Quadrature Decoder Control and Status"]
pub struct QDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Decoder Control and Status"]
pub mod qdctrl;
#[doc = "Configuration"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod conf;
