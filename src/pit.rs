#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved1: [u8; 220usize],
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    pub ltmr64h: LTMR64H,
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    pub ltmr64l: LTMR64L,
    _reserved3: [u8; 24usize],
    #[doc = "0x100 - Timer Load Value Register"]
    pub ldval0: LDVAL,
    #[doc = "0x104 - Current Timer Value Register"]
    pub cval0: CVAL,
    #[doc = "0x108 - Timer Control Register"]
    pub tctrl0: TCTRL,
    #[doc = "0x10c - Timer Flag Register"]
    pub tflg0: TFLG,
    #[doc = "0x110 - Timer Load Value Register"]
    pub ldval1: LDVAL,
    #[doc = "0x114 - Current Timer Value Register"]
    pub cval1: CVAL,
    #[doc = "0x118 - Timer Control Register"]
    pub tctrl1: TCTRL,
    #[doc = "0x11c - Timer Flag Register"]
    pub tflg1: TFLG,
    #[doc = "0x120 - Timer Load Value Register"]
    pub ldval2: LDVAL,
    #[doc = "0x124 - Current Timer Value Register"]
    pub cval2: CVAL,
    #[doc = "0x128 - Timer Control Register"]
    pub tctrl2: TCTRL,
    #[doc = "0x12c - Timer Flag Register"]
    pub tflg2: TFLG,
    #[doc = "0x130 - Timer Load Value Register"]
    pub ldval3: LDVAL,
    #[doc = "0x134 - Current Timer Value Register"]
    pub cval3: CVAL,
    #[doc = "0x138 - Timer Control Register"]
    pub tctrl3: TCTRL,
    #[doc = "0x13c - Timer Flag Register"]
    pub tflg3: TFLG,
}
#[doc = "PIT Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "PIT Upper Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ltmr64h](ltmr64h) module"]
pub type LTMR64H = crate::Reg<u32, _LTMR64H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTMR64H;
#[doc = "`read()` method returns [ltmr64h::R](ltmr64h::R) reader structure"]
impl crate::Readable for LTMR64H {}
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "PIT Lower Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ltmr64l](ltmr64l) module"]
pub type LTMR64L = crate::Reg<u32, _LTMR64L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTMR64L;
#[doc = "`read()` method returns [ltmr64l::R](ltmr64l::R) reader structure"]
impl crate::Readable for LTMR64L {}
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
#[doc = "Timer Load Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldval](ldval) module"]
pub type LDVAL = crate::Reg<u32, _LDVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDVAL;
#[doc = "`read()` method returns [ldval::R](ldval::R) reader structure"]
impl crate::Readable for LDVAL {}
#[doc = "`write(|w| ..)` method takes [ldval::W](ldval::W) writer structure"]
impl crate::Writable for LDVAL {}
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "Current Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cval](cval) module"]
pub type CVAL = crate::Reg<u32, _CVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL;
#[doc = "`read()` method returns [cval::R](cval::R) reader structure"]
impl crate::Readable for CVAL {}
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tctrl](tctrl) module"]
pub type TCTRL = crate::Reg<u32, _TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL;
#[doc = "`read()` method returns [tctrl::R](tctrl::R) reader structure"]
impl crate::Readable for TCTRL {}
#[doc = "`write(|w| ..)` method takes [tctrl::W](tctrl::W) writer structure"]
impl crate::Writable for TCTRL {}
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "Timer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tflg](tflg) module"]
pub type TFLG = crate::Reg<u32, _TFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFLG;
#[doc = "`read()` method returns [tflg::R](tflg::R) reader structure"]
impl crate::Readable for TFLG {}
#[doc = "`write(|w| ..)` method takes [tflg::W](tflg::W) writer structure"]
impl crate::Writable for TFLG {}
#[doc = "Timer Flag Register"]
pub mod tflg;
