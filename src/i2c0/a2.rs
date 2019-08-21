#[doc = "Reader of register A2"]
pub type R = crate::R<u8, super::A2>;
#[doc = "Writer for register A2"]
pub type W = crate::W<u8, super::A2>;
#[doc = "Register A2 `reset()`'s with value 0xc2"]
impl crate::ResetValue for super::A2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc2
    }
}
#[doc = "Reader of field `SAD`"]
pub type SAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAD`"]
pub struct SAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    pub fn sad(&self) -> SAD_R {
        SAD_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    pub fn sad(&mut self) -> SAD_W {
        SAD_W { w: self }
    }
}
