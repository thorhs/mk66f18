#[doc = "Reader of register A1"]
pub type R = crate::R<u8, super::A1>;
#[doc = "Writer for register A1"]
pub type W = crate::W<u8, super::A1>;
#[doc = "Register A1 `reset()`'s with value 0"]
impl crate::ResetValue for super::A1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AD`"]
pub type AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD`"]
pub struct AD_W<'a> {
    w: &'a mut W,
}
impl<'a> AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn ad(&mut self) -> AD_W {
        AD_W { w: self }
    }
}
