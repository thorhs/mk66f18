#[doc = "Reader of register SCGC2"]
pub type R = crate::R<u32, super::SCGC2>;
#[doc = "Writer for register SCGC2"]
pub type W = crate::W<u32, super::SCGC2>;
#[doc = "Register SCGC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ENET Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<ENET_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_A) -> Self {
        match variant {
            ENET_A::_0 => false,
            ENET_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET`"]
pub type ENET_R = crate::R<bool, ENET_A>;
impl ENET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_A {
        match self.bits {
            false => ENET_A::_0,
            true => ENET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENET_A::_1
    }
}
#[doc = "Write proxy for field `ENET`"]
pub struct ENET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENET_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENET_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "LPUART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<LPUART0_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART0_A) -> Self {
        match variant {
            LPUART0_A::_0 => false,
            LPUART0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART0`"]
pub type LPUART0_R = crate::R<bool, LPUART0_A>;
impl LPUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART0_A {
        match self.bits {
            false => LPUART0_A::_0,
            true => LPUART0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPUART0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPUART0_A::_1
    }
}
#[doc = "Write proxy for field `LPUART0`"]
pub struct LPUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "TPM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<TPM1_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1_A) -> Self {
        match variant {
            TPM1_A::_0 => false,
            TPM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TPM1`"]
pub type TPM1_R = crate::R<bool, TPM1_A>;
impl TPM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1_A {
        match self.bits {
            false => TPM1_A::_0,
            true => TPM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM1_A::_1
    }
}
#[doc = "Write proxy for field `TPM1`"]
pub struct TPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "TPM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<TPM2_A> for bool {
    #[inline(always)]
    fn from(variant: TPM2_A) -> Self {
        match variant {
            TPM2_A::_0 => false,
            TPM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TPM2`"]
pub type TPM2_R = crate::R<bool, TPM2_A>;
impl TPM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM2_A {
        match self.bits {
            false => TPM2_A::_0,
            true => TPM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM2_A::_1
    }
}
#[doc = "Write proxy for field `TPM2`"]
pub struct TPM2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        match variant {
            DAC0_A::_0 => false,
            DAC0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DAC0`"]
pub type DAC0_R = crate::R<bool, DAC0_A>;
impl DAC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Write proxy for field `DAC0`"]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "DAC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<DAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_A) -> Self {
        match variant {
            DAC1_A::_0 => false,
            DAC1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DAC1`"]
pub type DAC1_R = crate::R<bool, DAC1_A>;
impl DAC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_A {
        match self.bits {
            false => DAC1_A::_0,
            true => DAC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC1_A::_1
    }
}
#[doc = "Write proxy for field `DAC1`"]
pub struct DAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ENET Clock Gate Control"]
    #[inline(always)]
    pub fn enet(&self) -> ENET_R {
        ENET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart0(&self) -> LPUART0_R {
        LPUART0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TPM1 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm1(&self) -> TPM1_R {
        TPM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TPM2 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm2(&self) -> TPM2_R {
        TPM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC1 Clock Gate Control"]
    #[inline(always)]
    pub fn dac1(&self) -> DAC1_R {
        DAC1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENET Clock Gate Control"]
    #[inline(always)]
    pub fn enet(&mut self) -> ENET_W {
        ENET_W { w: self }
    }
    #[doc = "Bit 4 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart0(&mut self) -> LPUART0_W {
        LPUART0_W { w: self }
    }
    #[doc = "Bit 9 - TPM1 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm1(&mut self) -> TPM1_W {
        TPM1_W { w: self }
    }
    #[doc = "Bit 10 - TPM2 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm2(&mut self) -> TPM2_W {
        TPM2_W { w: self }
    }
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
    #[doc = "Bit 13 - DAC1 Clock Gate Control"]
    #[inline(always)]
    pub fn dac1(&mut self) -> DAC1_W {
        DAC1_W { w: self }
    }
}
