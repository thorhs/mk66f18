#[doc = "Reader of register CLM1"]
pub type R = crate::R<u32, super::CLM1>;
#[doc = "Writer for register CLM1"]
pub type W = crate::W<u32, super::CLM1>;
#[doc = "Register CLM1 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::CLM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `CLM1`"]
pub type CLM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLM1`"]
pub struct CLM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clm1(&self) -> CLM1_R {
        CLM1_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clm1(&mut self) -> CLM1_W {
        CLM1_W { w: self }
    }
}
