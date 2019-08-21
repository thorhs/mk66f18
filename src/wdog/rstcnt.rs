#[doc = "Reader of register RSTCNT"]
pub type R = crate::R<u16, super::RSTCNT>;
#[doc = "Writer for register RSTCNT"]
pub type W = crate::W<u16, super::RSTCNT>;
#[doc = "Register RSTCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCNT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTCNT`"]
pub type RSTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSTCNT`"]
pub struct RSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&mut self) -> RSTCNT_W {
        RSTCNT_W { w: self }
    }
}
