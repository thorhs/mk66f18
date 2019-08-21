#[doc = "Writer for register CAU_LDR_CA6"]
pub type W = crate::W<u32, super::CAU_LDR_CA6>;
#[doc = "Register CAU_LDR_CA6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_LDR_CA6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA6`"]
pub struct CA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA6"]
    #[inline(always)]
    pub fn ca6(&mut self) -> CA6_W {
        CA6_W { w: self }
    }
}
