#[doc = "Reader of register CLMS"]
pub type R = crate::R<u32, super::CLMS>;
#[doc = "Writer for register CLMS"]
pub type W = crate::W<u32, super::CLMS>;
#[doc = "Register CLMS `reset()`'s with value 0x20"]
impl crate::ResetValue for super::CLMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `CLMS`"]
pub type CLMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLMS`"]
pub struct CLMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLMS_W<'a> {
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
    pub fn clms(&self) -> CLMS_R {
        CLMS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clms(&mut self) -> CLMS_W {
        CLMS_W { w: self }
    }
}
