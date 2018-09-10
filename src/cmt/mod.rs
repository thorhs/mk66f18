#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMT Carrier Generator High Data Register 1"]
    pub cgh1: CGH1,
    #[doc = "0x01 - CMT Carrier Generator Low Data Register 1"]
    pub cgl1: CGL1,
    #[doc = "0x02 - CMT Carrier Generator High Data Register 2"]
    pub cgh2: CGH2,
    #[doc = "0x03 - CMT Carrier Generator Low Data Register 2"]
    pub cgl2: CGL2,
    #[doc = "0x04 - CMT Output Control Register"]
    pub oc: OC,
    #[doc = "0x05 - CMT Modulator Status and Control Register"]
    pub msc: MSC,
    #[doc = "0x06 - CMT Modulator Data Register Mark High"]
    pub cmd1: CMD1,
    #[doc = "0x07 - CMT Modulator Data Register Mark Low"]
    pub cmd2: CMD2,
    #[doc = "0x08 - CMT Modulator Data Register Space High"]
    pub cmd3: CMD3,
    #[doc = "0x09 - CMT Modulator Data Register Space Low"]
    pub cmd4: CMD4,
    #[doc = "0x0a - CMT Primary Prescaler Register"]
    pub pps: PPS,
    #[doc = "0x0b - CMT Direct Memory Access Register"]
    pub dma: DMA,
}
#[doc = "CMT Carrier Generator High Data Register 1"]
pub struct CGH1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Carrier Generator High Data Register 1"]
pub mod cgh1;
#[doc = "CMT Carrier Generator Low Data Register 1"]
pub struct CGL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Carrier Generator Low Data Register 1"]
pub mod cgl1;
#[doc = "CMT Carrier Generator High Data Register 2"]
pub struct CGH2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Carrier Generator High Data Register 2"]
pub mod cgh2;
#[doc = "CMT Carrier Generator Low Data Register 2"]
pub struct CGL2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Carrier Generator Low Data Register 2"]
pub mod cgl2;
#[doc = "CMT Output Control Register"]
pub struct OC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Output Control Register"]
pub mod oc;
#[doc = "CMT Modulator Status and Control Register"]
pub struct MSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Modulator Status and Control Register"]
pub mod msc;
#[doc = "CMT Modulator Data Register Mark High"]
pub struct CMD1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Modulator Data Register Mark High"]
pub mod cmd1;
#[doc = "CMT Modulator Data Register Mark Low"]
pub struct CMD2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Modulator Data Register Mark Low"]
pub mod cmd2;
#[doc = "CMT Modulator Data Register Space High"]
pub struct CMD3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Modulator Data Register Space High"]
pub mod cmd3;
#[doc = "CMT Modulator Data Register Space Low"]
pub struct CMD4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Modulator Data Register Space Low"]
pub mod cmd4;
#[doc = "CMT Primary Prescaler Register"]
pub struct PPS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Primary Prescaler Register"]
pub mod pps;
#[doc = "CMT Direct Memory Access Register"]
pub struct DMA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMT Direct Memory Access Register"]
pub mod dma;
