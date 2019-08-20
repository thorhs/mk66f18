#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Voltage Detect Status And Control 1 register"]
    pub lvdsc1: LVDSC1,
    #[doc = "0x01 - Low Voltage Detect Status And Control 2 register"]
    pub lvdsc2: LVDSC2,
    #[doc = "0x02 - Regulator Status And Control register"]
    pub regsc: REGSC,
}
#[doc = "Low Voltage Detect Status And Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lvdsc1](lvdsc1) module"]
pub type LVDSC1 = crate::Reg<u8, _LVDSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVDSC1;
#[doc = "`read()` method returns [lvdsc1::R](lvdsc1::R) reader structure"]
impl crate::Readable for LVDSC1 {}
#[doc = "`write(|w| ..)` method takes [lvdsc1::W](lvdsc1::W) writer structure"]
impl crate::Writable for LVDSC1 {}
#[doc = "Low Voltage Detect Status And Control 1 register"]
pub mod lvdsc1;
#[doc = "Low Voltage Detect Status And Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lvdsc2](lvdsc2) module"]
pub type LVDSC2 = crate::Reg<u8, _LVDSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVDSC2;
#[doc = "`read()` method returns [lvdsc2::R](lvdsc2::R) reader structure"]
impl crate::Readable for LVDSC2 {}
#[doc = "`write(|w| ..)` method takes [lvdsc2::W](lvdsc2::W) writer structure"]
impl crate::Writable for LVDSC2 {}
#[doc = "Low Voltage Detect Status And Control 2 register"]
pub mod lvdsc2;
#[doc = "Regulator Status And Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [regsc](regsc) module"]
pub type REGSC = crate::Reg<u8, _REGSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGSC;
#[doc = "`read()` method returns [regsc::R](regsc::R) reader structure"]
impl crate::Readable for REGSC {}
#[doc = "`write(|w| ..)` method takes [regsc::W](regsc::W) writer structure"]
impl crate::Writable for REGSC {}
#[doc = "Regulator Status And Control register"]
pub mod regsc;
