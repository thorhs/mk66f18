#[doc = "Reader of register CMD4"]
pub type R = crate::R<u8, super::CMD4>;
#[doc = "Writer for register CMD4"]
pub type W = crate::W<u8, super::CMD4>;
#[doc = "Register CMD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SB`"]
pub type SB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SB`"]
pub struct SB_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SB\\[7:0\\]"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SB\\[7:0\\]"]
    #[inline(always)]
    pub fn sb(&mut self) -> SB_W {
        SB_W { w: self }
    }
}
