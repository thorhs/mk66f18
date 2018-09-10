#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Pin Enable 5 register"]
    pub pe5: PE5,
    #[doc = "0x05 - LLWU Pin Enable 6 register"]
    pub pe6: PE6,
    #[doc = "0x06 - LLWU Pin Enable 7 register"]
    pub pe7: PE7,
    #[doc = "0x07 - LLWU Pin Enable 8 register"]
    pub pe8: PE8,
    #[doc = "0x08 - LLWU Module Enable register"]
    pub me: ME,
    #[doc = "0x09 - LLWU Pin Flag 1 register"]
    pub pf1: PF1,
    #[doc = "0x0a - LLWU Pin Flag 2 register"]
    pub pf2: PF2,
    #[doc = "0x0b - LLWU Pin Flag 3 register"]
    pub pf3: PF3,
    #[doc = "0x0c - LLWU Pin Flag 4 register"]
    pub pf4: PF4,
    #[doc = "0x0d - LLWU Module Flag 5 register"]
    pub mf5: MF5,
    #[doc = "0x0e - LLWU Pin Filter 1 register"]
    pub filt1: FILT1,
    #[doc = "0x0f - LLWU Pin Filter 2 register"]
    pub filt2: FILT2,
    #[doc = "0x10 - LLWU Pin Filter 3 register"]
    pub filt3: FILT3,
    #[doc = "0x11 - LLWU Pin Filter 4 register"]
    pub filt4: FILT4,
}
#[doc = "LLWU Pin Enable 1 register"]
pub struct PE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "LLWU Pin Enable 2 register"]
pub struct PE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "LLWU Pin Enable 3 register"]
pub struct PE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "LLWU Pin Enable 4 register"]
pub struct PE4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "LLWU Pin Enable 5 register"]
pub struct PE5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 5 register"]
pub mod pe5;
#[doc = "LLWU Pin Enable 6 register"]
pub struct PE6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 6 register"]
pub mod pe6;
#[doc = "LLWU Pin Enable 7 register"]
pub struct PE7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 7 register"]
pub mod pe7;
#[doc = "LLWU Pin Enable 8 register"]
pub struct PE8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Enable 8 register"]
pub mod pe8;
#[doc = "LLWU Module Enable register"]
pub struct ME {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "LLWU Pin Flag 1 register"]
pub struct PF1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Flag 1 register"]
pub mod pf1;
#[doc = "LLWU Pin Flag 2 register"]
pub struct PF2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Flag 2 register"]
pub mod pf2;
#[doc = "LLWU Pin Flag 3 register"]
pub struct PF3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Flag 3 register"]
pub mod pf3;
#[doc = "LLWU Pin Flag 4 register"]
pub struct PF4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Flag 4 register"]
pub mod pf4;
#[doc = "LLWU Module Flag 5 register"]
pub struct MF5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Module Flag 5 register"]
pub mod mf5;
#[doc = "LLWU Pin Filter 1 register"]
pub struct FILT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "LLWU Pin Filter 2 register"]
pub struct FILT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
#[doc = "LLWU Pin Filter 3 register"]
pub struct FILT3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 3 register"]
pub mod filt3;
#[doc = "LLWU Pin Filter 4 register"]
pub struct FILT4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "LLWU Pin Filter 4 register"]
pub mod filt4;
