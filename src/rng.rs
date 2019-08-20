#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNGA Control Register"]
    pub cr: CR,
    #[doc = "0x04 - RNGA Status Register"]
    pub sr: SR,
    #[doc = "0x08 - RNGA Entropy Register"]
    pub er: ER,
    #[doc = "0x0c - RNGA Output Register"]
    pub or: OR,
}
#[doc = "RNGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "RNGA Control Register"]
pub mod cr;
#[doc = "RNGA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "RNGA Status Register"]
pub mod sr;
#[doc = "RNGA Entropy Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [er](er) module"]
pub type ER = crate::Reg<u32, _ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER;
#[doc = "`write(|w| ..)` method takes [er::W](er::W) writer structure"]
impl crate::Writable for ER {}
#[doc = "RNGA Entropy Register"]
pub mod er;
#[doc = "RNGA Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [or](or) module"]
pub type OR = crate::Reg<u32, _OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR;
#[doc = "`read()` method returns [or::R](or::R) reader structure"]
impl crate::Readable for OR {}
#[doc = "RNGA Output Register"]
pub mod or;
