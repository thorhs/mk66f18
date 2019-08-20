#[doc = "Reader of register MPRA"]
pub type R = crate::R<u32, super::MPRA>;
#[doc = "Writer for register MPRA"]
pub type W = crate::W<u32, super::MPRA>;
#[doc = "Register MPRA `reset()`'s with value 0x7770_0000"]
impl crate::ResetValue for super::MPRA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7770_0000
    }
}
#[doc = "Master 6 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL6_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL6_A> for bool {
    #[inline(always)]
    fn from(variant: MPL6_A) -> Self {
        match variant {
            MPL6_A::_0 => false,
            MPL6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL6`"]
pub type MPL6_R = crate::R<bool, MPL6_A>;
impl MPL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL6_A {
        match self.bits {
            false => MPL6_A::_0,
            true => MPL6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL6_A::_1
    }
}
#[doc = "Write proxy for field `MPL6`"]
pub struct MPL6_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL6_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL6_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 6 Trusted for Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW6_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW6_A> for bool {
    #[inline(always)]
    fn from(variant: MTW6_A) -> Self {
        match variant {
            MTW6_A::_0 => false,
            MTW6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW6`"]
pub type MTW6_R = crate::R<bool, MTW6_A>;
impl MTW6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW6_A {
        match self.bits {
            false => MTW6_A::_0,
            true => MTW6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW6_A::_1
    }
}
#[doc = "Write proxy for field `MTW6`"]
pub struct MTW6_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW6_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW6_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 6 Trusted for Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR6_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR6_A> for bool {
    #[inline(always)]
    fn from(variant: MTR6_A) -> Self {
        match variant {
            MTR6_A::_0 => false,
            MTR6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR6`"]
pub type MTR6_R = crate::R<bool, MTR6_A>;
impl MTR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR6_A {
        match self.bits {
            false => MTR6_A::_0,
            true => MTR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR6_A::_1
    }
}
#[doc = "Write proxy for field `MTR6`"]
pub struct MTR6_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR6_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR6_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 5 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL5_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL5_A> for bool {
    #[inline(always)]
    fn from(variant: MPL5_A) -> Self {
        match variant {
            MPL5_A::_0 => false,
            MPL5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL5`"]
pub type MPL5_R = crate::R<bool, MPL5_A>;
impl MPL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL5_A {
        match self.bits {
            false => MPL5_A::_0,
            true => MPL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL5_A::_1
    }
}
#[doc = "Write proxy for field `MPL5`"]
pub struct MPL5_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL5_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 5 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW5_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW5_A> for bool {
    #[inline(always)]
    fn from(variant: MTW5_A) -> Self {
        match variant {
            MTW5_A::_0 => false,
            MTW5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW5`"]
pub type MTW5_R = crate::R<bool, MTW5_A>;
impl MTW5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW5_A {
        match self.bits {
            false => MTW5_A::_0,
            true => MTW5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW5_A::_1
    }
}
#[doc = "Write proxy for field `MTW5`"]
pub struct MTW5_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW5_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 5 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR5_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR5_A> for bool {
    #[inline(always)]
    fn from(variant: MTR5_A) -> Self {
        match variant {
            MTR5_A::_0 => false,
            MTR5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR5`"]
pub type MTR5_R = crate::R<bool, MTR5_A>;
impl MTR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR5_A {
        match self.bits {
            false => MTR5_A::_0,
            true => MTR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR5_A::_1
    }
}
#[doc = "Write proxy for field `MTR5`"]
pub struct MTR5_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR5_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 4 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL4_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL4_A> for bool {
    #[inline(always)]
    fn from(variant: MPL4_A) -> Self {
        match variant {
            MPL4_A::_0 => false,
            MPL4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL4`"]
pub type MPL4_R = crate::R<bool, MPL4_A>;
impl MPL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL4_A {
        match self.bits {
            false => MPL4_A::_0,
            true => MPL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL4_A::_1
    }
}
#[doc = "Write proxy for field `MPL4`"]
pub struct MPL4_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL4_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL4_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 4 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW4_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW4_A> for bool {
    #[inline(always)]
    fn from(variant: MTW4_A) -> Self {
        match variant {
            MTW4_A::_0 => false,
            MTW4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW4`"]
pub type MTW4_R = crate::R<bool, MTW4_A>;
impl MTW4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW4_A {
        match self.bits {
            false => MTW4_A::_0,
            true => MTW4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW4_A::_1
    }
}
#[doc = "Write proxy for field `MTW4`"]
pub struct MTW4_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW4_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW4_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 4 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR4_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR4_A> for bool {
    #[inline(always)]
    fn from(variant: MTR4_A) -> Self {
        match variant {
            MTR4_A::_0 => false,
            MTR4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR4`"]
pub type MTR4_R = crate::R<bool, MTR4_A>;
impl MTR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR4_A {
        match self.bits {
            false => MTR4_A::_0,
            true => MTR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR4_A::_1
    }
}
#[doc = "Write proxy for field `MTR4`"]
pub struct MTR4_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR4_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR4_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 3 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL3_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL3_A> for bool {
    #[inline(always)]
    fn from(variant: MPL3_A) -> Self {
        match variant {
            MPL3_A::_0 => false,
            MPL3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL3`"]
pub type MPL3_R = crate::R<bool, MPL3_A>;
impl MPL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL3_A {
        match self.bits {
            false => MPL3_A::_0,
            true => MPL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL3_A::_1
    }
}
#[doc = "Write proxy for field `MPL3`"]
pub struct MPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL3_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 3 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW3_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW3_A> for bool {
    #[inline(always)]
    fn from(variant: MTW3_A) -> Self {
        match variant {
            MTW3_A::_0 => false,
            MTW3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW3`"]
pub type MTW3_R = crate::R<bool, MTW3_A>;
impl MTW3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW3_A {
        match self.bits {
            false => MTW3_A::_0,
            true => MTW3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW3_A::_1
    }
}
#[doc = "Write proxy for field `MTW3`"]
pub struct MTW3_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW3_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 3 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR3_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR3_A> for bool {
    #[inline(always)]
    fn from(variant: MTR3_A) -> Self {
        match variant {
            MTR3_A::_0 => false,
            MTR3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR3`"]
pub type MTR3_R = crate::R<bool, MTR3_A>;
impl MTR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR3_A {
        match self.bits {
            false => MTR3_A::_0,
            true => MTR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR3_A::_1
    }
}
#[doc = "Write proxy for field `MTR3`"]
pub struct MTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR3_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 2 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL2_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL2_A> for bool {
    #[inline(always)]
    fn from(variant: MPL2_A) -> Self {
        match variant {
            MPL2_A::_0 => false,
            MPL2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL2`"]
pub type MPL2_R = crate::R<bool, MPL2_A>;
impl MPL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL2_A {
        match self.bits {
            false => MPL2_A::_0,
            true => MPL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL2_A::_1
    }
}
#[doc = "Write proxy for field `MPL2`"]
pub struct MPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 2 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW2_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW2_A> for bool {
    #[inline(always)]
    fn from(variant: MTW2_A) -> Self {
        match variant {
            MTW2_A::_0 => false,
            MTW2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW2`"]
pub type MTW2_R = crate::R<bool, MTW2_A>;
impl MTW2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW2_A {
        match self.bits {
            false => MTW2_A::_0,
            true => MTW2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW2_A::_1
    }
}
#[doc = "Write proxy for field `MTW2`"]
pub struct MTW2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 2 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR2_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR2_A> for bool {
    #[inline(always)]
    fn from(variant: MTR2_A) -> Self {
        match variant {
            MTR2_A::_0 => false,
            MTR2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR2`"]
pub type MTR2_R = crate::R<bool, MTR2_A>;
impl MTR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR2_A {
        match self.bits {
            false => MTR2_A::_0,
            true => MTR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR2_A::_1
    }
}
#[doc = "Write proxy for field `MTR2`"]
pub struct MTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 1 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL1_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL1_A> for bool {
    #[inline(always)]
    fn from(variant: MPL1_A) -> Self {
        match variant {
            MPL1_A::_0 => false,
            MPL1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL1`"]
pub type MPL1_R = crate::R<bool, MPL1_A>;
impl MPL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL1_A {
        match self.bits {
            false => MPL1_A::_0,
            true => MPL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL1_A::_1
    }
}
#[doc = "Write proxy for field `MPL1`"]
pub struct MPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 1 Trusted for Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW1_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW1_A> for bool {
    #[inline(always)]
    fn from(variant: MTW1_A) -> Self {
        match variant {
            MTW1_A::_0 => false,
            MTW1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW1`"]
pub type MTW1_R = crate::R<bool, MTW1_A>;
impl MTW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW1_A {
        match self.bits {
            false => MTW1_A::_0,
            true => MTW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW1_A::_1
    }
}
#[doc = "Write proxy for field `MTW1`"]
pub struct MTW1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 1 Trusted for Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR1_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR1_A> for bool {
    #[inline(always)]
    fn from(variant: MTR1_A) -> Self {
        match variant {
            MTR1_A::_0 => false,
            MTR1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR1`"]
pub type MTR1_R = crate::R<bool, MTR1_A>;
impl MTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR1_A {
        match self.bits {
            false => MTR1_A::_0,
            true => MTR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR1_A::_1
    }
}
#[doc = "Write proxy for field `MTR1`"]
pub struct MTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 0 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL0_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1,
}
impl From<MPL0_A> for bool {
    #[inline(always)]
    fn from(variant: MPL0_A) -> Self {
        match variant {
            MPL0_A::_0 => false,
            MPL0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPL0`"]
pub type MPL0_R = crate::R<bool, MPL0_A>;
impl MPL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL0_A {
        match self.bits {
            false => MPL0_A::_0,
            true => MPL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL0_A::_1
    }
}
#[doc = "Write proxy for field `MPL0`"]
pub struct MPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 0 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW0_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0,
    #[doc = "1: This master is trusted for write accesses."]
    _1,
}
impl From<MTW0_A> for bool {
    #[inline(always)]
    fn from(variant: MTW0_A) -> Self {
        match variant {
            MTW0_A::_0 => false,
            MTW0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTW0`"]
pub type MTW0_R = crate::R<bool, MTW0_A>;
impl MTW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW0_A {
        match self.bits {
            false => MTW0_A::_0,
            true => MTW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW0_A::_1
    }
}
#[doc = "Write proxy for field `MTW0`"]
pub struct MTW0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Master 0 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR0_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0,
    #[doc = "1: This master is trusted for read accesses."]
    _1,
}
impl From<MTR0_A> for bool {
    #[inline(always)]
    fn from(variant: MTR0_A) -> Self {
        match variant {
            MTR0_A::_0 => false,
            MTR0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MTR0`"]
pub type MTR0_R = crate::R<bool, MTR0_A>;
impl MTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR0_A {
        match self.bits {
            false => MTR0_A::_0,
            true => MTR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR0_A::_1
    }
}
#[doc = "Write proxy for field `MTR0`"]
pub struct MTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 4 - Master 6 Privilege Level"]
    #[inline(always)]
    pub fn mpl6(&self) -> MPL6_R {
        MPL6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master 6 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw6(&self) -> MTW6_R {
        MTW6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master 6 Trusted for Read"]
    #[inline(always)]
    pub fn mtr6(&self) -> MTR6_R {
        MTR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline(always)]
    pub fn mpl5(&self) -> MPL5_R {
        MPL5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw5(&self) -> MTW5_R {
        MTW5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline(always)]
    pub fn mtr5(&self) -> MTR5_R {
        MTR5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    pub fn mpl4(&self) -> MPL4_R {
        MPL4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw4(&self) -> MTW4_R {
        MTW4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    pub fn mtr4(&self) -> MTR4_R {
        MTR4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    pub fn mpl3(&self) -> MPL3_R {
        MPL3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw3(&self) -> MTW3_R {
        MTW3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    pub fn mtr3(&self) -> MTR3_R {
        MTR3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&self) -> MPL2_R {
        MPL2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&self) -> MTW2_R {
        MTW2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&self) -> MTR2_R {
        MTR2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&self) -> MPL1_R {
        MPL1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&self) -> MTW1_R {
        MTW1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&self) -> MTR1_R {
        MTR1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&self) -> MPL0_R {
        MPL0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&self) -> MTW0_R {
        MTW0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&self) -> MTR0_R {
        MTR0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Master 6 Privilege Level"]
    #[inline(always)]
    pub fn mpl6(&mut self) -> MPL6_W {
        MPL6_W { w: self }
    }
    #[doc = "Bit 5 - Master 6 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw6(&mut self) -> MTW6_W {
        MTW6_W { w: self }
    }
    #[doc = "Bit 6 - Master 6 Trusted for Read"]
    #[inline(always)]
    pub fn mtr6(&mut self) -> MTR6_W {
        MTR6_W { w: self }
    }
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline(always)]
    pub fn mpl5(&mut self) -> MPL5_W {
        MPL5_W { w: self }
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw5(&mut self) -> MTW5_W {
        MTW5_W { w: self }
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline(always)]
    pub fn mtr5(&mut self) -> MTR5_W {
        MTR5_W { w: self }
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    pub fn mpl4(&mut self) -> MPL4_W {
        MPL4_W { w: self }
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw4(&mut self) -> MTW4_W {
        MTW4_W { w: self }
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    pub fn mtr4(&mut self) -> MTR4_W {
        MTR4_W { w: self }
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    pub fn mpl3(&mut self) -> MPL3_W {
        MPL3_W { w: self }
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw3(&mut self) -> MTW3_W {
        MTW3_W { w: self }
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    pub fn mtr3(&mut self) -> MTR3_W {
        MTR3_W { w: self }
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&mut self) -> MPL2_W {
        MPL2_W { w: self }
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&mut self) -> MTW2_W {
        MTW2_W { w: self }
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&mut self) -> MTR2_W {
        MTR2_W { w: self }
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&mut self) -> MPL1_W {
        MPL1_W { w: self }
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&mut self) -> MTW1_W {
        MTW1_W { w: self }
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&mut self) -> MTR1_W {
        MTR1_W { w: self }
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&mut self) -> MPL0_W {
        MPL0_W { w: self }
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&mut self) -> MTW0_W {
        MTW0_W { w: self }
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&mut self) -> MTR0_W {
        MTR0_W { w: self }
    }
}
