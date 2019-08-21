#[doc = "Reader of register CGL1"]
pub type R = crate::R<u8, super::CGL1>;
#[doc = "Writer for register CGL1"]
pub type W = crate::W<u8, super::CGL1>;
#[doc = "Register CGL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PL`"]
pub type PL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PL`"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
}
