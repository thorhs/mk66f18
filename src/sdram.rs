#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 66usize],
    #[doc = "0x42 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x48 - Address and Control Register"]
    pub ac0: AC,
    #[doc = "0x4c - Control Mask"]
    pub cm0: CM,
    #[doc = "0x50 - Address and Control Register"]
    pub ac1: AC,
    #[doc = "0x54 - Control Mask"]
    pub cm1: CM,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Address and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ac](ac) module"]
pub type AC = crate::Reg<u32, _AC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AC;
#[doc = "`read()` method returns [ac::R](ac::R) reader structure"]
impl crate::Readable for AC {}
#[doc = "`write(|w| ..)` method takes [ac::W](ac::W) writer structure"]
impl crate::Writable for AC {}
#[doc = "Address and Control Register"]
pub mod ac;
#[doc = "Control Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cm](cm) module"]
pub type CM = crate::Reg<u32, _CM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM;
#[doc = "`read()` method returns [cm::R](cm::R) reader structure"]
impl crate::Readable for CM {}
#[doc = "`write(|w| ..)` method takes [cm::W](cm::W) writer structure"]
impl crate::Writable for CM {}
#[doc = "Control Mask"]
pub mod cm;
