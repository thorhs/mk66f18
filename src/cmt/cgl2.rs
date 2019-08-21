#[doc = "Reader of register CGL2"]
pub type R = crate::R<u8, super::CGL2>;
#[doc = "Writer for register CGL2"]
pub type W = crate::W<u8, super::CGL2>;
#[doc = "Register CGL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGL2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SL`"]
pub type SL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SL`"]
pub struct SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn sl(&self) -> SL_R {
        SL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn sl(&mut self) -> SL_W {
        SL_W { w: self }
    }
}
