#[doc = "Reader of register RA"]
pub type R = crate::R<u8, super::RA>;
#[doc = "Writer for register RA"]
pub type W = crate::W<u8, super::RA>;
#[doc = "Register RA `reset()`'s with value 0"]
impl crate::ResetValue for super::RA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAD`"]
pub type RAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAD`"]
pub struct RAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    pub fn rad(&self) -> RAD_R {
        RAD_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    pub fn rad(&mut self) -> RAD_W {
        RAD_W { w: self }
    }
}
