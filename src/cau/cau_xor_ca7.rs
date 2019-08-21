#[doc = "Writer for register CAU_XOR_CA7"]
pub type W = crate::W<u32, super::CAU_XOR_CA7>;
#[doc = "Register CAU_XOR_CA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_XOR_CA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA7`"]
pub struct CA7_W<'a> {
    w: &'a mut W,
}
impl<'a> CA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA7"]
    #[inline(always)]
    pub fn ca7(&mut self) -> CA7_W {
        CA7_W { w: self }
    }
}
