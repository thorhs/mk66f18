#[doc = "Reader of register CLP4"]
pub type R = crate::R<u32, super::CLP4>;
#[doc = "Writer for register CLP4"]
pub type W = crate::W<u32, super::CLP4>;
#[doc = "Register CLP4 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::CLP4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `CLP4`"]
pub type CLP4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLP4`"]
pub struct CLP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp4(&self) -> CLP4_R {
        CLP4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp4(&mut self) -> CLP4_W {
        CLP4_W { w: self }
    }
}
