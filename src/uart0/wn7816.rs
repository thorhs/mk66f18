#[doc = "Reader of register WN7816"]
pub type R = crate::R<u8, super::WN7816>;
#[doc = "Writer for register WN7816"]
pub type W = crate::W<u8, super::WN7816>;
#[doc = "Register WN7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::WN7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GTN`"]
pub type GTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTN`"]
pub struct GTN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Guard Band N"]
    #[inline(always)]
    pub fn gtn(&self) -> GTN_R {
        GTN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Guard Band N"]
    #[inline(always)]
    pub fn gtn(&mut self) -> GTN_W {
        GTN_W { w: self }
    }
}
