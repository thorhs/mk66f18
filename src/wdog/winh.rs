#[doc = "Reader of register WINH"]
pub type R = crate::R<u16, super::WINH>;
#[doc = "Writer for register WINH"]
pub type W = crate::W<u16, super::WINH>;
#[doc = "Register WINH `reset()`'s with value 0"]
impl crate::ResetValue for super::WINH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINHIGH`"]
pub type WINHIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINHIGH`"]
pub struct WINHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WINHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winhigh(&mut self) -> WINHIGH_W {
        WINHIGH_W { w: self }
    }
}
