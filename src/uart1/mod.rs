#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Registers: High"]
    pub bdh: BDH,
    #[doc = "0x01 - UART Baud Rate Registers: Low"]
    pub bdl: BDL,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: C2,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: S1,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: S2,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: C3,
    #[doc = "0x07 - UART Data Register"]
    pub d: D,
    #[doc = "0x08 - UART Match Address Registers 1"]
    pub ma1: MA1,
    #[doc = "0x09 - UART Match Address Registers 2"]
    pub ma2: MA2,
    #[doc = "0x0a - UART Control Register 4"]
    pub c4: C4,
    #[doc = "0x0b - UART Control Register 5"]
    pub c5: C5,
    #[doc = "0x0c - UART Extended Data Register"]
    pub ed: ED,
    #[doc = "0x0d - UART Modem Register"]
    pub modem: MODEM,
    #[doc = "0x0e - UART Infrared Register"]
    pub ir: IR,
    _reserved0: [u8; 1usize],
    #[doc = "0x10 - UART FIFO Parameters"]
    pub pfifo: PFIFO,
    #[doc = "0x11 - UART FIFO Control Register"]
    pub cfifo: CFIFO,
    #[doc = "0x12 - UART FIFO Status Register"]
    pub sfifo: SFIFO,
    #[doc = "0x13 - UART FIFO Transmit Watermark"]
    pub twfifo: TWFIFO,
    #[doc = "0x14 - UART FIFO Transmit Count"]
    pub tcfifo: TCFIFO,
    #[doc = "0x15 - UART FIFO Receive Watermark"]
    pub rwfifo: RWFIFO,
    #[doc = "0x16 - UART FIFO Receive Count"]
    pub rcfifo: RCFIFO,
}
#[doc = "UART Baud Rate Registers: High"]
pub struct BDH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Baud Rate Registers: High"]
pub mod bdh;
#[doc = "UART Baud Rate Registers: Low"]
pub struct BDL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Baud Rate Registers: Low"]
pub mod bdl;
#[doc = "UART Control Register 1"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "UART Control Register 2"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "UART Status Register 1"]
pub struct S1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "UART Status Register 2"]
pub struct S2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "UART Control Register 3"]
pub struct C3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "UART Data Register"]
pub struct D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Data Register"]
pub mod d;
#[doc = "UART Match Address Registers 1"]
pub struct MA1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "UART Match Address Registers 2"]
pub struct MA2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "UART Control Register 4"]
pub struct C4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "UART Control Register 5"]
pub struct C5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Control Register 5"]
pub mod c5;
#[doc = "UART Extended Data Register"]
pub struct ED {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Extended Data Register"]
pub mod ed;
#[doc = "UART Modem Register"]
pub struct MODEM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Modem Register"]
pub mod modem;
#[doc = "UART Infrared Register"]
pub struct IR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART Infrared Register"]
pub mod ir;
#[doc = "UART FIFO Parameters"]
pub struct PFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Parameters"]
pub mod pfifo;
#[doc = "UART FIFO Control Register"]
pub struct CFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Control Register"]
pub mod cfifo;
#[doc = "UART FIFO Status Register"]
pub struct SFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Status Register"]
pub mod sfifo;
#[doc = "UART FIFO Transmit Watermark"]
pub struct TWFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Transmit Watermark"]
pub mod twfifo;
#[doc = "UART FIFO Transmit Count"]
pub struct TCFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Transmit Count"]
pub mod tcfifo;
#[doc = "UART FIFO Receive Watermark"]
pub struct RWFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Receive Watermark"]
pub mod rwfifo;
#[doc = "UART FIFO Receive Count"]
pub struct RCFIFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART FIFO Receive Count"]
pub mod rcfifo;
