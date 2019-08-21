#[doc = "Reader of register C4"]
pub type R = crate::R<u8, super::C4>;
#[doc = "Writer for register C4"]
pub type W = crate::W<u8, super::C4>;
#[doc = "Register C4 `reset()`'s with value 0"]
impl crate::ResetValue for super::C4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCFTRIM`"]
pub type SCFTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCFTRIM`"]
pub struct SCFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFTRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FCTRIM`"]
pub type FCTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCTRIM`"]
pub struct FCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u8) & 0x0f) << 1);
        self.w
    }
}
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRST_DRS_A {
    #[doc = "0: Encoding 0 - Low range (reset default)."]
    _00,
    #[doc = "1: Encoding 1 - Mid range."]
    _01,
    #[doc = "2: Encoding 2 - Mid-high range."]
    _10,
    #[doc = "3: Encoding 3 - High range."]
    _11,
}
impl From<DRST_DRS_A> for u8 {
    #[inline(always)]
    fn from(variant: DRST_DRS_A) -> Self {
        match variant {
            DRST_DRS_A::_00 => 0,
            DRST_DRS_A::_01 => 1,
            DRST_DRS_A::_10 => 2,
            DRST_DRS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `DRST_DRS`"]
pub type DRST_DRS_R = crate::R<u8, DRST_DRS_A>;
impl DRST_DRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRST_DRS_A {
        match self.bits {
            0 => DRST_DRS_A::_00,
            1 => DRST_DRS_A::_01,
            2 => DRST_DRS_A::_10,
            3 => DRST_DRS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DRST_DRS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DRST_DRS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DRST_DRS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DRST_DRS_A::_11
    }
}
#[doc = "Write proxy for field `DRST_DRS`"]
pub struct DRST_DRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRST_DRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRST_DRS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Encoding 0 - Low range (reset default)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_00)
    }
    #[doc = "Encoding 1 - Mid range."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_01)
    }
    #[doc = "Encoding 2 - Mid-high range."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_10)
    }
    #[doc = "Encoding 3 - High range."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DRST_DRS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u8) & 0x03) << 5);
        self.w
    }
}
#[doc = "DCO Maximum Frequency with 32.768 kHz Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMX32_A {
    #[doc = "0: DCO has a default range of 25%."]
    _0,
    #[doc = "1: DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    _1,
}
impl From<DMX32_A> for bool {
    #[inline(always)]
    fn from(variant: DMX32_A) -> Self {
        match variant {
            DMX32_A::_0 => false,
            DMX32_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMX32`"]
pub type DMX32_R = crate::R<bool, DMX32_A>;
impl DMX32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMX32_A {
        match self.bits {
            false => DMX32_A::_0,
            true => DMX32_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMX32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMX32_A::_1
    }
}
#[doc = "Write proxy for field `DMX32`"]
pub struct DMX32_W<'a> {
    w: &'a mut W,
}
impl<'a> DMX32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMX32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DCO has a default range of 25%."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMX32_A::_0)
    }
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMX32_A::_1)
    }
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
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&self) -> SCFTRIM_R {
        SCFTRIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn fctrim(&self) -> FCTRIM_R {
        FCTRIM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    pub fn drst_drs(&self) -> DRST_DRS_R {
        DRST_DRS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    pub fn dmx32(&self) -> DMX32_R {
        DMX32_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&mut self) -> SCFTRIM_W {
        SCFTRIM_W { w: self }
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn fctrim(&mut self) -> FCTRIM_W {
        FCTRIM_W { w: self }
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    pub fn drst_drs(&mut self) -> DRST_DRS_W {
        DRST_DRS_W { w: self }
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    pub fn dmx32(&mut self) -> DMX32_W {
        DMX32_W { w: self }
    }
}
