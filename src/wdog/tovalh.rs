#[doc = "Reader of register TOVALH"]
pub type R = crate::R<u16, super::TOVALH>;
#[doc = "Writer for register TOVALH"]
pub type W = crate::W<u16, super::TOVALH>;
#[doc = "Register TOVALH `reset()`'s with value 0x4c"]
impl crate::ResetValue for super::TOVALH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4c
    }
}
#[doc = "Reader of field `TOVALHIGH`"]
pub type TOVALHIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOVALHIGH`"]
pub struct TOVALHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W {
        TOVALHIGH_W { w: self }
    }
}
