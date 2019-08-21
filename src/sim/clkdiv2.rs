#[doc = "Reader of register CLKDIV2"]
pub type R = crate::R<u32, super::CLKDIV2>;
#[doc = "Writer for register CLKDIV2"]
pub type W = crate::W<u32, super::CLKDIV2>;
#[doc = "Register CLKDIV2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBFRAC`"]
pub type USBFRAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFRAC`"]
pub struct USBFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFRAC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `USBDIV`"]
pub type USBDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBDIV`"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    pub fn usbfrac(&self) -> USBFRAC_R {
        USBFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    pub fn usbfrac(&mut self) -> USBFRAC_W {
        USBFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
}
