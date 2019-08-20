#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Status and Control Register High"]
    pub stctrlh: STCTRLH,
    #[doc = "0x02 - Watchdog Status and Control Register Low"]
    pub stctrll: STCTRLL,
    #[doc = "0x04 - Watchdog Time-out Value Register High"]
    pub tovalh: TOVALH,
    #[doc = "0x06 - Watchdog Time-out Value Register Low"]
    pub tovall: TOVALL,
    #[doc = "0x08 - Watchdog Window Register High"]
    pub winh: WINH,
    #[doc = "0x0a - Watchdog Window Register Low"]
    pub winl: WINL,
    #[doc = "0x0c - Watchdog Refresh register"]
    pub refresh: REFRESH,
    #[doc = "0x0e - Watchdog Unlock register"]
    pub unlock: UNLOCK,
    #[doc = "0x10 - Watchdog Timer Output Register High"]
    pub tmrouth: TMROUTH,
    #[doc = "0x12 - Watchdog Timer Output Register Low"]
    pub tmroutl: TMROUTL,
    #[doc = "0x14 - Watchdog Reset Count register"]
    pub rstcnt: RSTCNT,
    #[doc = "0x16 - Watchdog Prescaler register"]
    pub presc: PRESC,
}
#[doc = "Watchdog Status and Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stctrlh](stctrlh) module"]
pub type STCTRLH = crate::Reg<u16, _STCTRLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCTRLH;
#[doc = "`read()` method returns [stctrlh::R](stctrlh::R) reader structure"]
impl crate::Readable for STCTRLH {}
#[doc = "`write(|w| ..)` method takes [stctrlh::W](stctrlh::W) writer structure"]
impl crate::Writable for STCTRLH {}
#[doc = "Watchdog Status and Control Register High"]
pub mod stctrlh;
#[doc = "Watchdog Status and Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stctrll](stctrll) module"]
pub type STCTRLL = crate::Reg<u16, _STCTRLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCTRLL;
#[doc = "`read()` method returns [stctrll::R](stctrll::R) reader structure"]
impl crate::Readable for STCTRLL {}
#[doc = "`write(|w| ..)` method takes [stctrll::W](stctrll::W) writer structure"]
impl crate::Writable for STCTRLL {}
#[doc = "Watchdog Status and Control Register Low"]
pub mod stctrll;
#[doc = "Watchdog Time-out Value Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tovalh](tovalh) module"]
pub type TOVALH = crate::Reg<u16, _TOVALH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALH;
#[doc = "`read()` method returns [tovalh::R](tovalh::R) reader structure"]
impl crate::Readable for TOVALH {}
#[doc = "`write(|w| ..)` method takes [tovalh::W](tovalh::W) writer structure"]
impl crate::Writable for TOVALH {}
#[doc = "Watchdog Time-out Value Register High"]
pub mod tovalh;
#[doc = "Watchdog Time-out Value Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tovall](tovall) module"]
pub type TOVALL = crate::Reg<u16, _TOVALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALL;
#[doc = "`read()` method returns [tovall::R](tovall::R) reader structure"]
impl crate::Readable for TOVALL {}
#[doc = "`write(|w| ..)` method takes [tovall::W](tovall::W) writer structure"]
impl crate::Writable for TOVALL {}
#[doc = "Watchdog Time-out Value Register Low"]
pub mod tovall;
#[doc = "Watchdog Window Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [winh](winh) module"]
pub type WINH = crate::Reg<u16, _WINH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINH;
#[doc = "`read()` method returns [winh::R](winh::R) reader structure"]
impl crate::Readable for WINH {}
#[doc = "`write(|w| ..)` method takes [winh::W](winh::W) writer structure"]
impl crate::Writable for WINH {}
#[doc = "Watchdog Window Register High"]
pub mod winh;
#[doc = "Watchdog Window Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [winl](winl) module"]
pub type WINL = crate::Reg<u16, _WINL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINL;
#[doc = "`read()` method returns [winl::R](winl::R) reader structure"]
impl crate::Readable for WINL {}
#[doc = "`write(|w| ..)` method takes [winl::W](winl::W) writer structure"]
impl crate::Writable for WINL {}
#[doc = "Watchdog Window Register Low"]
pub mod winl;
#[doc = "Watchdog Refresh register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refresh](refresh) module"]
pub type REFRESH = crate::Reg<u16, _REFRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFRESH;
#[doc = "`read()` method returns [refresh::R](refresh::R) reader structure"]
impl crate::Readable for REFRESH {}
#[doc = "`write(|w| ..)` method takes [refresh::W](refresh::W) writer structure"]
impl crate::Writable for REFRESH {}
#[doc = "Watchdog Refresh register"]
pub mod refresh;
#[doc = "Watchdog Unlock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [unlock](unlock) module"]
pub type UNLOCK = crate::Reg<u16, _UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNLOCK;
#[doc = "`read()` method returns [unlock::R](unlock::R) reader structure"]
impl crate::Readable for UNLOCK {}
#[doc = "`write(|w| ..)` method takes [unlock::W](unlock::W) writer structure"]
impl crate::Writable for UNLOCK {}
#[doc = "Watchdog Unlock register"]
pub mod unlock;
#[doc = "Watchdog Timer Output Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tmrouth](tmrouth) module"]
pub type TMROUTH = crate::Reg<u16, _TMROUTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMROUTH;
#[doc = "`read()` method returns [tmrouth::R](tmrouth::R) reader structure"]
impl crate::Readable for TMROUTH {}
#[doc = "`write(|w| ..)` method takes [tmrouth::W](tmrouth::W) writer structure"]
impl crate::Writable for TMROUTH {}
#[doc = "Watchdog Timer Output Register High"]
pub mod tmrouth;
#[doc = "Watchdog Timer Output Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tmroutl](tmroutl) module"]
pub type TMROUTL = crate::Reg<u16, _TMROUTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMROUTL;
#[doc = "`read()` method returns [tmroutl::R](tmroutl::R) reader structure"]
impl crate::Readable for TMROUTL {}
#[doc = "`write(|w| ..)` method takes [tmroutl::W](tmroutl::W) writer structure"]
impl crate::Writable for TMROUTL {}
#[doc = "Watchdog Timer Output Register Low"]
pub mod tmroutl;
#[doc = "Watchdog Reset Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstcnt](rstcnt) module"]
pub type RSTCNT = crate::Reg<u16, _RSTCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCNT;
#[doc = "`read()` method returns [rstcnt::R](rstcnt::R) reader structure"]
impl crate::Readable for RSTCNT {}
#[doc = "`write(|w| ..)` method takes [rstcnt::W](rstcnt::W) writer structure"]
impl crate::Writable for RSTCNT {}
#[doc = "Watchdog Reset Count register"]
pub mod rstcnt;
#[doc = "Watchdog Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presc](presc) module"]
pub type PRESC = crate::Reg<u16, _PRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESC;
#[doc = "`read()` method returns [presc::R](presc::R) reader structure"]
impl crate::Readable for PRESC {}
#[doc = "`write(|w| ..)` method takes [presc::W](presc::W) writer structure"]
impl crate::Writable for PRESC {}
#[doc = "Watchdog Prescaler register"]
pub mod presc;
