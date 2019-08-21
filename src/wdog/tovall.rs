#[doc = "Reader of register TOVALL"]
pub type R = crate::R<u16, super::TOVALL>;
#[doc = "Writer for register TOVALL"]
pub type W = crate::W<u16, super::TOVALL>;
#[doc = "Register TOVALL `reset()`'s with value 0x4b4c"]
impl crate::ResetValue for super::TOVALL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4b4c
    }
}
#[doc = "Reader of field `TOVALLOW`"]
pub type TOVALLOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOVALLOW`"]
pub struct TOVALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovallow(&mut self) -> TOVALLOW_W {
        TOVALLOW_W { w: self }
    }
}
