#[doc = "Reader of register CLM3"]
pub type R = crate::R<u32, super::CLM3>;
#[doc = "Writer for register CLM3"]
pub type W = crate::W<u32, super::CLM3>;
#[doc = "Register CLM3 `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::CLM3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `CLM3`"]
pub type CLM3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLM3`"]
pub struct CLM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clm3(&self) -> CLM3_R {
        CLM3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clm3(&mut self) -> CLM3_W {
        CLM3_W { w: self }
    }
}
