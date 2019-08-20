#[doc = "Reader of register BDL"]
pub type R = crate::R<u8, super::BDL>;
#[doc = "Writer for register BDL"]
pub type W = crate::W<u8, super::BDL>;
#[doc = "Register BDL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::BDL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `SBR`"]
pub type SBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBR`"]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
}
