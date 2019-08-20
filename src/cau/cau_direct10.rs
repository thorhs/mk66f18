#[doc = "Writer for register CAU_DIRECT10"]
pub type W = crate::W<u32, super::CAU_DIRECT10>;
#[doc = "Register CAU_DIRECT10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_DIRECT10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAU_DIRECT10`"]
pub struct CAU_DIRECT10_W<'a> {
    w: &'a mut W,
}
impl<'a> CAU_DIRECT10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Direct register 10"]
    #[inline(always)]
    pub fn cau_direct10(&mut self) -> CAU_DIRECT10_W {
        CAU_DIRECT10_W { w: self }
    }
}
