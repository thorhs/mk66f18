#[doc = "Writer for register CAU_AESC_CA1"]
pub type W = crate::W<u32, super::CAU_AESC_CA1>;
#[doc = "Register CAU_AESC_CA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_AESC_CA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA1`"]
pub struct CA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA1"]
    #[inline(always)]
    pub fn ca1(&mut self) -> CA1_W {
        CA1_W { w: self }
    }
}
