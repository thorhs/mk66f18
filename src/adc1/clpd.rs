#[doc = "Reader of register CLPD"]
pub type R = crate::R<u32, super::CLPD>;
#[doc = "Writer for register CLPD"]
pub type W = crate::W<u32, super::CLPD>;
#[doc = "Register CLPD `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::CLPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `CLPD`"]
pub type CLPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLPD`"]
pub struct CLPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clpd(&self) -> CLPD_R {
        CLPD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clpd(&mut self) -> CLPD_W {
        CLPD_W { w: self }
    }
}
