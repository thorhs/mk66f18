#[doc = "Reader of register CLM4"]
pub type R = crate::R<u32, super::CLM4>;
#[doc = "Writer for register CLM4"]
pub type W = crate::W<u32, super::CLM4>;
#[doc = "Register CLM4 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::CLM4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `CLM4`"]
pub type CLM4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLM4`"]
pub struct CLM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clm4(&self) -> CLM4_R {
        CLM4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clm4(&mut self) -> CLM4_W {
        CLM4_W { w: self }
    }
}
