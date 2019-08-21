#[doc = "Reader of register GPOLYH"]
pub type R = crate::R<u16, super::GPOLYH>;
#[doc = "Writer for register GPOLYH"]
pub type W = crate::W<u16, super::GPOLYH>;
#[doc = "Register GPOLYH `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::GPOLYH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `GPOLYH`"]
pub type GPOLYH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPOLYH`"]
pub struct GPOLYH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyh(&self) -> GPOLYH_R {
        GPOLYH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyh(&mut self) -> GPOLYH_W {
        GPOLYH_W { w: self }
    }
}
