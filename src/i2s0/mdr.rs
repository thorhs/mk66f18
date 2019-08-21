#[doc = "Reader of register MDR"]
pub type R = crate::R<u32, super::MDR>;
#[doc = "Writer for register MDR"]
pub type W = crate::W<u32, super::MDR>;
#[doc = "Register MDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVIDE`"]
pub type DIVIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVIDE`"]
pub struct DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `FRACT`"]
pub type FRACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACT`"]
pub struct FRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    pub fn fract(&self) -> FRACT_R {
        FRACT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    pub fn divide(&mut self) -> DIVIDE_W {
        DIVIDE_W { w: self }
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    pub fn fract(&mut self) -> FRACT_W {
        FRACT_W { w: self }
    }
}
