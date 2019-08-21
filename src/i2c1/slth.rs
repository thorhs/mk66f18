#[doc = "Reader of register SLTH"]
pub type R = crate::R<u8, super::SLTH>;
#[doc = "Writer for register SLTH"]
pub type W = crate::W<u8, super::SLTH>;
#[doc = "Register SLTH `reset()`'s with value 0"]
impl crate::ResetValue for super::SLTH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSLT`"]
pub type SSLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSLT`"]
pub struct SSLT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SSLT\\[15:8\\]"]
    #[inline(always)]
    pub fn sslt(&self) -> SSLT_R {
        SSLT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSLT\\[15:8\\]"]
    #[inline(always)]
    pub fn sslt(&mut self) -> SSLT_W {
        SSLT_W { w: self }
    }
}
