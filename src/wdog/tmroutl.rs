#[doc = "Reader of register TMROUTL"]
pub type R = crate::R<u16, super::TMROUTL>;
#[doc = "Writer for register TMROUTL"]
pub type W = crate::W<u16, super::TMROUTL>;
#[doc = "Register TMROUTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TMROUTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMEROUTLOW`"]
pub type TIMEROUTLOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMEROUTLOW`"]
pub struct TIMEROUTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEROUTLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shows the value of the lower 16 bits of the watchdog timer."]
    #[inline(always)]
    pub fn timeroutlow(&self) -> TIMEROUTLOW_R {
        TIMEROUTLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the value of the lower 16 bits of the watchdog timer."]
    #[inline(always)]
    pub fn timeroutlow(&mut self) -> TIMEROUTLOW_W {
        TIMEROUTLOW_W { w: self }
    }
}
