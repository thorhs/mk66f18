#[doc = "Reader of register DAT%sL"]
pub type R = crate::R<u8, super::DATL>;
#[doc = "Writer for register DAT%sL"]
pub type W = crate::W<u8, super::DATL>;
#[doc = "Register DAT%sL `reset()`'s with value 0"]
impl crate::ResetValue for super::DATL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0`"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
}
