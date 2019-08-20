#[doc = "Reader of register CLKDIV3"]
pub type R = crate::R<u32, super::CLKDIV3>;
#[doc = "Writer for register CLKDIV3"]
pub type W = crate::W<u32, super::CLKDIV3>;
#[doc = "Register CLKDIV3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLFLLFRAC`"]
pub type PLLFLLFRAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLFLLFRAC`"]
pub struct PLLFLLFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLFRAC_W<'a> {
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
#[doc = "Reader of field `PLLFLLDIV`"]
pub type PLLFLLDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLFLLDIV`"]
pub struct PLLFLLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLLFLL clock divider fraction"]
    #[inline(always)]
    pub fn pllfllfrac(&self) -> PLLFLLFRAC_R {
        PLLFLLFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - PLLFLL clock divider divisor"]
    #[inline(always)]
    pub fn pllflldiv(&self) -> PLLFLLDIV_R {
        PLLFLLDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLLFLL clock divider fraction"]
    #[inline(always)]
    pub fn pllfllfrac(&mut self) -> PLLFLLFRAC_W {
        PLLFLLFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - PLLFLL clock divider divisor"]
    #[inline(always)]
    pub fn pllflldiv(&mut self) -> PLLFLLDIV_W {
        PLLFLLDIV_W { w: self }
    }
}
