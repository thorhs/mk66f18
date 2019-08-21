#[doc = "Reader of register RWFIFO"]
pub type R = crate::R<u8, super::RWFIFO>;
#[doc = "Writer for register RWFIFO"]
pub type W = crate::W<u8, super::RWFIFO>;
#[doc = "Register RWFIFO `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RWFIFO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RXWATER`"]
pub type RXWATER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXWATER`"]
pub struct RXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&mut self) -> RXWATER_W {
        RXWATER_W { w: self }
    }
}
