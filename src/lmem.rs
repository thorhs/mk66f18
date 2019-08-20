#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub pcccr: PCCCR,
    #[doc = "0x04 - Cache line control register"]
    pub pcclcr: PCCLCR,
    #[doc = "0x08 - Cache search address register"]
    pub pccsar: PCCSAR,
    #[doc = "0x0c - Cache read/write value register"]
    pub pcccvr: PCCCVR,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: PCCRMR,
}
#[doc = "Cache control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcccr](pcccr) module"]
pub type PCCCR = crate::Reg<u32, _PCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCCR;
#[doc = "`read()` method returns [pcccr::R](pcccr::R) reader structure"]
impl crate::Readable for PCCCR {}
#[doc = "`write(|w| ..)` method takes [pcccr::W](pcccr::W) writer structure"]
impl crate::Writable for PCCCR {}
#[doc = "Cache control register"]
pub mod pcccr;
#[doc = "Cache line control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcclcr](pcclcr) module"]
pub type PCCLCR = crate::Reg<u32, _PCCLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCLCR;
#[doc = "`read()` method returns [pcclcr::R](pcclcr::R) reader structure"]
impl crate::Readable for PCCLCR {}
#[doc = "`write(|w| ..)` method takes [pcclcr::W](pcclcr::W) writer structure"]
impl crate::Writable for PCCLCR {}
#[doc = "Cache line control register"]
pub mod pcclcr;
#[doc = "Cache search address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pccsar](pccsar) module"]
pub type PCCSAR = crate::Reg<u32, _PCCSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCSAR;
#[doc = "`read()` method returns [pccsar::R](pccsar::R) reader structure"]
impl crate::Readable for PCCSAR {}
#[doc = "`write(|w| ..)` method takes [pccsar::W](pccsar::W) writer structure"]
impl crate::Writable for PCCSAR {}
#[doc = "Cache search address register"]
pub mod pccsar;
#[doc = "Cache read/write value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcccvr](pcccvr) module"]
pub type PCCCVR = crate::Reg<u32, _PCCCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCCVR;
#[doc = "`read()` method returns [pcccvr::R](pcccvr::R) reader structure"]
impl crate::Readable for PCCCVR {}
#[doc = "`write(|w| ..)` method takes [pcccvr::W](pcccvr::W) writer structure"]
impl crate::Writable for PCCCVR {}
#[doc = "Cache read/write value register"]
pub mod pcccvr;
#[doc = "Cache regions mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pccrmr](pccrmr) module"]
pub type PCCRMR = crate::Reg<u32, _PCCRMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCRMR;
#[doc = "`read()` method returns [pccrmr::R](pccrmr::R) reader structure"]
impl crate::Readable for PCCRMR {}
#[doc = "`write(|w| ..)` method takes [pccrmr::W](pccrmr::W) writer structure"]
impl crate::Writable for PCCRMR {}
#[doc = "Cache regions mode register"]
pub mod pccrmr;
