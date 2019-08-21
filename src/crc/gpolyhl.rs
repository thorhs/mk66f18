#[doc = "Reader of register GPOLYHL"]
pub type R = crate::R<u8, super::GPOLYHL>;
#[doc = "Writer for register GPOLYHL"]
pub type W = crate::W<u8, super::GPOLYHL>;
#[doc = "Register GPOLYHL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::GPOLYHL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `GPOLYHL`"]
pub type GPOLYHL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOLYHL`"]
pub struct GPOLYHL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYHL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhl(&self) -> GPOLYHL_R {
        GPOLYHL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhl(&mut self) -> GPOLYHL_W {
        GPOLYHL_W { w: self }
    }
}
