#[doc = "Reader of register GPOLYLU"]
pub type R = crate::R<u8, super::GPOLYLU>;
#[doc = "Writer for register GPOLYLU"]
pub type W = crate::W<u8, super::GPOLYLU>;
#[doc = "Register GPOLYLU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::GPOLYLU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `GPOLYLU`"]
pub type GPOLYLU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOLYLU`"]
pub struct GPOLYLU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYLU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&self) -> GPOLYLU_R {
        GPOLYLU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&mut self) -> GPOLYLU_W {
        GPOLYLU_W { w: self }
    }
}
