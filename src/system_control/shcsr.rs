#[doc = "Reader of register SHCSR"]
pub type R = crate::R<u32, super::SHCSR>;
#[doc = "Writer for register SHCSR"]
pub type W = crate::W<u32, super::SHCSR>;
#[doc = "Register SHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<MEMFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTACT_A) -> Self {
        match variant {
            MEMFAULTACT_A::_0 => false,
            MEMFAULTACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MEMFAULTACT`"]
pub type MEMFAULTACT_R = crate::R<bool, MEMFAULTACT_A>;
impl MEMFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTACT_A {
        match self.bits {
            false => MEMFAULTACT_A::_0,
            true => MEMFAULTACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTACT_A::_1
    }
}
#[doc = "Write proxy for field `MEMFAULTACT`"]
pub struct MEMFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<BUSFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTACT_A) -> Self {
        match variant {
            BUSFAULTACT_A::_0 => false,
            BUSFAULTACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUSFAULTACT`"]
pub type BUSFAULTACT_R = crate::R<bool, BUSFAULTACT_A>;
impl BUSFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTACT_A {
        match self.bits {
            false => BUSFAULTACT_A::_0,
            true => BUSFAULTACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTACT_A::_1
    }
}
#[doc = "Write proxy for field `BUSFAULTACT`"]
pub struct BUSFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<USGFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTACT_A) -> Self {
        match variant {
            USGFAULTACT_A::_0 => false,
            USGFAULTACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USGFAULTACT`"]
pub type USGFAULTACT_R = crate::R<bool, USGFAULTACT_A>;
impl USGFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTACT_A {
        match self.bits {
            false => USGFAULTACT_A::_0,
            true => USGFAULTACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTACT_A::_1
    }
}
#[doc = "Write proxy for field `USGFAULTACT`"]
pub struct USGFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<SVCALLACT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLACT_A) -> Self {
        match variant {
            SVCALLACT_A::_0 => false,
            SVCALLACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SVCALLACT`"]
pub type SVCALLACT_R = crate::R<bool, SVCALLACT_A>;
impl SVCALLACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLACT_A {
        match self.bits {
            false => SVCALLACT_A::_0,
            true => SVCALLACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVCALLACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVCALLACT_A::_1
    }
}
#[doc = "Write proxy for field `SVCALLACT`"]
pub struct SVCALLACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCALLACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCALLACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<MONITORACT_A> for bool {
    #[inline(always)]
    fn from(variant: MONITORACT_A) -> Self {
        match variant {
            MONITORACT_A::_0 => false,
            MONITORACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MONITORACT`"]
pub type MONITORACT_R = crate::R<bool, MONITORACT_A>;
impl MONITORACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONITORACT_A {
        match self.bits {
            false => MONITORACT_A::_0,
            true => MONITORACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONITORACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONITORACT_A::_1
    }
}
#[doc = "Write proxy for field `MONITORACT`"]
pub struct MONITORACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONITORACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONITORACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONITORACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONITORACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<PENDSVACT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVACT_A) -> Self {
        match variant {
            PENDSVACT_A::_0 => false,
            PENDSVACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSVACT`"]
pub type PENDSVACT_R = crate::R<bool, PENDSVACT_A>;
impl PENDSVACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVACT_A {
        match self.bits {
            false => PENDSVACT_A::_0,
            true => PENDSVACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PENDSVACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PENDSVACT_A::_1
    }
}
#[doc = "Write proxy for field `PENDSVACT`"]
pub struct PENDSVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACT_A {
    #[doc = "0: exception is not active"]
    _0,
    #[doc = "1: exception is active"]
    _1,
}
impl From<SYSTICKACT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTICKACT_A) -> Self {
        match variant {
            SYSTICKACT_A::_0 => false,
            SYSTICKACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SYSTICKACT`"]
pub type SYSTICKACT_R = crate::R<bool, SYSTICKACT_A>;
impl SYSTICKACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKACT_A {
        match self.bits {
            false => SYSTICKACT_A::_0,
            true => SYSTICKACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYSTICKACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYSTICKACT_A::_1
    }
}
#[doc = "Write proxy for field `SYSTICKACT`"]
pub struct SYSTICKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICKACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTICKACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    _0,
    #[doc = "1: exception is pending"]
    _1,
}
impl From<USGFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTPENDED_A) -> Self {
        match variant {
            USGFAULTPENDED_A::_0 => false,
            USGFAULTPENDED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USGFAULTPENDED`"]
pub type USGFAULTPENDED_R = crate::R<bool, USGFAULTPENDED_A>;
impl USGFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTPENDED_A {
        match self.bits {
            false => USGFAULTPENDED_A::_0,
            true => USGFAULTPENDED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTPENDED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTPENDED_A::_1
    }
}
#[doc = "Write proxy for field `USGFAULTPENDED`"]
pub struct USGFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    _0,
    #[doc = "1: exception is pending"]
    _1,
}
impl From<MEMFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTPENDED_A) -> Self {
        match variant {
            MEMFAULTPENDED_A::_0 => false,
            MEMFAULTPENDED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MEMFAULTPENDED`"]
pub type MEMFAULTPENDED_R = crate::R<bool, MEMFAULTPENDED_A>;
impl MEMFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTPENDED_A {
        match self.bits {
            false => MEMFAULTPENDED_A::_0,
            true => MEMFAULTPENDED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTPENDED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTPENDED_A::_1
    }
}
#[doc = "Write proxy for field `MEMFAULTPENDED`"]
pub struct MEMFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    _0,
    #[doc = "1: exception is pending"]
    _1,
}
impl From<BUSFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTPENDED_A) -> Self {
        match variant {
            BUSFAULTPENDED_A::_0 => false,
            BUSFAULTPENDED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUSFAULTPENDED`"]
pub type BUSFAULTPENDED_R = crate::R<bool, BUSFAULTPENDED_A>;
impl BUSFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTPENDED_A {
        match self.bits {
            false => BUSFAULTPENDED_A::_0,
            true => BUSFAULTPENDED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTPENDED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTPENDED_A::_1
    }
}
#[doc = "Write proxy for field `BUSFAULTPENDED`"]
pub struct BUSFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDED_A {
    #[doc = "0: exception is not pending"]
    _0,
    #[doc = "1: exception is pending"]
    _1,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        match variant {
            SVCALLPENDED_A::_0 => false,
            SVCALLPENDED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SVCALLPENDED`"]
pub type SVCALLPENDED_R = crate::R<bool, SVCALLPENDED_A>;
impl SVCALLPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            false => SVCALLPENDED_A::_0,
            true => SVCALLPENDED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVCALLPENDED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVCALLPENDED_A::_1
    }
}
#[doc = "Write proxy for field `SVCALLPENDED`"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENA_A {
    #[doc = "0: disable the exception"]
    _0,
    #[doc = "1: enable the exception"]
    _1,
}
impl From<MEMFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTENA_A) -> Self {
        match variant {
            MEMFAULTENA_A::_0 => false,
            MEMFAULTENA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MEMFAULTENA`"]
pub type MEMFAULTENA_R = crate::R<bool, MEMFAULTENA_A>;
impl MEMFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTENA_A {
        match self.bits {
            false => MEMFAULTENA_A::_0,
            true => MEMFAULTENA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEMFAULTENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEMFAULTENA_A::_1
    }
}
#[doc = "Write proxy for field `MEMFAULTENA`"]
pub struct MEMFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENA_A {
    #[doc = "0: disable the exception"]
    _0,
    #[doc = "1: enable the exception"]
    _1,
}
impl From<BUSFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTENA_A) -> Self {
        match variant {
            BUSFAULTENA_A::_0 => false,
            BUSFAULTENA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUSFAULTENA`"]
pub type BUSFAULTENA_R = crate::R<bool, BUSFAULTENA_A>;
impl BUSFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTENA_A {
        match self.bits {
            false => BUSFAULTENA_A::_0,
            true => BUSFAULTENA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSFAULTENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSFAULTENA_A::_1
    }
}
#[doc = "Write proxy for field `BUSFAULTENA`"]
pub struct BUSFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENA_A {
    #[doc = "0: disable the exception"]
    _0,
    #[doc = "1: enable the exception"]
    _1,
}
impl From<USGFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTENA_A) -> Self {
        match variant {
            USGFAULTENA_A::_0 => false,
            USGFAULTENA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USGFAULTENA`"]
pub type USGFAULTENA_R = crate::R<bool, USGFAULTENA_A>;
impl USGFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTENA_A {
        match self.bits {
            false => USGFAULTENA_A::_0,
            true => USGFAULTENA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGFAULTENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGFAULTENA_A::_1
    }
}
#[doc = "Write proxy for field `USGFAULTENA`"]
pub struct USGFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W {
        MEMFAULTACT_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W {
        BUSFAULTACT_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W {
        USGFAULTACT_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W {
        SVCALLACT_W { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W {
        MONITORACT_W { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W {
        PENDSVACT_W { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W {
        SYSTICKACT_W { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W {
        USGFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W {
        MEMFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W {
        BUSFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W {
        MEMFAULTENA_W { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W {
        BUSFAULTENA_W { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W {
        USGFAULTENA_W { w: self }
    }
}
