#[doc = "Reader of register TWFIFO"]
pub type R = crate::R<u8, super::TWFIFO>;
#[doc = "Writer for register TWFIFO"]
pub type W = crate::W<u8, super::TWFIFO>;
#[doc = "Register TWFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::TWFIFO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXWATER`"]
pub type TXWATER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXWATER`"]
pub struct TXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&mut self) -> TXWATER_W {
        TXWATER_W { w: self }
    }
}
