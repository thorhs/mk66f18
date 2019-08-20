#[doc = "Reader of register TIMER2_BC12"]
pub type R = crate::R<u32, super::TIMER2_BC12>;
#[doc = "Writer for register TIMER2_BC12"]
pub type W = crate::W<u32, super::TIMER2_BC12>;
#[doc = "Register TIMER2_BC12 `reset()`'s with value 0x0001_0028"]
impl crate::ResetValue for super::TIMER2_BC12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0028
    }
}
#[doc = "Reader of field `TVDMSRC_ON`"]
pub type TVDMSRC_ON_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TVDMSRC_ON`"]
pub struct TVDMSRC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> TVDMSRC_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `TWAIT_AFTER_PRD`"]
pub type TWAIT_AFTER_PRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TWAIT_AFTER_PRD`"]
pub struct TWAIT_AFTER_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TWAIT_AFTER_PRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline(always)]
    pub fn tvdmsrc_on(&self) -> TVDMSRC_ON_R {
        TVDMSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline(always)]
    pub fn twait_after_prd(&self) -> TWAIT_AFTER_PRD_R {
        TWAIT_AFTER_PRD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline(always)]
    pub fn tvdmsrc_on(&mut self) -> TVDMSRC_ON_W {
        TVDMSRC_ON_W { w: self }
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline(always)]
    pub fn twait_after_prd(&mut self) -> TWAIT_AFTER_PRD_W {
        TWAIT_AFTER_PRD_W { w: self }
    }
}
