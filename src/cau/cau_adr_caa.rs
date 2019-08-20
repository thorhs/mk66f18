#[doc = "Writer for register CAU_ADR_CAA"]
pub type W = crate::W<u32, super::CAU_ADR_CAA>;
#[doc = "Register CAU_ADR_CAA `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_ADR_CAA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACC`"]
pub struct ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - ACC"]
    #[inline(always)]
    pub fn acc(&mut self) -> ACC_W {
        ACC_W { w: self }
    }
}