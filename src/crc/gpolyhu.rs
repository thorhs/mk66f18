#[doc = "Reader of register GPOLYHU"]
pub type R = crate::R<u8, super::GPOLYHU>;
#[doc = "Writer for register GPOLYHU"]
pub type W = crate::W<u8, super::GPOLYHU>;
#[doc = "Register GPOLYHU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::GPOLYHU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `GPOLYHU`"]
pub type GPOLYHU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOLYHU`"]
pub struct GPOLYHU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&self) -> GPOLYHU_R {
        GPOLYHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&mut self) -> GPOLYHU_W {
        GPOLYHU_W { w: self }
    }
}
