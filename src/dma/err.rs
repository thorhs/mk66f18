#[doc = "Reader of register ERR"]
pub type R = crate::R<u32, super::ERR>;
#[doc = "Writer for register ERR"]
pub type W = crate::W<u32, super::ERR>;
#[doc = "Register ERR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Error In Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR0_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        match variant {
            ERR0_A::_0 => false,
            ERR0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR0`"]
pub type ERR0_R = crate::R<bool, ERR0_A>;
impl ERR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::_0,
            true => ERR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR0_A::_1
    }
}
#[doc = "Write proxy for field `ERR0`"]
pub struct ERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0_A::_1)
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
#[doc = "Error In Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR1_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        match variant {
            ERR1_A::_0 => false,
            ERR1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR1`"]
pub type ERR1_R = crate::R<bool, ERR1_A>;
impl ERR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::_0,
            true => ERR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR1_A::_1
    }
}
#[doc = "Write proxy for field `ERR1`"]
pub struct ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1_A::_1)
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
#[doc = "Error In Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR2_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        match variant {
            ERR2_A::_0 => false,
            ERR2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR2`"]
pub type ERR2_R = crate::R<bool, ERR2_A>;
impl ERR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::_0,
            true => ERR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR2_A::_1
    }
}
#[doc = "Write proxy for field `ERR2`"]
pub struct ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2_A::_1)
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
#[doc = "Error In Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR3_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        match variant {
            ERR3_A::_0 => false,
            ERR3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR3`"]
pub type ERR3_R = crate::R<bool, ERR3_A>;
impl ERR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::_0,
            true => ERR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR3_A::_1
    }
}
#[doc = "Write proxy for field `ERR3`"]
pub struct ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3_A::_1)
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
#[doc = "Error In Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR4_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        match variant {
            ERR4_A::_0 => false,
            ERR4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR4`"]
pub type ERR4_R = crate::R<bool, ERR4_A>;
impl ERR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::_0,
            true => ERR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR4_A::_1
    }
}
#[doc = "Write proxy for field `ERR4`"]
pub struct ERR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4_A::_1)
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
#[doc = "Error In Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR5_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        match variant {
            ERR5_A::_0 => false,
            ERR5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR5`"]
pub type ERR5_R = crate::R<bool, ERR5_A>;
impl ERR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::_0,
            true => ERR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR5_A::_1
    }
}
#[doc = "Write proxy for field `ERR5`"]
pub struct ERR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5_A::_1)
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
#[doc = "Error In Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR6_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        match variant {
            ERR6_A::_0 => false,
            ERR6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR6`"]
pub type ERR6_R = crate::R<bool, ERR6_A>;
impl ERR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::_0,
            true => ERR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR6_A::_1
    }
}
#[doc = "Write proxy for field `ERR6`"]
pub struct ERR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6_A::_1)
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
#[doc = "Error In Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR7_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        match variant {
            ERR7_A::_0 => false,
            ERR7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR7`"]
