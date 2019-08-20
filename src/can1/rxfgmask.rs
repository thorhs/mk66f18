#[doc = "Reader of register RXFGMASK"]
pub type R = crate::R<u32, super::RXFGMASK>;
#[doc = "Writer for register RXFGMASK"]
pub type W = crate::W<u32, super::RXFGMASK>;
#[doc = "Register RXFGMASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RXFGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM0_A> for bool {
    #[inline(always)]
    fn from(variant: FGM0_A) -> Self {
        match variant {
            FGM0_A::_0 => false,
            FGM0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM0`"]
pub type FGM0_R = crate::R<bool, FGM0_A>;
impl FGM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM0_A {
        match self.bits {
            false => FGM0_A::_0,
            true => FGM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM0_A::_1
    }
}
#[doc = "Write proxy for field `FGM0`"]
pub struct FGM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM0_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM1_A> for bool {
    #[inline(always)]
    fn from(variant: FGM1_A) -> Self {
        match variant {
            FGM1_A::_0 => false,
            FGM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM1`"]
pub type FGM1_R = crate::R<bool, FGM1_A>;
impl FGM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM1_A {
        match self.bits {
            false => FGM1_A::_0,
            true => FGM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM1_A::_1
    }
}
#[doc = "Write proxy for field `FGM1`"]
pub struct FGM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM1_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM2_A> for bool {
    #[inline(always)]
    fn from(variant: FGM2_A) -> Self {
        match variant {
            FGM2_A::_0 => false,
            FGM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM2`"]
pub type FGM2_R = crate::R<bool, FGM2_A>;
impl FGM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM2_A {
        match self.bits {
            false => FGM2_A::_0,
            true => FGM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM2_A::_1
    }
}
#[doc = "Write proxy for field `FGM2`"]
pub struct FGM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM2_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM3_A> for bool {
    #[inline(always)]
    fn from(variant: FGM3_A) -> Self {
        match variant {
            FGM3_A::_0 => false,
            FGM3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM3`"]
pub type FGM3_R = crate::R<bool, FGM3_A>;
impl FGM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM3_A {
        match self.bits {
            false => FGM3_A::_0,
            true => FGM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM3_A::_1
    }
}
#[doc = "Write proxy for field `FGM3`"]
pub struct FGM3_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM3_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM4_A> for bool {
    #[inline(always)]
    fn from(variant: FGM4_A) -> Self {
        match variant {
            FGM4_A::_0 => false,
            FGM4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM4`"]
pub type FGM4_R = crate::R<bool, FGM4_A>;
impl FGM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM4_A {
        match self.bits {
            false => FGM4_A::_0,
            true => FGM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM4_A::_1
    }
}
#[doc = "Write proxy for field `FGM4`"]
pub struct FGM4_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM4_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM5_A> for bool {
    #[inline(always)]
    fn from(variant: FGM5_A) -> Self {
        match variant {
            FGM5_A::_0 => false,
            FGM5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM5`"]
pub type FGM5_R = crate::R<bool, FGM5_A>;
impl FGM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM5_A {
        match self.bits {
            false => FGM5_A::_0,
            true => FGM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM5_A::_1
    }
}
#[doc = "Write proxy for field `FGM5`"]
pub struct FGM5_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM6_A> for bool {
    #[inline(always)]
    fn from(variant: FGM6_A) -> Self {
        match variant {
            FGM6_A::_0 => false,
            FGM6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM6`"]
pub type FGM6_R = crate::R<bool, FGM6_A>;
impl FGM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM6_A {
        match self.bits {
            false => FGM6_A::_0,
            true => FGM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM6_A::_1
    }
}
#[doc = "Write proxy for field `FGM6`"]
pub struct FGM6_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM7_A> for bool {
    #[inline(always)]
    fn from(variant: FGM7_A) -> Self {
        match variant {
            FGM7_A::_0 => false,
            FGM7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM7`"]
pub type FGM7_R = crate::R<bool, FGM7_A>;
impl FGM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM7_A {
        match self.bits {
            false => FGM7_A::_0,
            true => FGM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM7_A::_1
    }
}
#[doc = "Write proxy for field `FGM7`"]
pub struct FGM7_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM8_A> for bool {
    #[inline(always)]
    fn from(variant: FGM8_A) -> Self {
        match variant {
            FGM8_A::_0 => false,
            FGM8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM8`"]
pub type FGM8_R = crate::R<bool, FGM8_A>;
impl FGM8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM8_A {
        match self.bits {
            false => FGM8_A::_0,
            true => FGM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM8_A::_1
    }
}
#[doc = "Write proxy for field `FGM8`"]
pub struct FGM8_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM8_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM9_A> for bool {
    #[inline(always)]
    fn from(variant: FGM9_A) -> Self {
        match variant {
            FGM9_A::_0 => false,
            FGM9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM9`"]
pub type FGM9_R = crate::R<bool, FGM9_A>;
impl FGM9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM9_A {
        match self.bits {
            false => FGM9_A::_0,
            true => FGM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM9_A::_1
    }
}
#[doc = "Write proxy for field `FGM9`"]
pub struct FGM9_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM9_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM10_A> for bool {
    #[inline(always)]
    fn from(variant: FGM10_A) -> Self {
        match variant {
            FGM10_A::_0 => false,
            FGM10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM10`"]
pub type FGM10_R = crate::R<bool, FGM10_A>;
impl FGM10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM10_A {
        match self.bits {
            false => FGM10_A::_0,
            true => FGM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM10_A::_1
    }
}
#[doc = "Write proxy for field `FGM10`"]
pub struct FGM10_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM10_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM11_A> for bool {
    #[inline(always)]
    fn from(variant: FGM11_A) -> Self {
        match variant {
            FGM11_A::_0 => false,
            FGM11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM11`"]
pub type FGM11_R = crate::R<bool, FGM11_A>;
impl FGM11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM11_A {
        match self.bits {
            false => FGM11_A::_0,
            true => FGM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM11_A::_1
    }
}
#[doc = "Write proxy for field `FGM11`"]
pub struct FGM11_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM11_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM12_A> for bool {
    #[inline(always)]
    fn from(variant: FGM12_A) -> Self {
        match variant {
            FGM12_A::_0 => false,
            FGM12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM12`"]
pub type FGM12_R = crate::R<bool, FGM12_A>;
impl FGM12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM12_A {
        match self.bits {
            false => FGM12_A::_0,
            true => FGM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM12_A::_1
    }
}
#[doc = "Write proxy for field `FGM12`"]
pub struct FGM12_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM12_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM13_A> for bool {
    #[inline(always)]
    fn from(variant: FGM13_A) -> Self {
        match variant {
            FGM13_A::_0 => false,
            FGM13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM13`"]
pub type FGM13_R = crate::R<bool, FGM13_A>;
impl FGM13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM13_A {
        match self.bits {
            false => FGM13_A::_0,
            true => FGM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM13_A::_1
    }
}
#[doc = "Write proxy for field `FGM13`"]
pub struct FGM13_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM13_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM14_A> for bool {
    #[inline(always)]
    fn from(variant: FGM14_A) -> Self {
        match variant {
            FGM14_A::_0 => false,
            FGM14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM14`"]
pub type FGM14_R = crate::R<bool, FGM14_A>;
impl FGM14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM14_A {
        match self.bits {
            false => FGM14_A::_0,
            true => FGM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM14_A::_1
    }
}
#[doc = "Write proxy for field `FGM14`"]
pub struct FGM14_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM15_A> for bool {
    #[inline(always)]
    fn from(variant: FGM15_A) -> Self {
        match variant {
            FGM15_A::_0 => false,
            FGM15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM15`"]
pub type FGM15_R = crate::R<bool, FGM15_A>;
impl FGM15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM15_A {
        match self.bits {
            false => FGM15_A::_0,
            true => FGM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM15_A::_1
    }
}
#[doc = "Write proxy for field `FGM15`"]
pub struct FGM15_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM16_A> for bool {
    #[inline(always)]
    fn from(variant: FGM16_A) -> Self {
        match variant {
            FGM16_A::_0 => false,
            FGM16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM16`"]
pub type FGM16_R = crate::R<bool, FGM16_A>;
impl FGM16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM16_A {
        match self.bits {
            false => FGM16_A::_0,
            true => FGM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM16_A::_1
    }
}
#[doc = "Write proxy for field `FGM16`"]
pub struct FGM16_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM16_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM17_A> for bool {
    #[inline(always)]
    fn from(variant: FGM17_A) -> Self {
        match variant {
            FGM17_A::_0 => false,
            FGM17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM17`"]
pub type FGM17_R = crate::R<bool, FGM17_A>;
impl FGM17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM17_A {
        match self.bits {
            false => FGM17_A::_0,
            true => FGM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM17_A::_1
    }
}
#[doc = "Write proxy for field `FGM17`"]
pub struct FGM17_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM17_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM18_A> for bool {
    #[inline(always)]
    fn from(variant: FGM18_A) -> Self {
        match variant {
            FGM18_A::_0 => false,
            FGM18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM18`"]
pub type FGM18_R = crate::R<bool, FGM18_A>;
impl FGM18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM18_A {
        match self.bits {
            false => FGM18_A::_0,
            true => FGM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM18_A::_1
    }
}
#[doc = "Write proxy for field `FGM18`"]
pub struct FGM18_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM18_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM19_A> for bool {
    #[inline(always)]
    fn from(variant: FGM19_A) -> Self {
        match variant {
            FGM19_A::_0 => false,
            FGM19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM19`"]
pub type FGM19_R = crate::R<bool, FGM19_A>;
impl FGM19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM19_A {
        match self.bits {
            false => FGM19_A::_0,
            true => FGM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM19_A::_1
    }
}
#[doc = "Write proxy for field `FGM19`"]
pub struct FGM19_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM19_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM20_A> for bool {
    #[inline(always)]
    fn from(variant: FGM20_A) -> Self {
        match variant {
            FGM20_A::_0 => false,
            FGM20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM20`"]
pub type FGM20_R = crate::R<bool, FGM20_A>;
impl FGM20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM20_A {
        match self.bits {
            false => FGM20_A::_0,
            true => FGM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM20_A::_1
    }
}
#[doc = "Write proxy for field `FGM20`"]
pub struct FGM20_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM20_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM21_A> for bool {
    #[inline(always)]
    fn from(variant: FGM21_A) -> Self {
        match variant {
            FGM21_A::_0 => false,
            FGM21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM21`"]
pub type FGM21_R = crate::R<bool, FGM21_A>;
impl FGM21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM21_A {
        match self.bits {
            false => FGM21_A::_0,
            true => FGM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM21_A::_1
    }
}
#[doc = "Write proxy for field `FGM21`"]
pub struct FGM21_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM21_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM22_A> for bool {
    #[inline(always)]
    fn from(variant: FGM22_A) -> Self {
        match variant {
            FGM22_A::_0 => false,
            FGM22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM22`"]
pub type FGM22_R = crate::R<bool, FGM22_A>;
impl FGM22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM22_A {
        match self.bits {
            false => FGM22_A::_0,
            true => FGM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM22_A::_1
    }
}
#[doc = "Write proxy for field `FGM22`"]
pub struct FGM22_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM22_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM23_A> for bool {
    #[inline(always)]
    fn from(variant: FGM23_A) -> Self {
        match variant {
            FGM23_A::_0 => false,
            FGM23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM23`"]
pub type FGM23_R = crate::R<bool, FGM23_A>;
impl FGM23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM23_A {
        match self.bits {
            false => FGM23_A::_0,
            true => FGM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM23_A::_1
    }
}
#[doc = "Write proxy for field `FGM23`"]
pub struct FGM23_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM23_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM24_A> for bool {
    #[inline(always)]
    fn from(variant: FGM24_A) -> Self {
        match variant {
            FGM24_A::_0 => false,
            FGM24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM24`"]
pub type FGM24_R = crate::R<bool, FGM24_A>;
impl FGM24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM24_A {
        match self.bits {
            false => FGM24_A::_0,
            true => FGM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM24_A::_1
    }
}
#[doc = "Write proxy for field `FGM24`"]
pub struct FGM24_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM24_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM25_A> for bool {
    #[inline(always)]
    fn from(variant: FGM25_A) -> Self {
        match variant {
            FGM25_A::_0 => false,
            FGM25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM25`"]
pub type FGM25_R = crate::R<bool, FGM25_A>;
impl FGM25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM25_A {
        match self.bits {
            false => FGM25_A::_0,
            true => FGM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM25_A::_1
    }
}
#[doc = "Write proxy for field `FGM25`"]
pub struct FGM25_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM25_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM26_A> for bool {
    #[inline(always)]
    fn from(variant: FGM26_A) -> Self {
        match variant {
            FGM26_A::_0 => false,
            FGM26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM26`"]
pub type FGM26_R = crate::R<bool, FGM26_A>;
impl FGM26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM26_A {
        match self.bits {
            false => FGM26_A::_0,
            true => FGM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM26_A::_1
    }
}
#[doc = "Write proxy for field `FGM26`"]
pub struct FGM26_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM26_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM27_A> for bool {
    #[inline(always)]
    fn from(variant: FGM27_A) -> Self {
        match variant {
            FGM27_A::_0 => false,
            FGM27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM27`"]
pub type FGM27_R = crate::R<bool, FGM27_A>;
impl FGM27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM27_A {
        match self.bits {
            false => FGM27_A::_0,
            true => FGM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM27_A::_1
    }
}
#[doc = "Write proxy for field `FGM27`"]
pub struct FGM27_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM27_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM28_A> for bool {
    #[inline(always)]
    fn from(variant: FGM28_A) -> Self {
        match variant {
            FGM28_A::_0 => false,
            FGM28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM28`"]
pub type FGM28_R = crate::R<bool, FGM28_A>;
impl FGM28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM28_A {
        match self.bits {
            false => FGM28_A::_0,
            true => FGM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM28_A::_1
    }
}
#[doc = "Write proxy for field `FGM28`"]
pub struct FGM28_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM28_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM29_A> for bool {
    #[inline(always)]
    fn from(variant: FGM29_A) -> Self {
        match variant {
            FGM29_A::_0 => false,
            FGM29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM29`"]
pub type FGM29_R = crate::R<bool, FGM29_A>;
impl FGM29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM29_A {
        match self.bits {
            false => FGM29_A::_0,
            true => FGM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM29_A::_1
    }
}
#[doc = "Write proxy for field `FGM29`"]
pub struct FGM29_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM29_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM30_A> for bool {
    #[inline(always)]
    fn from(variant: FGM30_A) -> Self {
        match variant {
            FGM30_A::_0 => false,
            FGM30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM30`"]
pub type FGM30_R = crate::R<bool, FGM30_A>;
impl FGM30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM30_A {
        match self.bits {
            false => FGM30_A::_0,
            true => FGM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM30_A::_1
    }
}
#[doc = "Write proxy for field `FGM30`"]
pub struct FGM30_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM30_A::_1)
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
#[doc = "Rx FIFO Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<FGM31_A> for bool {
    #[inline(always)]
    fn from(variant: FGM31_A) -> Self {
        match variant {
            FGM31_A::_0 => false,
            FGM31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FGM31`"]
pub type FGM31_R = crate::R<bool, FGM31_A>;
impl FGM31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGM31_A {
        match self.bits {
            false => FGM31_A::_0,
            true => FGM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FGM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FGM31_A::_1
    }
}
#[doc = "Write proxy for field `FGM31`"]
pub struct FGM31_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM31_A::_1)
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
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm0(&self) -> FGM0_R {
        FGM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm1(&self) -> FGM1_R {
        FGM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm2(&self) -> FGM2_R {
        FGM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm3(&self) -> FGM3_R {
        FGM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm4(&self) -> FGM4_R {
        FGM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm5(&self) -> FGM5_R {
        FGM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm6(&self) -> FGM6_R {
        FGM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm7(&self) -> FGM7_R {
        FGM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm8(&self) -> FGM8_R {
        FGM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm9(&self) -> FGM9_R {
        FGM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm10(&self) -> FGM10_R {
        FGM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm11(&self) -> FGM11_R {
        FGM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm12(&self) -> FGM12_R {
        FGM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm13(&self) -> FGM13_R {
        FGM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm14(&self) -> FGM14_R {
        FGM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm15(&self) -> FGM15_R {
        FGM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm16(&self) -> FGM16_R {
        FGM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm17(&self) -> FGM17_R {
        FGM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm18(&self) -> FGM18_R {
        FGM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm19(&self) -> FGM19_R {
        FGM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm20(&self) -> FGM20_R {
        FGM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm21(&self) -> FGM21_R {
        FGM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm22(&self) -> FGM22_R {
        FGM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm23(&self) -> FGM23_R {
        FGM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm24(&self) -> FGM24_R {
        FGM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm25(&self) -> FGM25_R {
        FGM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm26(&self) -> FGM26_R {
        FGM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm27(&self) -> FGM27_R {
        FGM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm28(&self) -> FGM28_R {
        FGM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm29(&self) -> FGM29_R {
        FGM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm30(&self) -> FGM30_R {
        FGM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm31(&self) -> FGM31_R {
        FGM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm0(&mut self) -> FGM0_W {
        FGM0_W { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm1(&mut self) -> FGM1_W {
        FGM1_W { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm2(&mut self) -> FGM2_W {
        FGM2_W { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm3(&mut self) -> FGM3_W {
        FGM3_W { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm4(&mut self) -> FGM4_W {
        FGM4_W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm5(&mut self) -> FGM5_W {
        FGM5_W { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm6(&mut self) -> FGM6_W {
        FGM6_W { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm7(&mut self) -> FGM7_W {
        FGM7_W { w: self }
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm8(&mut self) -> FGM8_W {
        FGM8_W { w: self }
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm9(&mut self) -> FGM9_W {
        FGM9_W { w: self }
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm10(&mut self) -> FGM10_W {
        FGM10_W { w: self }
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm11(&mut self) -> FGM11_W {
        FGM11_W { w: self }
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm12(&mut self) -> FGM12_W {
        FGM12_W { w: self }
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm13(&mut self) -> FGM13_W {
        FGM13_W { w: self }
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm14(&mut self) -> FGM14_W {
        FGM14_W { w: self }
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm15(&mut self) -> FGM15_W {
        FGM15_W { w: self }
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm16(&mut self) -> FGM16_W {
        FGM16_W { w: self }
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm17(&mut self) -> FGM17_W {
        FGM17_W { w: self }
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm18(&mut self) -> FGM18_W {
        FGM18_W { w: self }
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm19(&mut self) -> FGM19_W {
        FGM19_W { w: self }
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm20(&mut self) -> FGM20_W {
        FGM20_W { w: self }
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm21(&mut self) -> FGM21_W {
        FGM21_W { w: self }
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm22(&mut self) -> FGM22_W {
        FGM22_W { w: self }
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm23(&mut self) -> FGM23_W {
        FGM23_W { w: self }
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm24(&mut self) -> FGM24_W {
        FGM24_W { w: self }
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm25(&mut self) -> FGM25_W {
        FGM25_W { w: self }
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm26(&mut self) -> FGM26_W {
        FGM26_W { w: self }
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm27(&mut self) -> FGM27_W {
        FGM27_W { w: self }
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm28(&mut self) -> FGM28_W {
        FGM28_W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm29(&mut self) -> FGM29_W {
        FGM29_W { w: self }
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm30(&mut self) -> FGM30_W {
        FGM30_W { w: self }
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm31(&mut self) -> FGM31_W {
        FGM31_W { w: self }
    }
}
