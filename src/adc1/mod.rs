#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Registers 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC Status and Control Registers 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC Configuration Register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - ADC Configuration Register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10 - ADC Data Result Register"]
    pub ra: R,
    #[doc = "0x14 - ADC Data Result Register"]
    pub rb: R,
    #[doc = "0x18 - Compare Value Registers"]
    pub cv1: CV,
    #[doc = "0x1c - Compare Value Registers"]
    pub cv2: CV,
    #[doc = "0x20 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x24 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x28 - ADC Offset Correction Register"]
    pub ofs: OFS,
    #[doc = "0x2c - ADC Plus-Side Gain Register"]
    pub pg: PG,
    #[doc = "0x30 - ADC Minus-Side Gain Register"]
    pub mg: MG,
    #[doc = "0x34 - ADC Plus-Side General Calibration Value Register"]
    pub clpd: CLPD,
    #[doc = "0x38 - ADC Plus-Side General Calibration Value Register"]
    pub clps: CLPS,
    #[doc = "0x3c - ADC Plus-Side General Calibration Value Register"]
    pub clp4: CLP4,
    #[doc = "0x40 - ADC Plus-Side General Calibration Value Register"]
    pub clp3: CLP3,
    #[doc = "0x44 - ADC Plus-Side General Calibration Value Register"]
    pub clp2: CLP2,
    #[doc = "0x48 - ADC Plus-Side General Calibration Value Register"]
    pub clp1: CLP1,
    #[doc = "0x4c - ADC Plus-Side General Calibration Value Register"]
    pub clp0: CLP0,
    _reserved0: [u8; 4usize],
    #[doc = "0x54 - ADC Minus-Side General Calibration Value Register"]
    pub clmd: CLMD,
    #[doc = "0x58 - ADC Minus-Side General Calibration Value Register"]
    pub clms: CLMS,
    #[doc = "0x5c - ADC Minus-Side General Calibration Value Register"]
    pub clm4: CLM4,
    #[doc = "0x60 - ADC Minus-Side General Calibration Value Register"]
    pub clm3: CLM3,
    #[doc = "0x64 - ADC Minus-Side General Calibration Value Register"]
    pub clm2: CLM2,
    #[doc = "0x68 - ADC Minus-Side General Calibration Value Register"]
    pub clm1: CLM1,
    #[doc = "0x6c - ADC Minus-Side General Calibration Value Register"]
    pub clm0: CLM0,
}
#[doc = "ADC Status and Control Registers 1"]
pub struct SC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Status and Control Registers 1"]
pub mod sc1;
#[doc = "ADC Configuration Register 1"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "ADC Configuration Register 2"]
pub struct CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "ADC Data Result Register"]
pub struct R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Data Result Register"]
pub mod r;
#[doc = "Compare Value Registers"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "Status and Control Register 2"]
pub struct SC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "Status and Control Register 3"]
pub struct SC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "ADC Offset Correction Register"]
pub struct OFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "ADC Plus-Side Gain Register"]
pub struct PG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side Gain Register"]
pub mod pg;
#[doc = "ADC Minus-Side Gain Register"]
pub struct MG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side Gain Register"]
pub mod mg;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clpd;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clps;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp4;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp3;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp2;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp1;
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub struct CLP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp0;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clmd;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clms;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm4;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm3;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm2;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm1;
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub struct CLM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Minus-Side General Calibration Value Register"]
pub mod clm0;
