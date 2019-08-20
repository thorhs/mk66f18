#[doc = "Writer for register CAU_AESIC_CA2"]
pub type W = crate::W<u32, super::CAU_AESIC_CA2>;
#[doc = "Register CAU_AESIC_CA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_AESIC_CA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA2`"]
pub struct CA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA2"]
    #[inline(always)]
    pub fn ca2(&mut self) -> CA2_W {
        CA2_W { w: self }
    }
}
