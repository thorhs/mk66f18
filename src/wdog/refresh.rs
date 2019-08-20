#[doc = "Reader of register REFRESH"]
pub type R = crate::R<u16, super::REFRESH>;
#[doc = "Writer for register REFRESH"]
pub type W = crate::W<u16, super::REFRESH>;
#[doc = "Register REFRESH `reset()`'s with value 0xb480"]
impl crate::ResetValue for super::REFRESH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb480
    }
}
#[doc = "Reader of field `WDOGREFRESH`"]
pub type WDOGREFRESH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDOGREFRESH`"]
pub struct WDOGREFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGREFRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    pub fn wdogrefresh(&self) -> WDOGREFRESH_R {
        WDOGREFRESH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    pub fn wdogrefresh(&mut self) -> WDOGREFRESH_W {
        WDOGREFRESH_W { w: self }
    }
}
