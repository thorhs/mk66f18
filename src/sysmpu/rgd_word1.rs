#[doc = "Reader of register RGD%s_WORD1"]
pub type R = crate::R<u32, super::RGD_WORD1>;
#[doc = "Writer for register RGD%s_WORD1"]
pub type W = crate::W<u32, super::RGD_WORD1>;
#[doc = "Register RGD%s_WORD1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RGD_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ENDADDR`"]
pub type ENDADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENDADDR`"]
pub struct ENDADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - End Address"]
    #[inline(always)]
    pub fn endaddr(&self) -> ENDADDR_R {
        ENDADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:31 - End Address"]
    #[inline(always)]
    pub fn endaddr(&mut self) -> ENDADDR_W {
        ENDADDR_W { w: self }
    }
}
