#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TSI General Control and Status Register"]
    pub gencs: GENCS,
    #[doc = "0x04 - TSI DATA Register"]
    pub data: DATA,
    #[doc = "0x08 - TSI Threshold Register"]
    pub tshd: TSHD,
}
#[doc = "TSI General Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gencs](gencs) module"]
pub type GENCS = crate::Reg<u32, _GENCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENCS;
#[doc = "`read()` method returns [gencs::R](gencs::R) reader structure"]
impl crate::Readable for GENCS {}
#[doc = "`write(|w| ..)` method takes [gencs::W](gencs::W) writer structure"]
impl crate::Writable for GENCS {}
#[doc = "TSI General Control and Status Register"]
pub mod gencs;
#[doc = "TSI DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "TSI DATA Register"]
pub mod data;
#[doc = "TSI Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tshd](tshd) module"]
pub type TSHD = crate::Reg<u32, _TSHD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSHD;
#[doc = "`read()` method returns [tshd::R](tshd::R) reader structure"]
impl crate::Readable for TSHD {}
#[doc = "`write(|w| ..)` method takes [tshd::W](tshd::W) writer structure"]
impl crate::Writable for TSHD {}
#[doc = "TSI Threshold Register"]
pub mod tshd;
