#[doc = "Reader of register D"]
pub type R = crate::R<u8, super::D>;
#[doc = "Writer for register D"]
pub type W = crate::W<u8, super::D>;
#[doc = "Register D `reset()`'s with value 0"]
impl crate::ResetValue for super::D {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT`"]
pub type RT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RT`"]
pub struct RT_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
}
