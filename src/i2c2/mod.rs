#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Address Register 1"]
    pub a1: A1,
    #[doc = "0x01 - I2C Frequency Divider register"]
    pub f: F,
    #[doc = "0x02 - I2C Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - I2C Status register"]
    pub s: S,
    #[doc = "0x04 - I2C Data I/O register"]
    pub d: D,
    #[doc = "0x05 - I2C Control Register 2"]
    pub c2: C2,
    #[doc = "0x06 - I2C Programmable Input Glitch Filter Register"]
    pub flt: FLT,
    #[doc = "0x07 - I2C Range Address register"]
    pub ra: RA,
    #[doc = "0x08 - I2C SMBus Control and Status register"]
    pub smb: SMB,
    #[doc = "0x09 - I2C Address Register 2"]
    pub a2: A2,
    #[doc = "0x0a - I2C SCL Low Timeout Register High"]
    pub slth: SLTH,
    #[doc = "0x0b - I2C SCL Low Timeout Register Low"]
    pub sltl: SLTL,
}
#[doc = "I2C Address Register 1"]
pub struct A1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Address Register 1"]
pub mod a1;
#[doc = "I2C Frequency Divider register"]
pub struct F {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Frequency Divider register"]
pub mod f;
#[doc = "I2C Control Register 1"]
pub struct C1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Control Register 1"]
pub mod c1;
#[doc = "I2C Status register"]
pub struct S {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Status register"]
pub mod s;
#[doc = "I2C Data I/O register"]
pub struct D {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Data I/O register"]
pub mod d;
#[doc = "I2C Control Register 2"]
pub struct C2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Control Register 2"]
pub mod c2;
#[doc = "I2C Programmable Input Glitch Filter Register"]
pub struct FLT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Programmable Input Glitch Filter Register"]
pub mod flt;
#[doc = "I2C Range Address register"]
pub struct RA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Range Address register"]
pub mod ra;
#[doc = "I2C SMBus Control and Status register"]
pub struct SMB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C SMBus Control and Status register"]
pub mod smb;
#[doc = "I2C Address Register 2"]
pub struct A2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C Address Register 2"]
pub mod a2;
#[doc = "I2C SCL Low Timeout Register High"]
pub struct SLTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C SCL Low Timeout Register High"]
pub mod slth;
#[doc = "I2C SCL Low Timeout Register Low"]
pub struct SLTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C SCL Low Timeout Register Low"]
pub mod sltl;
