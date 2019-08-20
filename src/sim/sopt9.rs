#[doc = "Reader of register SOPT9"]
pub type R = crate::R<u32, super::SOPT9>;
#[doc = "Writer for register SOPT9"]
pub type W = crate::W<u32, super::SOPT9>;
#[doc = "Register SOPT9 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TPM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CH0SRC_A {
    #[doc = "0: TPM1_CH0 signal"]
    _00,
    #[doc = "1: CMP0 output"]
    _01,
    #[doc = "2: CMP1 output"]
    _10,
}
impl From<TPM1CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPM1CH0SRC_A) -> Self {
        match variant {
            TPM1CH0SRC_A::_00 => 0,
            TPM1CH0SRC_A::_01 => 1,
            TPM1CH0SRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `TPM1CH0SRC`"]
pub type TPM1CH0SRC_R = crate::R<u8, TPM1CH0SRC_A>;
impl TPM1CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TPM1CH0SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TPM1CH0SRC_A::_00),
            1 => Val(TPM1CH0SRC_A::_01),
            2 => Val(TPM1CH0SRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPM1CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPM1CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPM1CH0SRC_A::_10
    }
}
#[doc = "Write proxy for field `TPM1CH0SRC`"]
pub struct TPM1CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CH0SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPM1CH0SRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "TPM2 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CH0SRC_A {
    #[doc = "0: TPM2_CH0 signal"]
    _00,
    #[doc = "1: CMP0 output"]
    _01,
    #[doc = "2: CMP1 output"]
    _10,
}
impl From<TPM2CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPM2CH0SRC_A) -> Self {
        match variant {
            TPM2CH0SRC_A::_00 => 0,
            TPM2CH0SRC_A::_01 => 1,
            TPM2CH0SRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `TPM2CH0SRC`"]
pub type TPM2CH0SRC_R = crate::R<u8, TPM2CH0SRC_A>;
impl TPM2CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TPM2CH0SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TPM2CH0SRC_A::_00),
            1 => Val(TPM2CH0SRC_A::_01),
            2 => Val(TPM2CH0SRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPM2CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPM2CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPM2CH0SRC_A::_10
    }
}
#[doc = "Write proxy for field `TPM2CH0SRC`"]
pub struct TPM2CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM2CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM2CH0SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TPM2_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPM2CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPM2CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPM2CH0SRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "TPM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CLKSEL_A {
    #[doc = "0: TPM_CLKIN0 pin"]
    _0,
    #[doc = "1: TPM_CLKIN1 pin"]
    _1,
}
impl From<TPM1CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1CLKSEL_A) -> Self {
        match variant {
            TPM1CLKSEL_A::_0 => false,
            TPM1CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TPM1CLKSEL`"]
pub type TPM1CLKSEL_R = crate::R<bool, TPM1CLKSEL_A>;
impl TPM1CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1CLKSEL_A {
        match self.bits {
            false => TPM1CLKSEL_A::_0,
            true => TPM1CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM1CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM1CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TPM1CLKSEL`"]
pub struct TPM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM_CLKIN0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_0)
    }
    #[doc = "TPM_CLKIN1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "TPM2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CLKSEL_A {
    #[doc = "0: TPM_CLKIN0 pin"]
    _0,
    #[doc = "1: TPM_CLKIN1 pin"]
    _1,
}
impl From<TPM2CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TPM2CLKSEL_A) -> Self {
        match variant {
            TPM2CLKSEL_A::_0 => false,
            TPM2CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TPM2CLKSEL`"]
pub type TPM2CLKSEL_R = crate::R<bool, TPM2CLKSEL_A>;
impl TPM2CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM2CLKSEL_A {
        match self.bits {
            false => TPM2CLKSEL_A::_0,
            true => TPM2CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM2CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM2CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TPM2CLKSEL`"]
pub struct TPM2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM2CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM2CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM_CLKIN0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CLKSEL_A::_0)
    }
    #[doc = "TPM_CLKIN1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&self) -> TPM1CH0SRC_R {
        TPM1CH0SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - TPM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm2ch0src(&self) -> TPM2CH0SRC_R {
        TPM2CH0SRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&self) -> TPM1CLKSEL_R {
        TPM1CLKSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm2clksel(&self) -> TPM2CLKSEL_R {
        TPM2CLKSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&mut self) -> TPM1CH0SRC_W {
        TPM1CH0SRC_W { w: self }
    }
    #[doc = "Bits 20:21 - TPM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm2ch0src(&mut self) -> TPM2CH0SRC_W {
        TPM2CH0SRC_W { w: self }
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&mut self) -> TPM1CLKSEL_W {
        TPM1CLKSEL_W { w: self }
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm2clksel(&mut self) -> TPM2CLKSEL_W {
        TPM2CLKSEL_W { w: self }
    }
}
