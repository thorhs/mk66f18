#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock register"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    #[doc = "0x0c - Signal Override Register"]
    pub signal_override: SIGNAL_OVERRIDE,
    #[doc = "0x10 - TIMER0 register"]
    pub timer0: TIMER0,
    #[doc = "0x14 - TIMER1 register"]
    pub timer1: TIMER1,
    _reserved_6_timer2_bc1: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x18 - TIMER2_BC12 register"]
    #[inline(always)]
    pub fn timer2_bc12(&self) -> &TIMER2_BC12 {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMER2_BC12) }
    }
    #[doc = "0x18 - TIMER2_BC12 register"]
    #[inline(always)]
    pub fn timer2_bc12_mut(&self) -> &mut TIMER2_BC12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMER2_BC12) }
    }
    #[doc = "0x18 - TIMER2_BC11 register"]
    #[inline(always)]
    pub fn timer2_bc11(&self) -> &TIMER2_BC11 {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMER2_BC11) }
    }
    #[doc = "0x18 - TIMER2_BC11 register"]
    #[inline(always)]
    pub fn timer2_bc11_mut(&self) -> &mut TIMER2_BC11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMER2_BC11) }
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Control register"]
pub mod control;
#[doc = "Clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "Clock register"]
pub mod clock;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status register"]
pub mod status;
#[doc = "Signal Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [signal_override](signal_override) module"]
pub type SIGNAL_OVERRIDE = crate::Reg<u32, _SIGNAL_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGNAL_OVERRIDE;
#[doc = "`read()` method returns [signal_override::R](signal_override::R) reader structure"]
impl crate::Readable for SIGNAL_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [signal_override::W](signal_override::W) writer structure"]
impl crate::Writable for SIGNAL_OVERRIDE {}
#[doc = "Signal Override Register"]
pub mod signal_override;
#[doc = "TIMER0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer0](timer0) module"]
pub type TIMER0 = crate::Reg<u32, _TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0;
#[doc = "`read()` method returns [timer0::R](timer0::R) reader structure"]
impl crate::Readable for TIMER0 {}
#[doc = "`write(|w| ..)` method takes [timer0::W](timer0::W) writer structure"]
impl crate::Writable for TIMER0 {}
#[doc = "TIMER0 register"]
pub mod timer0;
#[doc = "TIMER1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer1](timer1) module"]
pub type TIMER1 = crate::Reg<u32, _TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1;
#[doc = "`read()` method returns [timer1::R](timer1::R) reader structure"]
impl crate::Readable for TIMER1 {}
#[doc = "`write(|w| ..)` method takes [timer1::W](timer1::W) writer structure"]
impl crate::Writable for TIMER1 {}
#[doc = "TIMER1 register"]
pub mod timer1;
#[doc = "TIMER2_BC11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_bc11](timer2_bc11) module"]
pub type TIMER2_BC11 = crate::Reg<u32, _TIMER2_BC11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_BC11;
#[doc = "`read()` method returns [timer2_bc11::R](timer2_bc11::R) reader structure"]
impl crate::Readable for TIMER2_BC11 {}
#[doc = "`write(|w| ..)` method takes [timer2_bc11::W](timer2_bc11::W) writer structure"]
impl crate::Writable for TIMER2_BC11 {}
#[doc = "TIMER2_BC11 register"]
pub mod timer2_bc11;
#[doc = "TIMER2_BC12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timer2_bc12](timer2_bc12) module"]
pub type TIMER2_BC12 = crate::Reg<u32, _TIMER2_BC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2_BC12;
#[doc = "`read()` method returns [timer2_bc12::R](timer2_bc12::R) reader structure"]
impl crate::Readable for TIMER2_BC12 {}
#[doc = "`write(|w| ..)` method takes [timer2_bc12::W](timer2_bc12::W) writer structure"]
impl crate::Writable for TIMER2_BC12 {}
#[doc = "TIMER2_BC12 register"]
pub mod timer2_bc12;
