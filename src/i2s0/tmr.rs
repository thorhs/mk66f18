#[doc = "Reader of register TMR"]
pub type R = crate::R<u32, super::TMR>;
#[doc = "Writer for register TMR"]
pub type W = crate::W<u32, super::TMR>;
#[doc = "Register TMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM0_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM0_A> for bool {
    #[inline(always)]
    fn from(variant: TWM0_A) -> Self {
        match variant {
            TWM0_A::_0 => false,
            TWM0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM0`"]
pub type TWM0_R = crate::R<bool, TWM0_A>;
impl TWM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM0_A {
        match self.bits {
            false => TWM0_A::_0,
            true => TWM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM0_A::_1
    }
}
#[doc = "Write proxy for field `TWM0`"]
pub struct TWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM0_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM0_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM1_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM1_A> for bool {
    #[inline(always)]
    fn from(variant: TWM1_A) -> Self {
        match variant {
            TWM1_A::_0 => false,
            TWM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM1`"]
pub type TWM1_R = crate::R<bool, TWM1_A>;
impl TWM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM1_A {
        match self.bits {
            false => TWM1_A::_0,
            true => TWM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM1_A::_1
    }
}
#[doc = "Write proxy for field `TWM1`"]
pub struct TWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM1_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM1_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM2_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM2_A> for bool {
    #[inline(always)]
    fn from(variant: TWM2_A) -> Self {
        match variant {
            TWM2_A::_0 => false,
            TWM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM2`"]
pub type TWM2_R = crate::R<bool, TWM2_A>;
impl TWM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM2_A {
        match self.bits {
            false => TWM2_A::_0,
            true => TWM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM2_A::_1
    }
}
#[doc = "Write proxy for field `TWM2`"]
pub struct TWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM2_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM2_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM3_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM3_A> for bool {
    #[inline(always)]
    fn from(variant: TWM3_A) -> Self {
        match variant {
            TWM3_A::_0 => false,
            TWM3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM3`"]
pub type TWM3_R = crate::R<bool, TWM3_A>;
impl TWM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM3_A {
        match self.bits {
            false => TWM3_A::_0,
            true => TWM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM3_A::_1
    }
}
#[doc = "Write proxy for field `TWM3`"]
pub struct TWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM3_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM3_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM4_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM4_A> for bool {
    #[inline(always)]
    fn from(variant: TWM4_A) -> Self {
        match variant {
            TWM4_A::_0 => false,
            TWM4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM4`"]
pub type TWM4_R = crate::R<bool, TWM4_A>;
impl TWM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM4_A {
        match self.bits {
            false => TWM4_A::_0,
            true => TWM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM4_A::_1
    }
}
#[doc = "Write proxy for field `TWM4`"]
pub struct TWM4_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM4_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM4_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM5_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM5_A> for bool {
    #[inline(always)]
    fn from(variant: TWM5_A) -> Self {
        match variant {
            TWM5_A::_0 => false,
            TWM5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM5`"]
pub type TWM5_R = crate::R<bool, TWM5_A>;
impl TWM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM5_A {
        match self.bits {
            false => TWM5_A::_0,
            true => TWM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM5_A::_1
    }
}
#[doc = "Write proxy for field `TWM5`"]
pub struct TWM5_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM5_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM5_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM6_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM6_A> for bool {
    #[inline(always)]
    fn from(variant: TWM6_A) -> Self {
        match variant {
            TWM6_A::_0 => false,
            TWM6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM6`"]
pub type TWM6_R = crate::R<bool, TWM6_A>;
impl TWM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM6_A {
        match self.bits {
            false => TWM6_A::_0,
            true => TWM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM6_A::_1
    }
}
#[doc = "Write proxy for field `TWM6`"]
pub struct TWM6_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM6_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM6_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM7_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM7_A> for bool {
    #[inline(always)]
    fn from(variant: TWM7_A) -> Self {
        match variant {
            TWM7_A::_0 => false,
            TWM7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM7`"]
pub type TWM7_R = crate::R<bool, TWM7_A>;
impl TWM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM7_A {
        match self.bits {
            false => TWM7_A::_0,
            true => TWM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM7_A::_1
    }
}
#[doc = "Write proxy for field `TWM7`"]
pub struct TWM7_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM7_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM7_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM8_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM8_A> for bool {
    #[inline(always)]
    fn from(variant: TWM8_A) -> Self {
        match variant {
            TWM8_A::_0 => false,
            TWM8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM8`"]
pub type TWM8_R = crate::R<bool, TWM8_A>;
impl TWM8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM8_A {
        match self.bits {
            false => TWM8_A::_0,
            true => TWM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM8_A::_1
    }
}
#[doc = "Write proxy for field `TWM8`"]
pub struct TWM8_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM8_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM8_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM9_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM9_A> for bool {
    #[inline(always)]
    fn from(variant: TWM9_A) -> Self {
        match variant {
            TWM9_A::_0 => false,
            TWM9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM9`"]
pub type TWM9_R = crate::R<bool, TWM9_A>;
impl TWM9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM9_A {
        match self.bits {
            false => TWM9_A::_0,
            true => TWM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM9_A::_1
    }
}
#[doc = "Write proxy for field `TWM9`"]
pub struct TWM9_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM9_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM9_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM10_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM10_A> for bool {
    #[inline(always)]
    fn from(variant: TWM10_A) -> Self {
        match variant {
            TWM10_A::_0 => false,
            TWM10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM10`"]
pub type TWM10_R = crate::R<bool, TWM10_A>;
impl TWM10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM10_A {
        match self.bits {
            false => TWM10_A::_0,
            true => TWM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM10_A::_1
    }
}
#[doc = "Write proxy for field `TWM10`"]
pub struct TWM10_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM10_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM10_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM11_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM11_A> for bool {
    #[inline(always)]
    fn from(variant: TWM11_A) -> Self {
        match variant {
            TWM11_A::_0 => false,
            TWM11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM11`"]
pub type TWM11_R = crate::R<bool, TWM11_A>;
impl TWM11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM11_A {
        match self.bits {
            false => TWM11_A::_0,
            true => TWM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM11_A::_1
    }
}
#[doc = "Write proxy for field `TWM11`"]
pub struct TWM11_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM11_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM11_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM12_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM12_A> for bool {
    #[inline(always)]
    fn from(variant: TWM12_A) -> Self {
        match variant {
            TWM12_A::_0 => false,
            TWM12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM12`"]
pub type TWM12_R = crate::R<bool, TWM12_A>;
impl TWM12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM12_A {
        match self.bits {
            false => TWM12_A::_0,
            true => TWM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM12_A::_1
    }
}
#[doc = "Write proxy for field `TWM12`"]
pub struct TWM12_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM12_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM12_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM13_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM13_A> for bool {
    #[inline(always)]
    fn from(variant: TWM13_A) -> Self {
        match variant {
            TWM13_A::_0 => false,
            TWM13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM13`"]
pub type TWM13_R = crate::R<bool, TWM13_A>;
impl TWM13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM13_A {
        match self.bits {
            false => TWM13_A::_0,
            true => TWM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM13_A::_1
    }
}
#[doc = "Write proxy for field `TWM13`"]
pub struct TWM13_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM13_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM13_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM14_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM14_A> for bool {
    #[inline(always)]
    fn from(variant: TWM14_A) -> Self {
        match variant {
            TWM14_A::_0 => false,
            TWM14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM14`"]
pub type TWM14_R = crate::R<bool, TWM14_A>;
impl TWM14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM14_A {
        match self.bits {
            false => TWM14_A::_0,
            true => TWM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM14_A::_1
    }
}
#[doc = "Write proxy for field `TWM14`"]
pub struct TWM14_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM14_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM14_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM15_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM15_A> for bool {
    #[inline(always)]
    fn from(variant: TWM15_A) -> Self {
        match variant {
            TWM15_A::_0 => false,
            TWM15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM15`"]
pub type TWM15_R = crate::R<bool, TWM15_A>;
impl TWM15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM15_A {
        match self.bits {
            false => TWM15_A::_0,
            true => TWM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM15_A::_1
    }
}
#[doc = "Write proxy for field `TWM15`"]
pub struct TWM15_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM15_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM15_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM16_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM16_A> for bool {
    #[inline(always)]
    fn from(variant: TWM16_A) -> Self {
        match variant {
            TWM16_A::_0 => false,
            TWM16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM16`"]
pub type TWM16_R = crate::R<bool, TWM16_A>;
impl TWM16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM16_A {
        match self.bits {
            false => TWM16_A::_0,
            true => TWM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM16_A::_1
    }
}
#[doc = "Write proxy for field `TWM16`"]
pub struct TWM16_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM16_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM16_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM17_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM17_A> for bool {
    #[inline(always)]
    fn from(variant: TWM17_A) -> Self {
        match variant {
            TWM17_A::_0 => false,
            TWM17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM17`"]
pub type TWM17_R = crate::R<bool, TWM17_A>;
impl TWM17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM17_A {
        match self.bits {
            false => TWM17_A::_0,
            true => TWM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM17_A::_1
    }
}
#[doc = "Write proxy for field `TWM17`"]
pub struct TWM17_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM17_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM17_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM18_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM18_A> for bool {
    #[inline(always)]
    fn from(variant: TWM18_A) -> Self {
        match variant {
            TWM18_A::_0 => false,
            TWM18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM18`"]
pub type TWM18_R = crate::R<bool, TWM18_A>;
impl TWM18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM18_A {
        match self.bits {
            false => TWM18_A::_0,
            true => TWM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM18_A::_1
    }
}
#[doc = "Write proxy for field `TWM18`"]
pub struct TWM18_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM18_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM18_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM19_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM19_A> for bool {
    #[inline(always)]
    fn from(variant: TWM19_A) -> Self {
        match variant {
            TWM19_A::_0 => false,
            TWM19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM19`"]
pub type TWM19_R = crate::R<bool, TWM19_A>;
impl TWM19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM19_A {
        match self.bits {
            false => TWM19_A::_0,
            true => TWM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM19_A::_1
    }
}
#[doc = "Write proxy for field `TWM19`"]
pub struct TWM19_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM19_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM19_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM20_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM20_A> for bool {
    #[inline(always)]
    fn from(variant: TWM20_A) -> Self {
        match variant {
            TWM20_A::_0 => false,
            TWM20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM20`"]
pub type TWM20_R = crate::R<bool, TWM20_A>;
impl TWM20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM20_A {
        match self.bits {
            false => TWM20_A::_0,
            true => TWM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM20_A::_1
    }
}
#[doc = "Write proxy for field `TWM20`"]
pub struct TWM20_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM20_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM20_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM21_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM21_A> for bool {
    #[inline(always)]
    fn from(variant: TWM21_A) -> Self {
        match variant {
            TWM21_A::_0 => false,
            TWM21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM21`"]
pub type TWM21_R = crate::R<bool, TWM21_A>;
impl TWM21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM21_A {
        match self.bits {
            false => TWM21_A::_0,
            true => TWM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM21_A::_1
    }
}
#[doc = "Write proxy for field `TWM21`"]
pub struct TWM21_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM21_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM21_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM22_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM22_A> for bool {
    #[inline(always)]
    fn from(variant: TWM22_A) -> Self {
        match variant {
            TWM22_A::_0 => false,
            TWM22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM22`"]
pub type TWM22_R = crate::R<bool, TWM22_A>;
impl TWM22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM22_A {
        match self.bits {
            false => TWM22_A::_0,
            true => TWM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM22_A::_1
    }
}
#[doc = "Write proxy for field `TWM22`"]
pub struct TWM22_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM22_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM22_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM23_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM23_A> for bool {
    #[inline(always)]
    fn from(variant: TWM23_A) -> Self {
        match variant {
            TWM23_A::_0 => false,
            TWM23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM23`"]
pub type TWM23_R = crate::R<bool, TWM23_A>;
impl TWM23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM23_A {
        match self.bits {
            false => TWM23_A::_0,
            true => TWM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM23_A::_1
    }
}
#[doc = "Write proxy for field `TWM23`"]
pub struct TWM23_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM23_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM23_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM24_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM24_A> for bool {
    #[inline(always)]
    fn from(variant: TWM24_A) -> Self {
        match variant {
            TWM24_A::_0 => false,
            TWM24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM24`"]
pub type TWM24_R = crate::R<bool, TWM24_A>;
impl TWM24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM24_A {
        match self.bits {
            false => TWM24_A::_0,
            true => TWM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM24_A::_1
    }
}
#[doc = "Write proxy for field `TWM24`"]
pub struct TWM24_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM24_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM24_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM25_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM25_A> for bool {
    #[inline(always)]
    fn from(variant: TWM25_A) -> Self {
        match variant {
            TWM25_A::_0 => false,
            TWM25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM25`"]
pub type TWM25_R = crate::R<bool, TWM25_A>;
impl TWM25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM25_A {
        match self.bits {
            false => TWM25_A::_0,
            true => TWM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM25_A::_1
    }
}
#[doc = "Write proxy for field `TWM25`"]
pub struct TWM25_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM25_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM25_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM26_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM26_A> for bool {
    #[inline(always)]
    fn from(variant: TWM26_A) -> Self {
        match variant {
            TWM26_A::_0 => false,
            TWM26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM26`"]
pub type TWM26_R = crate::R<bool, TWM26_A>;
impl TWM26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM26_A {
        match self.bits {
            false => TWM26_A::_0,
            true => TWM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM26_A::_1
    }
}
#[doc = "Write proxy for field `TWM26`"]
pub struct TWM26_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM26_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM26_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM27_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM27_A> for bool {
    #[inline(always)]
    fn from(variant: TWM27_A) -> Self {
        match variant {
            TWM27_A::_0 => false,
            TWM27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM27`"]
pub type TWM27_R = crate::R<bool, TWM27_A>;
impl TWM27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM27_A {
        match self.bits {
            false => TWM27_A::_0,
            true => TWM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM27_A::_1
    }
}
#[doc = "Write proxy for field `TWM27`"]
pub struct TWM27_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM27_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM27_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM28_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM28_A> for bool {
    #[inline(always)]
    fn from(variant: TWM28_A) -> Self {
        match variant {
            TWM28_A::_0 => false,
            TWM28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM28`"]
pub type TWM28_R = crate::R<bool, TWM28_A>;
impl TWM28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM28_A {
        match self.bits {
            false => TWM28_A::_0,
            true => TWM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM28_A::_1
    }
}
#[doc = "Write proxy for field `TWM28`"]
pub struct TWM28_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM28_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM28_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM29_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM29_A> for bool {
    #[inline(always)]
    fn from(variant: TWM29_A) -> Self {
        match variant {
            TWM29_A::_0 => false,
            TWM29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM29`"]
pub type TWM29_R = crate::R<bool, TWM29_A>;
impl TWM29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM29_A {
        match self.bits {
            false => TWM29_A::_0,
            true => TWM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM29_A::_1
    }
}
#[doc = "Write proxy for field `TWM29`"]
pub struct TWM29_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM29_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM29_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM30_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM30_A> for bool {
    #[inline(always)]
    fn from(variant: TWM30_A) -> Self {
        match variant {
            TWM30_A::_0 => false,
            TWM30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM30`"]
pub type TWM30_R = crate::R<bool, TWM30_A>;
impl TWM30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM30_A {
        match self.bits {
            false => TWM30_A::_0,
            true => TWM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM30_A::_1
    }
}
#[doc = "Write proxy for field `TWM30`"]
pub struct TWM30_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM30_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM30_A::_1)
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
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM31_A {
    #[doc = "0: Word N is enabled."]
    _0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl From<TWM31_A> for bool {
    #[inline(always)]
    fn from(variant: TWM31_A) -> Self {
        match variant {
            TWM31_A::_0 => false,
            TWM31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWM31`"]
pub type TWM31_R = crate::R<bool, TWM31_A>;
impl TWM31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWM31_A {
        match self.bits {
            false => TWM31_A::_0,
            true => TWM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWM31_A::_1
    }
}
#[doc = "Write proxy for field `TWM31`"]
pub struct TWM31_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM31_A::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM31_A::_1)
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
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm0(&self) -> TWM0_R {
        TWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm1(&self) -> TWM1_R {
        TWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm2(&self) -> TWM2_R {
        TWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm3(&self) -> TWM3_R {
        TWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm4(&self) -> TWM4_R {
        TWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm5(&self) -> TWM5_R {
        TWM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm6(&self) -> TWM6_R {
        TWM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm7(&self) -> TWM7_R {
        TWM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm8(&self) -> TWM8_R {
        TWM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm9(&self) -> TWM9_R {
        TWM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm10(&self) -> TWM10_R {
        TWM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm11(&self) -> TWM11_R {
        TWM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm12(&self) -> TWM12_R {
        TWM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm13(&self) -> TWM13_R {
        TWM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm14(&self) -> TWM14_R {
        TWM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm15(&self) -> TWM15_R {
        TWM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm16(&self) -> TWM16_R {
        TWM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm17(&self) -> TWM17_R {
        TWM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm18(&self) -> TWM18_R {
        TWM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm19(&self) -> TWM19_R {
        TWM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm20(&self) -> TWM20_R {
        TWM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm21(&self) -> TWM21_R {
        TWM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm22(&self) -> TWM22_R {
        TWM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm23(&self) -> TWM23_R {
        TWM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm24(&self) -> TWM24_R {
        TWM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm25(&self) -> TWM25_R {
        TWM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm26(&self) -> TWM26_R {
        TWM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm27(&self) -> TWM27_R {
        TWM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm28(&self) -> TWM28_R {
        TWM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm29(&self) -> TWM29_R {
        TWM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm30(&self) -> TWM30_R {
        TWM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm31(&self) -> TWM31_R {
        TWM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm0(&mut self) -> TWM0_W {
        TWM0_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm1(&mut self) -> TWM1_W {
        TWM1_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm2(&mut self) -> TWM2_W {
        TWM2_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm3(&mut self) -> TWM3_W {
        TWM3_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm4(&mut self) -> TWM4_W {
        TWM4_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm5(&mut self) -> TWM5_W {
        TWM5_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm6(&mut self) -> TWM6_W {
        TWM6_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm7(&mut self) -> TWM7_W {
        TWM7_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm8(&mut self) -> TWM8_W {
        TWM8_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm9(&mut self) -> TWM9_W {
        TWM9_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm10(&mut self) -> TWM10_W {
        TWM10_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm11(&mut self) -> TWM11_W {
        TWM11_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm12(&mut self) -> TWM12_W {
        TWM12_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm13(&mut self) -> TWM13_W {
        TWM13_W { w: self }
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm14(&mut self) -> TWM14_W {
        TWM14_W { w: self }
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm15(&mut self) -> TWM15_W {
        TWM15_W { w: self }
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm16(&mut self) -> TWM16_W {
        TWM16_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm17(&mut self) -> TWM17_W {
        TWM17_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm18(&mut self) -> TWM18_W {
        TWM18_W { w: self }
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm19(&mut self) -> TWM19_W {
        TWM19_W { w: self }
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm20(&mut self) -> TWM20_W {
        TWM20_W { w: self }
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm21(&mut self) -> TWM21_W {
        TWM21_W { w: self }
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm22(&mut self) -> TWM22_W {
        TWM22_W { w: self }
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm23(&mut self) -> TWM23_W {
        TWM23_W { w: self }
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm24(&mut self) -> TWM24_W {
        TWM24_W { w: self }
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm25(&mut self) -> TWM25_W {
        TWM25_W { w: self }
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm26(&mut self) -> TWM26_W {
        TWM26_W { w: self }
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm27(&mut self) -> TWM27_W {
        TWM27_W { w: self }
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm28(&mut self) -> TWM28_W {
        TWM28_W { w: self }
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm29(&mut self) -> TWM29_W {
        TWM29_W { w: self }
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm30(&mut self) -> TWM30_W {
        TWM30_W { w: self }
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm31(&mut self) -> TWM31_W {
        TWM31_W { w: self }
    }
}
