#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Control Register"]
    pub cr: CR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub iscr: ISCR,
    #[doc = "0x14 - ETB Counter Control register"]
    pub etbcc: ETBCC,
    #[doc = "0x18 - ETB Reload register"]
    pub etbrl: ETBRL,
    #[doc = "0x1c - ETB Counter Value register"]
    pub etbcnt: ETBCNT,
    #[doc = "0x20 - Fault address register"]
    pub fadr: FADR,
    #[doc = "0x24 - Fault attributes register"]
    pub fatr: FATR,
    #[doc = "0x28 - Fault data register"]
    pub fdr: FDR,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Process ID register"]
    pub pid: PID,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plasc](plasc) module"]
pub type PLASC = crate::Reg<u16, _PLASC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLASC;
#[doc = "`read()` method returns [plasc::R](plasc::R) reader structure"]
impl crate::Readable for PLASC {}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plamc](plamc) module"]
pub type PLAMC = crate::Reg<u16, _PLAMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLAMC;
#[doc = "`read()` method returns [plamc::R](plamc::R) reader structure"]
impl crate::Readable for PLAMC {}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iscr](iscr) module"]
pub type ISCR = crate::Reg<u32, _ISCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISCR;
#[doc = "`read()` method returns [iscr::R](iscr::R) reader structure"]
impl crate::Readable for ISCR {}
#[doc = "`write(|w| ..)` method takes [iscr::W](iscr::W) writer structure"]
impl crate::Writable for ISCR {}
#[doc = "Interrupt Status Register"]
pub mod iscr;
#[doc = "ETB Counter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etbcc](etbcc) module"]
pub type ETBCC = crate::Reg<u32, _ETBCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETBCC;
#[doc = "`read()` method returns [etbcc::R](etbcc::R) reader structure"]
impl crate::Readable for ETBCC {}
#[doc = "`write(|w| ..)` method takes [etbcc::W](etbcc::W) writer structure"]
impl crate::Writable for ETBCC {}
#[doc = "ETB Counter Control register"]
pub mod etbcc;
#[doc = "ETB Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etbrl](etbrl) module"]
pub type ETBRL = crate::Reg<u32, _ETBRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETBRL;
#[doc = "`read()` method returns [etbrl::R](etbrl::R) reader structure"]
impl crate::Readable for ETBRL {}
#[doc = "`write(|w| ..)` method takes [etbrl::W](etbrl::W) writer structure"]
impl crate::Writable for ETBRL {}
#[doc = "ETB Reload register"]
pub mod etbrl;
#[doc = "ETB Counter Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [etbcnt](etbcnt) module"]
pub type ETBCNT = crate::Reg<u32, _ETBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETBCNT;
#[doc = "`read()` method returns [etbcnt::R](etbcnt::R) reader structure"]
impl crate::Readable for ETBCNT {}
#[doc = "ETB Counter Value register"]
pub mod etbcnt;
#[doc = "Fault address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fadr](fadr) module"]
pub type FADR = crate::Reg<u32, _FADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADR;
#[doc = "`read()` method returns [fadr::R](fadr::R) reader structure"]
impl crate::Readable for FADR {}
#[doc = "Fault address register"]
pub mod fadr;
#[doc = "Fault attributes register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fatr](fatr) module"]
pub type FATR = crate::Reg<u32, _FATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FATR;
#[doc = "`read()` method returns [fatr::R](fatr::R) reader structure"]
impl crate::Readable for FATR {}
#[doc = "Fault attributes register"]
pub mod fatr;
#[doc = "Fault data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "Fault data register"]
pub mod fdr;
#[doc = "Process ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "`write(|w| ..)` method takes [pid::W](pid::W) writer structure"]
impl crate::Writable for PID {}
#[doc = "Process ID register"]
pub mod pid;
#[doc = "Compute Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpo](cpo) module"]
pub type CPO = crate::Reg<u32, _CPO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPO;
#[doc = "`read()` method returns [cpo::R](cpo::R) reader structure"]
impl crate::Readable for CPO {}
#[doc = "`write(|w| ..)` method takes [cpo::W](cpo::W) writer structure"]
impl crate::Writable for CPO {}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
