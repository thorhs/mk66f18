#[doc = "Reader of register TL7816"]
pub type R = crate::R<u8, super::TL7816>;
#[doc = "Writer for register TL7816"]
pub type W = crate::W<u8, super::TL7816>;
#[doc = "Register TL7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::TL7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLEN`"]
pub type TLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLEN`"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
}
