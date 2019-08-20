#[doc = "Reader of register TIMER1"]
pub type R = crate::R<u32, super::TIMER1>;
#[doc = "Writer for register TIMER1"]
pub type W = crate::W<u32, super::TIMER1>;
#[doc = "Register TIMER1 `reset()`'s with value 0x000a_0028"]
impl crate::ResetValue for super::TIMER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000a_0028
    }
}
#[doc = "Reader of field `TVDPSRC_ON`"]
pub type TVDPSRC_ON_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TVDPSRC_ON`"]
pub struct TVDPSRC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> TVDPSRC_ON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `TDCD_DBNC`"]
pub type TDCD_DBNC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDCD_DBNC`"]
pub struct TDCD_DBNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCD_DBNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn tvdpsrc_on(&self) -> TVDPSRC_ON_R {
        TVDPSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn tdcd_dbnc(&self) -> TDCD_DBNC_R {
        TDCD_DBNC_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn tvdpsrc_on(&mut self) -> TVDPSRC_ON_W {
        TVDPSRC_ON_W { w: self }
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn tdcd_dbnc(&mut self) -> TDCD_DBNC_W {
        TDCD_DBNC_W { w: self }
    }
}
