#[doc = "Reader of register TIMER0"]
pub type R = crate::R<u32, super::TIMER0>;
#[doc = "Writer for register TIMER0"]
pub type W = crate::W<u32, super::TIMER0>;
#[doc = "Register TIMER0 `reset()`'s with value 0x0010_0000"]
impl crate::ResetValue for super::TIMER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
#[doc = "Reader of field `TUNITCON`"]
pub type TUNITCON_R = crate::R<u16, u16>;
#[doc = "Reader of field `TSEQ_INIT`"]
pub type TSEQ_INIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSEQ_INIT`"]
pub struct TSEQ_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEQ_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub fn tunitcon(&self) -> TUNITCON_R {
        TUNITCON_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    pub fn tseq_init(&self) -> TSEQ_INIT_R {
        TSEQ_INIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    pub fn tseq_init(&mut self) -> TSEQ_INIT_W {
        TSEQ_INIT_W { w: self }
    }
}
