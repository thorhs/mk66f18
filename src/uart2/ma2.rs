#[doc = "Reader of register MA2"]
pub type R = crate::R<u8, super::MA2>;
#[doc = "Writer for register MA2"]
pub type W = crate::W<u8, super::MA2>;
#[doc = "Register MA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MA2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MA`"]
pub type MA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MA`"]
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
}
