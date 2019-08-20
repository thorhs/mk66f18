#[doc = "Reader of register CLM0"]
pub type R = crate::R<u32, super::CLM0>;
#[doc = "Writer for register CLM0"]
pub type W = crate::W<u32, super::CLM0>;
#[doc = "Register CLM0 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::CLM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `CLM0`"]
pub type CLM0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLM0`"]
pub struct CLM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM0_W<'a> {
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
    pub fn clm0(&self) -> CLM0_R {
        CLM0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clm0(&mut self) -> CLM0_W {
        CLM0_W { w: self }
    }
}
