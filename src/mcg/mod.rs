#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCG Control 1 Register"]
    pub c1: C1,
    #[doc = "0x01 - MCG Control 2 Register"]
    pub c2: C2,
    #[doc = "0x02 - MCG Control 3 Register"]
    pub c3: C3,
    #[doc = "0x03 - MCG Control 4 Register"]
    pub c4: C4,
    #[doc = "0x04 - MCG Control 5 Register"]
    pub c5: C5,
    #[doc = "0x05 - MCG Control 6 Register"]
    pub c6: C6,
    #[doc = "0x06 - MCG Status Register"]
    pub s: S,
    _reserved0: [u8; 1usize],
    #[doc = "0x08 - MCG Status and Control Register"]
    pub sc: SC,
    _reserved1: [u8; 1usize],
    #[doc = "0x0a - MCG Auto Trim Compare Value High Register"]
    pub atcvh: ATCVH,
    #[doc = "0x0b - MCG Auto Trim Compare Value Low Register"]
    pub atcvl: ATCVL,
    #[doc = "0x0c - MCG Control 7 Register"]
    pub c7: C7,
    #[doc = "0x0d - MCG Control 8 Register"]
    pub c8: C8,
    #[doc = "0x0e - MCG Control 9 Register"]
    pub c9: C9,
    _reserved2: [u8; 1usize],
    #[doc = "0x10 - MCG Control 11 Register"]
    pub c11: C11,
    _reserved3: [u8; 1usize],
    #[doc = "0x12 - MCG Status 2 Register"]
    pub s2: S2,
}
#[doc = "MCG Control 1 Register"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 1 Register"]
pub mod c1;
#[doc = "MCG Control 2 Register"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 2 Register"]
pub mod c2;
#[doc = "MCG Control 3 Register"]
pub struct C3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 3 Register"]
pub mod c3;
#[doc = "MCG Control 4 Register"]
pub struct C4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 4 Register"]
pub mod c4;
#[doc = "MCG Control 5 Register"]
pub struct C5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 5 Register"]
pub mod c5;
#[doc = "MCG Control 6 Register"]
pub struct C6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 6 Register"]
pub mod c6;
#[doc = "MCG Status Register"]
pub struct S {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Status Register"]
pub mod s;
#[doc = "MCG Status and Control Register"]
pub struct SC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Status and Control Register"]
pub mod sc;
#[doc = "MCG Auto Trim Compare Value High Register"]
pub struct ATCVH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Auto Trim Compare Value High Register"]
pub mod atcvh;
#[doc = "MCG Auto Trim Compare Value Low Register"]
pub struct ATCVL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Auto Trim Compare Value Low Register"]
pub mod atcvl;
#[doc = "MCG Control 7 Register"]
pub struct C7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 7 Register"]
pub mod c7;
#[doc = "MCG Control 8 Register"]
pub struct C8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 8 Register"]
pub mod c8;
#[doc = "MCG Control 9 Register"]
pub struct C9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 9 Register"]
pub mod c9;
#[doc = "MCG Control 11 Register"]
pub struct C11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Control 11 Register"]
pub mod c11;
#[doc = "MCG Status 2 Register"]
pub struct S2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MCG Status 2 Register"]
pub mod s2;
