#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Data Low Register"]
    pub dat0l: DATL,
    #[doc = "0x01 - DAC Data High Register"]
    pub dat0h: DATH,
    #[doc = "0x02 - DAC Data Low Register"]
    pub dat1l: DATL,
    #[doc = "0x03 - DAC Data High Register"]
    pub dat1h: DATH,
    #[doc = "0x04 - DAC Data Low Register"]
    pub dat2l: DATL,
    #[doc = "0x05 - DAC Data High Register"]
    pub dat2h: DATH,
    #[doc = "0x06 - DAC Data Low Register"]
    pub dat3l: DATL,
    #[doc = "0x07 - DAC Data High Register"]
    pub dat3h: DATH,
    #[doc = "0x08 - DAC Data Low Register"]
    pub dat4l: DATL,
    #[doc = "0x09 - DAC Data High Register"]
    pub dat4h: DATH,
    #[doc = "0x0a - DAC Data Low Register"]
    pub dat5l: DATL,
    #[doc = "0x0b - DAC Data High Register"]
    pub dat5h: DATH,
    #[doc = "0x0c - DAC Data Low Register"]
    pub dat6l: DATL,
    #[doc = "0x0d - DAC Data High Register"]
    pub dat6h: DATH,
    #[doc = "0x0e - DAC Data Low Register"]
    pub dat7l: DATL,
    #[doc = "0x0f - DAC Data High Register"]
    pub dat7h: DATH,
    #[doc = "0x10 - DAC Data Low Register"]
    pub dat8l: DATL,
    #[doc = "0x11 - DAC Data High Register"]
    pub dat8h: DATH,
    #[doc = "0x12 - DAC Data Low Register"]
    pub dat9l: DATL,
    #[doc = "0x13 - DAC Data High Register"]
    pub dat9h: DATH,
    #[doc = "0x14 - DAC Data Low Register"]
    pub dat10l: DATL,
    #[doc = "0x15 - DAC Data High Register"]
    pub dat10h: DATH,
    #[doc = "0x16 - DAC Data Low Register"]
    pub dat11l: DATL,
    #[doc = "0x17 - DAC Data High Register"]
    pub dat11h: DATH,
    #[doc = "0x18 - DAC Data Low Register"]
    pub dat12l: DATL,
    #[doc = "0x19 - DAC Data High Register"]
    pub dat12h: DATH,
    #[doc = "0x1a - DAC Data Low Register"]
    pub dat13l: DATL,
    #[doc = "0x1b - DAC Data High Register"]
    pub dat13h: DATH,
    #[doc = "0x1c - DAC Data Low Register"]
    pub dat14l: DATL,
    #[doc = "0x1d - DAC Data High Register"]
    pub dat14h: DATH,
    #[doc = "0x1e - DAC Data Low Register"]
    pub dat15l: DATL,
    #[doc = "0x1f - DAC Data High Register"]
    pub dat15h: DATH,
    #[doc = "0x20 - DAC Status Register"]
    pub sr: SR,
    #[doc = "0x21 - DAC Control Register"]
    pub c0: C0,
    #[doc = "0x22 - DAC Control Register 1"]
    pub c1: C1,
    #[doc = "0x23 - DAC Control Register 2"]
    pub c2: C2,
}
#[doc = "DAC Data Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datl](datl) module"]
pub type DATL = crate::Reg<u8, _DATL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATL;
#[doc = "`read()` method returns [datl::R](datl::R) reader structure"]
impl crate::Readable for DATL {}
#[doc = "`write(|w| ..)` method takes [datl::W](datl::W) writer structure"]
impl crate::Writable for DATL {}
#[doc = "DAC Data Low Register"]
pub mod datl;
#[doc = "DAC Data High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dath](dath) module"]
pub type DATH = crate::Reg<u8, _DATH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATH;
#[doc = "`read()` method returns [dath::R](dath::R) reader structure"]
impl crate::Readable for DATH {}
#[doc = "`write(|w| ..)` method takes [dath::W](dath::W) writer structure"]
impl crate::Writable for DATH {}
#[doc = "DAC Data High Register"]
pub mod dath;
#[doc = "DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u8, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "DAC Status Register"]
pub mod sr;
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c0](c0) module"]
pub type C0 = crate::Reg<u8, _C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0;
#[doc = "`read()` method returns [c0::R](c0::R) reader structure"]
impl crate::Readable for C0 {}
#[doc = "`write(|w| ..)` method takes [c0::W](c0::W) writer structure"]
impl crate::Writable for C0 {}
#[doc = "DAC Control Register"]
pub mod c0;
#[doc = "DAC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "DAC Control Register 1"]
pub mod c1;
#[doc = "DAC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "DAC Control Register 2"]
pub mod c2;
