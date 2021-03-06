#[doc = "Writer for register CAU_DIRECT5"]
pub type W = crate::W<u32, super::CAU_DIRECT5>;
#[doc = "Register CAU_DIRECT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_DIRECT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAU_DIRECT5`"]
pub struct CAU_DIRECT5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAU_DIRECT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Direct register 5"]
    #[inline(always)]
    pub fn cau_direct5(&mut self) -> CAU_DIRECT5_W {
        CAU_DIRECT5_W { w: self }
    }
}
