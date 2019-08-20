#[doc = "Writer for register ER"]
pub type W = crate::W<u32, super::ER>;
#[doc = "Register ER `reset()`'s with value 0"]
impl crate::ResetValue for super::ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXT_ENT`"]
pub struct EXT_ENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - External Entropy"]
    #[inline(always)]
    pub fn ext_ent(&mut self) -> EXT_ENT_W {
        EXT_ENT_W { w: self }
    }
}
