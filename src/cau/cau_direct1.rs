#[doc = "Writer for register CAU_DIRECT1"]
pub type W = crate::W<u32, super::CAU_DIRECT1>;
#[doc = "Register CAU_DIRECT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_DIRECT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAU_DIRECT1`"]
pub struct CAU_DIRECT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAU_DIRECT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Direct register 1"]
    #[inline(always)]
    pub fn cau_direct1(&mut self) -> CAU_DIRECT1_W {
        CAU_DIRECT1_W { w: self }
    }
}
