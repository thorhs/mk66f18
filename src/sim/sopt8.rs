#[doc = "Reader of register SOPT8"]
pub type R = crate::R<u32, super::SOPT8>;
#[doc = "Writer for register SOPT8"]
pub type W = crate::W<u32, super::SOPT8>;
#[doc = "Register SOPT8 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM0 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0SYNCBIT_A {
    #[doc = "0: No effect"]
    _0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl From<FTM0SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0SYNCBIT_A) -> Self {
        match variant {
            FTM0SYNCBIT_A::_0 => false,
            FTM0SYNCBIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0SYNCBIT`"]
pub type FTM0SYNCBIT_R = crate::R<bool, FTM0SYNCBIT_A>;
impl FTM0SYNCBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0SYNCBIT_A {
        match self.bits {
            false => FTM0SYNCBIT_A::_0,
            true => FTM0SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0SYNCBIT_A::_1
    }
}
#[doc = "Write proxy for field `FTM0SYNCBIT`"]
pub struct FTM0SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0SYNCBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0SYNCBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0SYNCBIT_A::_1)
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
#[doc = "FTM1 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1SYNCBIT_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl From<FTM1SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1SYNCBIT_A) -> Self {
        match variant {
            FTM1SYNCBIT_A::_0 => false,
            FTM1SYNCBIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM1SYNCBIT`"]
pub type FTM1SYNCBIT_R = crate::R<bool, FTM1SYNCBIT_A>;
impl FTM1SYNCBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1SYNCBIT_A {
        match self.bits {
            false => FTM1SYNCBIT_A::_0,
            true => FTM1SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1SYNCBIT_A::_1
    }
}
#[doc = "Write proxy for field `FTM1SYNCBIT`"]
pub struct FTM1SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1SYNCBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1SYNCBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1SYNCBIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "FTM2 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2SYNCBIT_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl From<FTM2SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2SYNCBIT_A) -> Self {
        match variant {
            FTM2SYNCBIT_A::_0 => false,
            FTM2SYNCBIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM2SYNCBIT`"]
pub type FTM2SYNCBIT_R = crate::R<bool, FTM2SYNCBIT_A>;
impl FTM2SYNCBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2SYNCBIT_A {
        match self.bits {
            false => FTM2SYNCBIT_A::_0,
            true => FTM2SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2SYNCBIT_A::_1
    }
}
#[doc = "Write proxy for field `FTM2SYNCBIT`"]
pub struct FTM2SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2SYNCBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2SYNCBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2SYNCBIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "FTM3 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3SYNCBIT_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM3, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl From<FTM3SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3SYNCBIT_A) -> Self {
        match variant {
            FTM3SYNCBIT_A::_0 => false,
            FTM3SYNCBIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3SYNCBIT`"]
pub type FTM3SYNCBIT_R = crate::R<bool, FTM3SYNCBIT_A>;
impl FTM3SYNCBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3SYNCBIT_A {
        match self.bits {
            false => FTM3SYNCBIT_A::_0,
            true => FTM3SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3SYNCBIT_A::_1
    }
}
#[doc = "Write proxy for field `FTM3SYNCBIT`"]
pub struct FTM3SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3SYNCBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3SYNCBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM3, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3SYNCBIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "FTM0 channel 0 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH0SRC_A {
    #[doc = "0: FTM0_CH0 pin is output of FTM0 channel 0 output"]
    _0,
    #[doc = "1: FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH0SRC_A) -> Self {
        match variant {
            FTM0OCH0SRC_A::_0 => false,
            FTM0OCH0SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH0SRC`"]
pub type FTM0OCH0SRC_R = crate::R<bool, FTM0OCH0SRC_A>;
impl FTM0OCH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH0SRC_A {
        match self.bits {
            false => FTM0OCH0SRC_A::_0,
            true => FTM0OCH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH0SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH0SRC`"]
pub struct FTM0OCH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH0SRC_A::_0)
    }
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH0SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "FTM0 channel 1 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH1SRC_A {
    #[doc = "0: FTM0_CH1 pin is output of FTM0 channel 1 output"]
    _0,
    #[doc = "1: FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH1SRC_A) -> Self {
        match variant {
            FTM0OCH1SRC_A::_0 => false,
            FTM0OCH1SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH1SRC`"]
pub type FTM0OCH1SRC_R = crate::R<bool, FTM0OCH1SRC_A>;
impl FTM0OCH1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH1SRC_A {
        match self.bits {
            false => FTM0OCH1SRC_A::_0,
            true => FTM0OCH1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH1SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH1SRC`"]
pub struct FTM0OCH1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH1SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH1SRC_A::_0)
    }
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH1SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "FTM0 channel 2 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH2SRC_A {
    #[doc = "0: FTM0_CH2 pin is output of FTM0 channel 2 output"]
    _0,
    #[doc = "1: FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH2SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH2SRC_A) -> Self {
        match variant {
            FTM0OCH2SRC_A::_0 => false,
            FTM0OCH2SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH2SRC`"]
pub type FTM0OCH2SRC_R = crate::R<bool, FTM0OCH2SRC_A>;
impl FTM0OCH2SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH2SRC_A {
        match self.bits {
            false => FTM0OCH2SRC_A::_0,
            true => FTM0OCH2SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH2SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH2SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH2SRC`"]
pub struct FTM0OCH2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH2SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH2SRC_A::_0)
    }
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH2SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "FTM0 channel 3 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH3SRC_A {
    #[doc = "0: FTM0_CH3 pin is output of FTM0 channel 3 output"]
    _0,
    #[doc = "1: FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH3SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH3SRC_A) -> Self {
        match variant {
            FTM0OCH3SRC_A::_0 => false,
            FTM0OCH3SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH3SRC`"]
pub type FTM0OCH3SRC_R = crate::R<bool, FTM0OCH3SRC_A>;
impl FTM0OCH3SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH3SRC_A {
        match self.bits {
            false => FTM0OCH3SRC_A::_0,
            true => FTM0OCH3SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH3SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH3SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH3SRC`"]
pub struct FTM0OCH3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH3SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH3SRC_A::_0)
    }
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH3SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "FTM0 channel 4 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH4SRC_A {
    #[doc = "0: FTM0_CH4 pin is output of FTM0 channel 4 output"]
    _0,
    #[doc = "1: FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH4SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH4SRC_A) -> Self {
        match variant {
            FTM0OCH4SRC_A::_0 => false,
            FTM0OCH4SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH4SRC`"]
pub type FTM0OCH4SRC_R = crate::R<bool, FTM0OCH4SRC_A>;
impl FTM0OCH4SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH4SRC_A {
        match self.bits {
            false => FTM0OCH4SRC_A::_0,
            true => FTM0OCH4SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH4SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH4SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH4SRC`"]
pub struct FTM0OCH4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH4SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH4SRC_A::_0)
    }
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH4SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "FTM0 channel 5 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH5SRC_A {
    #[doc = "0: FTM0_CH5 pin is output of FTM0 channel 5 output"]
    _0,
    #[doc = "1: FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH5SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH5SRC_A) -> Self {
        match variant {
            FTM0OCH5SRC_A::_0 => false,
            FTM0OCH5SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH5SRC`"]
pub type FTM0OCH5SRC_R = crate::R<bool, FTM0OCH5SRC_A>;
impl FTM0OCH5SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH5SRC_A {
        match self.bits {
            false => FTM0OCH5SRC_A::_0,
            true => FTM0OCH5SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH5SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH5SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH5SRC`"]
pub struct FTM0OCH5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH5SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH5SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH5SRC_A::_0)
    }
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH5SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "FTM0 channel 6 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH6SRC_A {
    #[doc = "0: FTM0_CH6 pin is output of FTM0 channel 6 output"]
    _0,
    #[doc = "1: FTM0_CH6 pin is output of FTM0 channel 6 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH6SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH6SRC_A) -> Self {
        match variant {
            FTM0OCH6SRC_A::_0 => false,
            FTM0OCH6SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH6SRC`"]
pub type FTM0OCH6SRC_R = crate::R<bool, FTM0OCH6SRC_A>;
impl FTM0OCH6SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH6SRC_A {
        match self.bits {
            false => FTM0OCH6SRC_A::_0,
            true => FTM0OCH6SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH6SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH6SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH6SRC`"]
pub struct FTM0OCH6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH6SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH6SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH6SRC_A::_0)
    }
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH6SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "FTM0 channel 7 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH7SRC_A {
    #[doc = "0: FTM0_CH7 pin is output of FTM0 channel 7 output"]
    _0,
    #[doc = "1: FTM0_CH7 pin is output of FTM0 channel 7 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl From<FTM0OCH7SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH7SRC_A) -> Self {
        match variant {
            FTM0OCH7SRC_A::_0 => false,
            FTM0OCH7SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0OCH7SRC`"]
pub type FTM0OCH7SRC_R = crate::R<bool, FTM0OCH7SRC_A>;
impl FTM0OCH7SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH7SRC_A {
        match self.bits {
            false => FTM0OCH7SRC_A::_0,
            true => FTM0OCH7SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH7SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH7SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0OCH7SRC`"]
pub struct FTM0OCH7SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0OCH7SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0OCH7SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH7SRC_A::_0)
    }
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH7SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "FTM3 channel 0 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH0SRC_A {
    #[doc = "0: FTM3_CH0 pin is output of FTM3 channel 0 output"]
    _0,
    #[doc = "1: FTM3_CH0 pin is output of FTM3 channel 0 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH0SRC_A) -> Self {
        match variant {
            FTM3OCH0SRC_A::_0 => false,
            FTM3OCH0SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH0SRC`"]
pub type FTM3OCH0SRC_R = crate::R<bool, FTM3OCH0SRC_A>;
impl FTM3OCH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH0SRC_A {
        match self.bits {
            false => FTM3OCH0SRC_A::_0,
            true => FTM3OCH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH0SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH0SRC`"]
pub struct FTM3OCH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH0SRC_A::_0)
    }
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH0SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "FTM3 channel 1 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH1SRC_A {
    #[doc = "0: FTM3_CH1 pin is output of FTM3 channel 1 output"]
    _0,
    #[doc = "1: FTM3_CH1 pin is output of FTM3 channel 1 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH1SRC_A) -> Self {
        match variant {
            FTM3OCH1SRC_A::_0 => false,
            FTM3OCH1SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH1SRC`"]
pub type FTM3OCH1SRC_R = crate::R<bool, FTM3OCH1SRC_A>;
impl FTM3OCH1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH1SRC_A {
        match self.bits {
            false => FTM3OCH1SRC_A::_0,
            true => FTM3OCH1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH1SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH1SRC`"]
pub struct FTM3OCH1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH1SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH1SRC_A::_0)
    }
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH1SRC_A::_1)
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
#[doc = "FTM3 channel 2 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH2SRC_A {
    #[doc = "0: FTM3_CH2 pin is output of FTM3 channel 2 output"]
    _0,
    #[doc = "1: FTM3_CH2 pin is output of FTM3 channel 2 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH2SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH2SRC_A) -> Self {
        match variant {
            FTM3OCH2SRC_A::_0 => false,
            FTM3OCH2SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH2SRC`"]
pub type FTM3OCH2SRC_R = crate::R<bool, FTM3OCH2SRC_A>;
impl FTM3OCH2SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH2SRC_A {
        match self.bits {
            false => FTM3OCH2SRC_A::_0,
            true => FTM3OCH2SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH2SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH2SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH2SRC`"]
pub struct FTM3OCH2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH2SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH2SRC_A::_0)
    }
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH2SRC_A::_1)
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
#[doc = "FTM3 channel 3 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH3SRC_A {
    #[doc = "0: FTM3_CH3 pin is output of FTM3 channel 3 output"]
    _0,
    #[doc = "1: FTM3_CH3 pin is output of FTM3 channel 3 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH3SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH3SRC_A) -> Self {
        match variant {
            FTM3OCH3SRC_A::_0 => false,
            FTM3OCH3SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH3SRC`"]
pub type FTM3OCH3SRC_R = crate::R<bool, FTM3OCH3SRC_A>;
impl FTM3OCH3SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH3SRC_A {
        match self.bits {
            false => FTM3OCH3SRC_A::_0,
            true => FTM3OCH3SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH3SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH3SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH3SRC`"]
pub struct FTM3OCH3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH3SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH3SRC_A::_0)
    }
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH3SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "FTM3 channel 4 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH4SRC_A {
    #[doc = "0: FTM3_CH4 pin is output of FTM3 channel 4 output"]
    _0,
    #[doc = "1: FTM3_CH4 pin is output of FTM3 channel 4 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH4SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH4SRC_A) -> Self {
        match variant {
            FTM3OCH4SRC_A::_0 => false,
            FTM3OCH4SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH4SRC`"]
pub type FTM3OCH4SRC_R = crate::R<bool, FTM3OCH4SRC_A>;
impl FTM3OCH4SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH4SRC_A {
        match self.bits {
            false => FTM3OCH4SRC_A::_0,
            true => FTM3OCH4SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH4SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH4SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH4SRC`"]
pub struct FTM3OCH4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH4SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH4SRC_A::_0)
    }
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH4SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "FTM3 channel 5 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH5SRC_A {
    #[doc = "0: FTM3_CH5 pin is output of FTM3 channel 5 output"]
    _0,
    #[doc = "1: FTM3_CH5 pin is output of FTM3 channel 5 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH5SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH5SRC_A) -> Self {
        match variant {
            FTM3OCH5SRC_A::_0 => false,
            FTM3OCH5SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH5SRC`"]
pub type FTM3OCH5SRC_R = crate::R<bool, FTM3OCH5SRC_A>;
impl FTM3OCH5SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH5SRC_A {
        match self.bits {
            false => FTM3OCH5SRC_A::_0,
            true => FTM3OCH5SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH5SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH5SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH5SRC`"]
pub struct FTM3OCH5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH5SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH5SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH5SRC_A::_0)
    }
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH5SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "FTM3 channel 6 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH6SRC_A {
    #[doc = "0: FTM3_CH6 pin is output of FTM3 channel 6 output"]
    _0,
    #[doc = "1: FTM3_CH6 pin is output of FTM3 channel 6 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH6SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH6SRC_A) -> Self {
        match variant {
            FTM3OCH6SRC_A::_0 => false,
            FTM3OCH6SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH6SRC`"]
pub type FTM3OCH6SRC_R = crate::R<bool, FTM3OCH6SRC_A>;
impl FTM3OCH6SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH6SRC_A {
        match self.bits {
            false => FTM3OCH6SRC_A::_0,
            true => FTM3OCH6SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH6SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH6SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH6SRC`"]
pub struct FTM3OCH6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH6SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH6SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH6SRC_A::_0)
    }
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH6SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "FTM3 channel 7 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH7SRC_A {
    #[doc = "0: FTM3_CH7 pin is output of FTM3 channel 7 output"]
    _0,
    #[doc = "1: FTM3_CH7 pin is output of FTM3 channel 7 output modulated by FTM2 channel 1 output."]
    _1,
}
impl From<FTM3OCH7SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3OCH7SRC_A) -> Self {
        match variant {
            FTM3OCH7SRC_A::_0 => false,
            FTM3OCH7SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3OCH7SRC`"]
pub type FTM3OCH7SRC_R = crate::R<bool, FTM3OCH7SRC_A>;
impl FTM3OCH7SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3OCH7SRC_A {
        match self.bits {
            false => FTM3OCH7SRC_A::_0,
            true => FTM3OCH7SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH7SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH7SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3OCH7SRC`"]
pub struct FTM3OCH7SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3OCH7SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3OCH7SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH7SRC_A::_0)
    }
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output modulated by FTM2 channel 1 output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH7SRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm0syncbit(&self) -> FTM0SYNCBIT_R {
        FTM0SYNCBIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm1syncbit(&self) -> FTM1SYNCBIT_R {
        FTM1SYNCBIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm2syncbit(&self) -> FTM2SYNCBIT_R {
        FTM2SYNCBIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FTM3 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm3syncbit(&self) -> FTM3SYNCBIT_R {
        FTM3SYNCBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline(always)]
    pub fn ftm0och0src(&self) -> FTM0OCH0SRC_R {
        FTM0OCH0SRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline(always)]
    pub fn ftm0och1src(&self) -> FTM0OCH1SRC_R {
        FTM0OCH1SRC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline(always)]
    pub fn ftm0och2src(&self) -> FTM0OCH2SRC_R {
        FTM0OCH2SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline(always)]
    pub fn ftm0och3src(&self) -> FTM0OCH3SRC_R {
        FTM0OCH3SRC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline(always)]
    pub fn ftm0och4src(&self) -> FTM0OCH4SRC_R {
        FTM0OCH4SRC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline(always)]
    pub fn ftm0och5src(&self) -> FTM0OCH5SRC_R {
        FTM0OCH5SRC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FTM0 channel 6 output source"]
    #[inline(always)]
    pub fn ftm0och6src(&self) -> FTM0OCH6SRC_R {
        FTM0OCH6SRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - FTM0 channel 7 output source"]
    #[inline(always)]
    pub fn ftm0och7src(&self) -> FTM0OCH7SRC_R {
        FTM0OCH7SRC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FTM3 channel 0 output source"]
    #[inline(always)]
    pub fn ftm3och0src(&self) -> FTM3OCH0SRC_R {
        FTM3OCH0SRC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FTM3 channel 1 output source"]
    #[inline(always)]
    pub fn ftm3och1src(&self) -> FTM3OCH1SRC_R {
        FTM3OCH1SRC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FTM3 channel 2 output source"]
    #[inline(always)]
    pub fn ftm3och2src(&self) -> FTM3OCH2SRC_R {
        FTM3OCH2SRC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FTM3 channel 3 output source"]
    #[inline(always)]
    pub fn ftm3och3src(&self) -> FTM3OCH3SRC_R {
        FTM3OCH3SRC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - FTM3 channel 4 output source"]
    #[inline(always)]
    pub fn ftm3och4src(&self) -> FTM3OCH4SRC_R {
        FTM3OCH4SRC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - FTM3 channel 5 output source"]
    #[inline(always)]
    pub fn ftm3och5src(&self) -> FTM3OCH5SRC_R {
        FTM3OCH5SRC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - FTM3 channel 6 output source"]
    #[inline(always)]
    pub fn ftm3och6src(&self) -> FTM3OCH6SRC_R {
        FTM3OCH6SRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FTM3 channel 7 output source"]
    #[inline(always)]
    pub fn ftm3och7src(&self) -> FTM3OCH7SRC_R {
        FTM3OCH7SRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm0syncbit(&mut self) -> FTM0SYNCBIT_W {
        FTM0SYNCBIT_W { w: self }
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm1syncbit(&mut self) -> FTM1SYNCBIT_W {
        FTM1SYNCBIT_W { w: self }
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm2syncbit(&mut self) -> FTM2SYNCBIT_W {
        FTM2SYNCBIT_W { w: self }
    }
    #[doc = "Bit 3 - FTM3 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm3syncbit(&mut self) -> FTM3SYNCBIT_W {
        FTM3SYNCBIT_W { w: self }
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline(always)]
    pub fn ftm0och0src(&mut self) -> FTM0OCH0SRC_W {
        FTM0OCH0SRC_W { w: self }
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline(always)]
    pub fn ftm0och1src(&mut self) -> FTM0OCH1SRC_W {
        FTM0OCH1SRC_W { w: self }
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline(always)]
    pub fn ftm0och2src(&mut self) -> FTM0OCH2SRC_W {
        FTM0OCH2SRC_W { w: self }
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline(always)]
    pub fn ftm0och3src(&mut self) -> FTM0OCH3SRC_W {
        FTM0OCH3SRC_W { w: self }
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline(always)]
    pub fn ftm0och4src(&mut self) -> FTM0OCH4SRC_W {
        FTM0OCH4SRC_W { w: self }
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline(always)]
    pub fn ftm0och5src(&mut self) -> FTM0OCH5SRC_W {
        FTM0OCH5SRC_W { w: self }
    }
    #[doc = "Bit 22 - FTM0 channel 6 output source"]
    #[inline(always)]
    pub fn ftm0och6src(&mut self) -> FTM0OCH6SRC_W {
        FTM0OCH6SRC_W { w: self }
    }
    #[doc = "Bit 23 - FTM0 channel 7 output source"]
    #[inline(always)]
    pub fn ftm0och7src(&mut self) -> FTM0OCH7SRC_W {
        FTM0OCH7SRC_W { w: self }
    }
    #[doc = "Bit 24 - FTM3 channel 0 output source"]
    #[inline(always)]
    pub fn ftm3och0src(&mut self) -> FTM3OCH0SRC_W {
        FTM3OCH0SRC_W { w: self }
    }
    #[doc = "Bit 25 - FTM3 channel 1 output source"]
    #[inline(always)]
    pub fn ftm3och1src(&mut self) -> FTM3OCH1SRC_W {
        FTM3OCH1SRC_W { w: self }
    }
    #[doc = "Bit 26 - FTM3 channel 2 output source"]
    #[inline(always)]
    pub fn ftm3och2src(&mut self) -> FTM3OCH2SRC_W {
        FTM3OCH2SRC_W { w: self }
    }
    #[doc = "Bit 27 - FTM3 channel 3 output source"]
    #[inline(always)]
    pub fn ftm3och3src(&mut self) -> FTM3OCH3SRC_W {
        FTM3OCH3SRC_W { w: self }
    }
    #[doc = "Bit 28 - FTM3 channel 4 output source"]
    #[inline(always)]
    pub fn ftm3och4src(&mut self) -> FTM3OCH4SRC_W {
        FTM3OCH4SRC_W { w: self }
    }
    #[doc = "Bit 29 - FTM3 channel 5 output source"]
    #[inline(always)]
    pub fn ftm3och5src(&mut self) -> FTM3OCH5SRC_W {
        FTM3OCH5SRC_W { w: self }
    }
    #[doc = "Bit 30 - FTM3 channel 6 output source"]
    #[inline(always)]
    pub fn ftm3och6src(&mut self) -> FTM3OCH6SRC_W {
        FTM3OCH6SRC_W { w: self }
    }
    #[doc = "Bit 31 - FTM3 channel 7 output source"]
    #[inline(always)]
    pub fn ftm3och7src(&mut self) -> FTM3OCH7SRC_W {
        FTM3OCH7SRC_W { w: self }
    }
}
