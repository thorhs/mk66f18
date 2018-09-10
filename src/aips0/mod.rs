#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: MPRA,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: PACRA,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: PACRB,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: PACRC,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: PACRD,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Peripheral Access Control Register"]
    pub pacre: PACRE,
    #[doc = "0x44 - Peripheral Access Control Register"]
    pub pacrf: PACRF,
    #[doc = "0x48 - Peripheral Access Control Register"]
    pub pacrg: PACRG,
    #[doc = "0x4c - Peripheral Access Control Register"]
    pub pacrh: PACRH,
    #[doc = "0x50 - Peripheral Access Control Register"]
    pub pacri: PACRI,
    #[doc = "0x54 - Peripheral Access Control Register"]
    pub pacrj: PACRJ,
    #[doc = "0x58 - Peripheral Access Control Register"]
    pub pacrk: PACRK,
    #[doc = "0x5c - Peripheral Access Control Register"]
    pub pacrl: PACRL,
    #[doc = "0x60 - Peripheral Access Control Register"]
    pub pacrm: PACRM,
    #[doc = "0x64 - Peripheral Access Control Register"]
    pub pacrn: PACRN,
    #[doc = "0x68 - Peripheral Access Control Register"]
    pub pacro: PACRO,
    #[doc = "0x6c - Peripheral Access Control Register"]
    pub pacrp: PACRP,
}
#[doc = "Master Privilege Register A"]
pub struct MPRA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "Peripheral Access Control Register"]
pub struct PACRA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "Peripheral Access Control Register"]
pub struct PACRB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "Peripheral Access Control Register"]
pub struct PACRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "Peripheral Access Control Register"]
pub struct PACRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "Peripheral Access Control Register"]
pub struct PACRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacre;
#[doc = "Peripheral Access Control Register"]
pub struct PACRF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrf;
#[doc = "Peripheral Access Control Register"]
pub struct PACRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrg;
#[doc = "Peripheral Access Control Register"]
pub struct PACRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrh;
#[doc = "Peripheral Access Control Register"]
pub struct PACRI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacri;
#[doc = "Peripheral Access Control Register"]
pub struct PACRJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrj;
#[doc = "Peripheral Access Control Register"]
pub struct PACRK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrk;
#[doc = "Peripheral Access Control Register"]
pub struct PACRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrl;
#[doc = "Peripheral Access Control Register"]
pub struct PACRM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrm;
#[doc = "Peripheral Access Control Register"]
pub struct PACRN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrn;
#[doc = "Peripheral Access Control Register"]
pub struct PACRO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacro;
#[doc = "Peripheral Access Control Register"]
pub struct PACRP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrp;
