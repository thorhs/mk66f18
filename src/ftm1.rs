#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: CSC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: CV,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: CSC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: CV,
    _reserved7: [u8; 48usize],
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: CNTIN,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: STATUS,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: MODE,
    #[doc = "0x58 - Synchronization"]
    pub sync: SYNC,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: OUTINIT,
    #[doc = "0x60 - Output Mask"]
    pub outmask: OUTMASK,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: COMBINE,
    #[doc = "0x68 - Deadtime Insertion Control"]
    pub deadtime: DEADTIME,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: EXTTRIG,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: POL,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: FMS,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: FILTER,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: FLTCTRL,
    #[doc = "0x80 - Quadrature Decoder Control And Status"]
    pub qdctrl: QDCTRL,
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: FLTPOL,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: SYNCONF,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: INVCTRL,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: SWOCTRL,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: PWMLOAD,
}
#[doc = "Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Status And Control"]
pub mod sc;
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csc](csc) module"]
pub type CSC = crate::Reg<u32, _CSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSC;
#[doc = "`read()` method returns [csc::R](csc::R) reader structure"]
impl crate::Readable for CSC {}
#[doc = "`write(|w| ..)` method takes [csc::W](csc::W) writer structure"]
impl crate::Writable for CSC {}
#[doc = "Channel (n) Status And Control"]
pub mod csc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "Counter Initial Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cntin](cntin) module"]
pub type CNTIN = crate::Reg<u32, _CNTIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTIN;
#[doc = "`read()` method returns [cntin::R](cntin::R) reader structure"]
impl crate::Readable for CNTIN {}
#[doc = "`write(|w| ..)` method takes [cntin::W](cntin::W) writer structure"]
impl crate::Writable for CNTIN {}
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "Capture And Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "Features Mode Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "Synchronization"]
pub mod sync;
#[doc = "Initial State For Channels Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outinit](outinit) module"]
pub type OUTINIT = crate::Reg<u32, _OUTINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTINIT;
#[doc = "`read()` method returns [outinit::R](outinit::R) reader structure"]
impl crate::Readable for OUTINIT {}
#[doc = "`write(|w| ..)` method takes [outinit::W](outinit::W) writer structure"]
impl crate::Writable for OUTINIT {}
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "Output Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outmask](outmask) module"]
pub type OUTMASK = crate::Reg<u32, _OUTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTMASK;
#[doc = "`read()` method returns [outmask::R](outmask::R) reader structure"]
impl crate::Readable for OUTMASK {}
#[doc = "`write(|w| ..)` method takes [outmask::W](outmask::W) writer structure"]
impl crate::Writable for OUTMASK {}
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "Function For Linked Channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [combine](combine) module"]
pub type COMBINE = crate::Reg<u32, _COMBINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBINE;
#[doc = "`read()` method returns [combine::R](combine::R) reader structure"]
impl crate::Readable for COMBINE {}
#[doc = "`write(|w| ..)` method takes [combine::W](combine::W) writer structure"]
impl crate::Writable for COMBINE {}
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "Deadtime Insertion Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deadtime](deadtime) module"]
pub type DEADTIME = crate::Reg<u32, _DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEADTIME;
#[doc = "`read()` method returns [deadtime::R](deadtime::R) reader structure"]
impl crate::Readable for DEADTIME {}
#[doc = "`write(|w| ..)` method takes [deadtime::W](deadtime::W) writer structure"]
impl crate::Writable for DEADTIME {}
#[doc = "Deadtime Insertion Control"]
pub mod deadtime;
#[doc = "FTM External Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [exttrig](exttrig) module"]
pub type EXTTRIG = crate::Reg<u32, _EXTTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTTRIG;
#[doc = "`read()` method returns [exttrig::R](exttrig::R) reader structure"]
impl crate::Readable for EXTTRIG {}
#[doc = "`write(|w| ..)` method takes [exttrig::W](exttrig::W) writer structure"]
impl crate::Writable for EXTTRIG {}
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "Channels Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pol](pol) module"]
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
#[doc = "`read()` method returns [pol::R](pol::R) reader structure"]
impl crate::Readable for POL {}
#[doc = "`write(|w| ..)` method takes [pol::W](pol::W) writer structure"]
impl crate::Writable for POL {}
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "Fault Mode Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fms](fms) module"]
pub type FMS = crate::Reg<u32, _FMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMS;
#[doc = "`read()` method returns [fms::R](fms::R) reader structure"]
impl crate::Readable for FMS {}
#[doc = "`write(|w| ..)` method takes [fms::W](fms::W) writer structure"]
impl crate::Writable for FMS {}
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "Input Capture Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filter](filter) module"]
pub type FILTER = crate::Reg<u32, _FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER;
#[doc = "`read()` method returns [filter::R](filter::R) reader structure"]
impl crate::Readable for FILTER {}
#[doc = "`write(|w| ..)` method takes [filter::W](filter::W) writer structure"]
impl crate::Writable for FILTER {}
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "Fault Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fltctrl](fltctrl) module"]
pub type FLTCTRL = crate::Reg<u32, _FLTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTCTRL;
#[doc = "`read()` method returns [fltctrl::R](fltctrl::R) reader structure"]
impl crate::Readable for FLTCTRL {}
#[doc = "`write(|w| ..)` method takes [fltctrl::W](fltctrl::W) writer structure"]
impl crate::Writable for FLTCTRL {}
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "Quadrature Decoder Control And Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdctrl](qdctrl) module"]
pub type QDCTRL = crate::Reg<u32, _QDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDCTRL;
#[doc = "`read()` method returns [qdctrl::R](qdctrl::R) reader structure"]
impl crate::Readable for QDCTRL {}
#[doc = "`write(|w| ..)` method takes [qdctrl::W](qdctrl::W) writer structure"]
impl crate::Writable for QDCTRL {}
#[doc = "Quadrature Decoder Control And Status"]
pub mod qdctrl;
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "Configuration"]
pub mod conf;
#[doc = "FTM Fault Input Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fltpol](fltpol) module"]
pub type FLTPOL = crate::Reg<u32, _FLTPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTPOL;
#[doc = "`read()` method returns [fltpol::R](fltpol::R) reader structure"]
impl crate::Readable for FLTPOL {}
#[doc = "`write(|w| ..)` method takes [fltpol::W](fltpol::W) writer structure"]
impl crate::Writable for FLTPOL {}
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "Synchronization Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [synconf](synconf) module"]
pub type SYNCONF = crate::Reg<u32, _SYNCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCONF;
#[doc = "`read()` method returns [synconf::R](synconf::R) reader structure"]
impl crate::Readable for SYNCONF {}
#[doc = "`write(|w| ..)` method takes [synconf::W](synconf::W) writer structure"]
impl crate::Writable for SYNCONF {}
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "FTM Inverting Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [invctrl](invctrl) module"]
pub type INVCTRL = crate::Reg<u32, _INVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INVCTRL;
#[doc = "`read()` method returns [invctrl::R](invctrl::R) reader structure"]
impl crate::Readable for INVCTRL {}
#[doc = "`write(|w| ..)` method takes [invctrl::W](invctrl::W) writer structure"]
impl crate::Writable for INVCTRL {}
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "FTM Software Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swoctrl](swoctrl) module"]
pub type SWOCTRL = crate::Reg<u32, _SWOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWOCTRL;
#[doc = "`read()` method returns [swoctrl::R](swoctrl::R) reader structure"]
impl crate::Readable for SWOCTRL {}
#[doc = "`write(|w| ..)` method takes [swoctrl::W](swoctrl::W) writer structure"]
impl crate::Writable for SWOCTRL {}
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "FTM PWM Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwmload](pwmload) module"]
pub type PWMLOAD = crate::Reg<u32, _PWMLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMLOAD;
#[doc = "`read()` method returns [pwmload::R](pwmload::R) reader structure"]
impl crate::Readable for PWMLOAD {}
#[doc = "`write(|w| ..)` method takes [pwmload::W](pwmload::W) writer structure"]
impl crate::Writable for PWMLOAD {}
#[doc = "FTM PWM Load"]
pub mod pwmload;
