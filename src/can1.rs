#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: TIMER,
    _reserved3: [u8; 4usize],
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
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: IMASK1,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: ESR2,
    _reserved12: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved15: [u8; 48usize],
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
    _reserved79: [u8; 1792usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 16],
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "Free Running Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer](timer) module"]
pub type TIMER = crate::Reg<u32, _TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER;
#[doc = "`read()` method returns [timer::R](timer::R) reader structure"]
impl crate::Readable for TIMER {}
#[doc = "`write(|w| ..)` method takes [timer::W](timer::W) writer structure"]
impl crate::Writable for TIMER {}
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxmgmask](rxmgmask) module"]
pub type RXMGMASK = crate::Reg<u32, _RXMGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMGMASK;
#[doc = "`read()` method returns [rxmgmask::R](rxmgmask::R) reader structure"]
impl crate::Readable for RXMGMASK {}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](rxmgmask::W) writer structure"]
impl crate::Writable for RXMGMASK {}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx 14 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx14mask](rx14mask) module"]
pub type RX14MASK = crate::Reg<u32, _RX14MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX14MASK;
#[doc = "`read()` method returns [rx14mask::R](rx14mask::R) reader structure"]
impl crate::Readable for RX14MASK {}
#[doc = "`write(|w| ..)` method takes [rx14mask::W](rx14mask::W) writer structure"]
impl crate::Writable for RX14MASK {}
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "Rx 15 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx15mask](rx15mask) module"]
pub type RX15MASK = crate::Reg<u32, _RX15MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX15MASK;
#[doc = "`read()` method returns [rx15mask::R](rx15mask::R) reader structure"]
impl crate::Readable for RX15MASK {}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](rx15mask::W) writer structure"]
impl crate::Writable for RX15MASK {}
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "Error and Status 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esr1](esr1) module"]
pub type ESR1 = crate::Reg<u32, _ESR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR1;
#[doc = "`read()` method returns [esr1::R](esr1::R) reader structure"]
impl crate::Readable for ESR1 {}
#[doc = "`write(|w| ..)` method takes [esr1::W](esr1::W) writer structure"]
impl crate::Writable for ESR1 {}
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "Interrupt Masks 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imask1](imask1) module"]
pub type IMASK1 = crate::Reg<u32, _IMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMASK1;
#[doc = "`read()` method returns [imask1::R](imask1::R) reader structure"]
impl crate::Readable for IMASK1 {}
#[doc = "`write(|w| ..)` method takes [imask1::W](imask1::W) writer structure"]
impl crate::Writable for IMASK1 {}
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "Interrupt Flags 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iflag1](iflag1) module"]
pub type IFLAG1 = crate::Reg<u32, _IFLAG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLAG1;
#[doc = "`read()` method returns [iflag1::R](iflag1::R) reader structure"]
impl crate::Readable for IFLAG1 {}
#[doc = "`write(|w| ..)` method takes [iflag1::W](iflag1::W) writer structure"]
impl crate::Writable for IFLAG1 {}
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "Error and Status 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esr2](esr2) module"]
pub type ESR2 = crate::Reg<u32, _ESR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR2;
#[doc = "`read()` method returns [esr2::R](esr2::R) reader structure"]
impl crate::Readable for ESR2 {}
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crcr](crcr) module"]
pub type CRCR = crate::Reg<u32, _CRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCR;
#[doc = "`read()` method returns [crcr::R](crcr::R) reader structure"]
impl crate::Readable for CRCR {}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfgmask](rxfgmask) module"]
pub type RXFGMASK = crate::Reg<u32, _RXFGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFGMASK;
#[doc = "`read()` method returns [rxfgmask::R](rxfgmask::R) reader structure"]
impl crate::Readable for RXFGMASK {}
#[doc = "`write(|w| ..)` method takes [rxfgmask::W](rxfgmask::W) writer structure"]
impl crate::Writable for RXFGMASK {}
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfir](rxfir) module"]
pub type RXFIR = crate::Reg<u32, _RXFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIR;
#[doc = "`read()` method returns [rxfir::R](rxfir::R) reader structure"]
impl crate::Readable for RXFIR {}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "Message Buffer 0 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs0](cs0) module"]
pub type CS0 = crate::Reg<u32, _CS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS0;
#[doc = "`read()` method returns [cs0::R](cs0::R) reader structure"]
impl crate::Readable for CS0 {}
#[doc = "`write(|w| ..)` method takes [cs0::W](cs0::W) writer structure"]
impl crate::Writable for CS0 {}
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "Message Buffer 0 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id0](id0) module"]
pub type ID0 = crate::Reg<u32, _ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID0;
#[doc = "`read()` method returns [id0::R](id0::R) reader structure"]
impl crate::Readable for ID0 {}
#[doc = "`write(|w| ..)` method takes [id0::W](id0::W) writer structure"]
impl crate::Writable for ID0 {}
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "Message Buffer 0 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word00](word00) module"]
pub type WORD00 = crate::Reg<u32, _WORD00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD00;
#[doc = "`read()` method returns [word00::R](word00::R) reader structure"]
impl crate::Readable for WORD00 {}
#[doc = "`write(|w| ..)` method takes [word00::W](word00::W) writer structure"]
impl crate::Writable for WORD00 {}
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "Message Buffer 0 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word10](word10) module"]
pub type WORD10 = crate::Reg<u32, _WORD10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD10;
#[doc = "`read()` method returns [word10::R](word10::R) reader structure"]
impl crate::Readable for WORD10 {}
#[doc = "`write(|w| ..)` method takes [word10::W](word10::W) writer structure"]
impl crate::Writable for WORD10 {}
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "Message Buffer 1 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs1](cs1) module"]
pub type CS1 = crate::Reg<u32, _CS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS1;
#[doc = "`read()` method returns [cs1::R](cs1::R) reader structure"]
impl crate::Readable for CS1 {}
#[doc = "`write(|w| ..)` method takes [cs1::W](cs1::W) writer structure"]
impl crate::Writable for CS1 {}
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "Message Buffer 1 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id1](id1) module"]
pub type ID1 = crate::Reg<u32, _ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID1;
#[doc = "`read()` method returns [id1::R](id1::R) reader structure"]
impl crate::Readable for ID1 {}
#[doc = "`write(|w| ..)` method takes [id1::W](id1::W) writer structure"]
impl crate::Writable for ID1 {}
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "Message Buffer 1 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word01](word01) module"]
pub type WORD01 = crate::Reg<u32, _WORD01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD01;
#[doc = "`read()` method returns [word01::R](word01::R) reader structure"]
impl crate::Readable for WORD01 {}
#[doc = "`write(|w| ..)` method takes [word01::W](word01::W) writer structure"]
impl crate::Writable for WORD01 {}
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "Message Buffer 1 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word11](word11) module"]
pub type WORD11 = crate::Reg<u32, _WORD11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD11;
#[doc = "`read()` method returns [word11::R](word11::R) reader structure"]
impl crate::Readable for WORD11 {}
#[doc = "`write(|w| ..)` method takes [word11::W](word11::W) writer structure"]
impl crate::Writable for WORD11 {}
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "Message Buffer 2 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs2](cs2) module"]
pub type CS2 = crate::Reg<u32, _CS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS2;
#[doc = "`read()` method returns [cs2::R](cs2::R) reader structure"]
impl crate::Readable for CS2 {}
#[doc = "`write(|w| ..)` method takes [cs2::W](cs2::W) writer structure"]
impl crate::Writable for CS2 {}
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "Message Buffer 2 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id2](id2) module"]
pub type ID2 = crate::Reg<u32, _ID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID2;
#[doc = "`read()` method returns [id2::R](id2::R) reader structure"]
impl crate::Readable for ID2 {}
#[doc = "`write(|w| ..)` method takes [id2::W](id2::W) writer structure"]
impl crate::Writable for ID2 {}
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "Message Buffer 2 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word02](word02) module"]
pub type WORD02 = crate::Reg<u32, _WORD02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD02;
#[doc = "`read()` method returns [word02::R](word02::R) reader structure"]
impl crate::Readable for WORD02 {}
#[doc = "`write(|w| ..)` method takes [word02::W](word02::W) writer structure"]
impl crate::Writable for WORD02 {}
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "Message Buffer 2 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word12](word12) module"]
pub type WORD12 = crate::Reg<u32, _WORD12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD12;
#[doc = "`read()` method returns [word12::R](word12::R) reader structure"]
impl crate::Readable for WORD12 {}
#[doc = "`write(|w| ..)` method takes [word12::W](word12::W) writer structure"]
impl crate::Writable for WORD12 {}
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "Message Buffer 3 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs3](cs3) module"]
pub type CS3 = crate::Reg<u32, _CS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS3;
#[doc = "`read()` method returns [cs3::R](cs3::R) reader structure"]
impl crate::Readable for CS3 {}
#[doc = "`write(|w| ..)` method takes [cs3::W](cs3::W) writer structure"]
impl crate::Writable for CS3 {}
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "Message Buffer 3 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id3](id3) module"]
pub type ID3 = crate::Reg<u32, _ID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID3;
#[doc = "`read()` method returns [id3::R](id3::R) reader structure"]
impl crate::Readable for ID3 {}
#[doc = "`write(|w| ..)` method takes [id3::W](id3::W) writer structure"]
impl crate::Writable for ID3 {}
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "Message Buffer 3 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word03](word03) module"]
pub type WORD03 = crate::Reg<u32, _WORD03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD03;
#[doc = "`read()` method returns [word03::R](word03::R) reader structure"]
impl crate::Readable for WORD03 {}
#[doc = "`write(|w| ..)` method takes [word03::W](word03::W) writer structure"]
impl crate::Writable for WORD03 {}
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "Message Buffer 3 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word13](word13) module"]
pub type WORD13 = crate::Reg<u32, _WORD13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD13;
#[doc = "`read()` method returns [word13::R](word13::R) reader structure"]
impl crate::Readable for WORD13 {}
#[doc = "`write(|w| ..)` method takes [word13::W](word13::W) writer structure"]
impl crate::Writable for WORD13 {}
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "Message Buffer 4 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs4](cs4) module"]
pub type CS4 = crate::Reg<u32, _CS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS4;
#[doc = "`read()` method returns [cs4::R](cs4::R) reader structure"]
impl crate::Readable for CS4 {}
#[doc = "`write(|w| ..)` method takes [cs4::W](cs4::W) writer structure"]
impl crate::Writable for CS4 {}
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "Message Buffer 4 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id4](id4) module"]
pub type ID4 = crate::Reg<u32, _ID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID4;
#[doc = "`read()` method returns [id4::R](id4::R) reader structure"]
impl crate::Readable for ID4 {}
#[doc = "`write(|w| ..)` method takes [id4::W](id4::W) writer structure"]
impl crate::Writable for ID4 {}
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "Message Buffer 4 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word04](word04) module"]
pub type WORD04 = crate::Reg<u32, _WORD04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD04;
#[doc = "`read()` method returns [word04::R](word04::R) reader structure"]
impl crate::Readable for WORD04 {}
#[doc = "`write(|w| ..)` method takes [word04::W](word04::W) writer structure"]
impl crate::Writable for WORD04 {}
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "Message Buffer 4 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word14](word14) module"]
pub type WORD14 = crate::Reg<u32, _WORD14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD14;
#[doc = "`read()` method returns [word14::R](word14::R) reader structure"]
impl crate::Readable for WORD14 {}
#[doc = "`write(|w| ..)` method takes [word14::W](word14::W) writer structure"]
impl crate::Writable for WORD14 {}
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "Message Buffer 5 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs5](cs5) module"]
pub type CS5 = crate::Reg<u32, _CS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS5;
#[doc = "`read()` method returns [cs5::R](cs5::R) reader structure"]
impl crate::Readable for CS5 {}
#[doc = "`write(|w| ..)` method takes [cs5::W](cs5::W) writer structure"]
impl crate::Writable for CS5 {}
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "Message Buffer 5 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id5](id5) module"]
pub type ID5 = crate::Reg<u32, _ID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID5;
#[doc = "`read()` method returns [id5::R](id5::R) reader structure"]
impl crate::Readable for ID5 {}
#[doc = "`write(|w| ..)` method takes [id5::W](id5::W) writer structure"]
impl crate::Writable for ID5 {}
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "Message Buffer 5 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word05](word05) module"]
pub type WORD05 = crate::Reg<u32, _WORD05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD05;
#[doc = "`read()` method returns [word05::R](word05::R) reader structure"]
impl crate::Readable for WORD05 {}
#[doc = "`write(|w| ..)` method takes [word05::W](word05::W) writer structure"]
impl crate::Writable for WORD05 {}
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "Message Buffer 5 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word15](word15) module"]
pub type WORD15 = crate::Reg<u32, _WORD15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD15;
#[doc = "`read()` method returns [word15::R](word15::R) reader structure"]
impl crate::Readable for WORD15 {}
#[doc = "`write(|w| ..)` method takes [word15::W](word15::W) writer structure"]
impl crate::Writable for WORD15 {}
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "Message Buffer 6 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs6](cs6) module"]
pub type CS6 = crate::Reg<u32, _CS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS6;
#[doc = "`read()` method returns [cs6::R](cs6::R) reader structure"]
impl crate::Readable for CS6 {}
#[doc = "`write(|w| ..)` method takes [cs6::W](cs6::W) writer structure"]
impl crate::Writable for CS6 {}
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "Message Buffer 6 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id6](id6) module"]
pub type ID6 = crate::Reg<u32, _ID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID6;
#[doc = "`read()` method returns [id6::R](id6::R) reader structure"]
impl crate::Readable for ID6 {}
#[doc = "`write(|w| ..)` method takes [id6::W](id6::W) writer structure"]
impl crate::Writable for ID6 {}
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "Message Buffer 6 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word06](word06) module"]
pub type WORD06 = crate::Reg<u32, _WORD06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD06;
#[doc = "`read()` method returns [word06::R](word06::R) reader structure"]
impl crate::Readable for WORD06 {}
#[doc = "`write(|w| ..)` method takes [word06::W](word06::W) writer structure"]
impl crate::Writable for WORD06 {}
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "Message Buffer 6 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word16](word16) module"]
pub type WORD16 = crate::Reg<u32, _WORD16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD16;
#[doc = "`read()` method returns [word16::R](word16::R) reader structure"]
impl crate::Readable for WORD16 {}
#[doc = "`write(|w| ..)` method takes [word16::W](word16::W) writer structure"]
impl crate::Writable for WORD16 {}
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "Message Buffer 7 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs7](cs7) module"]
pub type CS7 = crate::Reg<u32, _CS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS7;
#[doc = "`read()` method returns [cs7::R](cs7::R) reader structure"]
impl crate::Readable for CS7 {}
#[doc = "`write(|w| ..)` method takes [cs7::W](cs7::W) writer structure"]
impl crate::Writable for CS7 {}
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "Message Buffer 7 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id7](id7) module"]
pub type ID7 = crate::Reg<u32, _ID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID7;
#[doc = "`read()` method returns [id7::R](id7::R) reader structure"]
impl crate::Readable for ID7 {}
#[doc = "`write(|w| ..)` method takes [id7::W](id7::W) writer structure"]
impl crate::Writable for ID7 {}
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "Message Buffer 7 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word07](word07) module"]
pub type WORD07 = crate::Reg<u32, _WORD07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD07;
#[doc = "`read()` method returns [word07::R](word07::R) reader structure"]
impl crate::Readable for WORD07 {}
#[doc = "`write(|w| ..)` method takes [word07::W](word07::W) writer structure"]
impl crate::Writable for WORD07 {}
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "Message Buffer 7 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word17](word17) module"]
pub type WORD17 = crate::Reg<u32, _WORD17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD17;
#[doc = "`read()` method returns [word17::R](word17::R) reader structure"]
impl crate::Readable for WORD17 {}
#[doc = "`write(|w| ..)` method takes [word17::W](word17::W) writer structure"]
impl crate::Writable for WORD17 {}
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "Message Buffer 8 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs8](cs8) module"]
pub type CS8 = crate::Reg<u32, _CS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS8;
#[doc = "`read()` method returns [cs8::R](cs8::R) reader structure"]
impl crate::Readable for CS8 {}
#[doc = "`write(|w| ..)` method takes [cs8::W](cs8::W) writer structure"]
impl crate::Writable for CS8 {}
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "Message Buffer 8 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id8](id8) module"]
pub type ID8 = crate::Reg<u32, _ID8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID8;
#[doc = "`read()` method returns [id8::R](id8::R) reader structure"]
impl crate::Readable for ID8 {}
#[doc = "`write(|w| ..)` method takes [id8::W](id8::W) writer structure"]
impl crate::Writable for ID8 {}
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "Message Buffer 8 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word08](word08) module"]
pub type WORD08 = crate::Reg<u32, _WORD08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD08;
#[doc = "`read()` method returns [word08::R](word08::R) reader structure"]
impl crate::Readable for WORD08 {}
#[doc = "`write(|w| ..)` method takes [word08::W](word08::W) writer structure"]
impl crate::Writable for WORD08 {}
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "Message Buffer 8 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word18](word18) module"]
pub type WORD18 = crate::Reg<u32, _WORD18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD18;
#[doc = "`read()` method returns [word18::R](word18::R) reader structure"]
impl crate::Readable for WORD18 {}
#[doc = "`write(|w| ..)` method takes [word18::W](word18::W) writer structure"]
impl crate::Writable for WORD18 {}
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "Message Buffer 9 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs9](cs9) module"]
pub type CS9 = crate::Reg<u32, _CS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS9;
#[doc = "`read()` method returns [cs9::R](cs9::R) reader structure"]
impl crate::Readable for CS9 {}
#[doc = "`write(|w| ..)` method takes [cs9::W](cs9::W) writer structure"]
impl crate::Writable for CS9 {}
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "Message Buffer 9 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id9](id9) module"]
pub type ID9 = crate::Reg<u32, _ID9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID9;
#[doc = "`read()` method returns [id9::R](id9::R) reader structure"]
impl crate::Readable for ID9 {}
#[doc = "`write(|w| ..)` method takes [id9::W](id9::W) writer structure"]
impl crate::Writable for ID9 {}
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "Message Buffer 9 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word09](word09) module"]
pub type WORD09 = crate::Reg<u32, _WORD09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD09;
#[doc = "`read()` method returns [word09::R](word09::R) reader structure"]
impl crate::Readable for WORD09 {}
#[doc = "`write(|w| ..)` method takes [word09::W](word09::W) writer structure"]
impl crate::Writable for WORD09 {}
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "Message Buffer 9 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word19](word19) module"]
pub type WORD19 = crate::Reg<u32, _WORD19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD19;
#[doc = "`read()` method returns [word19::R](word19::R) reader structure"]
impl crate::Readable for WORD19 {}
#[doc = "`write(|w| ..)` method takes [word19::W](word19::W) writer structure"]
impl crate::Writable for WORD19 {}
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "Message Buffer 10 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs10](cs10) module"]
pub type CS10 = crate::Reg<u32, _CS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS10;
#[doc = "`read()` method returns [cs10::R](cs10::R) reader structure"]
impl crate::Readable for CS10 {}
#[doc = "`write(|w| ..)` method takes [cs10::W](cs10::W) writer structure"]
impl crate::Writable for CS10 {}
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "Message Buffer 10 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id10](id10) module"]
pub type ID10 = crate::Reg<u32, _ID10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID10;
#[doc = "`read()` method returns [id10::R](id10::R) reader structure"]
impl crate::Readable for ID10 {}
#[doc = "`write(|w| ..)` method takes [id10::W](id10::W) writer structure"]
impl crate::Writable for ID10 {}
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "Message Buffer 10 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word010](word010) module"]
pub type WORD010 = crate::Reg<u32, _WORD010>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD010;
#[doc = "`read()` method returns [word010::R](word010::R) reader structure"]
impl crate::Readable for WORD010 {}
#[doc = "`write(|w| ..)` method takes [word010::W](word010::W) writer structure"]
impl crate::Writable for WORD010 {}
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "Message Buffer 10 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word110](word110) module"]
pub type WORD110 = crate::Reg<u32, _WORD110>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD110;
#[doc = "`read()` method returns [word110::R](word110::R) reader structure"]
impl crate::Readable for WORD110 {}
#[doc = "`write(|w| ..)` method takes [word110::W](word110::W) writer structure"]
impl crate::Writable for WORD110 {}
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "Message Buffer 11 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs11](cs11) module"]
pub type CS11 = crate::Reg<u32, _CS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS11;
#[doc = "`read()` method returns [cs11::R](cs11::R) reader structure"]
impl crate::Readable for CS11 {}
#[doc = "`write(|w| ..)` method takes [cs11::W](cs11::W) writer structure"]
impl crate::Writable for CS11 {}
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "Message Buffer 11 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id11](id11) module"]
pub type ID11 = crate::Reg<u32, _ID11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID11;
#[doc = "`read()` method returns [id11::R](id11::R) reader structure"]
impl crate::Readable for ID11 {}
#[doc = "`write(|w| ..)` method takes [id11::W](id11::W) writer structure"]
impl crate::Writable for ID11 {}
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "Message Buffer 11 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word011](word011) module"]
pub type WORD011 = crate::Reg<u32, _WORD011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD011;
#[doc = "`read()` method returns [word011::R](word011::R) reader structure"]
impl crate::Readable for WORD011 {}
#[doc = "`write(|w| ..)` method takes [word011::W](word011::W) writer structure"]
impl crate::Writable for WORD011 {}
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "Message Buffer 11 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word111](word111) module"]
pub type WORD111 = crate::Reg<u32, _WORD111>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD111;
#[doc = "`read()` method returns [word111::R](word111::R) reader structure"]
impl crate::Readable for WORD111 {}
#[doc = "`write(|w| ..)` method takes [word111::W](word111::W) writer structure"]
impl crate::Writable for WORD111 {}
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "Message Buffer 12 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs12](cs12) module"]
pub type CS12 = crate::Reg<u32, _CS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS12;
#[doc = "`read()` method returns [cs12::R](cs12::R) reader structure"]
impl crate::Readable for CS12 {}
#[doc = "`write(|w| ..)` method takes [cs12::W](cs12::W) writer structure"]
impl crate::Writable for CS12 {}
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "Message Buffer 12 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id12](id12) module"]
pub type ID12 = crate::Reg<u32, _ID12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID12;
#[doc = "`read()` method returns [id12::R](id12::R) reader structure"]
impl crate::Readable for ID12 {}
#[doc = "`write(|w| ..)` method takes [id12::W](id12::W) writer structure"]
impl crate::Writable for ID12 {}
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "Message Buffer 12 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word012](word012) module"]
pub type WORD012 = crate::Reg<u32, _WORD012>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD012;
#[doc = "`read()` method returns [word012::R](word012::R) reader structure"]
impl crate::Readable for WORD012 {}
#[doc = "`write(|w| ..)` method takes [word012::W](word012::W) writer structure"]
impl crate::Writable for WORD012 {}
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "Message Buffer 12 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word112](word112) module"]
pub type WORD112 = crate::Reg<u32, _WORD112>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD112;
#[doc = "`read()` method returns [word112::R](word112::R) reader structure"]
impl crate::Readable for WORD112 {}
#[doc = "`write(|w| ..)` method takes [word112::W](word112::W) writer structure"]
impl crate::Writable for WORD112 {}
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "Message Buffer 13 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs13](cs13) module"]
pub type CS13 = crate::Reg<u32, _CS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS13;
#[doc = "`read()` method returns [cs13::R](cs13::R) reader structure"]
impl crate::Readable for CS13 {}
#[doc = "`write(|w| ..)` method takes [cs13::W](cs13::W) writer structure"]
impl crate::Writable for CS13 {}
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "Message Buffer 13 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id13](id13) module"]
pub type ID13 = crate::Reg<u32, _ID13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID13;
#[doc = "`read()` method returns [id13::R](id13::R) reader structure"]
impl crate::Readable for ID13 {}
#[doc = "`write(|w| ..)` method takes [id13::W](id13::W) writer structure"]
impl crate::Writable for ID13 {}
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "Message Buffer 13 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word013](word013) module"]
pub type WORD013 = crate::Reg<u32, _WORD013>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD013;
#[doc = "`read()` method returns [word013::R](word013::R) reader structure"]
impl crate::Readable for WORD013 {}
#[doc = "`write(|w| ..)` method takes [word013::W](word013::W) writer structure"]
impl crate::Writable for WORD013 {}
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "Message Buffer 13 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word113](word113) module"]
pub type WORD113 = crate::Reg<u32, _WORD113>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD113;
#[doc = "`read()` method returns [word113::R](word113::R) reader structure"]
impl crate::Readable for WORD113 {}
#[doc = "`write(|w| ..)` method takes [word113::W](word113::W) writer structure"]
impl crate::Writable for WORD113 {}
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "Message Buffer 14 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs14](cs14) module"]
pub type CS14 = crate::Reg<u32, _CS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS14;
#[doc = "`read()` method returns [cs14::R](cs14::R) reader structure"]
impl crate::Readable for CS14 {}
#[doc = "`write(|w| ..)` method takes [cs14::W](cs14::W) writer structure"]
impl crate::Writable for CS14 {}
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "Message Buffer 14 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id14](id14) module"]
pub type ID14 = crate::Reg<u32, _ID14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID14;
#[doc = "`read()` method returns [id14::R](id14::R) reader structure"]
impl crate::Readable for ID14 {}
#[doc = "`write(|w| ..)` method takes [id14::W](id14::W) writer structure"]
impl crate::Writable for ID14 {}
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "Message Buffer 14 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word014](word014) module"]
pub type WORD014 = crate::Reg<u32, _WORD014>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD014;
#[doc = "`read()` method returns [word014::R](word014::R) reader structure"]
impl crate::Readable for WORD014 {}
#[doc = "`write(|w| ..)` method takes [word014::W](word014::W) writer structure"]
impl crate::Writable for WORD014 {}
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "Message Buffer 14 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word114](word114) module"]
pub type WORD114 = crate::Reg<u32, _WORD114>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD114;
#[doc = "`read()` method returns [word114::R](word114::R) reader structure"]
impl crate::Readable for WORD114 {}
#[doc = "`write(|w| ..)` method takes [word114::W](word114::W) writer structure"]
impl crate::Writable for WORD114 {}
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "Message Buffer 15 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs15](cs15) module"]
pub type CS15 = crate::Reg<u32, _CS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS15;
#[doc = "`read()` method returns [cs15::R](cs15::R) reader structure"]
impl crate::Readable for CS15 {}
#[doc = "`write(|w| ..)` method takes [cs15::W](cs15::W) writer structure"]
impl crate::Writable for CS15 {}
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "Message Buffer 15 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id15](id15) module"]
pub type ID15 = crate::Reg<u32, _ID15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID15;
#[doc = "`read()` method returns [id15::R](id15::R) reader structure"]
impl crate::Readable for ID15 {}
#[doc = "`write(|w| ..)` method takes [id15::W](id15::W) writer structure"]
impl crate::Writable for ID15 {}
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "Message Buffer 15 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word015](word015) module"]
pub type WORD015 = crate::Reg<u32, _WORD015>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD015;
#[doc = "`read()` method returns [word015::R](word015::R) reader structure"]
impl crate::Readable for WORD015 {}
#[doc = "`write(|w| ..)` method takes [word015::W](word015::W) writer structure"]
impl crate::Writable for WORD015 {}
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "Message Buffer 15 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word115](word115) module"]
pub type WORD115 = crate::Reg<u32, _WORD115>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD115;
#[doc = "`read()` method returns [word115::R](word115::R) reader structure"]
impl crate::Readable for WORD115 {}
#[doc = "`write(|w| ..)` method takes [word115::W](word115::W) writer structure"]
impl crate::Writable for WORD115 {}
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rximr](rximr) module"]
pub type RXIMR = crate::Reg<u32, _RXIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIMR;
#[doc = "`read()` method returns [rximr::R](rximr::R) reader structure"]
impl crate::Readable for RXIMR {}
#[doc = "`write(|w| ..)` method takes [rximr::W](rximr::W) writer structure"]
impl crate::Writable for RXIMR {}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
