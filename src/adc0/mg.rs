#[doc = "Reader of register MG"]
pub type R = crate::R<u32, super::MG>;
#[doc = "Writer for register MG"]
pub type W = crate::W<u32, super::MG>;
#[doc = "Register MG `reset()`'s with value 0x8200"]
impl crate::ResetValue for super::MG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8200
    }
}
#[doc = "Reader of field `MG`"]
pub type MG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MG`"]
pub struct MG_W<'a> {
    w: &'a mut W,
}
impl<'a> MG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minus-Side Gain"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minus-Side Gain"]
    #[inline(always)]
    pub fn mg(&mut self) -> MG_W {
        MG_W { w: self }
    }
}
