#[doc = "Reader of register CLMD"]
pub type R = crate::R<u32, super::CLMD>;
#[doc = "Writer for register CLMD"]
pub type W = crate::W<u32, super::CLMD>;
#[doc = "Register CLMD `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::CLMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `CLMD`"]
pub type CLMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLMD`"]
pub struct CLMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLMD_W<'a> {
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
    pub fn clmd(&self) -> CLMD_R {
        CLMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clmd(&mut self) -> CLMD_W {
        CLMD_W { w: self }
    }
}
