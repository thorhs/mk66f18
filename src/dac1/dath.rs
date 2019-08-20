#[doc = "Reader of register DAT%sH"]
pub type R = crate::R<u8, super::DATH>;
#[doc = "Writer for register DAT%sH"]
pub type W = crate::W<u8, super::DATH>;
#[doc = "Register DAT%sH `reset()`'s with value 0"]
impl crate::ResetValue for super::DATH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA1`"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DATA1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
}
