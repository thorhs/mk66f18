#[doc = "Reader of register ADDR"]
pub type R = crate::R<u8, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u8, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `LSEN`"]
pub type LSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSEN`"]
pub struct LSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Low Speed Enable bit"]
    #[inline(always)]
    pub fn lsen(&self) -> LSEN_R {
        LSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 7 - Low Speed Enable bit"]
    #[inline(always)]
    pub fn lsen(&mut self) -> LSEN_W {
        LSEN_W { w: self }
    }
}
