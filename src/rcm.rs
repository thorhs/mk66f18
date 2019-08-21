#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: SRS0,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: SRS1,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Reset Pin Filter Control register"]
    pub rpfc: RPFC,
    #[doc = "0x05 - Reset Pin Filter Width register"]
    pub rpfw: RPFW,
    _reserved4: [u8; 1usize],
    #[doc = "0x07 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Sticky System Reset Status Register 0"]
    pub ssrs0: SSRS0,
    #[doc = "0x09 - Sticky System Reset Status Register 1"]
    pub ssrs1: SSRS1,
}
#[doc = "System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srs0](srs0) module"]
pub type SRS0 = crate::Reg<u8, _SRS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS0;
#[doc = "`read()` method returns [srs0::R](srs0::R) reader structure"]
impl crate::Readable for SRS0 {}
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "System Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srs1](srs1) module"]
pub type SRS1 = crate::Reg<u8, _SRS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS1;
#[doc = "`read()` method returns [srs1::R](srs1::R) reader structure"]
impl crate::Readable for SRS1 {}
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "Reset Pin Filter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rpfc](rpfc) module"]
pub type RPFC = crate::Reg<u8, _RPFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPFC;
#[doc = "`read()` method returns [rpfc::R](rpfc::R) reader structure"]
impl crate::Readable for RPFC {}
#[doc = "`write(|w| ..)` method takes [rpfc::W](rpfc::W) writer structure"]
impl crate::Writable for RPFC {}
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "Reset Pin Filter Width register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rpfw](rpfw) module"]
pub type RPFW = crate::Reg<u8, _RPFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPFW;
#[doc = "`read()` method returns [rpfw::R](rpfw::R) reader structure"]
impl crate::Readable for RPFW {}
#[doc = "`write(|w| ..)` method takes [rpfw::W](rpfw::W) writer structure"]
impl crate::Writable for RPFW {}
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr](mr) module"]
pub type MR = crate::Reg<u8, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Sticky System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssrs0](ssrs0) module"]
pub type SSRS0 = crate::Reg<u8, _SSRS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRS0;
#[doc = "`read()` method returns [ssrs0::R](ssrs0::R) reader structure"]
impl crate::Readable for SSRS0 {}
#[doc = "`write(|w| ..)` method takes [ssrs0::W](ssrs0::W) writer structure"]
impl crate::Writable for SSRS0 {}
#[doc = "Sticky System Reset Status Register 0"]
pub mod ssrs0;
#[doc = "Sticky System Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssrs1](ssrs1) module"]
pub type SSRS1 = crate::Reg<u8, _SSRS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRS1;
#[doc = "`read()` method returns [ssrs1::R](ssrs1::R) reader structure"]
impl crate::Readable for SSRS1 {}
#[doc = "`write(|w| ..)` method takes [ssrs1::W](ssrs1::W) writer structure"]
impl crate::Writable for SSRS1 {}
#[doc = "Sticky System Reset Status Register 1"]
pub mod ssrs1;
