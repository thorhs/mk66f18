#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: FCCOB,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: FCCOB,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: FCCOB,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: FCCOB,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: FCCOB,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: FCCOB,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: FCCOB,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: FCCOB,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: FCCOB,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: FCCOB,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: FCCOB,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: FCCOB,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: FPROT,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: FPROT,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: FPROT,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: FPROT,
    _reserved0: [u8; 2usize],
    #[doc = "0x16 - EEPROM Protection Register"]
    pub feprot: FEPROT,
    #[doc = "0x17 - Data Flash Protection Register"]
    pub fdprot: FDPROT,
    #[doc = "0x18 - Execute-only Access Registers"]
    pub xacch3: XACC,
    #[doc = "0x19 - Execute-only Access Registers"]
    pub xacch2: XACC,
    #[doc = "0x1a - Execute-only Access Registers"]
    pub xacch1: XACC,
    #[doc = "0x1b - Execute-only Access Registers"]
    pub xacch0: XACC,
    #[doc = "0x1c - Execute-only Access Registers"]
    pub xaccl3: XACC,
    #[doc = "0x1d - Execute-only Access Registers"]
    pub xaccl2: XACC,
    #[doc = "0x1e - Execute-only Access Registers"]
    pub xaccl1: XACC,
    #[doc = "0x1f - Execute-only Access Registers"]
    pub xaccl0: XACC,
    #[doc = "0x20 - Supervisor-only Access Registers"]
    pub sacch3: SACC,
    #[doc = "0x21 - Supervisor-only Access Registers"]
    pub sacch2: SACC,
    #[doc = "0x22 - Supervisor-only Access Registers"]
    pub sacch1: SACC,
    #[doc = "0x23 - Supervisor-only Access Registers"]
    pub sacch0: SACC,
    #[doc = "0x24 - Supervisor-only Access Registers"]
    pub saccl3: SACC,
    #[doc = "0x25 - Supervisor-only Access Registers"]
    pub saccl2: SACC,
    #[doc = "0x26 - Supervisor-only Access Registers"]
    pub saccl1: SACC,
    #[doc = "0x27 - Supervisor-only Access Registers"]
    pub saccl0: SACC,
    #[doc = "0x28 - Flash Access Segment Size Register"]
    pub facss: FACSS,
    _reserved1: [u8; 2usize],
    #[doc = "0x2b - Flash Access Segment Number Register"]
    pub facsn: FACSN,
}
#[doc = "Flash Status Register"]
pub struct FSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register"]
pub struct FCNFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Security Register"]
pub struct FSEC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Option Register"]
pub struct FOPT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "Program Flash Protection Registers"]
pub struct FPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
#[doc = "EEPROM Protection Register"]
pub struct FEPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "Data Flash Protection Register"]
pub struct FDPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Data Flash Protection Register"]
pub mod fdprot;
#[doc = "Execute-only Access Registers"]
pub struct XACC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Execute-only Access Registers"]
pub mod xacc;
#[doc = "Supervisor-only Access Registers"]
pub struct SACC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Supervisor-only Access Registers"]
pub mod sacc;
#[doc = "Flash Access Segment Size Register"]
pub struct FACSS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Access Segment Size Register"]
pub mod facss;
#[doc = "Flash Access Segment Number Register"]
pub struct FACSN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Flash Access Segment Number Register"]
pub mod facsn;
