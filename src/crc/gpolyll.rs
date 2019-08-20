#[doc = "Reader of register GPOLYLL"]
pub type R = crate::R<u8, super::GPOLYLL>;
#[doc = "Writer for register GPOLYLL"]
pub type W = crate::W<u8, super::GPOLYLL>;
#[doc = "Register GPOLYLL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::GPOLYLL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `GPOLYLL`"]
pub type GPOLYLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOLYLL`"]
pub struct GPOLYLL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyll(&self) -> GPOLYLL_R {
        GPOLYLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyll(&mut self) -> GPOLYLL_W {
        GPOLYLL_W { w: self }
    }
}
