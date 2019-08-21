#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPUART Baud Rate Register"]
    pub baud: BAUD,
    #[doc = "0x04 - LPUART Status Register"]
    pub stat: STAT,
    #[doc = "0x08 - LPUART Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - LPUART Data Register"]
    pub data: DATA,
    #[doc = "0x10 - LPUART Match Address Register"]
    pub match_: MATCH,
    #[doc = "0x14 - LPUART Modem IrDA Register"]
    pub modir: MODIR,
}
#[doc = "LPUART Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [baud](baud) module"]
pub type BAUD = crate::Reg<u32, _BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD;
#[doc = "`read()` method returns [baud::R](baud::R) reader structure"]
impl crate::Readable for BAUD {}
#[doc = "`write(|w| ..)` method takes [baud::W](baud::W) writer structure"]
impl crate::Writable for BAUD {}
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "LPUART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "LPUART Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "LPUART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "LPUART Match Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [match_](match_) module"]
pub type MATCH = crate::Reg<u32, _MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH;
#[doc = "`read()` method returns [match_::R](match_::R) reader structure"]
impl crate::Readable for MATCH {}
#[doc = "`write(|w| ..)` method takes [match_::W](match_::W) writer structure"]
impl crate::Writable for MATCH {}
#[doc = "LPUART Match Address Register"]
pub mod match_;
#[doc = "LPUART Modem IrDA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modir](modir) module"]
pub type MODIR = crate::Reg<u32, _MODIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODIR;
#[doc = "`read()` method returns [modir::R](modir::R) reader structure"]
impl crate::Readable for MODIR {}
#[doc = "`write(|w| ..)` method takes [modir::W](modir::W) writer structure"]
impl crate::Writable for MODIR {}
#[doc = "LPUART Modem IrDA Register"]
pub mod modir;
