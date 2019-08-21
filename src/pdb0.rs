#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"]
    pub sc: SC,
    #[doc = "0x04 - Modulus register"]
    pub mod_: MOD,
    #[doc = "0x08 - Counter register"]
    pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay register"]
    pub idly: IDLY,
    #[doc = "0x10 - Channel n Control register 1"]
    pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status register"]
    pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 register"]
    pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 register"]
    pub ch0dly1: CHDLY1,
    _reserved8: [u8; 24usize],
    #[doc = "0x38 - Channel n Control register 1"]
    pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status register"]
    pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 register"]
    pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 register"]
    pub ch1dly1: CHDLY1,
    _reserved12: [u8; 264usize],
    #[doc = "0x150 - DAC Interval Trigger n Control register"]
    pub dacintc0: DACINTC,
    #[doc = "0x154 - DAC Interval n register"]
    pub dacint0: DACINT,
    #[doc = "0x158 - DAC Interval Trigger n Control register"]
    pub dacintc1: DACINTC,
    #[doc = "0x15c - DAC Interval n register"]
    pub dacint1: DACINT,
    _reserved16: [u8; 48usize],
    #[doc = "0x190 - Pulse-Out n Enable register"]
    pub poen: POEN,
    #[doc = "0x194 - Pulse-Out n Delay register"]
    pub podly: [PODLY; 4],
}
#[doc = "Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "Modulus register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Interrupt Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idly](idly) module"]
pub type IDLY = crate::Reg<u32, _IDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLY;
#[doc = "`read()` method returns [idly::R](idly::R) reader structure"]
impl crate::Readable for IDLY {}
#[doc = "`write(|w| ..)` method takes [idly::W](idly::W) writer structure"]
impl crate::Writable for IDLY {}
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "Channel n Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chc1](chc1) module"]
pub type CHC1 = crate::Reg<u32, _CHC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHC1;
#[doc = "`read()` method returns [chc1::R](chc1::R) reader structure"]
impl crate::Readable for CHC1 {}
#[doc = "`write(|w| ..)` method takes [chc1::W](chc1::W) writer structure"]
impl crate::Writable for CHC1 {}
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "Channel n Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chs](chs) module"]
pub type CHS = crate::Reg<u32, _CHS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHS;
#[doc = "`read()` method returns [chs::R](chs::R) reader structure"]
impl crate::Readable for CHS {}
#[doc = "`write(|w| ..)` method takes [chs::W](chs::W) writer structure"]
impl crate::Writable for CHS {}
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "Channel n Delay 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chdly0](chdly0) module"]
pub type CHDLY0 = crate::Reg<u32, _CHDLY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY0;
#[doc = "`read()` method returns [chdly0::R](chdly0::R) reader structure"]
impl crate::Readable for CHDLY0 {}
#[doc = "`write(|w| ..)` method takes [chdly0::W](chdly0::W) writer structure"]
impl crate::Writable for CHDLY0 {}
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "Channel n Delay 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chdly1](chdly1) module"]
pub type CHDLY1 = crate::Reg<u32, _CHDLY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY1;
#[doc = "`read()` method returns [chdly1::R](chdly1::R) reader structure"]
impl crate::Readable for CHDLY1 {}
#[doc = "`write(|w| ..)` method takes [chdly1::W](chdly1::W) writer structure"]
impl crate::Writable for CHDLY1 {}
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "DAC Interval Trigger n Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dacintc](dacintc) module"]
pub type DACINTC = crate::Reg<u32, _DACINTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACINTC;
#[doc = "`read()` method returns [dacintc::R](dacintc::R) reader structure"]
impl crate::Readable for DACINTC {}
#[doc = "`write(|w| ..)` method takes [dacintc::W](dacintc::W) writer structure"]
impl crate::Writable for DACINTC {}
#[doc = "DAC Interval Trigger n Control register"]
pub mod dacintc;
#[doc = "DAC Interval n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dacint](dacint) module"]
pub type DACINT = crate::Reg<u32, _DACINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACINT;
#[doc = "`read()` method returns [dacint::R](dacint::R) reader structure"]
impl crate::Readable for DACINT {}
#[doc = "`write(|w| ..)` method takes [dacint::W](dacint::W) writer structure"]
impl crate::Writable for DACINT {}
#[doc = "DAC Interval n register"]
pub mod dacint;
#[doc = "Pulse-Out n Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [poen](poen) module"]
pub type POEN = crate::Reg<u32, _POEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POEN;
#[doc = "`read()` method returns [poen::R](poen::R) reader structure"]
impl crate::Readable for POEN {}
#[doc = "`write(|w| ..)` method takes [poen::W](poen::W) writer structure"]
impl crate::Writable for POEN {}
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "Pulse-Out n Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [podly](podly) module"]
pub type PODLY = crate::Reg<u32, _PODLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PODLY;
#[doc = "`read()` method returns [podly::R](podly::R) reader structure"]
impl crate::Readable for PODLY {}
#[doc = "`write(|w| ..)` method takes [podly::W](podly::W) writer structure"]
impl crate::Writable for PODLY {}
#[doc = "Pulse-Out n Delay register"]
pub mod podly;
