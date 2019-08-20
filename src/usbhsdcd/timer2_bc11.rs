#[doc = "Reader of register TIMER2_BC11"]
pub type R = crate::R<u32, super::TIMER2_BC11>;
#[doc = "Writer for register TIMER2_BC11"]
pub type W = crate::W<u32, super::TIMER2_BC11>;
#[doc = "Register TIMER2_BC11 `reset()`'s with value 0x0028_0001"]
impl crate::ResetValue for super::TIMER2_BC11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0028_0001
    }
}
#[doc = "Reader of field `CHECK_DM`"]
pub type CHECK_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHECK_DM`"]
pub struct CHECK_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TVDPSRC_CON`"]
pub type TVDPSRC_CON_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TVDPSRC_CON`"]
pub struct TVDPSRC_CON_W<'a> {
    w: &'a mut W,
}
impl<'a> TVDPSRC_CON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    pub fn check_dm(&self) -> CHECK_DM_R {
        CHECK_DM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn tvdpsrc_con(&self) -> TVDPSRC_CON_R {
        TVDPSRC_CON_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    pub fn check_dm(&mut self) -> CHECK_DM_W {
        CHECK_DM_W { w: self }
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn tvdpsrc_con(&mut self) -> TVDPSRC_CON_W {
        TVDPSRC_CON_W { w: self }
    }
}
