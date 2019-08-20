#[doc = "Reader of register ADSADDR"]
pub type R = crate::R<u32, super::ADSADDR>;
#[doc = "Writer for register ADSADDR"]
pub type W = crate::W<u32, super::ADSADDR>;
#[doc = "Register ADSADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADSADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSADDR`"]
pub type ADSADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADSADDR`"]
pub struct ADSADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&self) -> ADSADDR_R {
        ADSADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&mut self) -> ADSADDR_W {
        ADSADDR_W { w: self }
    }
}
