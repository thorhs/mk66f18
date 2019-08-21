#[doc = "Reader of register CGH1"]
pub type R = crate::R<u8, super::CGH1>;
#[doc = "Writer for register CGH1"]
pub type W = crate::W<u8, super::CGH1>;
#[doc = "Register CGH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGH1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PH`"]
pub type PH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PH`"]
pub struct PH_W<'a> {
    w: &'a mut W,
}
impl<'a> PH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Primary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn ph(&self) -> PH_R {
        PH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn ph(&mut self) -> PH_W {
        PH_W { w: self }
    }
}
