#[doc = "Reader of register EPSETUPSR"]
pub type R = crate::R<u32, super::EPSETUPSR>;
#[doc = "Writer for register EPSETUPSR"]
pub type W = crate::W<u32, super::EPSETUPSR>;
#[doc = "Register EPSETUPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::EPSETUPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPSETUPSTAT`"]
pub type EPSETUPSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPSETUPSTAT`"]
pub struct EPSETUPSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSETUPSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Setup Endpoint Status"]
    #[inline(always)]
    pub fn epsetupstat(&self) -> EPSETUPSTAT_R {
        EPSETUPSTAT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Setup Endpoint Status"]
    #[inline(always)]
    pub fn epsetupstat(&mut self) -> EPSETUPSTAT_W {
        EPSETUPSTAT_W { w: self }
    }
}
