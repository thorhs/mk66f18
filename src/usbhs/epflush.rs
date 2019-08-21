#[doc = "Reader of register EPFLUSH"]
pub type R = crate::R<u32, super::EPFLUSH>;
#[doc = "Writer for register EPFLUSH"]
pub type W = crate::W<u32, super::EPFLUSH>;
#[doc = "Register EPFLUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::EPFLUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FERB`"]
pub type FERB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FERB`"]
pub struct FERB_W<'a> {
    w: &'a mut W,
}
impl<'a> FERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FETB`"]
pub type FETB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FETB`"]
pub struct FETB_W<'a> {
    w: &'a mut W,
}
impl<'a> FETB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flush Endpoint Receive Buffer"]
    #[inline(always)]
    pub fn ferb(&self) -> FERB_R {
        FERB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Flush Endpoint Transmit Buffer"]
    #[inline(always)]
    pub fn fetb(&self) -> FETB_R {
        FETB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flush Endpoint Receive Buffer"]
    #[inline(always)]
    pub fn ferb(&mut self) -> FERB_W {
        FERB_W { w: self }
    }
    #[doc = "Bits 16:19 - Flush Endpoint Transmit Buffer"]
    #[inline(always)]
    pub fn fetb(&mut self) -> FETB_W {
        FETB_W { w: self }
    }
}