pub type ERR7_R = crate::R<bool, ERR7_A>;
impl ERR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::_0,
            true => ERR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR7_A::_1
    }
}
#[doc = "Write proxy for field `ERR7`"]
pub struct ERR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7_A::_1)
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
#[doc = "Error In Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR8_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR8_A> for bool {
    #[inline(always)]
    fn from(variant: ERR8_A) -> Self {
        match variant {
            ERR8_A::_0 => false,
            ERR8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR8`"]
pub type ERR8_R = crate::R<bool, ERR8_A>;
impl ERR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR8_A {
        match self.bits {
            false => ERR8_A::_0,
            true => ERR8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR8_A::_1
    }
}
#[doc = "Write proxy for field `ERR8`"]
pub struct ERR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR8_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR8_A::_1)
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
#[doc = "Error In Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR9_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR9_A> for bool {
    #[inline(always)]
    fn from(variant: ERR9_A) -> Self {
        match variant {
            ERR9_A::_0 => false,
            ERR9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR9`"]
pub type ERR9_R = crate::R<bool, ERR9_A>;
impl ERR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR9_A {
        match self.bits {
            false => ERR9_A::_0,
            true => ERR9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR9_A::_1
    }
}
#[doc = "Write proxy for field `ERR9`"]
pub struct ERR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR9_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR9_A::_1)
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
#[doc = "Error In Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR10_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR10_A> for bool {
    #[inline(always)]
    fn from(variant: ERR10_A) -> Self {
        match variant {
            ERR10_A::_0 => false,
            ERR10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR10`"]
pub type ERR10_R = crate::R<bool, ERR10_A>;
impl ERR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR10_A {
        match self.bits {
            false => ERR10_A::_0,
            true => ERR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR10_A::_1
    }
}
#[doc = "Write proxy for field `ERR10`"]
pub struct ERR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR10_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR10_A::_1)
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
#[doc = "Error In Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR11_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR11_A> for bool {
    #[inline(always)]
    fn from(variant: ERR11_A) -> Self {
        match variant {
            ERR11_A::_0 => false,
            ERR11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR11`"]
pub type ERR11_R = crate::R<bool, ERR11_A>;
impl ERR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR11_A {
        match self.bits {
            false => ERR11_A::_0,
            true => ERR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR11_A::_1
    }
}
#[doc = "Write proxy for field `ERR11`"]
pub struct ERR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR11_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR11_A::_1)
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
#[doc = "Error In Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR12_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR12_A> for bool {
    #[inline(always)]
    fn from(variant: ERR12_A) -> Self {
        match variant {
            ERR12_A::_0 => false,
            ERR12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR12`"]
pub type ERR12_R = crate::R<bool, ERR12_A>;
impl ERR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR12_A {
        match self.bits {
            false => ERR12_A::_0,
            true => ERR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR12_A::_1
    }
}
#[doc = "Write proxy for field `ERR12`"]
pub struct ERR12_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR12_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR12_A::_1)
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
#[doc = "Error In Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR13_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR13_A> for bool {
    #[inline(always)]
    fn from(variant: ERR13_A) -> Self {
        match variant {
            ERR13_A::_0 => false,
            ERR13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR13`"]
pub type ERR13_R = crate::R<bool, ERR13_A>;
impl ERR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR13_A {
        match self.bits {
            false => ERR13_A::_0,
            true => ERR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR13_A::_1
    }
}
#[doc = "Write proxy for field `ERR13`"]
pub struct ERR13_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR13_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR13_A::_1)
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
#[doc = "Error In Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR14_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR14_A> for bool {
    #[inline(always)]
    fn from(variant: ERR14_A) -> Self {
        match variant {
            ERR14_A::_0 => false,
            ERR14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR14`"]
pub type ERR14_R = crate::R<bool, ERR14_A>;
impl ERR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR14_A {
        match self.bits {
            false => ERR14_A::_0,
            true => ERR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR14_A::_1
    }
}
#[doc = "Write proxy for field `ERR14`"]
pub struct ERR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR14_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR14_A::_1)
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
#[doc = "Error In Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR15_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR15_A> for bool {
    #[inline(always)]
    fn from(variant: ERR15_A) -> Self {
        match variant {
            ERR15_A::_0 => false,
            ERR15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR15`"]
pub type ERR15_R = crate::R<bool, ERR15_A>;
impl ERR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR15_A {
        match self.bits {
            false => ERR15_A::_0,
            true => ERR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR15_A::_1
    }
}
#[doc = "Write proxy for field `ERR15`"]
pub struct ERR15_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR15_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR15_A::_1)
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
#[doc = "Error In Channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR16_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR16_A> for bool {
    #[inline(always)]
    fn from(variant: ERR16_A) -> Self {
        match variant {
            ERR16_A::_0 => false,
            ERR16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR16`"]
pub type ERR16_R = crate::R<bool, ERR16_A>;
impl ERR16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR16_A {
        match self.bits {
            false => ERR16_A::_0,
            true => ERR16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR16_A::_1
    }
}
#[doc = "Write proxy for field `ERR16`"]
pub struct ERR16_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR16_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR16_A::_1)
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
#[doc = "Error In Channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR17_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR17_A> for bool {
    #[inline(always)]
    fn from(variant: ERR17_A) -> Self {
        match variant {
            ERR17_A::_0 => false,
            ERR17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR17`"]
pub type ERR17_R = crate::R<bool, ERR17_A>;
impl ERR17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR17_A {
        match self.bits {
            false => ERR17_A::_0,
            true => ERR17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR17_A::_1
    }
}
#[doc = "Write proxy for field `ERR17`"]
pub struct ERR17_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR17_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR17_A::_1)
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
#[doc = "Error In Channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR18_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR18_A> for bool {
    #[inline(always)]
    fn from(variant: ERR18_A) -> Self {
        match variant {
            ERR18_A::_0 => false,
            ERR18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR18`"]
pub type ERR18_R = crate::R<bool, ERR18_A>;
impl ERR18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR18_A {
        match self.bits {
            false => ERR18_A::_0,
            true => ERR18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR18_A::_1
    }
}
#[doc = "Write proxy for field `ERR18`"]
pub struct ERR18_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR18_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR18_A::_1)
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
#[doc = "Error In Channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR19_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR19_A> for bool {
    #[inline(always)]
    fn from(variant: ERR19_A) -> Self {
        match variant {
            ERR19_A::_0 => false,
            ERR19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR19`"]
pub type ERR19_R = crate::R<bool, ERR19_A>;
impl ERR19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR19_A {
        match self.bits {
            false => ERR19_A::_0,
            true => ERR19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR19_A::_1
    }
}
#[doc = "Write proxy for field `ERR19`"]
pub struct ERR19_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR19_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR19_A::_1)
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
#[doc = "Error In Channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR20_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR20_A> for bool {
    #[inline(always)]
    fn from(variant: ERR20_A) -> Self {
        match variant {
            ERR20_A::_0 => false,
            ERR20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR20`"]
pub type ERR20_R = crate::R<bool, ERR20_A>;
impl ERR20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR20_A {
        match self.bits {
            false => ERR20_A::_0,
            true => ERR20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR20_A::_1
    }
}
#[doc = "Write proxy for field `ERR20`"]
pub struct ERR20_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR20_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR20_A::_1)
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
#[doc = "Error In Channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR21_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR21_A> for bool {
    #[inline(always)]
    fn from(variant: ERR21_A) -> Self {
        match variant {
            ERR21_A::_0 => false,
            ERR21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR21`"]
pub type ERR21_R = crate::R<bool, ERR21_A>;
impl ERR21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR21_A {
        match self.bits {
            false => ERR21_A::_0,
            true => ERR21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR21_A::_1
    }
}
#[doc = "Write proxy for field `ERR21`"]
pub struct ERR21_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR21_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR21_A::_1)
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
#[doc = "Error In Channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR22_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR22_A> for bool {
    #[inline(always)]
    fn from(variant: ERR22_A) -> Self {
        match variant {
            ERR22_A::_0 => false,
            ERR22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR22`"]
pub type ERR22_R = crate::R<bool, ERR22_A>;
impl ERR22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR22_A {
        match self.bits {
            false => ERR22_A::_0,
            true => ERR22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR22_A::_1
    }
}
#[doc = "Write proxy for field `ERR22`"]
pub struct ERR22_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR22_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR22_A::_1)
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
#[doc = "Error In Channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR23_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR23_A> for bool {
    #[inline(always)]
    fn from(variant: ERR23_A) -> Self {
        match variant {
            ERR23_A::_0 => false,
            ERR23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR23`"]
pub type ERR23_R = crate::R<bool, ERR23_A>;
impl ERR23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR23_A {
        match self.bits {
            false => ERR23_A::_0,
            true => ERR23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR23_A::_1
    }
}
#[doc = "Write proxy for field `ERR23`"]
pub struct ERR23_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR23_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR23_A::_1)
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
#[doc = "Error In Channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR24_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR24_A> for bool {
    #[inline(always)]
    fn from(variant: ERR24_A) -> Self {
        match variant {
            ERR24_A::_0 => false,
            ERR24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR24`"]
pub type ERR24_R = crate::R<bool, ERR24_A>;
impl ERR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR24_A {
        match self.bits {
            false => ERR24_A::_0,
            true => ERR24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR24_A::_1
    }
}
#[doc = "Write proxy for field `ERR24`"]
pub struct ERR24_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR24_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR24_A::_1)
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
#[doc = "Error In Channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR25_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR25_A> for bool {
    #[inline(always)]
    fn from(variant: ERR25_A) -> Self {
        match variant {
            ERR25_A::_0 => false,
            ERR25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR25`"]
pub type ERR25_R = crate::R<bool, ERR25_A>;
impl ERR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR25_A {
        match self.bits {
            false => ERR25_A::_0,
            true => ERR25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR25_A::_1
    }
}
#[doc = "Write proxy for field `ERR25`"]
pub struct ERR25_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR25_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR25_A::_1)
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
#[doc = "Error In Channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR26_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR26_A> for bool {
    #[inline(always)]
    fn from(variant: ERR26_A) -> Self {
        match variant {
            ERR26_A::_0 => false,
            ERR26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR26`"]
pub type ERR26_R = crate::R<bool, ERR26_A>;
impl ERR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR26_A {
        match self.bits {
            false => ERR26_A::_0,
            true => ERR26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR26_A::_1
    }
}
#[doc = "Write proxy for field `ERR26`"]
pub struct ERR26_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR26_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR26_A::_1)
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
#[doc = "Error In Channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR27_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR27_A> for bool {
    #[inline(always)]
    fn from(variant: ERR27_A) -> Self {
        match variant {
            ERR27_A::_0 => false,
            ERR27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR27`"]
pub type ERR27_R = crate::R<bool, ERR27_A>;
impl ERR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR27_A {
        match self.bits {
            false => ERR27_A::_0,
            true => ERR27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR27_A::_1
    }
}
#[doc = "Write proxy for field `ERR27`"]
pub struct ERR27_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR27_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR27_A::_1)
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
#[doc = "Error In Channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR28_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR28_A> for bool {
    #[inline(always)]
    fn from(variant: ERR28_A) -> Self {
        match variant {
            ERR28_A::_0 => false,
            ERR28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR28`"]
pub type ERR28_R = crate::R<bool, ERR28_A>;
impl ERR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR28_A {
        match self.bits {
            false => ERR28_A::_0,
            true => ERR28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR28_A::_1
    }
}
#[doc = "Write proxy for field `ERR28`"]
pub struct ERR28_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR28_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR28_A::_1)
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
#[doc = "Error In Channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR29_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR29_A> for bool {
    #[inline(always)]
    fn from(variant: ERR29_A) -> Self {
        match variant {
            ERR29_A::_0 => false,
            ERR29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR29`"]
pub type ERR29_R = crate::R<bool, ERR29_A>;
impl ERR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR29_A {
        match self.bits {
            false => ERR29_A::_0,
            true => ERR29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR29_A::_1
    }
}
#[doc = "Write proxy for field `ERR29`"]
pub struct ERR29_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR29_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR29_A::_1)
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
#[doc = "Error In Channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR30_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR30_A> for bool {
    #[inline(always)]
    fn from(variant: ERR30_A) -> Self {
        match variant {
            ERR30_A::_0 => false,
            ERR30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR30`"]
pub type ERR30_R = crate::R<bool, ERR30_A>;
impl ERR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR30_A {
        match self.bits {
            false => ERR30_A::_0,
            true => ERR30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR30_A::_1
    }
}
#[doc = "Write proxy for field `ERR30`"]
pub struct ERR30_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR30_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR30_A::_1)
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
#[doc = "Error In Channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR31_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0,
    #[doc = "1: An error in this channel has occurred"]
    _1,
}
impl From<ERR31_A> for bool {
    #[inline(always)]
    fn from(variant: ERR31_A) -> Self {
        match variant {
            ERR31_A::_0 => false,
            ERR31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR31`"]
pub type ERR31_R = crate::R<bool, ERR31_A>;
impl ERR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR31_A {
        match self.bits {
            false => ERR31_A::_0,
            true => ERR31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR31_A::_1
    }
}
#[doc = "Write proxy for field `ERR31`"]
pub struct ERR31_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR31_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR31_A::_1)
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    pub fn err8(&self) -> ERR8_R {
        ERR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    pub fn err9(&self) -> ERR9_R {
        ERR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    pub fn err10(&self) -> ERR10_R {
        ERR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    pub fn err11(&self) -> ERR11_R {
        ERR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    pub fn err12(&self) -> ERR12_R {
        ERR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    pub fn err13(&self) -> ERR13_R {
        ERR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    pub fn err14(&self) -> ERR14_R {
        ERR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    pub fn err15(&self) -> ERR15_R {
        ERR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Error In Channel 16"]
    #[inline(always)]
    pub fn err16(&self) -> ERR16_R {
        ERR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Error In Channel 17"]
    #[inline(always)]
    pub fn err17(&self) -> ERR17_R {
        ERR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Error In Channel 18"]
    #[inline(always)]
    pub fn err18(&self) -> ERR18_R {
        ERR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Error In Channel 19"]
    #[inline(always)]
    pub fn err19(&self) -> ERR19_R {
        ERR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Error In Channel 20"]
    #[inline(always)]
    pub fn err20(&self) -> ERR20_R {
        ERR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Error In Channel 21"]
    #[inline(always)]
    pub fn err21(&self) -> ERR21_R {
        ERR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error In Channel 22"]
    #[inline(always)]
    pub fn err22(&self) -> ERR22_R {
        ERR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error In Channel 23"]
    #[inline(always)]
    pub fn err23(&self) -> ERR23_R {
        ERR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Error In Channel 24"]
    #[inline(always)]
    pub fn err24(&self) -> ERR24_R {
        ERR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Error In Channel 25"]
    #[inline(always)]
    pub fn err25(&self) -> ERR25_R {
        ERR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Error In Channel 26"]
    #[inline(always)]
    pub fn err26(&self) -> ERR26_R {
        ERR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Error In Channel 27"]
    #[inline(always)]
    pub fn err27(&self) -> ERR27_R {
        ERR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Error In Channel 28"]
    #[inline(always)]
    pub fn err28(&self) -> ERR28_R {
        ERR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Error In Channel 29"]
    #[inline(always)]
    pub fn err29(&self) -> ERR29_R {
        ERR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Error In Channel 30"]
    #[inline(always)]
    pub fn err30(&self) -> ERR30_R {
        ERR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Error In Channel 31"]
    #[inline(always)]
    pub fn err31(&self) -> ERR31_R {
        ERR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    pub fn err0(&mut self) -> ERR0_W {
        ERR0_W { w: self }
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    pub fn err1(&mut self) -> ERR1_W {
        ERR1_W { w: self }
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    pub fn err2(&mut self) -> ERR2_W {
        ERR2_W { w: self }
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    pub fn err3(&mut self) -> ERR3_W {
        ERR3_W { w: self }
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    pub fn err4(&mut self) -> ERR4_W {
        ERR4_W { w: self }
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    pub fn err5(&mut self) -> ERR5_W {
        ERR5_W { w: self }
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    pub fn err6(&mut self) -> ERR6_W {
        ERR6_W { w: self }
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    pub fn err7(&mut self) -> ERR7_W {
        ERR7_W { w: self }
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    pub fn err8(&mut self) -> ERR8_W {
        ERR8_W { w: self }
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    pub fn err9(&mut self) -> ERR9_W {
        ERR9_W { w: self }
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    pub fn err10(&mut self) -> ERR10_W {
        ERR10_W { w: self }
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    pub fn err11(&mut self) -> ERR11_W {
        ERR11_W { w: self }
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    pub fn err12(&mut self) -> ERR12_W {
        ERR12_W { w: self }
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    pub fn err13(&mut self) -> ERR13_W {
        ERR13_W { w: self }
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    pub fn err14(&mut self) -> ERR14_W {
        ERR14_W { w: self }
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    pub fn err15(&mut self) -> ERR15_W {
        ERR15_W { w: self }
    }
    #[doc = "Bit 16 - Error In Channel 16"]
    #[inline(always)]
    pub fn err16(&mut self) -> ERR16_W {
        ERR16_W { w: self }
    }
    #[doc = "Bit 17 - Error In Channel 17"]
    #[inline(always)]
    pub fn err17(&mut self) -> ERR17_W {
        ERR17_W { w: self }
    }
    #[doc = "Bit 18 - Error In Channel 18"]
    #[inline(always)]
    pub fn err18(&mut self) -> ERR18_W {
        ERR18_W { w: self }
    }
    #[doc = "Bit 19 - Error In Channel 19"]
    #[inline(always)]
    pub fn err19(&mut self) -> ERR19_W {
        ERR19_W { w: self }
    }
    #[doc = "Bit 20 - Error In Channel 20"]
    #[inline(always)]
    pub fn err20(&mut self) -> ERR20_W {
        ERR20_W { w: self }
    }
    #[doc = "Bit 21 - Error In Channel 21"]
    #[inline(always)]
    pub fn err21(&mut self) -> ERR21_W {
        ERR21_W { w: self }
    }
    #[doc = "Bit 22 - Error In Channel 22"]
    #[inline(always)]
    pub fn err22(&mut self) -> ERR22_W {
        ERR22_W { w: self }
    }
    #[doc = "Bit 23 - Error In Channel 23"]
    #[inline(always)]
    pub fn err23(&mut self) -> ERR23_W {
        ERR23_W { w: self }
    }
    #[doc = "Bit 24 - Error In Channel 24"]
    #[inline(always)]
    pub fn err24(&mut self) -> ERR24_W {
        ERR24_W { w: self }
    }
    #[doc = "Bit 25 - Error In Channel 25"]
    #[inline(always)]
    pub fn err25(&mut self) -> ERR25_W {
        ERR25_W { w: self }
    }
    #[doc = "Bit 26 - Error In Channel 26"]
    #[inline(always)]
    pub fn err26(&mut self) -> ERR26_W {
        ERR26_W { w: self }
    }
    #[doc = "Bit 27 - Error In Channel 27"]
    #[inline(always)]
    pub fn err27(&mut self) -> ERR27_W {
        ERR27_W { w: self }
    }
    #[doc = "Bit 28 - Error In Channel 28"]
    #[inline(always)]
    pub fn err28(&mut self) -> ERR28_W {
        ERR28_W { w: self }
    }
    #[doc = "Bit 29 - Error In Channel 29"]
    #[inline(always)]
    pub fn err29(&mut self) -> ERR29_W {
        ERR29_W { w: self }
    }
    #[doc = "Bit 30 - Error In Channel 30"]
    #[inline(always)]
    pub fn err30(&mut self) -> ERR30_W {
        ERR30_W { w: self }
    }
    #[doc = "Bit 31 - Error In Channel 31"]
    #[inline(always)]
    pub fn err31(&mut self) -> ERR31_W {
        ERR31_W { w: self }
    }
}
