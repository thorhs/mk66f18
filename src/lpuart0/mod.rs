#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPUART Baud Rate Register"]
    pub baud: BAUD,
    #[doc = "0x04 - LPUART Status Register"]
    pub stat: STAT,
    #[doc = "0x08 - LPUART Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - LPUART Data Register"]
    pub data: DATA,
    #[doc = "0x10 - LPUART Match Address Register"]
    pub match_: MATCH,
    #[doc = "0x14 - LPUART Modem IrDA Register"]
    pub modir: MODIR,
}
#[doc = "LPUART Baud Rate Register"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "LPUART Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "LPUART Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "LPUART Data Register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "LPUART Match Address Register"]
pub struct MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Match Address Register"]
pub mod match_;
#[doc = "LPUART Modem IrDA Register"]
pub struct MODIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPUART Modem IrDA Register"]
pub mod modir;
