#[doc = "Reader of register DACINT%s"]
pub type R = crate::R<u32, super::DACINT>;
#[doc = "Writer for register DACINT%s"]
pub type W = crate::W<u32, super::DACINT>;
#[doc = "Register DACINT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DACINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DAC Interval"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC Interval"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
}
