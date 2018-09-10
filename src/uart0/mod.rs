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
    _reserved1: [u8; 1usize],
    #[doc = "0x18 - UART 7816 Control Register"]
    pub c7816: C7816,
    #[doc = "0x19 - UART 7816 Interrupt Enable Register"]
    pub ie7816: IE7816,
    #[doc = "0x1a - UART 7816 Interrupt Status Register"]
    pub is7816: IS7816,
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    pub wp7816: WP7816,
    #[doc = "0x1c - UART 7816 Wait N Register"]
    pub wn7816: WN7816,
    #[doc = "0x1d - UART 7816 Wait FD Register"]
    pub wf7816: WF7816,
    #[doc = "0x1e - UART 7816 Error Threshold Register"]
    pub et7816: ET7816,
    #[doc = "0x1f - UART 7816 Transmit Length Register"]
    pub tl7816: TL7816,
    _reserved2: [u8; 26usize],
    #[doc = "0x3a - UART 7816 ATR Duration Timer Register A"]
    pub ap7816a_t0: AP7816A_T0,
    #[doc = "0x3b - UART 7816 ATR Duration Timer Register B"]
    pub ap7816b_t0: AP7816B_T0,
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    pub wp7816a_t0: WP7816A_T0,
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    pub wp7816b_t0: WP7816B_T0,
    #[doc = "0x3e - UART 7816 Wait and Guard Parameter Register"]
    pub wgp7816_t1: WGP7816_T1,
    #[doc = "0x3f - UART 7816 Wait Parameter Register C"]
    pub wp7816c_t1: WP7816C_T1,
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
#[doc = "UART 7816 Control Register"]
pub struct C7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Control Register"]
pub mod c7816;
#[doc = "UART 7816 Interrupt Enable Register"]
pub struct IE7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Interrupt Enable Register"]
pub mod ie7816;
#[doc = "UART 7816 Interrupt Status Register"]
pub struct IS7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Interrupt Status Register"]
pub mod is7816;
#[doc = "UART 7816 Wait Parameter Register"]
pub struct WP7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register"]
pub mod wp7816;
#[doc = "UART 7816 Wait N Register"]
pub struct WN7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait N Register"]
pub mod wn7816;
#[doc = "UART 7816 Wait FD Register"]
pub struct WF7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait FD Register"]
pub mod wf7816;
#[doc = "UART 7816 Error Threshold Register"]
pub struct ET7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Error Threshold Register"]
pub mod et7816;
#[doc = "UART 7816 Transmit Length Register"]
pub struct TL7816 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Transmit Length Register"]
pub mod tl7816;
#[doc = "UART 7816 ATR Duration Timer Register A"]
pub struct AP7816A_T0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 ATR Duration Timer Register A"]
pub mod ap7816a_t0;
#[doc = "UART 7816 ATR Duration Timer Register B"]
pub struct AP7816B_T0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 ATR Duration Timer Register B"]
pub mod ap7816b_t0;
#[doc = "UART 7816 Wait Parameter Register A"]
pub struct WP7816A_T0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod wp7816a_t0;
#[doc = "UART 7816 Wait Parameter Register A"]
pub struct WP7816A_T1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod wp7816a_t1;
#[doc = "UART 7816 Wait Parameter Register B"]
pub struct WP7816B_T0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod wp7816b_t0;
#[doc = "UART 7816 Wait Parameter Register B"]
pub struct WP7816B_T1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod wp7816b_t1;
#[doc = "UART 7816 Wait and Guard Parameter Register"]
pub struct WGP7816_T1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait and Guard Parameter Register"]
pub mod wgp7816_t1;
#[doc = "UART 7816 Wait Parameter Register C"]
pub struct WP7816C_T1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "UART 7816 Wait Parameter Register C"]
pub mod wp7816c_t1;
