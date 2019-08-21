#[doc = "Writer for register CAU_RADR_CA4"]
pub type W = crate::W<u32, super::CAU_RADR_CA4>;
#[doc = "Register CAU_RADR_CA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_RADR_CA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA4`"]
pub struct CA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA4"]
    #[inline(always)]
    pub fn ca4(&mut self) -> CA4_W {
        CA4_W { w: self }
    }
}
