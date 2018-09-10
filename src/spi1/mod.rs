#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Transfer Count Register"]
    pub tcr: TCR,
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Master Mode)"]
    pub ctar: [CTAR; 2],
    _reserved1: [u8; 24usize],
    #[doc = "0x2c - Status Register"]
    pub sr: SR,
    #[doc = "0x30 - DMA/Interrupt Request Select and Enable Register"]
    pub rser: RSER,
    #[doc = "0x34 - PUSH TX FIFO Register In Master Mode"]
    pub pushr: PUSHR,
    #[doc = "0x38 - POP RX FIFO Register"]
    pub popr: POPR,
    #[doc = "0x3c - Transmit FIFO Registers"]
    pub txfr: [TXFR; 4],
    _reserved2: [u8; 48usize],
    #[doc = "0x7c - Receive FIFO Registers"]
    pub rxfr: [RXFR; 4],
}
#[doc = "Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Transfer Count Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Count Register"]
pub mod tcr;
#[doc = "Clock and Transfer Attributes Register (In Master Mode)"]
pub struct CTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock and Transfer Attributes Register (In Master Mode)"]
pub mod ctar;
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)"]
pub struct CTAR_SLAVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)"]
pub mod ctar_slave;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "DMA/Interrupt Request Select and Enable Register"]
pub struct RSER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA/Interrupt Request Select and Enable Register"]
pub mod rser;
#[doc = "PUSH TX FIFO Register In Master Mode"]
pub struct PUSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUSH TX FIFO Register In Master Mode"]
pub mod pushr;
#[doc = "PUSH TX FIFO Register In Slave Mode"]
pub struct PUSHR_SLAVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUSH TX FIFO Register In Slave Mode"]
pub mod pushr_slave;
#[doc = "POP RX FIFO Register"]
pub struct POPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POP RX FIFO Register"]
pub mod popr;
#[doc = "Transmit FIFO Registers"]
pub struct TXFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Registers"]
pub mod txfr;
#[doc = "Receive FIFO Registers"]
pub struct RXFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Registers"]
pub mod rxfr;
