#[doc = "Reader of register PERIODICLISTBASE"]
pub type R = crate::R<u32, super::PERIODICLISTBASE>;
#[doc = "Writer for register PERIODICLISTBASE"]
pub type W = crate::W<u32, super::PERIODICLISTBASE>;
#[doc = "Register PERIODICLISTBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIODICLISTBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERBASE`"]
pub type PERBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERBASE`"]
pub struct PERBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - Base address"]
    #[inline(always)]
    pub fn perbase(&self) -> PERBASE_R {
        PERBASE_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base address"]
    #[inline(always)]
    pub fn perbase(&mut self) -> PERBASE_W {
        PERBASE_W { w: self }
    }
}
