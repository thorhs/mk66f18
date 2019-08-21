#[doc = "Reader of register C3"]
pub type R = crate::R<u8, super::C3>;
#[doc = "Writer for register C3"]
pub type W = crate::W<u8, super::C3>;
#[doc = "Register C3 `reset()`'s with value 0"]
impl crate::ResetValue for super::C3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCTRIM`"]
pub type SCTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCTRIM`"]
pub struct SCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn sctrim(&self) -> SCTRIM_R {
        SCTRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn sctrim(&mut self) -> SCTRIM_W {
        SCTRIM_W { w: self }
    }
}
