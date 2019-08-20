#[doc = "Reader of register GPTIMER%sLD"]
pub type R = crate::R<u32, super::GPTIMERLD>;
#[doc = "Writer for register GPTIMER%sLD"]
pub type W = crate::W<u32, super::GPTIMERLD>;
#[doc = "Register GPTIMER%sLD `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTIMERLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTLD`"]
pub type GPTLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPTLD`"]
pub struct GPTLD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Specifies the value to be loaded into the countdown timer on a reset"]
    #[inline(always)]
    pub fn gptld(&self) -> GPTLD_R {
        GPTLD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the value to be loaded into the countdown timer on a reset"]
    #[inline(always)]
    pub fn gptld(&mut self) -> GPTLD_W {
        GPTLD_W { w: self }
    }
}
