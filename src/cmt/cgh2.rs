#[doc = "Reader of register CGH2"]
pub type R = crate::R<u8, super::CGH2>;
#[doc = "Writer for register CGH2"]
pub type W = crate::W<u8, super::CGH2>;
#[doc = "Register CGH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGH2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SH`"]
pub type SH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SH`"]
pub struct SH_W<'a> {
    w: &'a mut W,
}
impl<'a> SH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn sh(&self) -> SH_R {
        SH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn sh(&mut self) -> SH_W {
        SH_W { w: self }
    }
}
