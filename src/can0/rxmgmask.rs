#[doc = "Reader of register RXMGMASK"]
pub type R = crate::R<u32, super::RXMGMASK>;
#[doc = "Writer for register RXMGMASK"]
pub type W = crate::W<u32, super::RXMGMASK>;
#[doc = "Register RXMGMASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RXMGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG0_A> for bool {
    #[inline(always)]
    fn from(variant: MG0_A) -> Self {
        match variant {
            MG0_A::_0 => false,
            MG0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG0`"]
pub type MG0_R = crate::R<bool, MG0_A>;
impl MG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG0_A {
        match self.bits {
            false => MG0_A::_0,
            true => MG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG0_A::_1
    }
}
#[doc = "Write proxy for field `MG0`"]
pub struct MG0_W<'a> {
    w: &'a mut W,
}
impl<'a> MG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG0_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG1_A> for bool {
    #[inline(always)]
    fn from(variant: MG1_A) -> Self {
        match variant {
            MG1_A::_0 => false,
            MG1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG1`"]
pub type MG1_R = crate::R<bool, MG1_A>;
impl MG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG1_A {
        match self.bits {
            false => MG1_A::_0,
            true => MG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG1_A::_1
    }
}
#[doc = "Write proxy for field `MG1`"]
pub struct MG1_W<'a> {
    w: &'a mut W,
}
impl<'a> MG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG1_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG2_A> for bool {
    #[inline(always)]
    fn from(variant: MG2_A) -> Self {
        match variant {
            MG2_A::_0 => false,
            MG2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG2`"]
pub type MG2_R = crate::R<bool, MG2_A>;
impl MG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG2_A {
        match self.bits {
            false => MG2_A::_0,
            true => MG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG2_A::_1
    }
}
#[doc = "Write proxy for field `MG2`"]
pub struct MG2_W<'a> {
    w: &'a mut W,
}
impl<'a> MG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG2_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG3_A> for bool {
    #[inline(always)]
    fn from(variant: MG3_A) -> Self {
        match variant {
            MG3_A::_0 => false,
            MG3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG3`"]
pub type MG3_R = crate::R<bool, MG3_A>;
impl MG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG3_A {
        match self.bits {
            false => MG3_A::_0,
            true => MG3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG3_A::_1
    }
}
#[doc = "Write proxy for field `MG3`"]
pub struct MG3_W<'a> {
    w: &'a mut W,
}
impl<'a> MG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG3_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG4_A> for bool {
    #[inline(always)]
    fn from(variant: MG4_A) -> Self {
        match variant {
            MG4_A::_0 => false,
            MG4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG4`"]
pub type MG4_R = crate::R<bool, MG4_A>;
impl MG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG4_A {
        match self.bits {
            false => MG4_A::_0,
            true => MG4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG4_A::_1
    }
}
#[doc = "Write proxy for field `MG4`"]
pub struct MG4_W<'a> {
    w: &'a mut W,
}
impl<'a> MG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG4_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG5_A> for bool {
    #[inline(always)]
    fn from(variant: MG5_A) -> Self {
        match variant {
            MG5_A::_0 => false,
            MG5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG5`"]
pub type MG5_R = crate::R<bool, MG5_A>;
impl MG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG5_A {
        match self.bits {
            false => MG5_A::_0,
            true => MG5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG5_A::_1
    }
}
#[doc = "Write proxy for field `MG5`"]
pub struct MG5_W<'a> {
    w: &'a mut W,
}
impl<'a> MG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG5_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG6_A> for bool {
    #[inline(always)]
    fn from(variant: MG6_A) -> Self {
        match variant {
            MG6_A::_0 => false,
            MG6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG6`"]
pub type MG6_R = crate::R<bool, MG6_A>;
impl MG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG6_A {
        match self.bits {
            false => MG6_A::_0,
            true => MG6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG6_A::_1
    }
}
#[doc = "Write proxy for field `MG6`"]
pub struct MG6_W<'a> {
    w: &'a mut W,
}
impl<'a> MG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG6_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG7_A> for bool {
    #[inline(always)]
    fn from(variant: MG7_A) -> Self {
        match variant {
            MG7_A::_0 => false,
            MG7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG7`"]
pub type MG7_R = crate::R<bool, MG7_A>;
impl MG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG7_A {
        match self.bits {
            false => MG7_A::_0,
            true => MG7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG7_A::_1
    }
}
#[doc = "Write proxy for field `MG7`"]
pub struct MG7_W<'a> {
    w: &'a mut W,
}
impl<'a> MG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG7_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG8_A> for bool {
    #[inline(always)]
    fn from(variant: MG8_A) -> Self {
        match variant {
            MG8_A::_0 => false,
            MG8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG8`"]
pub type MG8_R = crate::R<bool, MG8_A>;
impl MG8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG8_A {
        match self.bits {
            false => MG8_A::_0,
            true => MG8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG8_A::_1
    }
}
#[doc = "Write proxy for field `MG8`"]
pub struct MG8_W<'a> {
    w: &'a mut W,
}
impl<'a> MG8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG8_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG9_A> for bool {
    #[inline(always)]
    fn from(variant: MG9_A) -> Self {
        match variant {
            MG9_A::_0 => false,
            MG9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG9`"]
pub type MG9_R = crate::R<bool, MG9_A>;
impl MG9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG9_A {
        match self.bits {
            false => MG9_A::_0,
            true => MG9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG9_A::_1
    }
}
#[doc = "Write proxy for field `MG9`"]
pub struct MG9_W<'a> {
    w: &'a mut W,
}
impl<'a> MG9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG9_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG10_A> for bool {
    #[inline(always)]
    fn from(variant: MG10_A) -> Self {
        match variant {
            MG10_A::_0 => false,
            MG10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG10`"]
pub type MG10_R = crate::R<bool, MG10_A>;
impl MG10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG10_A {
        match self.bits {
            false => MG10_A::_0,
            true => MG10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG10_A::_1
    }
}
#[doc = "Write proxy for field `MG10`"]
pub struct MG10_W<'a> {
    w: &'a mut W,
}
impl<'a> MG10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG10_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG11_A> for bool {
    #[inline(always)]
    fn from(variant: MG11_A) -> Self {
        match variant {
            MG11_A::_0 => false,
            MG11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG11`"]
pub type MG11_R = crate::R<bool, MG11_A>;
impl MG11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG11_A {
        match self.bits {
            false => MG11_A::_0,
            true => MG11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG11_A::_1
    }
}
#[doc = "Write proxy for field `MG11`"]
pub struct MG11_W<'a> {
    w: &'a mut W,
}
impl<'a> MG11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG11_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG12_A> for bool {
    #[inline(always)]
    fn from(variant: MG12_A) -> Self {
        match variant {
            MG12_A::_0 => false,
            MG12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG12`"]
pub type MG12_R = crate::R<bool, MG12_A>;
impl MG12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG12_A {
        match self.bits {
            false => MG12_A::_0,
            true => MG12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG12_A::_1
    }
}
#[doc = "Write proxy for field `MG12`"]
pub struct MG12_W<'a> {
    w: &'a mut W,
}
impl<'a> MG12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG12_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG13_A> for bool {
    #[inline(always)]
    fn from(variant: MG13_A) -> Self {
        match variant {
            MG13_A::_0 => false,
            MG13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG13`"]
pub type MG13_R = crate::R<bool, MG13_A>;
impl MG13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG13_A {
        match self.bits {
            false => MG13_A::_0,
            true => MG13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG13_A::_1
    }
}
#[doc = "Write proxy for field `MG13`"]
pub struct MG13_W<'a> {
    w: &'a mut W,
}
impl<'a> MG13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG13_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG14_A> for bool {
    #[inline(always)]
    fn from(variant: MG14_A) -> Self {
        match variant {
            MG14_A::_0 => false,
            MG14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG14`"]
pub type MG14_R = crate::R<bool, MG14_A>;
impl MG14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG14_A {
        match self.bits {
            false => MG14_A::_0,
            true => MG14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG14_A::_1
    }
}
#[doc = "Write proxy for field `MG14`"]
pub struct MG14_W<'a> {
    w: &'a mut W,
}
impl<'a> MG14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG14_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG15_A> for bool {
    #[inline(always)]
    fn from(variant: MG15_A) -> Self {
        match variant {
            MG15_A::_0 => false,
            MG15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG15`"]
pub type MG15_R = crate::R<bool, MG15_A>;
impl MG15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG15_A {
        match self.bits {
            false => MG15_A::_0,
            true => MG15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG15_A::_1
    }
}
#[doc = "Write proxy for field `MG15`"]
pub struct MG15_W<'a> {
    w: &'a mut W,
}
impl<'a> MG15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG15_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG16_A> for bool {
    #[inline(always)]
    fn from(variant: MG16_A) -> Self {
        match variant {
            MG16_A::_0 => false,
            MG16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG16`"]
pub type MG16_R = crate::R<bool, MG16_A>;
impl MG16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG16_A {
        match self.bits {
            false => MG16_A::_0,
            true => MG16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG16_A::_1
    }
}
#[doc = "Write proxy for field `MG16`"]
pub struct MG16_W<'a> {
    w: &'a mut W,
}
impl<'a> MG16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG16_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG17_A> for bool {
    #[inline(always)]
    fn from(variant: MG17_A) -> Self {
        match variant {
            MG17_A::_0 => false,
            MG17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG17`"]
pub type MG17_R = crate::R<bool, MG17_A>;
impl MG17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG17_A {
        match self.bits {
            false => MG17_A::_0,
            true => MG17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG17_A::_1
    }
}
#[doc = "Write proxy for field `MG17`"]
pub struct MG17_W<'a> {
    w: &'a mut W,
}
impl<'a> MG17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG17_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG18_A> for bool {
    #[inline(always)]
    fn from(variant: MG18_A) -> Self {
        match variant {
            MG18_A::_0 => false,
            MG18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG18`"]
pub type MG18_R = crate::R<bool, MG18_A>;
impl MG18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG18_A {
        match self.bits {
            false => MG18_A::_0,
            true => MG18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG18_A::_1
    }
}
#[doc = "Write proxy for field `MG18`"]
pub struct MG18_W<'a> {
    w: &'a mut W,
}
impl<'a> MG18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG18_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG19_A> for bool {
    #[inline(always)]
    fn from(variant: MG19_A) -> Self {
        match variant {
            MG19_A::_0 => false,
            MG19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG19`"]
pub type MG19_R = crate::R<bool, MG19_A>;
impl MG19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG19_A {
        match self.bits {
            false => MG19_A::_0,
            true => MG19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG19_A::_1
    }
}
#[doc = "Write proxy for field `MG19`"]
pub struct MG19_W<'a> {
    w: &'a mut W,
}
impl<'a> MG19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG19_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG20_A> for bool {
    #[inline(always)]
    fn from(variant: MG20_A) -> Self {
        match variant {
            MG20_A::_0 => false,
            MG20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG20`"]
pub type MG20_R = crate::R<bool, MG20_A>;
impl MG20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG20_A {
        match self.bits {
            false => MG20_A::_0,
            true => MG20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG20_A::_1
    }
}
#[doc = "Write proxy for field `MG20`"]
pub struct MG20_W<'a> {
    w: &'a mut W,
}
impl<'a> MG20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG20_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG21_A> for bool {
    #[inline(always)]
    fn from(variant: MG21_A) -> Self {
        match variant {
            MG21_A::_0 => false,
            MG21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG21`"]
pub type MG21_R = crate::R<bool, MG21_A>;
impl MG21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG21_A {
        match self.bits {
            false => MG21_A::_0,
            true => MG21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG21_A::_1
    }
}
#[doc = "Write proxy for field `MG21`"]
pub struct MG21_W<'a> {
    w: &'a mut W,
}
impl<'a> MG21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG21_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG22_A> for bool {
    #[inline(always)]
    fn from(variant: MG22_A) -> Self {
        match variant {
            MG22_A::_0 => false,
            MG22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG22`"]
pub type MG22_R = crate::R<bool, MG22_A>;
impl MG22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG22_A {
        match self.bits {
            false => MG22_A::_0,
            true => MG22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG22_A::_1
    }
}
#[doc = "Write proxy for field `MG22`"]
pub struct MG22_W<'a> {
    w: &'a mut W,
}
impl<'a> MG22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG22_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG23_A> for bool {
    #[inline(always)]
    fn from(variant: MG23_A) -> Self {
        match variant {
            MG23_A::_0 => false,
            MG23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG23`"]
pub type MG23_R = crate::R<bool, MG23_A>;
impl MG23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG23_A {
        match self.bits {
            false => MG23_A::_0,
            true => MG23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG23_A::_1
    }
}
#[doc = "Write proxy for field `MG23`"]
pub struct MG23_W<'a> {
    w: &'a mut W,
}
impl<'a> MG23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG23_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG24_A> for bool {
    #[inline(always)]
    fn from(variant: MG24_A) -> Self {
        match variant {
            MG24_A::_0 => false,
            MG24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG24`"]
pub type MG24_R = crate::R<bool, MG24_A>;
impl MG24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG24_A {
        match self.bits {
            false => MG24_A::_0,
            true => MG24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG24_A::_1
    }
}
#[doc = "Write proxy for field `MG24`"]
pub struct MG24_W<'a> {
    w: &'a mut W,
}
impl<'a> MG24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG24_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG25_A> for bool {
    #[inline(always)]
    fn from(variant: MG25_A) -> Self {
        match variant {
            MG25_A::_0 => false,
            MG25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG25`"]
pub type MG25_R = crate::R<bool, MG25_A>;
impl MG25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG25_A {
        match self.bits {
            false => MG25_A::_0,
            true => MG25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG25_A::_1
    }
}
#[doc = "Write proxy for field `MG25`"]
pub struct MG25_W<'a> {
    w: &'a mut W,
}
impl<'a> MG25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG25_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG26_A> for bool {
    #[inline(always)]
    fn from(variant: MG26_A) -> Self {
        match variant {
            MG26_A::_0 => false,
            MG26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG26`"]
pub type MG26_R = crate::R<bool, MG26_A>;
impl MG26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG26_A {
        match self.bits {
            false => MG26_A::_0,
            true => MG26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG26_A::_1
    }
}
#[doc = "Write proxy for field `MG26`"]
pub struct MG26_W<'a> {
    w: &'a mut W,
}
impl<'a> MG26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG26_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG27_A> for bool {
    #[inline(always)]
    fn from(variant: MG27_A) -> Self {
        match variant {
            MG27_A::_0 => false,
            MG27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG27`"]
pub type MG27_R = crate::R<bool, MG27_A>;
impl MG27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG27_A {
        match self.bits {
            false => MG27_A::_0,
            true => MG27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG27_A::_1
    }
}
#[doc = "Write proxy for field `MG27`"]
pub struct MG27_W<'a> {
    w: &'a mut W,
}
impl<'a> MG27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG27_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG28_A> for bool {
    #[inline(always)]
    fn from(variant: MG28_A) -> Self {
        match variant {
            MG28_A::_0 => false,
            MG28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG28`"]
pub type MG28_R = crate::R<bool, MG28_A>;
impl MG28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG28_A {
        match self.bits {
            false => MG28_A::_0,
            true => MG28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG28_A::_1
    }
}
#[doc = "Write proxy for field `MG28`"]
pub struct MG28_W<'a> {
    w: &'a mut W,
}
impl<'a> MG28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG28_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG29_A> for bool {
    #[inline(always)]
    fn from(variant: MG29_A) -> Self {
        match variant {
            MG29_A::_0 => false,
            MG29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG29`"]
pub type MG29_R = crate::R<bool, MG29_A>;
impl MG29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG29_A {
        match self.bits {
            false => MG29_A::_0,
            true => MG29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG29_A::_1
    }
}
#[doc = "Write proxy for field `MG29`"]
pub struct MG29_W<'a> {
    w: &'a mut W,
}
impl<'a> MG29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG29_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG30_A> for bool {
    #[inline(always)]
    fn from(variant: MG30_A) -> Self {
        match variant {
            MG30_A::_0 => false,
            MG30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG30`"]
pub type MG30_R = crate::R<bool, MG30_A>;
impl MG30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG30_A {
        match self.bits {
            false => MG30_A::_0,
            true => MG30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG30_A::_1
    }
}
#[doc = "Write proxy for field `MG30`"]
pub struct MG30_W<'a> {
    w: &'a mut W,
}
impl<'a> MG30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG30_A::_1)
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
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MG31_A> for bool {
    #[inline(always)]
    fn from(variant: MG31_A) -> Self {
        match variant {
            MG31_A::_0 => false,
            MG31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MG31`"]
pub type MG31_R = crate::R<bool, MG31_A>;
impl MG31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG31_A {
        match self.bits {
            false => MG31_A::_0,
            true => MG31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG31_A::_1
    }
}
#[doc = "Write proxy for field `MG31`"]
pub struct MG31_W<'a> {
    w: &'a mut W,
}
impl<'a> MG31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG31_A::_1)
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
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg0(&self) -> MG0_R {
        MG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg1(&self) -> MG1_R {
        MG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg2(&self) -> MG2_R {
        MG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg3(&self) -> MG3_R {
        MG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg4(&self) -> MG4_R {
        MG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg5(&self) -> MG5_R {
        MG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg6(&self) -> MG6_R {
        MG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg7(&self) -> MG7_R {
        MG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg8(&self) -> MG8_R {
        MG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg9(&self) -> MG9_R {
        MG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg10(&self) -> MG10_R {
        MG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg11(&self) -> MG11_R {
        MG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg12(&self) -> MG12_R {
        MG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg13(&self) -> MG13_R {
        MG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg14(&self) -> MG14_R {
        MG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg15(&self) -> MG15_R {
        MG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg16(&self) -> MG16_R {
        MG16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg17(&self) -> MG17_R {
        MG17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg18(&self) -> MG18_R {
        MG18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg19(&self) -> MG19_R {
        MG19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg20(&self) -> MG20_R {
        MG20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg21(&self) -> MG21_R {
        MG21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg22(&self) -> MG22_R {
        MG22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg23(&self) -> MG23_R {
        MG23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg24(&self) -> MG24_R {
        MG24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg25(&self) -> MG25_R {
        MG25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg26(&self) -> MG26_R {
        MG26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg27(&self) -> MG27_R {
        MG27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg28(&self) -> MG28_R {
        MG28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg29(&self) -> MG29_R {
        MG29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg30(&self) -> MG30_R {
        MG30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg31(&self) -> MG31_R {
        MG31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg0(&mut self) -> MG0_W {
        MG0_W { w: self }
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg1(&mut self) -> MG1_W {
        MG1_W { w: self }
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg2(&mut self) -> MG2_W {
        MG2_W { w: self }
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg3(&mut self) -> MG3_W {
        MG3_W { w: self }
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg4(&mut self) -> MG4_W {
        MG4_W { w: self }
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg5(&mut self) -> MG5_W {
        MG5_W { w: self }
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg6(&mut self) -> MG6_W {
        MG6_W { w: self }
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg7(&mut self) -> MG7_W {
        MG7_W { w: self }
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg8(&mut self) -> MG8_W {
        MG8_W { w: self }
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg9(&mut self) -> MG9_W {
        MG9_W { w: self }
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg10(&mut self) -> MG10_W {
        MG10_W { w: self }
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg11(&mut self) -> MG11_W {
        MG11_W { w: self }
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg12(&mut self) -> MG12_W {
        MG12_W { w: self }
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg13(&mut self) -> MG13_W {
        MG13_W { w: self }
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg14(&mut self) -> MG14_W {
        MG14_W { w: self }
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg15(&mut self) -> MG15_W {
        MG15_W { w: self }
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg16(&mut self) -> MG16_W {
        MG16_W { w: self }
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg17(&mut self) -> MG17_W {
        MG17_W { w: self }
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg18(&mut self) -> MG18_W {
        MG18_W { w: self }
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg19(&mut self) -> MG19_W {
        MG19_W { w: self }
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg20(&mut self) -> MG20_W {
        MG20_W { w: self }
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg21(&mut self) -> MG21_W {
        MG21_W { w: self }
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg22(&mut self) -> MG22_W {
        MG22_W { w: self }
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg23(&mut self) -> MG23_W {
        MG23_W { w: self }
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg24(&mut self) -> MG24_W {
        MG24_W { w: self }
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg25(&mut self) -> MG25_W {
        MG25_W { w: self }
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg26(&mut self) -> MG26_W {
        MG26_W { w: self }
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg27(&mut self) -> MG27_W {
        MG27_W { w: self }
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg28(&mut self) -> MG28_W {
        MG28_W { w: self }
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg29(&mut self) -> MG29_W {
        MG29_W { w: self }
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg30(&mut self) -> MG30_W {
        MG30_W { w: self }
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg31(&mut self) -> MG31_W {
        MG31_W { w: self }
    }
}
