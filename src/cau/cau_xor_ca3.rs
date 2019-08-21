#[doc = "Writer for register CAU_XOR_CA3"]
pub type W = crate::W<u32, super::CAU_XOR_CA3>;
#[doc = "Register CAU_XOR_CA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_XOR_CA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA3`"]
pub struct CA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA3"]
    #[inline(always)]
    pub fn ca3(&mut self) -> CA3_W {
        CA3_W { w: self }
    }
}
