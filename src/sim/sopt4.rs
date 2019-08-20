#[doc = "Reader of register SOPT4"]
pub type R = crate::R<u32, super::SOPT4>;
#[doc = "Writer for register SOPT4"]
pub type W = crate::W<u32, super::SOPT4>;
#[doc = "Register SOPT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM0 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT0_A {
    #[doc = "0: FTM0_FLT0 pin"]
    _0,
    #[doc = "1: CMP0 out"]
    _1,
}
impl From<FTM0FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT0_A) -> Self {
        match variant {
            FTM0FLT0_A::_0 => false,
            FTM0FLT0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0FLT0`"]
pub type FTM0FLT0_R = crate::R<bool, FTM0FLT0_A>;
impl FTM0FLT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT0_A {
        match self.bits {
            false => FTM0FLT0_A::_0,
            true => FTM0FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT0_A::_1
    }
}
#[doc = "Write proxy for field `FTM0FLT0`"]
pub struct FTM0FLT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT0_A::_1)
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
#[doc = "FTM0 Fault 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT1_A {
    #[doc = "0: FTM0_FLT1 pin"]
    _0,
    #[doc = "1: CMP1 out"]
    _1,
}
impl From<FTM0FLT1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT1_A) -> Self {
        match variant {
            FTM0FLT1_A::_0 => false,
            FTM0FLT1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0FLT1`"]
pub type FTM0FLT1_R = crate::R<bool, FTM0FLT1_A>;
impl FTM0FLT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT1_A {
        match self.bits {
            false => FTM0FLT1_A::_0,
            true => FTM0FLT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT1_A::_1
    }
}
#[doc = "Write proxy for field `FTM0FLT1`"]
pub struct FTM0FLT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_FLT1 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT1_A::_0)
    }
    #[doc = "CMP1 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT1_A::_1)
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
#[doc = "FTM0 Fault 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT2_A {
    #[doc = "0: FTM0_FLT2 pin"]
    _0,
    #[doc = "1: CMP2 out"]
    _1,
}
impl From<FTM0FLT2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT2_A) -> Self {
        match variant {
            FTM0FLT2_A::_0 => false,
            FTM0FLT2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0FLT2`"]
pub type FTM0FLT2_R = crate::R<bool, FTM0FLT2_A>;
impl FTM0FLT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT2_A {
        match self.bits {
            false => FTM0FLT2_A::_0,
            true => FTM0FLT2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT2_A::_1
    }
}
#[doc = "Write proxy for field `FTM0FLT2`"]
pub struct FTM0FLT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_FLT2 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT2_A::_0)
    }
    #[doc = "CMP2 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT2_A::_1)
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
#[doc = "FTM0 Fault 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT3_A {
    #[doc = "0: FTM0_FLT3 pin"]
    _0,
    #[doc = "1: CMP3 out"]
    _1,
}
impl From<FTM0FLT3_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0FLT3_A) -> Self {
        match variant {
            FTM0FLT3_A::_0 => false,
            FTM0FLT3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0FLT3`"]
pub type FTM0FLT3_R = crate::R<bool, FTM0FLT3_A>;
impl FTM0FLT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0FLT3_A {
        match self.bits {
            false => FTM0FLT3_A::_0,
            true => FTM0FLT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT3_A::_1
    }
}
#[doc = "Write proxy for field `FTM0FLT3`"]
pub struct FTM0FLT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0_FLT3 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT3_A::_0)
    }
    #[doc = "CMP3 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT3_A::_1)
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
#[doc = "FTM1 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1FLT0_A {
    #[doc = "0: FTM1_FLT0 pin"]
    _0,
    #[doc = "1: CMP0 out"]
    _1,
}
impl From<FTM1FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1FLT0_A) -> Self {
        match variant {
            FTM1FLT0_A::_0 => false,
            FTM1FLT0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM1FLT0`"]
pub type FTM1FLT0_R = crate::R<bool, FTM1FLT0_A>;
impl FTM1FLT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1FLT0_A {
        match self.bits {
            false => FTM1FLT0_A::_0,
            true => FTM1FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1FLT0_A::_1
    }
}
#[doc = "Write proxy for field `FTM1FLT0`"]
pub struct FTM1FLT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1FLT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1FLT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1FLT0_A::_1)
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
#[doc = "FTM2 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2FLT0_A {
    #[doc = "0: FTM2_FLT0 pin"]
    _0,
    #[doc = "1: CMP0 out"]
    _1,
}
impl From<FTM2FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2FLT0_A) -> Self {
        match variant {
            FTM2FLT0_A::_0 => false,
            FTM2FLT0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM2FLT0`"]
pub type FTM2FLT0_R = crate::R<bool, FTM2FLT0_A>;
impl FTM2FLT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2FLT0_A {
        match self.bits {
            false => FTM2FLT0_A::_0,
            true => FTM2FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2FLT0_A::_1
    }
}
#[doc = "Write proxy for field `FTM2FLT0`"]
pub struct FTM2FLT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2FLT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2FLT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2FLT0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "FTM3 Fault 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3FLT0_A {
    #[doc = "0: FTM3_FLT0 pin"]
    _0,
    #[doc = "1: CMP0 out"]
    _1,
}
impl From<FTM3FLT0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3FLT0_A) -> Self {
        match variant {
            FTM3FLT0_A::_0 => false,
            FTM3FLT0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3FLT0`"]
pub type FTM3FLT0_R = crate::R<bool, FTM3FLT0_A>;
impl FTM3FLT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3FLT0_A {
        match self.bits {
            false => FTM3FLT0_A::_0,
            true => FTM3FLT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3FLT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3FLT0_A::_1
    }
}
#[doc = "Write proxy for field `FTM3FLT0`"]
pub struct FTM3FLT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3FLT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3FLT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3_FLT0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3FLT0_A::_0)
    }
    #[doc = "CMP0 out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3FLT0_A::_1)
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
#[doc = "FTM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CH0SRC_A {
    #[doc = "0: FTM1_CH0 signal"]
    _00,
    #[doc = "1: CMP0 output"]
    _01,
    #[doc = "2: CMP1 output"]
    _10,
    #[doc = "3: USB start of frame pulse"]
    _11,
}
impl From<FTM1CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CH0SRC_A) -> Self {
        match variant {
            FTM1CH0SRC_A::_00 => 0,
            FTM1CH0SRC_A::_01 => 1,
            FTM1CH0SRC_A::_10 => 2,
            FTM1CH0SRC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FTM1CH0SRC`"]
pub type FTM1CH0SRC_R = crate::R<u8, FTM1CH0SRC_A>;
impl FTM1CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CH0SRC_A {
        match self.bits {
            0 => FTM1CH0SRC_A::_00,
            1 => FTM1CH0SRC_A::_01,
            2 => FTM1CH0SRC_A::_10,
            3 => FTM1CH0SRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM1CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM1CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM1CH0SRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM1CH0SRC_A::_11
    }
}
#[doc = "Write proxy for field `FTM1CH0SRC`"]
pub struct FTM1CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CH0SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FTM1_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_10)
    }
    #[doc = "USB start of frame pulse"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CH0SRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "FTM2 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH0SRC_A {
    #[doc = "0: FTM2_CH0 signal"]
    _00,
    #[doc = "1: CMP0 output"]
    _01,
    #[doc = "2: CMP1 output"]
    _10,
}
impl From<FTM2CH0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CH0SRC_A) -> Self {
        match variant {
            FTM2CH0SRC_A::_00 => 0,
            FTM2CH0SRC_A::_01 => 1,
            FTM2CH0SRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `FTM2CH0SRC`"]
pub type FTM2CH0SRC_R = crate::R<u8, FTM2CH0SRC_A>;
impl FTM2CH0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2CH0SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2CH0SRC_A::_00),
            1 => Val(FTM2CH0SRC_A::_01),
            2 => Val(FTM2CH0SRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2CH0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2CH0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2CH0SRC_A::_10
    }
}
#[doc = "Write proxy for field `FTM2CH0SRC`"]
pub struct FTM2CH0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CH0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CH0SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2_CH0 signal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CH0SRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "FTM2 channel 1 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH1SRC_A {
    #[doc = "0: FTM2_CH1 signal"]
    _0,
    #[doc = "1: Exclusive OR of FTM2_CH1, FTM2_CH0 and FTM1_CH1."]
    _1,
}
impl From<FTM2CH1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2CH1SRC_A) -> Self {
        match variant {
            FTM2CH1SRC_A::_0 => false,
            FTM2CH1SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM2CH1SRC`"]
pub type FTM2CH1SRC_R = crate::R<bool, FTM2CH1SRC_A>;
impl FTM2CH1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CH1SRC_A {
        match self.bits {
            false => FTM2CH1SRC_A::_0,
            true => FTM2CH1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2CH1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2CH1SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM2CH1SRC`"]
pub struct FTM2CH1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CH1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CH1SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2_CH1 signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CH1SRC_A::_0)
    }
    #[doc = "Exclusive OR of FTM2_CH1, FTM2_CH0 and FTM1_CH1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CH1SRC_A::_1)
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
#[doc = "FlexTimer 0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0CLKSEL_A {
    #[doc = "0: FTM_CLK0 pin"]
    _0,
    #[doc = "1: FTM_CLK1 pin"]
    _1,
}
impl From<FTM0CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0CLKSEL_A) -> Self {
        match variant {
            FTM0CLKSEL_A::_0 => false,
            FTM0CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0CLKSEL`"]
pub type FTM0CLKSEL_R = crate::R<bool, FTM0CLKSEL_A>;
impl FTM0CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0CLKSEL_A {
        match self.bits {
            false => FTM0CLKSEL_A::_0,
            true => FTM0CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `FTM0CLKSEL`"]
pub struct FTM0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM_CLK0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_1)
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
#[doc = "FTM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CLKSEL_A {
    #[doc = "0: FTM_CLK0 pin"]
    _0,
    #[doc = "1: FTM_CLK1 pin"]
    _1,
}
impl From<FTM1CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1CLKSEL_A) -> Self {
        match variant {
            FTM1CLKSEL_A::_0 => false,
            FTM1CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM1CLKSEL`"]
pub type FTM1CLKSEL_R = crate::R<bool, FTM1CLKSEL_A>;
impl FTM1CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CLKSEL_A {
        match self.bits {
            false => FTM1CLKSEL_A::_0,
            true => FTM1CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `FTM1CLKSEL`"]
pub struct FTM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM_CLK0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_1)
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
#[doc = "FlexTimer 2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CLKSEL_A {
    #[doc = "0: FTM2 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "1: FTM2 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl From<FTM2CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2CLKSEL_A) -> Self {
        match variant {
            FTM2CLKSEL_A::_0 => false,
            FTM2CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM2CLKSEL`"]
pub type FTM2CLKSEL_R = crate::R<bool, FTM2CLKSEL_A>;
impl FTM2CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CLKSEL_A {
        match self.bits {
            false => FTM2CLKSEL_A::_0,
            true => FTM2CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `FTM2CLKSEL`"]
pub struct FTM2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2 external clock driven by FTM_CLK0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_0)
    }
    #[doc = "FTM2 external clock driven by FTM_CLK1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_1)
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
#[doc = "FlexTimer 3 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3CLKSEL_A {
    #[doc = "0: FTM3 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "1: FTM3 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl From<FTM3CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3CLKSEL_A) -> Self {
        match variant {
            FTM3CLKSEL_A::_0 => false,
            FTM3CLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3CLKSEL`"]
pub type FTM3CLKSEL_R = crate::R<bool, FTM3CLKSEL_A>;
impl FTM3CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3CLKSEL_A {
        match self.bits {
            false => FTM3CLKSEL_A::_0,
            true => FTM3CLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3CLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3CLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `FTM3CLKSEL`"]
pub struct FTM3CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM3 external clock driven by FTM_CLK0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_0)
    }
    #[doc = "FTM3 external clock driven by FTM_CLK1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_1)
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
#[doc = "FlexTimer 0 Hardware Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0TRG0SRC_A {
    #[doc = "0: HSCMP0 output drives FTM0 hardware trigger 0"]
    _0,
    #[doc = "1: FTM1 channel match drives FTM0 hardware trigger 0"]
    _1,
}
impl From<FTM0TRG0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0TRG0SRC_A) -> Self {
        match variant {
            FTM0TRG0SRC_A::_0 => false,
            FTM0TRG0SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0TRG0SRC`"]
pub type FTM0TRG0SRC_R = crate::R<bool, FTM0TRG0SRC_A>;
impl FTM0TRG0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0TRG0SRC_A {
        match self.bits {
            false => FTM0TRG0SRC_A::_0,
            true => FTM0TRG0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG0SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0TRG0SRC`"]
pub struct FTM0TRG0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0TRG0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0TRG0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSCMP0 output drives FTM0 hardware trigger 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG0SRC_A::_0)
    }
    #[doc = "FTM1 channel match drives FTM0 hardware trigger 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG0SRC_A::_1)
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
#[doc = "FlexTimer 0 Hardware Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0TRG1SRC_A {
    #[doc = "0: PDB output trigger 1 drives FTM0 hardware trigger 1"]
    _0,
    #[doc = "1: FTM2 channel match drives FTM0 hardware trigger 1"]
    _1,
}
impl From<FTM0TRG1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0TRG1SRC_A) -> Self {
        match variant {
            FTM0TRG1SRC_A::_0 => false,
            FTM0TRG1SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0TRG1SRC`"]
pub type FTM0TRG1SRC_R = crate::R<bool, FTM0TRG1SRC_A>;
impl FTM0TRG1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0TRG1SRC_A {
        match self.bits {
            false => FTM0TRG1SRC_A::_0,
            true => FTM0TRG1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG1SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM0TRG1SRC`"]
pub struct FTM0TRG1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0TRG1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0TRG1SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB output trigger 1 drives FTM0 hardware trigger 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG1SRC_A::_0)
    }
    #[doc = "FTM2 channel match drives FTM0 hardware trigger 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG1SRC_A::_1)
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
#[doc = "FlexTimer 3 Hardware Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3TRG0SRC_A {
    #[doc = "1: FTM1 channel match drives FTM3 hardware trigger 0"]
    _1,
}
impl From<FTM3TRG0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3TRG0SRC_A) -> Self {
        match variant {
            FTM3TRG0SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3TRG0SRC`"]
pub type FTM3TRG0SRC_R = crate::R<bool, FTM3TRG0SRC_A>;
impl FTM3TRG0SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FTM3TRG0SRC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FTM3TRG0SRC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG0SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3TRG0SRC`"]
pub struct FTM3TRG0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3TRG0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3TRG0SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1 channel match drives FTM3 hardware trigger 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG0SRC_A::_1)
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
#[doc = "FlexTimer 3 Hardware Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3TRG1SRC_A {
    #[doc = "1: FTM2 channel match drives FTM3 hardware trigger 1"]
    _1,
}
impl From<FTM3TRG1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3TRG1SRC_A) -> Self {
        match variant {
            FTM3TRG1SRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3TRG1SRC`"]
pub type FTM3TRG1SRC_R = crate::R<bool, FTM3TRG1SRC_A>;
impl FTM3TRG1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FTM3TRG1SRC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FTM3TRG1SRC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG1SRC_A::_1
    }
}
#[doc = "Write proxy for field `FTM3TRG1SRC`"]
pub struct FTM3TRG1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3TRG1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3TRG1SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2 channel match drives FTM3 hardware trigger 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG1SRC_A::_1)
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
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm0flt0(&self) -> FTM0FLT0_R {
        FTM0FLT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline(always)]
    pub fn ftm0flt1(&self) -> FTM0FLT1_R {
        FTM0FLT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline(always)]
    pub fn ftm0flt2(&self) -> FTM0FLT2_R {
        FTM0FLT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FTM0 Fault 3 Select"]
    #[inline(always)]
    pub fn ftm0flt3(&self) -> FTM0FLT3_R {
        FTM0FLT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm1flt0(&self) -> FTM1FLT0_R {
        FTM1FLT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm2flt0(&self) -> FTM2FLT0_R {
        FTM2FLT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm3flt0(&self) -> FTM3FLT0_R {
        FTM3FLT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm1ch0src(&self) -> FTM1CH0SRC_R {
        FTM1CH0SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm2ch0src(&self) -> FTM2CH0SRC_R {
        FTM2CH0SRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - FTM2 channel 1 input capture source select"]
    #[inline(always)]
    pub fn ftm2ch1src(&self) -> FTM2CH1SRC_R {
        FTM2CH1SRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&self) -> FTM0CLKSEL_R {
        FTM0CLKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&self) -> FTM1CLKSEL_R {
        FTM1CLKSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&self) -> FTM2CLKSEL_R {
        FTM2CLKSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&self) -> FTM3CLKSEL_R {
        FTM3CLKSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm0trg0src(&self) -> FTM0TRG0SRC_R {
        FTM0TRG0SRC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm0trg1src(&self) -> FTM0TRG1SRC_R {
        FTM0TRG1SRC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm3trg0src(&self) -> FTM3TRG0SRC_R {
        FTM3TRG0SRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm3trg1src(&self) -> FTM3TRG1SRC_R {
        FTM3TRG1SRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm0flt0(&mut self) -> FTM0FLT0_W {
        FTM0FLT0_W { w: self }
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline(always)]
    pub fn ftm0flt1(&mut self) -> FTM0FLT1_W {
        FTM0FLT1_W { w: self }
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline(always)]
    pub fn ftm0flt2(&mut self) -> FTM0FLT2_W {
        FTM0FLT2_W { w: self }
    }
    #[doc = "Bit 3 - FTM0 Fault 3 Select"]
    #[inline(always)]
    pub fn ftm0flt3(&mut self) -> FTM0FLT3_W {
        FTM0FLT3_W { w: self }
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm1flt0(&mut self) -> FTM1FLT0_W {
        FTM1FLT0_W { w: self }
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm2flt0(&mut self) -> FTM2FLT0_W {
        FTM2FLT0_W { w: self }
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline(always)]
    pub fn ftm3flt0(&mut self) -> FTM3FLT0_W {
        FTM3FLT0_W { w: self }
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm1ch0src(&mut self) -> FTM1CH0SRC_W {
        FTM1CH0SRC_W { w: self }
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline(always)]
    pub fn ftm2ch0src(&mut self) -> FTM2CH0SRC_W {
        FTM2CH0SRC_W { w: self }
    }
    #[doc = "Bit 22 - FTM2 channel 1 input capture source select"]
    #[inline(always)]
    pub fn ftm2ch1src(&mut self) -> FTM2CH1SRC_W {
        FTM2CH1SRC_W { w: self }
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&mut self) -> FTM0CLKSEL_W {
        FTM0CLKSEL_W { w: self }
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&mut self) -> FTM1CLKSEL_W {
        FTM1CLKSEL_W { w: self }
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&mut self) -> FTM2CLKSEL_W {
        FTM2CLKSEL_W { w: self }
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&mut self) -> FTM3CLKSEL_W {
        FTM3CLKSEL_W { w: self }
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm0trg0src(&mut self) -> FTM0TRG0SRC_W {
        FTM0TRG0SRC_W { w: self }
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm0trg1src(&mut self) -> FTM0TRG1SRC_W {
        FTM0TRG1SRC_W { w: self }
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline(always)]
    pub fn ftm3trg0src(&mut self) -> FTM3TRG0SRC_W {
        FTM3TRG0SRC_W { w: self }
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline(always)]
    pub fn ftm3trg1src(&mut self) -> FTM3TRG1SRC_W {
        FTM3TRG1SRC_W { w: self }
    }
}
