#[doc = "Reader of register RXIMR%s"]
pub type R = crate::R<u32, super::RXIMR>;
#[doc = "Writer for register RXIMR%s"]
pub type W = crate::W<u32, super::RXIMR>;
#[doc = "Register RXIMR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI0_A> for bool {
    #[inline(always)]
    fn from(variant: MI0_A) -> Self {
        match variant {
            MI0_A::_0 => false,
            MI0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI0`"]
pub type MI0_R = crate::R<bool, MI0_A>;
impl MI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI0_A {
        match self.bits {
            false => MI0_A::_0,
            true => MI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI0_A::_1
    }
}
#[doc = "Write proxy for field `MI0`"]
pub struct MI0_W<'a> {
    w: &'a mut W,
}
impl<'a> MI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI0_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI1_A> for bool {
    #[inline(always)]
    fn from(variant: MI1_A) -> Self {
        match variant {
            MI1_A::_0 => false,
            MI1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI1`"]
pub type MI1_R = crate::R<bool, MI1_A>;
impl MI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI1_A {
        match self.bits {
            false => MI1_A::_0,
            true => MI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI1_A::_1
    }
}
#[doc = "Write proxy for field `MI1`"]
pub struct MI1_W<'a> {
    w: &'a mut W,
}
impl<'a> MI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI1_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI2_A> for bool {
    #[inline(always)]
    fn from(variant: MI2_A) -> Self {
        match variant {
            MI2_A::_0 => false,
            MI2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI2`"]
pub type MI2_R = crate::R<bool, MI2_A>;
impl MI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI2_A {
        match self.bits {
            false => MI2_A::_0,
            true => MI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI2_A::_1
    }
}
#[doc = "Write proxy for field `MI2`"]
pub struct MI2_W<'a> {
    w: &'a mut W,
}
impl<'a> MI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI2_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI3_A> for bool {
    #[inline(always)]
    fn from(variant: MI3_A) -> Self {
        match variant {
            MI3_A::_0 => false,
            MI3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI3`"]
pub type MI3_R = crate::R<bool, MI3_A>;
impl MI3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI3_A {
        match self.bits {
            false => MI3_A::_0,
            true => MI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI3_A::_1
    }
}
#[doc = "Write proxy for field `MI3`"]
pub struct MI3_W<'a> {
    w: &'a mut W,
}
impl<'a> MI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI3_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI4_A> for bool {
    #[inline(always)]
    fn from(variant: MI4_A) -> Self {
        match variant {
            MI4_A::_0 => false,
            MI4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI4`"]
pub type MI4_R = crate::R<bool, MI4_A>;
impl MI4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI4_A {
        match self.bits {
            false => MI4_A::_0,
            true => MI4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI4_A::_1
    }
}
#[doc = "Write proxy for field `MI4`"]
pub struct MI4_W<'a> {
    w: &'a mut W,
}
impl<'a> MI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI4_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI5_A> for bool {
    #[inline(always)]
    fn from(variant: MI5_A) -> Self {
        match variant {
            MI5_A::_0 => false,
            MI5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI5`"]
pub type MI5_R = crate::R<bool, MI5_A>;
impl MI5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI5_A {
        match self.bits {
            false => MI5_A::_0,
            true => MI5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI5_A::_1
    }
}
#[doc = "Write proxy for field `MI5`"]
pub struct MI5_W<'a> {
    w: &'a mut W,
}
impl<'a> MI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI5_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI6_A> for bool {
    #[inline(always)]
    fn from(variant: MI6_A) -> Self {
        match variant {
            MI6_A::_0 => false,
            MI6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI6`"]
pub type MI6_R = crate::R<bool, MI6_A>;
impl MI6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI6_A {
        match self.bits {
            false => MI6_A::_0,
            true => MI6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI6_A::_1
    }
}
#[doc = "Write proxy for field `MI6`"]
pub struct MI6_W<'a> {
    w: &'a mut W,
}
impl<'a> MI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI6_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI7_A> for bool {
    #[inline(always)]
    fn from(variant: MI7_A) -> Self {
        match variant {
            MI7_A::_0 => false,
            MI7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI7`"]
pub type MI7_R = crate::R<bool, MI7_A>;
impl MI7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI7_A {
        match self.bits {
            false => MI7_A::_0,
            true => MI7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI7_A::_1
    }
}
#[doc = "Write proxy for field `MI7`"]
pub struct MI7_W<'a> {
    w: &'a mut W,
}
impl<'a> MI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI7_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI8_A> for bool {
    #[inline(always)]
    fn from(variant: MI8_A) -> Self {
        match variant {
            MI8_A::_0 => false,
            MI8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI8`"]
pub type MI8_R = crate::R<bool, MI8_A>;
impl MI8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI8_A {
        match self.bits {
            false => MI8_A::_0,
            true => MI8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI8_A::_1
    }
}
#[doc = "Write proxy for field `MI8`"]
pub struct MI8_W<'a> {
    w: &'a mut W,
}
impl<'a> MI8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI8_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI9_A> for bool {
    #[inline(always)]
    fn from(variant: MI9_A) -> Self {
        match variant {
            MI9_A::_0 => false,
            MI9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI9`"]
pub type MI9_R = crate::R<bool, MI9_A>;
impl MI9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI9_A {
        match self.bits {
            false => MI9_A::_0,
            true => MI9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI9_A::_1
    }
}
#[doc = "Write proxy for field `MI9`"]
pub struct MI9_W<'a> {
    w: &'a mut W,
}
impl<'a> MI9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI9_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI10_A> for bool {
    #[inline(always)]
    fn from(variant: MI10_A) -> Self {
        match variant {
            MI10_A::_0 => false,
            MI10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI10`"]
pub type MI10_R = crate::R<bool, MI10_A>;
impl MI10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI10_A {
        match self.bits {
            false => MI10_A::_0,
            true => MI10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI10_A::_1
    }
}
#[doc = "Write proxy for field `MI10`"]
pub struct MI10_W<'a> {
    w: &'a mut W,
}
impl<'a> MI10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI10_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI11_A> for bool {
    #[inline(always)]
    fn from(variant: MI11_A) -> Self {
        match variant {
            MI11_A::_0 => false,
            MI11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI11`"]
pub type MI11_R = crate::R<bool, MI11_A>;
impl MI11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI11_A {
        match self.bits {
            false => MI11_A::_0,
            true => MI11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI11_A::_1
    }
}
#[doc = "Write proxy for field `MI11`"]
pub struct MI11_W<'a> {
    w: &'a mut W,
}
impl<'a> MI11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI11_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI12_A> for bool {
    #[inline(always)]
    fn from(variant: MI12_A) -> Self {
        match variant {
            MI12_A::_0 => false,
            MI12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI12`"]
pub type MI12_R = crate::R<bool, MI12_A>;
impl MI12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI12_A {
        match self.bits {
            false => MI12_A::_0,
            true => MI12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI12_A::_1
    }
}
#[doc = "Write proxy for field `MI12`"]
pub struct MI12_W<'a> {
    w: &'a mut W,
}
impl<'a> MI12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI12_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI13_A> for bool {
    #[inline(always)]
    fn from(variant: MI13_A) -> Self {
        match variant {
            MI13_A::_0 => false,
            MI13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI13`"]
pub type MI13_R = crate::R<bool, MI13_A>;
impl MI13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI13_A {
        match self.bits {
            false => MI13_A::_0,
            true => MI13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI13_A::_1
    }
}
#[doc = "Write proxy for field `MI13`"]
pub struct MI13_W<'a> {
    w: &'a mut W,
}
impl<'a> MI13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI13_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI14_A> for bool {
    #[inline(always)]
    fn from(variant: MI14_A) -> Self {
        match variant {
            MI14_A::_0 => false,
            MI14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI14`"]
pub type MI14_R = crate::R<bool, MI14_A>;
impl MI14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI14_A {
        match self.bits {
            false => MI14_A::_0,
            true => MI14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI14_A::_1
    }
}
#[doc = "Write proxy for field `MI14`"]
pub struct MI14_W<'a> {
    w: &'a mut W,
}
impl<'a> MI14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI14_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI15_A> for bool {
    #[inline(always)]
    fn from(variant: MI15_A) -> Self {
        match variant {
            MI15_A::_0 => false,
            MI15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI15`"]
pub type MI15_R = crate::R<bool, MI15_A>;
impl MI15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI15_A {
        match self.bits {
            false => MI15_A::_0,
            true => MI15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI15_A::_1
    }
}
#[doc = "Write proxy for field `MI15`"]
pub struct MI15_W<'a> {
    w: &'a mut W,
}
impl<'a> MI15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI15_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI16_A> for bool {
    #[inline(always)]
    fn from(variant: MI16_A) -> Self {
        match variant {
            MI16_A::_0 => false,
            MI16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI16`"]
pub type MI16_R = crate::R<bool, MI16_A>;
impl MI16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI16_A {
        match self.bits {
            false => MI16_A::_0,
            true => MI16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI16_A::_1
    }
}
#[doc = "Write proxy for field `MI16`"]
pub struct MI16_W<'a> {
    w: &'a mut W,
}
impl<'a> MI16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI16_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI17_A> for bool {
    #[inline(always)]
    fn from(variant: MI17_A) -> Self {
        match variant {
            MI17_A::_0 => false,
            MI17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI17`"]
pub type MI17_R = crate::R<bool, MI17_A>;
impl MI17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI17_A {
        match self.bits {
            false => MI17_A::_0,
            true => MI17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI17_A::_1
    }
}
#[doc = "Write proxy for field `MI17`"]
pub struct MI17_W<'a> {
    w: &'a mut W,
}
impl<'a> MI17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI17_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI18_A> for bool {
    #[inline(always)]
    fn from(variant: MI18_A) -> Self {
        match variant {
            MI18_A::_0 => false,
            MI18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI18`"]
pub type MI18_R = crate::R<bool, MI18_A>;
impl MI18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI18_A {
        match self.bits {
            false => MI18_A::_0,
            true => MI18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI18_A::_1
    }
}
#[doc = "Write proxy for field `MI18`"]
pub struct MI18_W<'a> {
    w: &'a mut W,
}
impl<'a> MI18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI18_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI19_A> for bool {
    #[inline(always)]
    fn from(variant: MI19_A) -> Self {
        match variant {
            MI19_A::_0 => false,
            MI19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI19`"]
pub type MI19_R = crate::R<bool, MI19_A>;
impl MI19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI19_A {
        match self.bits {
            false => MI19_A::_0,
            true => MI19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI19_A::_1
    }
}
#[doc = "Write proxy for field `MI19`"]
pub struct MI19_W<'a> {
    w: &'a mut W,
}
impl<'a> MI19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI19_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI20_A> for bool {
    #[inline(always)]
    fn from(variant: MI20_A) -> Self {
        match variant {
            MI20_A::_0 => false,
            MI20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI20`"]
pub type MI20_R = crate::R<bool, MI20_A>;
impl MI20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI20_A {
        match self.bits {
            false => MI20_A::_0,
            true => MI20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI20_A::_1
    }
}
#[doc = "Write proxy for field `MI20`"]
pub struct MI20_W<'a> {
    w: &'a mut W,
}
impl<'a> MI20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI20_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI21_A> for bool {
    #[inline(always)]
    fn from(variant: MI21_A) -> Self {
        match variant {
            MI21_A::_0 => false,
            MI21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI21`"]
pub type MI21_R = crate::R<bool, MI21_A>;
impl MI21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI21_A {
        match self.bits {
            false => MI21_A::_0,
            true => MI21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI21_A::_1
    }
}
#[doc = "Write proxy for field `MI21`"]
pub struct MI21_W<'a> {
    w: &'a mut W,
}
impl<'a> MI21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI21_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI22_A> for bool {
    #[inline(always)]
    fn from(variant: MI22_A) -> Self {
        match variant {
            MI22_A::_0 => false,
            MI22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI22`"]
pub type MI22_R = crate::R<bool, MI22_A>;
impl MI22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI22_A {
        match self.bits {
            false => MI22_A::_0,
            true => MI22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI22_A::_1
    }
}
#[doc = "Write proxy for field `MI22`"]
pub struct MI22_W<'a> {
    w: &'a mut W,
}
impl<'a> MI22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI22_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI23_A> for bool {
    #[inline(always)]
    fn from(variant: MI23_A) -> Self {
        match variant {
            MI23_A::_0 => false,
            MI23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI23`"]
pub type MI23_R = crate::R<bool, MI23_A>;
impl MI23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI23_A {
        match self.bits {
            false => MI23_A::_0,
            true => MI23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI23_A::_1
    }
}
#[doc = "Write proxy for field `MI23`"]
pub struct MI23_W<'a> {
    w: &'a mut W,
}
impl<'a> MI23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI23_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI24_A> for bool {
    #[inline(always)]
    fn from(variant: MI24_A) -> Self {
        match variant {
            MI24_A::_0 => false,
            MI24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI24`"]
pub type MI24_R = crate::R<bool, MI24_A>;
impl MI24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI24_A {
        match self.bits {
            false => MI24_A::_0,
            true => MI24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI24_A::_1
    }
}
#[doc = "Write proxy for field `MI24`"]
pub struct MI24_W<'a> {
    w: &'a mut W,
}
impl<'a> MI24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI24_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI25_A> for bool {
    #[inline(always)]
    fn from(variant: MI25_A) -> Self {
        match variant {
            MI25_A::_0 => false,
            MI25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI25`"]
pub type MI25_R = crate::R<bool, MI25_A>;
impl MI25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI25_A {
        match self.bits {
            false => MI25_A::_0,
            true => MI25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI25_A::_1
    }
}
#[doc = "Write proxy for field `MI25`"]
pub struct MI25_W<'a> {
    w: &'a mut W,
}
impl<'a> MI25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI25_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI26_A> for bool {
    #[inline(always)]
    fn from(variant: MI26_A) -> Self {
        match variant {
            MI26_A::_0 => false,
            MI26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI26`"]
pub type MI26_R = crate::R<bool, MI26_A>;
impl MI26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI26_A {
        match self.bits {
            false => MI26_A::_0,
            true => MI26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI26_A::_1
    }
}
#[doc = "Write proxy for field `MI26`"]
pub struct MI26_W<'a> {
    w: &'a mut W,
}
impl<'a> MI26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI26_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI27_A> for bool {
    #[inline(always)]
    fn from(variant: MI27_A) -> Self {
        match variant {
            MI27_A::_0 => false,
            MI27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI27`"]
pub type MI27_R = crate::R<bool, MI27_A>;
impl MI27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI27_A {
        match self.bits {
            false => MI27_A::_0,
            true => MI27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI27_A::_1
    }
}
#[doc = "Write proxy for field `MI27`"]
pub struct MI27_W<'a> {
    w: &'a mut W,
}
impl<'a> MI27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI27_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI28_A> for bool {
    #[inline(always)]
    fn from(variant: MI28_A) -> Self {
        match variant {
            MI28_A::_0 => false,
            MI28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI28`"]
pub type MI28_R = crate::R<bool, MI28_A>;
impl MI28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI28_A {
        match self.bits {
            false => MI28_A::_0,
            true => MI28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI28_A::_1
    }
}
#[doc = "Write proxy for field `MI28`"]
pub struct MI28_W<'a> {
    w: &'a mut W,
}
impl<'a> MI28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI28_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI29_A> for bool {
    #[inline(always)]
    fn from(variant: MI29_A) -> Self {
        match variant {
            MI29_A::_0 => false,
            MI29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI29`"]
pub type MI29_R = crate::R<bool, MI29_A>;
impl MI29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI29_A {
        match self.bits {
            false => MI29_A::_0,
            true => MI29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI29_A::_1
    }
}
#[doc = "Write proxy for field `MI29`"]
pub struct MI29_W<'a> {
    w: &'a mut W,
}
impl<'a> MI29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI29_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI30_A> for bool {
    #[inline(always)]
    fn from(variant: MI30_A) -> Self {
        match variant {
            MI30_A::_0 => false,
            MI30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI30`"]
pub type MI30_R = crate::R<bool, MI30_A>;
impl MI30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI30_A {
        match self.bits {
            false => MI30_A::_0,
            true => MI30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI30_A::_1
    }
}
#[doc = "Write proxy for field `MI30`"]
pub struct MI30_W<'a> {
    w: &'a mut W,
}
impl<'a> MI30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI30_A::_1)
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
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1,
}
impl From<MI31_A> for bool {
    #[inline(always)]
    fn from(variant: MI31_A) -> Self {
        match variant {
            MI31_A::_0 => false,
            MI31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MI31`"]
pub type MI31_R = crate::R<bool, MI31_A>;
impl MI31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI31_A {
        match self.bits {
            false => MI31_A::_0,
            true => MI31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI31_A::_1
    }
}
#[doc = "Write proxy for field `MI31`"]
pub struct MI31_W<'a> {
    w: &'a mut W,
}
impl<'a> MI31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI31_A::_1)
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
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi0(&self) -> MI0_R {
        MI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi1(&self) -> MI1_R {
        MI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi2(&self) -> MI2_R {
        MI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi3(&self) -> MI3_R {
        MI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi4(&self) -> MI4_R {
        MI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi5(&self) -> MI5_R {
        MI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi6(&self) -> MI6_R {
        MI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi7(&self) -> MI7_R {
        MI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi8(&self) -> MI8_R {
        MI8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi9(&self) -> MI9_R {
        MI9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi10(&self) -> MI10_R {
        MI10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi11(&self) -> MI11_R {
        MI11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi12(&self) -> MI12_R {
        MI12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi13(&self) -> MI13_R {
        MI13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi14(&self) -> MI14_R {
        MI14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi15(&self) -> MI15_R {
        MI15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi16(&self) -> MI16_R {
        MI16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi17(&self) -> MI17_R {
        MI17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi18(&self) -> MI18_R {
        MI18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi19(&self) -> MI19_R {
        MI19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi20(&self) -> MI20_R {
        MI20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi21(&self) -> MI21_R {
        MI21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi22(&self) -> MI22_R {
        MI22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi23(&self) -> MI23_R {
        MI23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi24(&self) -> MI24_R {
        MI24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi25(&self) -> MI25_R {
        MI25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi26(&self) -> MI26_R {
        MI26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi27(&self) -> MI27_R {
        MI27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi28(&self) -> MI28_R {
        MI28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi29(&self) -> MI29_R {
        MI29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi30(&self) -> MI30_R {
        MI30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi31(&self) -> MI31_R {
        MI31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi0(&mut self) -> MI0_W {
        MI0_W { w: self }
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi1(&mut self) -> MI1_W {
        MI1_W { w: self }
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi2(&mut self) -> MI2_W {
        MI2_W { w: self }
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi3(&mut self) -> MI3_W {
        MI3_W { w: self }
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi4(&mut self) -> MI4_W {
        MI4_W { w: self }
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi5(&mut self) -> MI5_W {
        MI5_W { w: self }
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi6(&mut self) -> MI6_W {
        MI6_W { w: self }
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi7(&mut self) -> MI7_W {
        MI7_W { w: self }
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi8(&mut self) -> MI8_W {
        MI8_W { w: self }
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi9(&mut self) -> MI9_W {
        MI9_W { w: self }
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi10(&mut self) -> MI10_W {
        MI10_W { w: self }
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi11(&mut self) -> MI11_W {
        MI11_W { w: self }
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi12(&mut self) -> MI12_W {
        MI12_W { w: self }
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi13(&mut self) -> MI13_W {
        MI13_W { w: self }
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi14(&mut self) -> MI14_W {
        MI14_W { w: self }
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi15(&mut self) -> MI15_W {
        MI15_W { w: self }
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi16(&mut self) -> MI16_W {
        MI16_W { w: self }
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi17(&mut self) -> MI17_W {
        MI17_W { w: self }
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi18(&mut self) -> MI18_W {
        MI18_W { w: self }
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi19(&mut self) -> MI19_W {
        MI19_W { w: self }
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi20(&mut self) -> MI20_W {
        MI20_W { w: self }
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi21(&mut self) -> MI21_W {
        MI21_W { w: self }
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi22(&mut self) -> MI22_W {
        MI22_W { w: self }
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi23(&mut self) -> MI23_W {
        MI23_W { w: self }
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi24(&mut self) -> MI24_W {
        MI24_W { w: self }
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi25(&mut self) -> MI25_W {
        MI25_W { w: self }
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi26(&mut self) -> MI26_W {
        MI26_W { w: self }
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi27(&mut self) -> MI27_W {
        MI27_W { w: self }
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi28(&mut self) -> MI28_W {
        MI28_W { w: self }
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi29(&mut self) -> MI29_W {
        MI29_W { w: self }
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi30(&mut self) -> MI30_W {
        MI30_W { w: self }
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi31(&mut self) -> MI31_W {
        MI31_W { w: self }
    }
}
