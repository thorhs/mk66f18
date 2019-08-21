#[doc = "Reader of register GPOLYL"]
pub type R = crate::R<u16, super::GPOLYL>;
#[doc = "Writer for register GPOLYL"]
pub type W = crate::W<u16, super::GPOLYL>;
#[doc = "Register GPOLYL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::GPOLYL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `GPOLYL`"]
pub type GPOLYL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPOLYL`"]
pub struct GPOLYL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyl(&self) -> GPOLYL_R {
        GPOLYL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyl(&mut self) -> GPOLYL_W {
        GPOLYL_W { w: self }
    }
}
