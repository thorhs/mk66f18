#[doc = "Reader of register IMASK1"]
pub type R = crate::R<u32, super::IMASK1>;
#[doc = "Writer for register IMASK1"]
pub type W = crate::W<u32, super::IMASK1>;
#[doc = "Register IMASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM0_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM0_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM0_A) -> Self {
        match variant {
            BUFLM0_A::_0 => false,
            BUFLM0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM0`"]
pub type BUFLM0_R = crate::R<bool, BUFLM0_A>;
impl BUFLM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM0_A {
        match self.bits {
            false => BUFLM0_A::_0,
            true => BUFLM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM0_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM0`"]
pub struct BUFLM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM0_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM1_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM1_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM1_A) -> Self {
        match variant {
            BUFLM1_A::_0 => false,
            BUFLM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM1`"]
pub type BUFLM1_R = crate::R<bool, BUFLM1_A>;
impl BUFLM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM1_A {
        match self.bits {
            false => BUFLM1_A::_0,
            true => BUFLM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM1_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM1`"]
pub struct BUFLM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM1_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM2_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM2_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM2_A) -> Self {
        match variant {
            BUFLM2_A::_0 => false,
            BUFLM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM2`"]
pub type BUFLM2_R = crate::R<bool, BUFLM2_A>;
impl BUFLM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM2_A {
        match self.bits {
            false => BUFLM2_A::_0,
            true => BUFLM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM2_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM2`"]
pub struct BUFLM2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM2_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM3_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM3_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM3_A) -> Self {
        match variant {
            BUFLM3_A::_0 => false,
            BUFLM3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM3`"]
pub type BUFLM3_R = crate::R<bool, BUFLM3_A>;
impl BUFLM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM3_A {
        match self.bits {
            false => BUFLM3_A::_0,
            true => BUFLM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM3_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM3`"]
pub struct BUFLM3_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM3_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM3_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM4_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM4_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM4_A) -> Self {
        match variant {
            BUFLM4_A::_0 => false,
            BUFLM4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM4`"]
pub type BUFLM4_R = crate::R<bool, BUFLM4_A>;
impl BUFLM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM4_A {
        match self.bits {
            false => BUFLM4_A::_0,
            true => BUFLM4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM4_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM4`"]
pub struct BUFLM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM4_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM4_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM5_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM5_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM5_A) -> Self {
        match variant {
            BUFLM5_A::_0 => false,
            BUFLM5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM5`"]
pub type BUFLM5_R = crate::R<bool, BUFLM5_A>;
impl BUFLM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM5_A {
        match self.bits {
            false => BUFLM5_A::_0,
            true => BUFLM5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM5_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM5`"]
pub struct BUFLM5_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM5_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM5_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM6_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM6_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM6_A) -> Self {
        match variant {
            BUFLM6_A::_0 => false,
            BUFLM6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM6`"]
pub type BUFLM6_R = crate::R<bool, BUFLM6_A>;
impl BUFLM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM6_A {
        match self.bits {
            false => BUFLM6_A::_0,
            true => BUFLM6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM6_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM6`"]
pub struct BUFLM6_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM6_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM6_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM7_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM7_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM7_A) -> Self {
        match variant {
            BUFLM7_A::_0 => false,
            BUFLM7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM7`"]
pub type BUFLM7_R = crate::R<bool, BUFLM7_A>;
impl BUFLM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM7_A {
        match self.bits {
            false => BUFLM7_A::_0,
            true => BUFLM7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM7_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM7`"]
pub struct BUFLM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM7_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM7_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM8_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM8_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM8_A) -> Self {
        match variant {
            BUFLM8_A::_0 => false,
            BUFLM8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM8`"]
pub type BUFLM8_R = crate::R<bool, BUFLM8_A>;
impl BUFLM8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM8_A {
        match self.bits {
            false => BUFLM8_A::_0,
            true => BUFLM8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM8_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM8`"]
pub struct BUFLM8_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM8_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM8_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM9_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM9_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM9_A) -> Self {
        match variant {
            BUFLM9_A::_0 => false,
            BUFLM9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM9`"]
pub type BUFLM9_R = crate::R<bool, BUFLM9_A>;
impl BUFLM9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM9_A {
        match self.bits {
            false => BUFLM9_A::_0,
            true => BUFLM9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM9_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM9`"]
pub struct BUFLM9_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM9_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM9_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM10_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM10_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM10_A) -> Self {
        match variant {
            BUFLM10_A::_0 => false,
            BUFLM10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM10`"]
pub type BUFLM10_R = crate::R<bool, BUFLM10_A>;
impl BUFLM10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM10_A {
        match self.bits {
            false => BUFLM10_A::_0,
            true => BUFLM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM10_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM10`"]
pub struct BUFLM10_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM10_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM10_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM11_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM11_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM11_A) -> Self {
        match variant {
            BUFLM11_A::_0 => false,
            BUFLM11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM11`"]
pub type BUFLM11_R = crate::R<bool, BUFLM11_A>;
impl BUFLM11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM11_A {
        match self.bits {
            false => BUFLM11_A::_0,
            true => BUFLM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM11_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM11`"]
pub struct BUFLM11_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM11_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM11_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM12_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM12_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM12_A) -> Self {
        match variant {
            BUFLM12_A::_0 => false,
            BUFLM12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM12`"]
pub type BUFLM12_R = crate::R<bool, BUFLM12_A>;
impl BUFLM12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM12_A {
        match self.bits {
            false => BUFLM12_A::_0,
            true => BUFLM12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM12_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM12`"]
pub struct BUFLM12_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM12_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM12_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM13_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM13_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM13_A) -> Self {
        match variant {
            BUFLM13_A::_0 => false,
            BUFLM13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM13`"]
pub type BUFLM13_R = crate::R<bool, BUFLM13_A>;
impl BUFLM13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM13_A {
        match self.bits {
            false => BUFLM13_A::_0,
            true => BUFLM13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM13_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM13`"]
pub struct BUFLM13_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM13_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM13_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM14_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM14_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM14_A) -> Self {
        match variant {
            BUFLM14_A::_0 => false,
            BUFLM14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM14`"]
pub type BUFLM14_R = crate::R<bool, BUFLM14_A>;
impl BUFLM14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM14_A {
        match self.bits {
            false => BUFLM14_A::_0,
            true => BUFLM14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM14_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM14`"]
pub struct BUFLM14_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM14_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM14_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM15_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM15_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM15_A) -> Self {
        match variant {
            BUFLM15_A::_0 => false,
            BUFLM15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM15`"]
pub type BUFLM15_R = crate::R<bool, BUFLM15_A>;
impl BUFLM15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM15_A {
        match self.bits {
            false => BUFLM15_A::_0,
            true => BUFLM15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM15_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM15`"]
pub struct BUFLM15_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM15_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM15_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM16_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM16_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM16_A) -> Self {
        match variant {
            BUFLM16_A::_0 => false,
            BUFLM16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM16`"]
pub type BUFLM16_R = crate::R<bool, BUFLM16_A>;
impl BUFLM16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM16_A {
        match self.bits {
            false => BUFLM16_A::_0,
            true => BUFLM16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM16_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM16`"]
pub struct BUFLM16_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM16_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM16_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM17_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM17_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM17_A) -> Self {
        match variant {
            BUFLM17_A::_0 => false,
            BUFLM17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM17`"]
pub type BUFLM17_R = crate::R<bool, BUFLM17_A>;
impl BUFLM17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM17_A {
        match self.bits {
            false => BUFLM17_A::_0,
            true => BUFLM17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM17_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM17`"]
pub struct BUFLM17_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM17_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM17_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM18_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM18_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM18_A) -> Self {
        match variant {
            BUFLM18_A::_0 => false,
            BUFLM18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM18`"]
pub type BUFLM18_R = crate::R<bool, BUFLM18_A>;
impl BUFLM18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM18_A {
        match self.bits {
            false => BUFLM18_A::_0,
            true => BUFLM18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM18_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM18`"]
pub struct BUFLM18_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM18_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM18_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM19_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM19_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM19_A) -> Self {
        match variant {
            BUFLM19_A::_0 => false,
            BUFLM19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM19`"]
pub type BUFLM19_R = crate::R<bool, BUFLM19_A>;
impl BUFLM19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM19_A {
        match self.bits {
            false => BUFLM19_A::_0,
            true => BUFLM19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM19_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM19`"]
pub struct BUFLM19_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM19_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM19_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM20_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM20_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM20_A) -> Self {
        match variant {
            BUFLM20_A::_0 => false,
            BUFLM20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM20`"]
pub type BUFLM20_R = crate::R<bool, BUFLM20_A>;
impl BUFLM20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM20_A {
        match self.bits {
            false => BUFLM20_A::_0,
            true => BUFLM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM20_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM20`"]
pub struct BUFLM20_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM20_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM20_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM21_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM21_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM21_A) -> Self {
        match variant {
            BUFLM21_A::_0 => false,
            BUFLM21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM21`"]
pub type BUFLM21_R = crate::R<bool, BUFLM21_A>;
impl BUFLM21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM21_A {
        match self.bits {
            false => BUFLM21_A::_0,
            true => BUFLM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM21_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM21`"]
pub struct BUFLM21_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM21_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM21_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM22_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM22_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM22_A) -> Self {
        match variant {
            BUFLM22_A::_0 => false,
            BUFLM22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM22`"]
pub type BUFLM22_R = crate::R<bool, BUFLM22_A>;
impl BUFLM22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM22_A {
        match self.bits {
            false => BUFLM22_A::_0,
            true => BUFLM22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM22_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM22`"]
pub struct BUFLM22_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM22_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM22_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM23_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM23_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM23_A) -> Self {
        match variant {
            BUFLM23_A::_0 => false,
            BUFLM23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM23`"]
pub type BUFLM23_R = crate::R<bool, BUFLM23_A>;
impl BUFLM23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM23_A {
        match self.bits {
            false => BUFLM23_A::_0,
            true => BUFLM23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM23_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM23`"]
pub struct BUFLM23_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM23_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM23_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM24_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM24_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM24_A) -> Self {
        match variant {
            BUFLM24_A::_0 => false,
            BUFLM24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM24`"]
pub type BUFLM24_R = crate::R<bool, BUFLM24_A>;
impl BUFLM24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM24_A {
        match self.bits {
            false => BUFLM24_A::_0,
            true => BUFLM24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM24_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM24`"]
pub struct BUFLM24_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM24_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM24_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM25_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM25_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM25_A) -> Self {
        match variant {
            BUFLM25_A::_0 => false,
            BUFLM25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM25`"]
pub type BUFLM25_R = crate::R<bool, BUFLM25_A>;
impl BUFLM25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM25_A {
        match self.bits {
            false => BUFLM25_A::_0,
            true => BUFLM25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM25_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM25`"]
pub struct BUFLM25_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM25_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM25_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM26_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM26_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM26_A) -> Self {
        match variant {
            BUFLM26_A::_0 => false,
            BUFLM26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM26`"]
pub type BUFLM26_R = crate::R<bool, BUFLM26_A>;
impl BUFLM26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM26_A {
        match self.bits {
            false => BUFLM26_A::_0,
            true => BUFLM26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM26_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM26`"]
pub struct BUFLM26_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM26_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM26_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM27_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM27_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM27_A) -> Self {
        match variant {
            BUFLM27_A::_0 => false,
            BUFLM27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM27`"]
pub type BUFLM27_R = crate::R<bool, BUFLM27_A>;
impl BUFLM27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM27_A {
        match self.bits {
            false => BUFLM27_A::_0,
            true => BUFLM27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM27_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM27`"]
pub struct BUFLM27_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM27_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM27_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM28_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM28_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM28_A) -> Self {
        match variant {
            BUFLM28_A::_0 => false,
            BUFLM28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM28`"]
pub type BUFLM28_R = crate::R<bool, BUFLM28_A>;
impl BUFLM28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM28_A {
        match self.bits {
            false => BUFLM28_A::_0,
            true => BUFLM28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM28_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM28`"]
pub struct BUFLM28_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM28_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM28_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM29_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM29_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM29_A) -> Self {
        match variant {
            BUFLM29_A::_0 => false,
            BUFLM29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM29`"]
pub type BUFLM29_R = crate::R<bool, BUFLM29_A>;
impl BUFLM29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM29_A {
        match self.bits {
            false => BUFLM29_A::_0,
            true => BUFLM29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM29_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM29`"]
pub struct BUFLM29_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM29_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM29_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM30_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM30_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM30_A) -> Self {
        match variant {
            BUFLM30_A::_0 => false,
            BUFLM30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM30`"]
pub type BUFLM30_R = crate::R<bool, BUFLM30_A>;
impl BUFLM30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM30_A {
        match self.bits {
            false => BUFLM30_A::_0,
            true => BUFLM30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM30_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM30`"]
pub struct BUFLM30_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM30_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM30_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Buffer MB i Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM31_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1,
}
impl From<BUFLM31_A> for bool {
    #[inline(always)]
    fn from(variant: BUFLM31_A) -> Self {
        match variant {
            BUFLM31_A::_0 => false,
            BUFLM31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUFLM31`"]
pub type BUFLM31_R = crate::R<bool, BUFLM31_A>;
impl BUFLM31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFLM31_A {
        match self.bits {
            false => BUFLM31_A::_0,
            true => BUFLM31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM31_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM31`"]
pub struct BUFLM31_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM31_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM31_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm0(&self) -> BUFLM0_R {
        BUFLM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm1(&self) -> BUFLM1_R {
        BUFLM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm2(&self) -> BUFLM2_R {
        BUFLM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm3(&self) -> BUFLM3_R {
        BUFLM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm4(&self) -> BUFLM4_R {
        BUFLM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm5(&self) -> BUFLM5_R {
        BUFLM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm6(&self) -> BUFLM6_R {
        BUFLM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm7(&self) -> BUFLM7_R {
        BUFLM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm8(&self) -> BUFLM8_R {
        BUFLM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm9(&self) -> BUFLM9_R {
        BUFLM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm10(&self) -> BUFLM10_R {
        BUFLM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm11(&self) -> BUFLM11_R {
        BUFLM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm12(&self) -> BUFLM12_R {
        BUFLM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm13(&self) -> BUFLM13_R {
        BUFLM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm14(&self) -> BUFLM14_R {
        BUFLM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm15(&self) -> BUFLM15_R {
        BUFLM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm16(&self) -> BUFLM16_R {
        BUFLM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm17(&self) -> BUFLM17_R {
        BUFLM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm18(&self) -> BUFLM18_R {
        BUFLM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm19(&self) -> BUFLM19_R {
        BUFLM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm20(&self) -> BUFLM20_R {
        BUFLM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm21(&self) -> BUFLM21_R {
        BUFLM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm22(&self) -> BUFLM22_R {
        BUFLM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm23(&self) -> BUFLM23_R {
        BUFLM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm24(&self) -> BUFLM24_R {
        BUFLM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm25(&self) -> BUFLM25_R {
        BUFLM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm26(&self) -> BUFLM26_R {
        BUFLM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm27(&self) -> BUFLM27_R {
        BUFLM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm28(&self) -> BUFLM28_R {
        BUFLM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm29(&self) -> BUFLM29_R {
        BUFLM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm30(&self) -> BUFLM30_R {
        BUFLM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm31(&self) -> BUFLM31_R {
        BUFLM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm0(&mut self) -> BUFLM0_W {
        BUFLM0_W { w: self }
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm1(&mut self) -> BUFLM1_W {
        BUFLM1_W { w: self }
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm2(&mut self) -> BUFLM2_W {
        BUFLM2_W { w: self }
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm3(&mut self) -> BUFLM3_W {
        BUFLM3_W { w: self }
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm4(&mut self) -> BUFLM4_W {
        BUFLM4_W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm5(&mut self) -> BUFLM5_W {
        BUFLM5_W { w: self }
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm6(&mut self) -> BUFLM6_W {
        BUFLM6_W { w: self }
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm7(&mut self) -> BUFLM7_W {
        BUFLM7_W { w: self }
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm8(&mut self) -> BUFLM8_W {
        BUFLM8_W { w: self }
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm9(&mut self) -> BUFLM9_W {
        BUFLM9_W { w: self }
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm10(&mut self) -> BUFLM10_W {
        BUFLM10_W { w: self }
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm11(&mut self) -> BUFLM11_W {
        BUFLM11_W { w: self }
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm12(&mut self) -> BUFLM12_W {
        BUFLM12_W { w: self }
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm13(&mut self) -> BUFLM13_W {
        BUFLM13_W { w: self }
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm14(&mut self) -> BUFLM14_W {
        BUFLM14_W { w: self }
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm15(&mut self) -> BUFLM15_W {
        BUFLM15_W { w: self }
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm16(&mut self) -> BUFLM16_W {
        BUFLM16_W { w: self }
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm17(&mut self) -> BUFLM17_W {
        BUFLM17_W { w: self }
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm18(&mut self) -> BUFLM18_W {
        BUFLM18_W { w: self }
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm19(&mut self) -> BUFLM19_W {
        BUFLM19_W { w: self }
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm20(&mut self) -> BUFLM20_W {
        BUFLM20_W { w: self }
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm21(&mut self) -> BUFLM21_W {
        BUFLM21_W { w: self }
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm22(&mut self) -> BUFLM22_W {
        BUFLM22_W { w: self }
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm23(&mut self) -> BUFLM23_W {
        BUFLM23_W { w: self }
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm24(&mut self) -> BUFLM24_W {
        BUFLM24_W { w: self }
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm25(&mut self) -> BUFLM25_W {
        BUFLM25_W { w: self }
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm26(&mut self) -> BUFLM26_W {
        BUFLM26_W { w: self }
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm27(&mut self) -> BUFLM27_W {
        BUFLM27_W { w: self }
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm28(&mut self) -> BUFLM28_W {
        BUFLM28_W { w: self }
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm29(&mut self) -> BUFLM29_W {
        BUFLM29_W { w: self }
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm30(&mut self) -> BUFLM30_W {
        BUFLM30_W { w: self }
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buflm31(&mut self) -> BUFLM31_W {
        BUFLM31_W { w: self }
    }
}
