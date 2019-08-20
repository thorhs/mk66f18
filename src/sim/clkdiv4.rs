#[doc = "Reader of register CLKDIV4"]
pub type R = crate::R<u32, super::CLKDIV4>;
#[doc = "Writer for register CLKDIV4"]
pub type W = crate::W<u32, super::CLKDIV4>;
#[doc = "Register CLKDIV4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRACEFRAC`"]
pub type TRACEFRAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACEFRAC`"]
pub struct TRACEFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEFRAC_W<'a> {
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
#[doc = "Reader of field `TRACEDIV`"]
pub type TRACEDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACEDIV`"]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trace clock divider fraction"]
    #[inline(always)]
    pub fn tracefrac(&self) -> TRACEFRAC_R {
        TRACEFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trace clock divider divisor"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trace clock divider fraction"]
    #[inline(always)]
    pub fn tracefrac(&mut self) -> TRACEFRAC_W {
        TRACEFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - Trace clock divider divisor"]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
}
