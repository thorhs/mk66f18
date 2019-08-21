#[doc = "Reader of register RX15MASK"]
pub type R = crate::R<u32, super::RX15MASK>;
#[doc = "Writer for register RX15MASK"]
pub type W = crate::W<u32, super::RX15MASK>;
#[doc = "Register RX15MASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RX15MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M0_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M0_A) -> Self {
        match variant {
            RX15M0_A::_0 => false,
            RX15M0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M0`"]
pub type RX15M0_R = crate::R<bool, RX15M0_A>;
impl RX15M0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M0_A {
        match self.bits {
            false => RX15M0_A::_0,
            true => RX15M0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M0_A::_1
    }
}
#[doc = "Write proxy for field `RX15M0`"]
pub struct RX15M0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M0_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M1_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M1_A) -> Self {
        match variant {
            RX15M1_A::_0 => false,
            RX15M1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M1`"]
pub type RX15M1_R = crate::R<bool, RX15M1_A>;
impl RX15M1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M1_A {
        match self.bits {
            false => RX15M1_A::_0,
            true => RX15M1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M1_A::_1
    }
}
#[doc = "Write proxy for field `RX15M1`"]
pub struct RX15M1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M1_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M2_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M2_A) -> Self {
        match variant {
            RX15M2_A::_0 => false,
            RX15M2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M2`"]
pub type RX15M2_R = crate::R<bool, RX15M2_A>;
impl RX15M2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M2_A {
        match self.bits {
            false => RX15M2_A::_0,
            true => RX15M2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M2_A::_1
    }
}
#[doc = "Write proxy for field `RX15M2`"]
pub struct RX15M2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M2_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M3_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M3_A) -> Self {
        match variant {
            RX15M3_A::_0 => false,
            RX15M3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M3`"]
pub type RX15M3_R = crate::R<bool, RX15M3_A>;
impl RX15M3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M3_A {
        match self.bits {
            false => RX15M3_A::_0,
            true => RX15M3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M3_A::_1
    }
}
#[doc = "Write proxy for field `RX15M3`"]
pub struct RX15M3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M3_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M4_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M4_A) -> Self {
        match variant {
            RX15M4_A::_0 => false,
            RX15M4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M4`"]
pub type RX15M4_R = crate::R<bool, RX15M4_A>;
impl RX15M4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M4_A {
        match self.bits {
            false => RX15M4_A::_0,
            true => RX15M4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M4_A::_1
    }
}
#[doc = "Write proxy for field `RX15M4`"]
pub struct RX15M4_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M4_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M5_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M5_A) -> Self {
        match variant {
            RX15M5_A::_0 => false,
            RX15M5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M5`"]
pub type RX15M5_R = crate::R<bool, RX15M5_A>;
impl RX15M5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M5_A {
        match self.bits {
            false => RX15M5_A::_0,
            true => RX15M5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M5_A::_1
    }
}
#[doc = "Write proxy for field `RX15M5`"]
pub struct RX15M5_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M5_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M6_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M6_A) -> Self {
        match variant {
            RX15M6_A::_0 => false,
            RX15M6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M6`"]
pub type RX15M6_R = crate::R<bool, RX15M6_A>;
impl RX15M6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M6_A {
        match self.bits {
            false => RX15M6_A::_0,
            true => RX15M6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M6_A::_1
    }
}
#[doc = "Write proxy for field `RX15M6`"]
pub struct RX15M6_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M6_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M7_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M7_A) -> Self {
        match variant {
            RX15M7_A::_0 => false,
            RX15M7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M7`"]
pub type RX15M7_R = crate::R<bool, RX15M7_A>;
impl RX15M7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M7_A {
        match self.bits {
            false => RX15M7_A::_0,
            true => RX15M7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M7_A::_1
    }
}
#[doc = "Write proxy for field `RX15M7`"]
pub struct RX15M7_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M7_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M8_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M8_A) -> Self {
        match variant {
            RX15M8_A::_0 => false,
            RX15M8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M8`"]
pub type RX15M8_R = crate::R<bool, RX15M8_A>;
impl RX15M8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M8_A {
        match self.bits {
            false => RX15M8_A::_0,
            true => RX15M8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M8_A::_1
    }
}
#[doc = "Write proxy for field `RX15M8`"]
pub struct RX15M8_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M8_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M9_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M9_A) -> Self {
        match variant {
            RX15M9_A::_0 => false,
            RX15M9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M9`"]
pub type RX15M9_R = crate::R<bool, RX15M9_A>;
impl RX15M9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M9_A {
        match self.bits {
            false => RX15M9_A::_0,
            true => RX15M9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M9_A::_1
    }
}
#[doc = "Write proxy for field `RX15M9`"]
pub struct RX15M9_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M9_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M10_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M10_A) -> Self {
        match variant {
            RX15M10_A::_0 => false,
            RX15M10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M10`"]
pub type RX15M10_R = crate::R<bool, RX15M10_A>;
impl RX15M10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M10_A {
        match self.bits {
            false => RX15M10_A::_0,
            true => RX15M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M10_A::_1
    }
}
#[doc = "Write proxy for field `RX15M10`"]
pub struct RX15M10_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M10_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M11_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M11_A) -> Self {
        match variant {
            RX15M11_A::_0 => false,
            RX15M11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M11`"]
pub type RX15M11_R = crate::R<bool, RX15M11_A>;
impl RX15M11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M11_A {
        match self.bits {
            false => RX15M11_A::_0,
            true => RX15M11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M11_A::_1
    }
}
#[doc = "Write proxy for field `RX15M11`"]
pub struct RX15M11_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M11_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M12_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M12_A) -> Self {
        match variant {
            RX15M12_A::_0 => false,
            RX15M12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M12`"]
pub type RX15M12_R = crate::R<bool, RX15M12_A>;
impl RX15M12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M12_A {
        match self.bits {
            false => RX15M12_A::_0,
            true => RX15M12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M12_A::_1
    }
}
#[doc = "Write proxy for field `RX15M12`"]
pub struct RX15M12_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M12_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M13_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M13_A) -> Self {
        match variant {
            RX15M13_A::_0 => false,
            RX15M13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M13`"]
pub type RX15M13_R = crate::R<bool, RX15M13_A>;
impl RX15M13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M13_A {
        match self.bits {
            false => RX15M13_A::_0,
            true => RX15M13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M13_A::_1
    }
}
#[doc = "Write proxy for field `RX15M13`"]
pub struct RX15M13_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M13_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M14_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M14_A) -> Self {
        match variant {
            RX15M14_A::_0 => false,
            RX15M14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M14`"]
pub type RX15M14_R = crate::R<bool, RX15M14_A>;
impl RX15M14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M14_A {
        match self.bits {
            false => RX15M14_A::_0,
            true => RX15M14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M14_A::_1
    }
}
#[doc = "Write proxy for field `RX15M14`"]
pub struct RX15M14_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M14_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M15_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M15_A) -> Self {
        match variant {
            RX15M15_A::_0 => false,
            RX15M15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M15`"]
pub type RX15M15_R = crate::R<bool, RX15M15_A>;
impl RX15M15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M15_A {
        match self.bits {
            false => RX15M15_A::_0,
            true => RX15M15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M15_A::_1
    }
}
#[doc = "Write proxy for field `RX15M15`"]
pub struct RX15M15_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M15_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M16_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M16_A) -> Self {
        match variant {
            RX15M16_A::_0 => false,
            RX15M16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M16`"]
pub type RX15M16_R = crate::R<bool, RX15M16_A>;
impl RX15M16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M16_A {
        match self.bits {
            false => RX15M16_A::_0,
            true => RX15M16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M16_A::_1
    }
}
#[doc = "Write proxy for field `RX15M16`"]
pub struct RX15M16_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M16_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M17_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M17_A) -> Self {
        match variant {
            RX15M17_A::_0 => false,
            RX15M17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M17`"]
pub type RX15M17_R = crate::R<bool, RX15M17_A>;
impl RX15M17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M17_A {
        match self.bits {
            false => RX15M17_A::_0,
            true => RX15M17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M17_A::_1
    }
}
#[doc = "Write proxy for field `RX15M17`"]
pub struct RX15M17_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M17_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M18_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M18_A) -> Self {
        match variant {
            RX15M18_A::_0 => false,
            RX15M18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M18`"]
pub type RX15M18_R = crate::R<bool, RX15M18_A>;
impl RX15M18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M18_A {
        match self.bits {
            false => RX15M18_A::_0,
            true => RX15M18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M18_A::_1
    }
}
#[doc = "Write proxy for field `RX15M18`"]
pub struct RX15M18_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M18_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M19_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M19_A) -> Self {
        match variant {
            RX15M19_A::_0 => false,
            RX15M19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M19`"]
pub type RX15M19_R = crate::R<bool, RX15M19_A>;
impl RX15M19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M19_A {
        match self.bits {
            false => RX15M19_A::_0,
            true => RX15M19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M19_A::_1
    }
}
#[doc = "Write proxy for field `RX15M19`"]
pub struct RX15M19_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M19_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M20_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M20_A) -> Self {
        match variant {
            RX15M20_A::_0 => false,
            RX15M20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M20`"]
pub type RX15M20_R = crate::R<bool, RX15M20_A>;
impl RX15M20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M20_A {
        match self.bits {
            false => RX15M20_A::_0,
            true => RX15M20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M20_A::_1
    }
}
#[doc = "Write proxy for field `RX15M20`"]
pub struct RX15M20_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M20_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M21_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M21_A) -> Self {
        match variant {
            RX15M21_A::_0 => false,
            RX15M21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M21`"]
pub type RX15M21_R = crate::R<bool, RX15M21_A>;
impl RX15M21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M21_A {
        match self.bits {
            false => RX15M21_A::_0,
            true => RX15M21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M21_A::_1
    }
}
#[doc = "Write proxy for field `RX15M21`"]
pub struct RX15M21_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M21_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M22_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M22_A) -> Self {
        match variant {
            RX15M22_A::_0 => false,
            RX15M22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M22`"]
pub type RX15M22_R = crate::R<bool, RX15M22_A>;
impl RX15M22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M22_A {
        match self.bits {
            false => RX15M22_A::_0,
            true => RX15M22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M22_A::_1
    }
}
#[doc = "Write proxy for field `RX15M22`"]
pub struct RX15M22_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M22_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M23_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M23_A) -> Self {
        match variant {
            RX15M23_A::_0 => false,
            RX15M23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M23`"]
pub type RX15M23_R = crate::R<bool, RX15M23_A>;
impl RX15M23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M23_A {
        match self.bits {
            false => RX15M23_A::_0,
            true => RX15M23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M23_A::_1
    }
}
#[doc = "Write proxy for field `RX15M23`"]
pub struct RX15M23_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M23_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M24_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M24_A) -> Self {
        match variant {
            RX15M24_A::_0 => false,
            RX15M24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M24`"]
pub type RX15M24_R = crate::R<bool, RX15M24_A>;
impl RX15M24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M24_A {
        match self.bits {
            false => RX15M24_A::_0,
            true => RX15M24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M24_A::_1
    }
}
#[doc = "Write proxy for field `RX15M24`"]
pub struct RX15M24_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M24_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M25_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M25_A) -> Self {
        match variant {
            RX15M25_A::_0 => false,
            RX15M25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M25`"]
pub type RX15M25_R = crate::R<bool, RX15M25_A>;
impl RX15M25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M25_A {
        match self.bits {
            false => RX15M25_A::_0,
            true => RX15M25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M25_A::_1
    }
}
#[doc = "Write proxy for field `RX15M25`"]
pub struct RX15M25_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M25_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M26_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M26_A) -> Self {
        match variant {
            RX15M26_A::_0 => false,
            RX15M26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M26`"]
pub type RX15M26_R = crate::R<bool, RX15M26_A>;
impl RX15M26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M26_A {
        match self.bits {
            false => RX15M26_A::_0,
            true => RX15M26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M26_A::_1
    }
}
#[doc = "Write proxy for field `RX15M26`"]
pub struct RX15M26_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M26_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M27_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M27_A) -> Self {
        match variant {
            RX15M27_A::_0 => false,
            RX15M27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M27`"]
pub type RX15M27_R = crate::R<bool, RX15M27_A>;
impl RX15M27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M27_A {
        match self.bits {
            false => RX15M27_A::_0,
            true => RX15M27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M27_A::_1
    }
}
#[doc = "Write proxy for field `RX15M27`"]
pub struct RX15M27_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M27_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M28_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M28_A) -> Self {
        match variant {
            RX15M28_A::_0 => false,
            RX15M28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M28`"]
pub type RX15M28_R = crate::R<bool, RX15M28_A>;
impl RX15M28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M28_A {
        match self.bits {
            false => RX15M28_A::_0,
            true => RX15M28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M28_A::_1
    }
}
#[doc = "Write proxy for field `RX15M28`"]
pub struct RX15M28_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M28_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M29_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M29_A) -> Self {
        match variant {
            RX15M29_A::_0 => false,
            RX15M29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M29`"]
pub type RX15M29_R = crate::R<bool, RX15M29_A>;
impl RX15M29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M29_A {
        match self.bits {
            false => RX15M29_A::_0,
            true => RX15M29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M29_A::_1
    }
}
#[doc = "Write proxy for field `RX15M29`"]
pub struct RX15M29_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M29_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M30_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M30_A) -> Self {
        match variant {
            RX15M30_A::_0 => false,
            RX15M30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M30`"]
pub type RX15M30_R = crate::R<bool, RX15M30_A>;
impl RX15M30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M30_A {
        match self.bits {
            false => RX15M30_A::_0,
            true => RX15M30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M30_A::_1
    }
}
#[doc = "Write proxy for field `RX15M30`"]
pub struct RX15M30_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M30_A::_1)
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
#[doc = "Rx Buffer 15 Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<RX15M31_A> for bool {
    #[inline(always)]
    fn from(variant: RX15M31_A) -> Self {
        match variant {
            RX15M31_A::_0 => false,
            RX15M31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX15M31`"]
pub type RX15M31_R = crate::R<bool, RX15M31_A>;
impl RX15M31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX15M31_A {
        match self.bits {
            false => RX15M31_A::_0,
            true => RX15M31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX15M31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX15M31_A::_1
    }
}
#[doc = "Write proxy for field `RX15M31`"]
pub struct RX15M31_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M31_A::_1)
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
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m0(&self) -> RX15M0_R {
        RX15M0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m1(&self) -> RX15M1_R {
        RX15M1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m2(&self) -> RX15M2_R {
        RX15M2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m3(&self) -> RX15M3_R {
        RX15M3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m4(&self) -> RX15M4_R {
        RX15M4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m5(&self) -> RX15M5_R {
        RX15M5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m6(&self) -> RX15M6_R {
        RX15M6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m7(&self) -> RX15M7_R {
        RX15M7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m8(&self) -> RX15M8_R {
        RX15M8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m9(&self) -> RX15M9_R {
        RX15M9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m10(&self) -> RX15M10_R {
        RX15M10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m11(&self) -> RX15M11_R {
        RX15M11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m12(&self) -> RX15M12_R {
        RX15M12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m13(&self) -> RX15M13_R {
        RX15M13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m14(&self) -> RX15M14_R {
        RX15M14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m15(&self) -> RX15M15_R {
        RX15M15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m16(&self) -> RX15M16_R {
        RX15M16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m17(&self) -> RX15M17_R {
        RX15M17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m18(&self) -> RX15M18_R {
        RX15M18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m19(&self) -> RX15M19_R {
        RX15M19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m20(&self) -> RX15M20_R {
        RX15M20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m21(&self) -> RX15M21_R {
        RX15M21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m22(&self) -> RX15M22_R {
        RX15M22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m23(&self) -> RX15M23_R {
        RX15M23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m24(&self) -> RX15M24_R {
        RX15M24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m25(&self) -> RX15M25_R {
        RX15M25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m26(&self) -> RX15M26_R {
        RX15M26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m27(&self) -> RX15M27_R {
        RX15M27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m28(&self) -> RX15M28_R {
        RX15M28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m29(&self) -> RX15M29_R {
        RX15M29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m30(&self) -> RX15M30_R {
        RX15M30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m31(&self) -> RX15M31_R {
        RX15M31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m0(&mut self) -> RX15M0_W {
        RX15M0_W { w: self }
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m1(&mut self) -> RX15M1_W {
        RX15M1_W { w: self }
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m2(&mut self) -> RX15M2_W {
        RX15M2_W { w: self }
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m3(&mut self) -> RX15M3_W {
        RX15M3_W { w: self }
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m4(&mut self) -> RX15M4_W {
        RX15M4_W { w: self }
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m5(&mut self) -> RX15M5_W {
        RX15M5_W { w: self }
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m6(&mut self) -> RX15M6_W {
        RX15M6_W { w: self }
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m7(&mut self) -> RX15M7_W {
        RX15M7_W { w: self }
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m8(&mut self) -> RX15M8_W {
        RX15M8_W { w: self }
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m9(&mut self) -> RX15M9_W {
        RX15M9_W { w: self }
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m10(&mut self) -> RX15M10_W {
        RX15M10_W { w: self }
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m11(&mut self) -> RX15M11_W {
        RX15M11_W { w: self }
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m12(&mut self) -> RX15M12_W {
        RX15M12_W { w: self }
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m13(&mut self) -> RX15M13_W {
        RX15M13_W { w: self }
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m14(&mut self) -> RX15M14_W {
        RX15M14_W { w: self }
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m15(&mut self) -> RX15M15_W {
        RX15M15_W { w: self }
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m16(&mut self) -> RX15M16_W {
        RX15M16_W { w: self }
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m17(&mut self) -> RX15M17_W {
        RX15M17_W { w: self }
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m18(&mut self) -> RX15M18_W {
        RX15M18_W { w: self }
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m19(&mut self) -> RX15M19_W {
        RX15M19_W { w: self }
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m20(&mut self) -> RX15M20_W {
        RX15M20_W { w: self }
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m21(&mut self) -> RX15M21_W {
        RX15M21_W { w: self }
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m22(&mut self) -> RX15M22_W {
        RX15M22_W { w: self }
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m23(&mut self) -> RX15M23_W {
        RX15M23_W { w: self }
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m24(&mut self) -> RX15M24_W {
        RX15M24_W { w: self }
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m25(&mut self) -> RX15M25_W {
        RX15M25_W { w: self }
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m26(&mut self) -> RX15M26_W {
        RX15M26_W { w: self }
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m27(&mut self) -> RX15M27_W {
        RX15M27_W { w: self }
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m28(&mut self) -> RX15M28_W {
        RX15M28_W { w: self }
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m29(&mut self) -> RX15M29_W {
        RX15M29_W { w: self }
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m30(&mut self) -> RX15M30_W {
        RX15M30_W { w: self }
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline(always)]
    pub fn rx15m31(&mut self) -> RX15M31_W {
        RX15M31_W { w: self }
    }
}
