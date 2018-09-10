#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: TIMER,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx 14 Mask register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx 15 Mask register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 register"]
    pub esr1: ESR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: IMASK1,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: ESR2,
    _reserved3: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved4: [u8; 48usize],
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    pub cs0: CS0,
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    pub id0: ID0,
    #[doc = "0x88 - Message Buffer 0 WORD0 Register"]
    pub word00: WORD00,
    #[doc = "0x8c - Message Buffer 0 WORD1 Register"]
    pub word10: WORD10,
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    pub cs1: CS1,
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    pub id1: ID1,
    #[doc = "0x98 - Message Buffer 1 WORD0 Register"]
    pub word01: WORD01,
    #[doc = "0x9c - Message Buffer 1 WORD1 Register"]
    pub word11: WORD11,
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    pub cs2: CS2,
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    pub id2: ID2,
    #[doc = "0xa8 - Message Buffer 2 WORD0 Register"]
    pub word02: WORD02,
    #[doc = "0xac - Message Buffer 2 WORD1 Register"]
    pub word12: WORD12,
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    pub cs3: CS3,
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    pub id3: ID3,
    #[doc = "0xb8 - Message Buffer 3 WORD0 Register"]
    pub word03: WORD03,
    #[doc = "0xbc - Message Buffer 3 WORD1 Register"]
    pub word13: WORD13,
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    pub cs4: CS4,
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    pub id4: ID4,
    #[doc = "0xc8 - Message Buffer 4 WORD0 Register"]
    pub word04: WORD04,
    #[doc = "0xcc - Message Buffer 4 WORD1 Register"]
    pub word14: WORD14,
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    pub cs5: CS5,
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    pub id5: ID5,
    #[doc = "0xd8 - Message Buffer 5 WORD0 Register"]
    pub word05: WORD05,
    #[doc = "0xdc - Message Buffer 5 WORD1 Register"]
    pub word15: WORD15,
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    pub cs6: CS6,
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    pub id6: ID6,
    #[doc = "0xe8 - Message Buffer 6 WORD0 Register"]
    pub word06: WORD06,
    #[doc = "0xec - Message Buffer 6 WORD1 Register"]
    pub word16: WORD16,
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    pub cs7: CS7,
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    pub id7: ID7,
    #[doc = "0xf8 - Message Buffer 7 WORD0 Register"]
    pub word07: WORD07,
    #[doc = "0xfc - Message Buffer 7 WORD1 Register"]
    pub word17: WORD17,
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    pub cs8: CS8,
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    pub id8: ID8,
    #[doc = "0x108 - Message Buffer 8 WORD0 Register"]
    pub word08: WORD08,
    #[doc = "0x10c - Message Buffer 8 WORD1 Register"]
    pub word18: WORD18,
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    pub cs9: CS9,
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    pub id9: ID9,
    #[doc = "0x118 - Message Buffer 9 WORD0 Register"]
    pub word09: WORD09,
    #[doc = "0x11c - Message Buffer 9 WORD1 Register"]
    pub word19: WORD19,
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    pub cs10: CS10,
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    pub id10: ID10,
    #[doc = "0x128 - Message Buffer 10 WORD0 Register"]
    pub word010: WORD010,
    #[doc = "0x12c - Message Buffer 10 WORD1 Register"]
    pub word110: WORD110,
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    pub cs11: CS11,
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    pub id11: ID11,
    #[doc = "0x138 - Message Buffer 11 WORD0 Register"]
    pub word011: WORD011,
    #[doc = "0x13c - Message Buffer 11 WORD1 Register"]
    pub word111: WORD111,
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    pub cs12: CS12,
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    pub id12: ID12,
    #[doc = "0x148 - Message Buffer 12 WORD0 Register"]
    pub word012: WORD012,
    #[doc = "0x14c - Message Buffer 12 WORD1 Register"]
    pub word112: WORD112,
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    pub cs13: CS13,
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    pub id13: ID13,
    #[doc = "0x158 - Message Buffer 13 WORD0 Register"]
    pub word013: WORD013,
    #[doc = "0x15c - Message Buffer 13 WORD1 Register"]
    pub word113: WORD113,
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    pub cs14: CS14,
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    pub id14: ID14,
    #[doc = "0x168 - Message Buffer 14 WORD0 Register"]
    pub word014: WORD014,
    #[doc = "0x16c - Message Buffer 14 WORD1 Register"]
    pub word114: WORD114,
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    pub cs15: CS15,
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    pub id15: ID15,
    #[doc = "0x178 - Message Buffer 15 WORD0 Register"]
    pub word015: WORD015,
    #[doc = "0x17c - Message Buffer 15 WORD1 Register"]
    pub word115: WORD115,
    _reserved5: [u8; 1792usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 16],
}
#[doc = "Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 register"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "Free Running Timer"]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register"]
pub struct RXMGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx 14 Mask register"]
pub struct RX14MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "Rx 15 Mask register"]
pub struct RX15MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "Error Counter"]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "Error and Status 1 register"]
pub struct ESR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "Interrupt Masks 1 register"]
pub struct IMASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "Interrupt Flags 1 register"]
pub struct IFLAG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "Control 2 register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "Error and Status 2 register"]
pub struct ESR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRC Register"]
pub struct CRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask register"]
pub struct RXFGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register"]
pub struct RXFIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "Message Buffer 0 CS Register"]
pub struct CS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "Message Buffer 0 ID Register"]
pub struct ID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "Message Buffer 0 WORD0 Register"]
pub struct WORD00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "Message Buffer 0 WORD1 Register"]
pub struct WORD10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "Message Buffer 1 CS Register"]
pub struct CS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "Message Buffer 1 ID Register"]
pub struct ID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "Message Buffer 1 WORD0 Register"]
pub struct WORD01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "Message Buffer 1 WORD1 Register"]
pub struct WORD11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "Message Buffer 2 CS Register"]
pub struct CS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "Message Buffer 2 ID Register"]
pub struct ID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "Message Buffer 2 WORD0 Register"]
pub struct WORD02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "Message Buffer 2 WORD1 Register"]
pub struct WORD12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "Message Buffer 3 CS Register"]
pub struct CS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "Message Buffer 3 ID Register"]
pub struct ID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "Message Buffer 3 WORD0 Register"]
pub struct WORD03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "Message Buffer 3 WORD1 Register"]
pub struct WORD13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "Message Buffer 4 CS Register"]
pub struct CS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "Message Buffer 4 ID Register"]
pub struct ID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "Message Buffer 4 WORD0 Register"]
pub struct WORD04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "Message Buffer 4 WORD1 Register"]
pub struct WORD14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "Message Buffer 5 CS Register"]
pub struct CS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "Message Buffer 5 ID Register"]
pub struct ID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "Message Buffer 5 WORD0 Register"]
pub struct WORD05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "Message Buffer 5 WORD1 Register"]
pub struct WORD15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "Message Buffer 6 CS Register"]
pub struct CS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "Message Buffer 6 ID Register"]
pub struct ID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "Message Buffer 6 WORD0 Register"]
pub struct WORD06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "Message Buffer 6 WORD1 Register"]
pub struct WORD16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "Message Buffer 7 CS Register"]
pub struct CS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "Message Buffer 7 ID Register"]
pub struct ID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "Message Buffer 7 WORD0 Register"]
pub struct WORD07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "Message Buffer 7 WORD1 Register"]
pub struct WORD17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "Message Buffer 8 CS Register"]
pub struct CS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "Message Buffer 8 ID Register"]
pub struct ID8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "Message Buffer 8 WORD0 Register"]
pub struct WORD08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "Message Buffer 8 WORD1 Register"]
pub struct WORD18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "Message Buffer 9 CS Register"]
pub struct CS9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "Message Buffer 9 ID Register"]
pub struct ID9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "Message Buffer 9 WORD0 Register"]
pub struct WORD09 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "Message Buffer 9 WORD1 Register"]
pub struct WORD19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "Message Buffer 10 CS Register"]
pub struct CS10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "Message Buffer 10 ID Register"]
pub struct ID10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "Message Buffer 10 WORD0 Register"]
pub struct WORD010 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "Message Buffer 10 WORD1 Register"]
pub struct WORD110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "Message Buffer 11 CS Register"]
pub struct CS11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "Message Buffer 11 ID Register"]
pub struct ID11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "Message Buffer 11 WORD0 Register"]
pub struct WORD011 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "Message Buffer 11 WORD1 Register"]
pub struct WORD111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "Message Buffer 12 CS Register"]
pub struct CS12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "Message Buffer 12 ID Register"]
pub struct ID12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "Message Buffer 12 WORD0 Register"]
pub struct WORD012 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "Message Buffer 12 WORD1 Register"]
pub struct WORD112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "Message Buffer 13 CS Register"]
pub struct CS13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "Message Buffer 13 ID Register"]
pub struct ID13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "Message Buffer 13 WORD0 Register"]
pub struct WORD013 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "Message Buffer 13 WORD1 Register"]
pub struct WORD113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "Message Buffer 14 CS Register"]
pub struct CS14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "Message Buffer 14 ID Register"]
pub struct ID14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "Message Buffer 14 WORD0 Register"]
pub struct WORD014 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "Message Buffer 14 WORD1 Register"]
pub struct WORD114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "Message Buffer 15 CS Register"]
pub struct CS15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "Message Buffer 15 ID Register"]
pub struct ID15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "Message Buffer 15 WORD0 Register"]
pub struct WORD015 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "Message Buffer 15 WORD1 Register"]
pub struct WORD115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "Rx Individual Mask Registers"]
pub struct RXIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
