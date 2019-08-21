#[doc = "Reader of register FRINDEX"]
pub type R = crate::R<u32, super::FRINDEX>;
#[doc = "Writer for register FRINDEX"]
pub type W = crate::W<u32, super::FRINDEX>;
#[doc = "Register FRINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::FRINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRINDEX`"]
pub type FRINDEX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRINDEX`"]
pub struct FRINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `Reerved`"]
pub type REERVED_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reerved(&self) -> REERVED_R {
        REERVED_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frindex(&mut self) -> FRINDEX_W {
        FRINDEX_W { w: self }
    }
}
