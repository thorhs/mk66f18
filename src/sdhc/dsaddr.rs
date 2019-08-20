#[doc = "Reader of register DSADDR"]
pub type R = crate::R<u32, super::DSADDR>;
#[doc = "Writer for register DSADDR"]
pub type W = crate::W<u32, super::DSADDR>;
#[doc = "Register DSADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSADDR`"]
pub type DSADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSADDR`"]
pub struct DSADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - DMA System Address"]
    #[inline(always)]
    pub fn dsaddr(&self) -> DSADDR_R {
        DSADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA System Address"]
    #[inline(always)]
    pub fn dsaddr(&mut self) -> DSADDR_W {
        DSADDR_W { w: self }
    }
}
