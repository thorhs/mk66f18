#[doc = "Reader of register CLM2"]
pub type R = crate::R<u32, super::CLM2>;
#[doc = "Writer for register CLM2"]
pub type W = crate::W<u32, super::CLM2>;
#[doc = "Register CLM2 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::CLM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `CLM2`"]
pub type CLM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLM2`"]
pub struct CLM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clm2(&self) -> CLM2_R {
        CLM2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clm2(&mut self) -> CLM2_W {
        CLM2_W { w: self }
    }
}
