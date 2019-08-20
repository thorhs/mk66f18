#[doc = "Reader of register WF7816"]
pub type R = crate::R<u8, super::WF7816>;
#[doc = "Writer for register WF7816"]
pub type W = crate::W<u8, super::WF7816>;
#[doc = "Register WF7816 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::WF7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `GTFD`"]
pub type GTFD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTFD`"]
pub struct GTFD_W<'a> {
    w: &'a mut W,
}
impl<'a> GTFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    pub fn gtfd(&self) -> GTFD_R {
        GTFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    pub fn gtfd(&mut self) -> GTFD_W {
        GTFD_W { w: self }
    }
}
