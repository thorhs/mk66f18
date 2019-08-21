#[doc = "Reader of register RMR"]
pub type R = crate::R<u32, super::RMR>;
#[doc = "Writer for register RMR"]
pub type W = crate::W<u32, super::RMR>;
#[doc = "Register RMR `reset()`'s with value 0"]
impl crate::ResetValue for super::RMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM0_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM0_A> for bool {
    #[inline(always)]
    fn from(variant: RWM0_A) -> Self {
        match variant {
            RWM0_A::_0 => false,
            RWM0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM0`"]
pub type RWM0_R = crate::R<bool, RWM0_A>;
impl RWM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM0_A {
        match self.bits {
            false => RWM0_A::_0,
            true => RWM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM0_A::_1
    }
}
#[doc = "Write proxy for field `RWM0`"]
pub struct RWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM0_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM0_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM1_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM1_A> for bool {
    #[inline(always)]
    fn from(variant: RWM1_A) -> Self {
        match variant {
            RWM1_A::_0 => false,
            RWM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM1`"]
pub type RWM1_R = crate::R<bool, RWM1_A>;
impl RWM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM1_A {
        match self.bits {
            false => RWM1_A::_0,
            true => RWM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM1_A::_1
    }
}
#[doc = "Write proxy for field `RWM1`"]
pub struct RWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM1_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM1_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM2_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM2_A> for bool {
    #[inline(always)]
    fn from(variant: RWM2_A) -> Self {
        match variant {
            RWM2_A::_0 => false,
            RWM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM2`"]
pub type RWM2_R = crate::R<bool, RWM2_A>;
impl RWM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM2_A {
        match self.bits {
            false => RWM2_A::_0,
            true => RWM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM2_A::_1
    }
}
#[doc = "Write proxy for field `RWM2`"]
pub struct RWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM2_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM2_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM3_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM3_A> for bool {
    #[inline(always)]
    fn from(variant: RWM3_A) -> Self {
        match variant {
            RWM3_A::_0 => false,
            RWM3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM3`"]
pub type RWM3_R = crate::R<bool, RWM3_A>;
impl RWM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM3_A {
        match self.bits {
            false => RWM3_A::_0,
            true => RWM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM3_A::_1
    }
}
#[doc = "Write proxy for field `RWM3`"]
pub struct RWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM3_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM3_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM4_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM4_A> for bool {
    #[inline(always)]
    fn from(variant: RWM4_A) -> Self {
        match variant {
            RWM4_A::_0 => false,
            RWM4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM4`"]
pub type RWM4_R = crate::R<bool, RWM4_A>;
impl RWM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM4_A {
        match self.bits {
            false => RWM4_A::_0,
            true => RWM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM4_A::_1
    }
}
#[doc = "Write proxy for field `RWM4`"]
pub struct RWM4_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM4_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM4_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM5_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM5_A> for bool {
    #[inline(always)]
    fn from(variant: RWM5_A) -> Self {
        match variant {
            RWM5_A::_0 => false,
            RWM5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM5`"]
pub type RWM5_R = crate::R<bool, RWM5_A>;
impl RWM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM5_A {
        match self.bits {
            false => RWM5_A::_0,
            true => RWM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM5_A::_1
    }
}
#[doc = "Write proxy for field `RWM5`"]
pub struct RWM5_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM5_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM5_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM6_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM6_A> for bool {
    #[inline(always)]
    fn from(variant: RWM6_A) -> Self {
        match variant {
            RWM6_A::_0 => false,
            RWM6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM6`"]
pub type RWM6_R = crate::R<bool, RWM6_A>;
impl RWM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM6_A {
        match self.bits {
            false => RWM6_A::_0,
            true => RWM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM6_A::_1
    }
}
#[doc = "Write proxy for field `RWM6`"]
pub struct RWM6_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM6_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM6_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM7_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM7_A> for bool {
    #[inline(always)]
    fn from(variant: RWM7_A) -> Self {
        match variant {
            RWM7_A::_0 => false,
            RWM7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM7`"]
pub type RWM7_R = crate::R<bool, RWM7_A>;
impl RWM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM7_A {
        match self.bits {
            false => RWM7_A::_0,
            true => RWM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM7_A::_1
    }
}
#[doc = "Write proxy for field `RWM7`"]
pub struct RWM7_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM7_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM7_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM8_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM8_A> for bool {
    #[inline(always)]
    fn from(variant: RWM8_A) -> Self {
        match variant {
            RWM8_A::_0 => false,
            RWM8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM8`"]
pub type RWM8_R = crate::R<bool, RWM8_A>;
impl RWM8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM8_A {
        match self.bits {
            false => RWM8_A::_0,
            true => RWM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM8_A::_1
    }
}
#[doc = "Write proxy for field `RWM8`"]
pub struct RWM8_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM8_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM8_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM9_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM9_A> for bool {
    #[inline(always)]
    fn from(variant: RWM9_A) -> Self {
        match variant {
            RWM9_A::_0 => false,
            RWM9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM9`"]
pub type RWM9_R = crate::R<bool, RWM9_A>;
impl RWM9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM9_A {
        match self.bits {
            false => RWM9_A::_0,
            true => RWM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM9_A::_1
    }
}
#[doc = "Write proxy for field `RWM9`"]
pub struct RWM9_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM9_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM9_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM10_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM10_A> for bool {
    #[inline(always)]
    fn from(variant: RWM10_A) -> Self {
        match variant {
            RWM10_A::_0 => false,
            RWM10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM10`"]
pub type RWM10_R = crate::R<bool, RWM10_A>;
impl RWM10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM10_A {
        match self.bits {
            false => RWM10_A::_0,
            true => RWM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM10_A::_1
    }
}
#[doc = "Write proxy for field `RWM10`"]
pub struct RWM10_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM10_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM10_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM11_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM11_A> for bool {
    #[inline(always)]
    fn from(variant: RWM11_A) -> Self {
        match variant {
            RWM11_A::_0 => false,
            RWM11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM11`"]
pub type RWM11_R = crate::R<bool, RWM11_A>;
impl RWM11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM11_A {
        match self.bits {
            false => RWM11_A::_0,
            true => RWM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM11_A::_1
    }
}
#[doc = "Write proxy for field `RWM11`"]
pub struct RWM11_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM11_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM11_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM12_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM12_A> for bool {
    #[inline(always)]
    fn from(variant: RWM12_A) -> Self {
        match variant {
            RWM12_A::_0 => false,
            RWM12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM12`"]
pub type RWM12_R = crate::R<bool, RWM12_A>;
impl RWM12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM12_A {
        match self.bits {
            false => RWM12_A::_0,
            true => RWM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM12_A::_1
    }
}
#[doc = "Write proxy for field `RWM12`"]
pub struct RWM12_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM12_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM12_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM13_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM13_A> for bool {
    #[inline(always)]
    fn from(variant: RWM13_A) -> Self {
        match variant {
            RWM13_A::_0 => false,
            RWM13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM13`"]
pub type RWM13_R = crate::R<bool, RWM13_A>;
impl RWM13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM13_A {
        match self.bits {
            false => RWM13_A::_0,
            true => RWM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM13_A::_1
    }
}
#[doc = "Write proxy for field `RWM13`"]
pub struct RWM13_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM13_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM13_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM14_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM14_A> for bool {
    #[inline(always)]
    fn from(variant: RWM14_A) -> Self {
        match variant {
            RWM14_A::_0 => false,
            RWM14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM14`"]
pub type RWM14_R = crate::R<bool, RWM14_A>;
impl RWM14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM14_A {
        match self.bits {
            false => RWM14_A::_0,
            true => RWM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM14_A::_1
    }
}
#[doc = "Write proxy for field `RWM14`"]
pub struct RWM14_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM14_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM14_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM15_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM15_A> for bool {
    #[inline(always)]
    fn from(variant: RWM15_A) -> Self {
        match variant {
            RWM15_A::_0 => false,
            RWM15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM15`"]
pub type RWM15_R = crate::R<bool, RWM15_A>;
impl RWM15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM15_A {
        match self.bits {
            false => RWM15_A::_0,
            true => RWM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM15_A::_1
    }
}
#[doc = "Write proxy for field `RWM15`"]
pub struct RWM15_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM15_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM15_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM16_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM16_A> for bool {
    #[inline(always)]
    fn from(variant: RWM16_A) -> Self {
        match variant {
            RWM16_A::_0 => false,
            RWM16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM16`"]
pub type RWM16_R = crate::R<bool, RWM16_A>;
impl RWM16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM16_A {
        match self.bits {
            false => RWM16_A::_0,
            true => RWM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM16_A::_1
    }
}
#[doc = "Write proxy for field `RWM16`"]
pub struct RWM16_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM16_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM16_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM17_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM17_A> for bool {
    #[inline(always)]
    fn from(variant: RWM17_A) -> Self {
        match variant {
            RWM17_A::_0 => false,
            RWM17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM17`"]
pub type RWM17_R = crate::R<bool, RWM17_A>;
impl RWM17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM17_A {
        match self.bits {
            false => RWM17_A::_0,
            true => RWM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM17_A::_1
    }
}
#[doc = "Write proxy for field `RWM17`"]
pub struct RWM17_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM17_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM17_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM18_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM18_A> for bool {
    #[inline(always)]
    fn from(variant: RWM18_A) -> Self {
        match variant {
            RWM18_A::_0 => false,
            RWM18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM18`"]
pub type RWM18_R = crate::R<bool, RWM18_A>;
impl RWM18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM18_A {
        match self.bits {
            false => RWM18_A::_0,
            true => RWM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM18_A::_1
    }
}
#[doc = "Write proxy for field `RWM18`"]
pub struct RWM18_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM18_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM18_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM19_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM19_A> for bool {
    #[inline(always)]
    fn from(variant: RWM19_A) -> Self {
        match variant {
            RWM19_A::_0 => false,
            RWM19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM19`"]
pub type RWM19_R = crate::R<bool, RWM19_A>;
impl RWM19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM19_A {
        match self.bits {
            false => RWM19_A::_0,
            true => RWM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM19_A::_1
    }
}
#[doc = "Write proxy for field `RWM19`"]
pub struct RWM19_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM19_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM19_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM20_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM20_A> for bool {
    #[inline(always)]
    fn from(variant: RWM20_A) -> Self {
        match variant {
            RWM20_A::_0 => false,
            RWM20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM20`"]
pub type RWM20_R = crate::R<bool, RWM20_A>;
impl RWM20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM20_A {
        match self.bits {
            false => RWM20_A::_0,
            true => RWM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM20_A::_1
    }
}
#[doc = "Write proxy for field `RWM20`"]
pub struct RWM20_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM20_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM20_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM21_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM21_A> for bool {
    #[inline(always)]
    fn from(variant: RWM21_A) -> Self {
        match variant {
            RWM21_A::_0 => false,
            RWM21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM21`"]
pub type RWM21_R = crate::R<bool, RWM21_A>;
impl RWM21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM21_A {
        match self.bits {
            false => RWM21_A::_0,
            true => RWM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM21_A::_1
    }
}
#[doc = "Write proxy for field `RWM21`"]
pub struct RWM21_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM21_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM21_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM22_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM22_A> for bool {
    #[inline(always)]
    fn from(variant: RWM22_A) -> Self {
        match variant {
            RWM22_A::_0 => false,
            RWM22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM22`"]
pub type RWM22_R = crate::R<bool, RWM22_A>;
impl RWM22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM22_A {
        match self.bits {
            false => RWM22_A::_0,
            true => RWM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM22_A::_1
    }
}
#[doc = "Write proxy for field `RWM22`"]
pub struct RWM22_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM22_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM22_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM23_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM23_A> for bool {
    #[inline(always)]
    fn from(variant: RWM23_A) -> Self {
        match variant {
            RWM23_A::_0 => false,
            RWM23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM23`"]
pub type RWM23_R = crate::R<bool, RWM23_A>;
impl RWM23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM23_A {
        match self.bits {
            false => RWM23_A::_0,
            true => RWM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM23_A::_1
    }
}
#[doc = "Write proxy for field `RWM23`"]
pub struct RWM23_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM23_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM23_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM24_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM24_A> for bool {
    #[inline(always)]
    fn from(variant: RWM24_A) -> Self {
        match variant {
            RWM24_A::_0 => false,
            RWM24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM24`"]
pub type RWM24_R = crate::R<bool, RWM24_A>;
impl RWM24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM24_A {
        match self.bits {
            false => RWM24_A::_0,
            true => RWM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM24_A::_1
    }
}
#[doc = "Write proxy for field `RWM24`"]
pub struct RWM24_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM24_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM24_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM25_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM25_A> for bool {
    #[inline(always)]
    fn from(variant: RWM25_A) -> Self {
        match variant {
            RWM25_A::_0 => false,
            RWM25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM25`"]
pub type RWM25_R = crate::R<bool, RWM25_A>;
impl RWM25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM25_A {
        match self.bits {
            false => RWM25_A::_0,
            true => RWM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM25_A::_1
    }
}
#[doc = "Write proxy for field `RWM25`"]
pub struct RWM25_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM25_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM25_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM26_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM26_A> for bool {
    #[inline(always)]
    fn from(variant: RWM26_A) -> Self {
        match variant {
            RWM26_A::_0 => false,
            RWM26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM26`"]
pub type RWM26_R = crate::R<bool, RWM26_A>;
impl RWM26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM26_A {
        match self.bits {
            false => RWM26_A::_0,
            true => RWM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM26_A::_1
    }
}
#[doc = "Write proxy for field `RWM26`"]
pub struct RWM26_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM26_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM26_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM27_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM27_A> for bool {
    #[inline(always)]
    fn from(variant: RWM27_A) -> Self {
        match variant {
            RWM27_A::_0 => false,
            RWM27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM27`"]
pub type RWM27_R = crate::R<bool, RWM27_A>;
impl RWM27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM27_A {
        match self.bits {
            false => RWM27_A::_0,
            true => RWM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM27_A::_1
    }
}
#[doc = "Write proxy for field `RWM27`"]
pub struct RWM27_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM27_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM27_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM28_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM28_A> for bool {
    #[inline(always)]
    fn from(variant: RWM28_A) -> Self {
        match variant {
            RWM28_A::_0 => false,
            RWM28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM28`"]
pub type RWM28_R = crate::R<bool, RWM28_A>;
impl RWM28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM28_A {
        match self.bits {
            false => RWM28_A::_0,
            true => RWM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM28_A::_1
    }
}
#[doc = "Write proxy for field `RWM28`"]
pub struct RWM28_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM28_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM28_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM29_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM29_A> for bool {
    #[inline(always)]
    fn from(variant: RWM29_A) -> Self {
        match variant {
            RWM29_A::_0 => false,
            RWM29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM29`"]
pub type RWM29_R = crate::R<bool, RWM29_A>;
impl RWM29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM29_A {
        match self.bits {
            false => RWM29_A::_0,
            true => RWM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM29_A::_1
    }
}
#[doc = "Write proxy for field `RWM29`"]
pub struct RWM29_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM29_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM29_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM30_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM30_A> for bool {
    #[inline(always)]
    fn from(variant: RWM30_A) -> Self {
        match variant {
            RWM30_A::_0 => false,
            RWM30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM30`"]
pub type RWM30_R = crate::R<bool, RWM30_A>;
impl RWM30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM30_A {
        match self.bits {
            false => RWM30_A::_0,
            true => RWM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM30_A::_1
    }
}
#[doc = "Write proxy for field `RWM30`"]
pub struct RWM30_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM30_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM30_A::_1)
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
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM31_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked."]
    _1,
}
impl From<RWM31_A> for bool {
    #[inline(always)]
    fn from(variant: RWM31_A) -> Self {
        match variant {
            RWM31_A::_0 => false,
            RWM31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWM31`"]
pub type RWM31_R = crate::R<bool, RWM31_A>;
impl RWM31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWM31_A {
        match self.bits {
            false => RWM31_A::_0,
            true => RWM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM31_A::_1
    }
}
#[doc = "Write proxy for field `RWM31`"]
pub struct RWM31_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM31_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM31_A::_1)
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
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm0(&self) -> RWM0_R {
        RWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm1(&self) -> RWM1_R {
        RWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm2(&self) -> RWM2_R {
        RWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm3(&self) -> RWM3_R {
        RWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm4(&self) -> RWM4_R {
        RWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm5(&self) -> RWM5_R {
        RWM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm6(&self) -> RWM6_R {
        RWM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm7(&self) -> RWM7_R {
        RWM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm8(&self) -> RWM8_R {
        RWM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm9(&self) -> RWM9_R {
        RWM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm10(&self) -> RWM10_R {
        RWM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm11(&self) -> RWM11_R {
        RWM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm12(&self) -> RWM12_R {
        RWM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm13(&self) -> RWM13_R {
        RWM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm14(&self) -> RWM14_R {
        RWM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm15(&self) -> RWM15_R {
        RWM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm16(&self) -> RWM16_R {
        RWM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm17(&self) -> RWM17_R {
        RWM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm18(&self) -> RWM18_R {
        RWM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm19(&self) -> RWM19_R {
        RWM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm20(&self) -> RWM20_R {
        RWM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm21(&self) -> RWM21_R {
        RWM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm22(&self) -> RWM22_R {
        RWM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm23(&self) -> RWM23_R {
        RWM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm24(&self) -> RWM24_R {
        RWM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm25(&self) -> RWM25_R {
        RWM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm26(&self) -> RWM26_R {
        RWM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm27(&self) -> RWM27_R {
        RWM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm28(&self) -> RWM28_R {
        RWM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm29(&self) -> RWM29_R {
        RWM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm30(&self) -> RWM30_R {
        RWM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm31(&self) -> RWM31_R {
        RWM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm0(&mut self) -> RWM0_W {
        RWM0_W { w: self }
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm1(&mut self) -> RWM1_W {
        RWM1_W { w: self }
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm2(&mut self) -> RWM2_W {
        RWM2_W { w: self }
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm3(&mut self) -> RWM3_W {
        RWM3_W { w: self }
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm4(&mut self) -> RWM4_W {
        RWM4_W { w: self }
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm5(&mut self) -> RWM5_W {
        RWM5_W { w: self }
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm6(&mut self) -> RWM6_W {
        RWM6_W { w: self }
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm7(&mut self) -> RWM7_W {
        RWM7_W { w: self }
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm8(&mut self) -> RWM8_W {
        RWM8_W { w: self }
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm9(&mut self) -> RWM9_W {
        RWM9_W { w: self }
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm10(&mut self) -> RWM10_W {
        RWM10_W { w: self }
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm11(&mut self) -> RWM11_W {
        RWM11_W { w: self }
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm12(&mut self) -> RWM12_W {
        RWM12_W { w: self }
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm13(&mut self) -> RWM13_W {
        RWM13_W { w: self }
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm14(&mut self) -> RWM14_W {
        RWM14_W { w: self }
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm15(&mut self) -> RWM15_W {
        RWM15_W { w: self }
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm16(&mut self) -> RWM16_W {
        RWM16_W { w: self }
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm17(&mut self) -> RWM17_W {
        RWM17_W { w: self }
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm18(&mut self) -> RWM18_W {
        RWM18_W { w: self }
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm19(&mut self) -> RWM19_W {
        RWM19_W { w: self }
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm20(&mut self) -> RWM20_W {
        RWM20_W { w: self }
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm21(&mut self) -> RWM21_W {
        RWM21_W { w: self }
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm22(&mut self) -> RWM22_W {
        RWM22_W { w: self }
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm23(&mut self) -> RWM23_W {
        RWM23_W { w: self }
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm24(&mut self) -> RWM24_W {
        RWM24_W { w: self }
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm25(&mut self) -> RWM25_W {
        RWM25_W { w: self }
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm26(&mut self) -> RWM26_W {
        RWM26_W { w: self }
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm27(&mut self) -> RWM27_W {
        RWM27_W { w: self }
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm28(&mut self) -> RWM28_W {
        RWM28_W { w: self }
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm29(&mut self) -> RWM29_W {
        RWM29_W { w: self }
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm30(&mut self) -> RWM30_W {
        RWM30_W { w: self }
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm31(&mut self) -> RWM31_W {
        RWM31_W { w: self }
    }
}
