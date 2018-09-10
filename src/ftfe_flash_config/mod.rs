#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backdoor Comparison Key 3."]
    pub backkey3: BACKKEY3,
    #[doc = "0x01 - Backdoor Comparison Key 2."]
    pub backkey2: BACKKEY2,
    #[doc = "0x02 - Backdoor Comparison Key 1."]
    pub backkey1: BACKKEY1,
    #[doc = "0x03 - Backdoor Comparison Key 0."]
    pub backkey0: BACKKEY0,
    #[doc = "0x04 - Backdoor Comparison Key 7."]
    pub backkey7: BACKKEY7,
    #[doc = "0x05 - Backdoor Comparison Key 6."]
    pub backkey6: BACKKEY6,
    #[doc = "0x06 - Backdoor Comparison Key 5."]
    pub backkey5: BACKKEY5,
    #[doc = "0x07 - Backdoor Comparison Key 4."]
    pub backkey4: BACKKEY4,
    #[doc = "0x08 - Non-volatile P-Flash Protection 1 - Low Register"]
    pub fprot3: FPROT3,
    #[doc = "0x09 - Non-volatile P-Flash Protection 1 - High Register"]
    pub fprot2: FPROT2,
    #[doc = "0x0a - Non-volatile P-Flash Protection 0 - Low Register"]
    pub fprot1: FPROT1,
    #[doc = "0x0b - Non-volatile P-Flash Protection 0 - High Register"]
    pub fprot0: FPROT0,
    #[doc = "0x0c - Non-volatile Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x0d - Non-volatile Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x0e - Non-volatile EERAM Protection Register"]
    pub feprot: FEPROT,
    #[doc = "0x0f - Non-volatile D-Flash Protection Register"]
    pub fdprot: FDPROT,
}
#[doc = "Backdoor Comparison Key 3."]
pub struct BACKKEY3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 3."]
pub mod backkey3;
#[doc = "Backdoor Comparison Key 2."]
pub struct BACKKEY2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 2."]
pub mod backkey2;
#[doc = "Backdoor Comparison Key 1."]
pub struct BACKKEY1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 1."]
pub mod backkey1;
#[doc = "Backdoor Comparison Key 0."]
pub struct BACKKEY0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 0."]
pub mod backkey0;
#[doc = "Backdoor Comparison Key 7."]
pub struct BACKKEY7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 7."]
pub mod backkey7;
#[doc = "Backdoor Comparison Key 6."]
pub struct BACKKEY6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 6."]
pub mod backkey6;
#[doc = "Backdoor Comparison Key 5."]
pub struct BACKKEY5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 5."]
pub mod backkey5;
#[doc = "Backdoor Comparison Key 4."]
pub struct BACKKEY4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backdoor Comparison Key 4."]
pub mod backkey4;
#[doc = "Non-volatile P-Flash Protection 1 - Low Register"]
pub struct FPROT3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile P-Flash Protection 1 - Low Register"]
pub mod fprot3;
#[doc = "Non-volatile P-Flash Protection 1 - High Register"]
pub struct FPROT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile P-Flash Protection 1 - High Register"]
pub mod fprot2;
#[doc = "Non-volatile P-Flash Protection 0 - Low Register"]
pub struct FPROT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile P-Flash Protection 0 - Low Register"]
pub mod fprot1;
#[doc = "Non-volatile P-Flash Protection 0 - High Register"]
pub struct FPROT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile P-Flash Protection 0 - High Register"]
pub mod fprot0;
#[doc = "Non-volatile Flash Security Register"]
pub struct FSEC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile Flash Security Register"]
pub mod fsec;
#[doc = "Non-volatile Flash Option Register"]
pub struct FOPT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile Flash Option Register"]
pub mod fopt;
#[doc = "Non-volatile EERAM Protection Register"]
pub struct FEPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile EERAM Protection Register"]
pub mod feprot;
#[doc = "Non-volatile D-Flash Protection Register"]
pub struct FDPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-volatile D-Flash Protection Register"]
pub mod fdprot;
