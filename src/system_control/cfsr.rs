#[doc = "Reader of register CFSR"]
pub type R = crate::R<u32, super::CFSR>;
#[doc = "Writer for register CFSR"]
pub type W = crate::W<u32, super::CFSR>;
#[doc = "Register CFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOL_A {
    #[doc = "0: no instruction access violation fault"]
    _0,
    #[doc = "1: the processor attempted an instruction fetch from a location that does not permit execution"]
    _1,
}
impl From<IACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: IACCVIOL_A) -> Self {
        match variant {
            IACCVIOL_A::_0 => false,
            IACCVIOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IACCVIOL`"]
pub type IACCVIOL_R = crate::R<bool, IACCVIOL_A>;
impl IACCVIOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IACCVIOL_A {
        match self.bits {
            false => IACCVIOL_A::_0,
            true => IACCVIOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IACCVIOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IACCVIOL_A::_1
    }
}
#[doc = "Write proxy for field `IACCVIOL`"]
pub struct IACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IACCVIOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IACCVIOL_A::_0)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IACCVIOL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum DACCVIOL_A {
    #[doc = "0: no data access violation fault"]
    _0,
    #[doc = "1: the processor attempted a load or store at a location that does not permit the operation"]
    _1,
}
impl From<DACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: DACCVIOL_A) -> Self {
        match variant {
            DACCVIOL_A::_0 => false,
            DACCVIOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACCVIOL`"]
pub type DACCVIOL_R = crate::R<bool, DACCVIOL_A>;
impl DACCVIOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCVIOL_A {
        match self.bits {
            false => DACCVIOL_A::_0,
            true => DACCVIOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACCVIOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACCVIOL_A::_1
    }
}
#[doc = "Write proxy for field `DACCVIOL`"]
pub struct DACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACCVIOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACCVIOL_A::_0)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACCVIOL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum MUNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    _0,
    #[doc = "1: unstack for an exception return has caused one or more access violations"]
    _1,
}
impl From<MUNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MUNSTKERR_A) -> Self {
        match variant {
            MUNSTKERR_A::_0 => false,
            MUNSTKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MUNSTKERR`"]
pub type MUNSTKERR_R = crate::R<bool, MUNSTKERR_A>;
impl MUNSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUNSTKERR_A {
        match self.bits {
            false => MUNSTKERR_A::_0,
            true => MUNSTKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MUNSTKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MUNSTKERR_A::_1
    }
}
#[doc = "Write proxy for field `MUNSTKERR`"]
pub struct MUNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MUNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUNSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::_0)
    }
    #[doc = "unstack for an exception return has caused one or more access violations"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum MSTKERR_A {
    #[doc = "0: no stacking fault"]
    _0,
    #[doc = "1: stacking for an exception entry has caused one or more access violations"]
    _1,
}
impl From<MSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTKERR_A) -> Self {
        match variant {
            MSTKERR_A::_0 => false,
            MSTKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MSTKERR`"]
pub type MSTKERR_R = crate::R<bool, MSTKERR_A>;
impl MSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTKERR_A {
        match self.bits {
            false => MSTKERR_A::_0,
            true => MSTKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTKERR_A::_1
    }
}
#[doc = "Write proxy for field `MSTKERR`"]
pub struct MSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTKERR_A::_0)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTKERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERR_A {
    #[doc = "0: No MemManage fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "1: A MemManage fault occurred during floating-point lazy state preservation"]
    _1,
}
impl From<MLSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: MLSPERR_A) -> Self {
        match variant {
            MLSPERR_A::_0 => false,
            MLSPERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MLSPERR`"]
pub type MLSPERR_R = crate::R<bool, MLSPERR_A>;
impl MLSPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MLSPERR_A {
        match self.bits {
            false => MLSPERR_A::_0,
            true => MLSPERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MLSPERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MLSPERR_A::_1
    }
}
#[doc = "Write proxy for field `MLSPERR`"]
pub struct MLSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MLSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLSPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MLSPERR_A::_0)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MLSPERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALID_A {
    #[doc = "0: value in MMAR is not a valid fault address"]
    _0,
    #[doc = "1: MMAR holds a valid fault address"]
    _1,
}
impl From<MMARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: MMARVALID_A) -> Self {
        match variant {
            MMARVALID_A::_0 => false,
            MMARVALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MMARVALID`"]
pub type MMARVALID_R = crate::R<bool, MMARVALID_A>;
impl MMARVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMARVALID_A {
        match self.bits {
            false => MMARVALID_A::_0,
            true => MMARVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMARVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMARVALID_A::_1
    }
}
#[doc = "Write proxy for field `MMARVALID`"]
pub struct MMARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> MMARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMARVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMARVALID_A::_0)
    }
    #[doc = "MMAR holds a valid fault address"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMARVALID_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum IBUSERR_A {
    #[doc = "0: no instruction bus error"]
    _0,
    #[doc = "1: instruction bus error"]
    _1,
}
impl From<IBUSERR_A> for bool {
    #[inline(always)]
    fn from(variant: IBUSERR_A) -> Self {
        match variant {
            IBUSERR_A::_0 => false,
            IBUSERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IBUSERR`"]
pub type IBUSERR_R = crate::R<bool, IBUSERR_A>;
impl IBUSERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBUSERR_A {
        match self.bits {
            false => IBUSERR_A::_0,
            true => IBUSERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IBUSERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IBUSERR_A::_1
    }
}
#[doc = "Write proxy for field `IBUSERR`"]
pub struct IBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBUSERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBUSERR_A::_0)
    }
    #[doc = "instruction bus error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBUSERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum PRECISERR_A {
    #[doc = "0: no precise data bus error"]
    _0,
    #[doc = "1: a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    _1,
}
impl From<PRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRECISERR_A) -> Self {
        match variant {
            PRECISERR_A::_0 => false,
            PRECISERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PRECISERR`"]
pub type PRECISERR_R = crate::R<bool, PRECISERR_A>;
impl PRECISERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRECISERR_A {
        match self.bits {
            false => PRECISERR_A::_0,
            true => PRECISERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRECISERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRECISERR_A::_1
    }
}
#[doc = "Write proxy for field `PRECISERR`"]
pub struct PRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRECISERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRECISERR_A::_0)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRECISERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERR_A {
    #[doc = "0: no imprecise data bus error"]
    _0,
    #[doc = "1: a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    _1,
}
impl From<IMPRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: IMPRECISERR_A) -> Self {
        match variant {
            IMPRECISERR_A::_0 => false,
            IMPRECISERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IMPRECISERR`"]
pub type IMPRECISERR_R = crate::R<bool, IMPRECISERR_A>;
impl IMPRECISERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMPRECISERR_A {
        match self.bits {
            false => IMPRECISERR_A::_0,
            true => IMPRECISERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IMPRECISERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IMPRECISERR_A::_1
    }
}
#[doc = "Write proxy for field `IMPRECISERR`"]
pub struct IMPRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMPRECISERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::_0)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum UNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    _0,
    #[doc = "1: unstack for an exception return has caused one or more BusFaults"]
    _1,
}
impl From<UNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: UNSTKERR_A) -> Self {
        match variant {
            UNSTKERR_A::_0 => false,
            UNSTKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UNSTKERR`"]
pub type UNSTKERR_R = crate::R<bool, UNSTKERR_A>;
impl UNSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNSTKERR_A {
        match self.bits {
            false => UNSTKERR_A::_0,
            true => UNSTKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNSTKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNSTKERR_A::_1
    }
}
#[doc = "Write proxy for field `UNSTKERR`"]
pub struct UNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNSTKERR_A::_0)
    }
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNSTKERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum STKERR_A {
    #[doc = "0: no stacking fault"]
    _0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults"]
    _1,
}
impl From<STKERR_A> for bool {
    #[inline(always)]
    fn from(variant: STKERR_A) -> Self {
        match variant {
            STKERR_A::_0 => false,
            STKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STKERR`"]
pub type STKERR_R = crate::R<bool, STKERR_A>;
impl STKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKERR_A {
        match self.bits {
            false => STKERR_A::_0,
            true => STKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STKERR_A::_1
    }
}
#[doc = "Write proxy for field `STKERR`"]
pub struct STKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STKERR_A::_0)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STKERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum LSPERR_A {
    #[doc = "0: No bus fault occurred during floating-point lazy state preservation"]
    _0,
    #[doc = "1: A bus fault occurred during floating-point lazy state preservation"]
    _1,
}
impl From<LSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSPERR_A) -> Self {
        match variant {
            LSPERR_A::_0 => false,
            LSPERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LSPERR`"]
pub type LSPERR_R = crate::R<bool, LSPERR_A>;
impl LSPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPERR_A {
        match self.bits {
            false => LSPERR_A::_0,
            true => LSPERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSPERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSPERR_A::_1
    }
}
#[doc = "Write proxy for field `LSPERR`"]
pub struct LSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSPERR_A::_0)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSPERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum BFARVALID_A {
    #[doc = "0: value in BFAR is not a valid fault address"]
    _0,
    #[doc = "1: BFAR holds a valid fault address"]
    _1,
}
impl From<BFARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: BFARVALID_A) -> Self {
        match variant {
            BFARVALID_A::_0 => false,
            BFARVALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BFARVALID`"]
pub type BFARVALID_R = crate::R<bool, BFARVALID_A>;
impl BFARVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFARVALID_A {
        match self.bits {
            false => BFARVALID_A::_0,
            true => BFARVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFARVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFARVALID_A::_1
    }
}
#[doc = "Write proxy for field `BFARVALID`"]
pub struct BFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> BFARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFARVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFARVALID_A::_0)
    }
    #[doc = "BFAR holds a valid fault address"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFARVALID_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum UNDEFINSTR_A {
    #[doc = "0: no undefined instruction UsageFault"]
    _0,
    #[doc = "1: the processor has attempted to execute an undefined instruction"]
    _1,
}
impl From<UNDEFINSTR_A> for bool {
    #[inline(always)]
    fn from(variant: UNDEFINSTR_A) -> Self {
        match variant {
            UNDEFINSTR_A::_0 => false,
            UNDEFINSTR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UNDEFINSTR`"]
pub type UNDEFINSTR_R = crate::R<bool, UNDEFINSTR_A>;
impl UNDEFINSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDEFINSTR_A {
        match self.bits {
            false => UNDEFINSTR_A::_0,
            true => UNDEFINSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDEFINSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDEFINSTR_A::_1
    }
}
#[doc = "Write proxy for field `UNDEFINSTR`"]
pub struct UNDEFINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDEFINSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDEFINSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::_0)
    }
    #[doc = "the processor has attempted to execute an undefined instruction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum INVSTATE_A {
    #[doc = "0: no invalid state UsageFault"]
    _0,
    #[doc = "1: the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    _1,
}
impl From<INVSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: INVSTATE_A) -> Self {
        match variant {
            INVSTATE_A::_0 => false,
            INVSTATE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INVSTATE`"]
pub type INVSTATE_R = crate::R<bool, INVSTATE_A>;
impl INVSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVSTATE_A {
        match self.bits {
            false => INVSTATE_A::_0,
            true => INVSTATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVSTATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVSTATE_A::_1
    }
}
#[doc = "Write proxy for field `INVSTATE`"]
pub struct INVSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVSTATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVSTATE_A::_0)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVSTATE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub enum INVPC_A {
    #[doc = "0: no invalid PC load UsageFault"]
    _0,
    #[doc = "1: the processor has attempted an illegal load of EXC_RETURN to the PC"]
    _1,
}
impl From<INVPC_A> for bool {
    #[inline(always)]
    fn from(variant: INVPC_A) -> Self {
        match variant {
            INVPC_A::_0 => false,
            INVPC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INVPC`"]
pub type INVPC_R = crate::R<bool, INVPC_A>;
impl INVPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVPC_A {
        match self.bits {
            false => INVPC_A::_0,
            true => INVPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVPC_A::_1
    }
}
#[doc = "Write proxy for field `INVPC`"]
pub struct INVPC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVPC_A::_0)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVPC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCP_A {
    #[doc = "0: no UsageFault caused by attempting to access a coprocessor"]
    _0,
    #[doc = "1: the processor has attempted to access a coprocessor"]
    _1,
}
impl From<NOCP_A> for bool {
    #[inline(always)]
    fn from(variant: NOCP_A) -> Self {
        match variant {
            NOCP_A::_0 => false,
            NOCP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NOCP`"]
pub type NOCP_R = crate::R<bool, NOCP_A>;
impl NOCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOCP_A {
        match self.bits {
            false => NOCP_A::_0,
            true => NOCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOCP_A::_1
    }
}
#[doc = "Write proxy for field `NOCP`"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOCP_A::_0)
    }
    #[doc = "the processor has attempted to access a coprocessor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOCP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNED_A {
    #[doc = "0: no unaligned access fault, or unaligned access trapping not enabled"]
    _0,
    #[doc = "1: the processor has made an unaligned memory access"]
    _1,
}
impl From<UNALIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGNED_A) -> Self {
        match variant {
            UNALIGNED_A::_0 => false,
            UNALIGNED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UNALIGNED`"]
pub type UNALIGNED_R = crate::R<bool, UNALIGNED_A>;
impl UNALIGNED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGNED_A {
        match self.bits {
            false => UNALIGNED_A::_0,
            true => UNALIGNED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNALIGNED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNALIGNED_A::_1
    }
}
#[doc = "Write proxy for field `UNALIGNED`"]
pub struct UNALIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGNED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UNALIGNED_A::_0)
    }
    #[doc = "the processor has made an unaligned memory access"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UNALIGNED_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZERO_A {
    #[doc = "0: no divide by zero fault, or divide by zero trapping not enabled"]
    _0,
    #[doc = "1: the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    _1,
}
impl From<DIVBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVBYZERO_A) -> Self {
        match variant {
            DIVBYZERO_A::_0 => false,
            DIVBYZERO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DIVBYZERO`"]
pub type DIVBYZERO_R = crate::R<bool, DIVBYZERO_A>;
impl DIVBYZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVBYZERO_A {
        match self.bits {
            false => DIVBYZERO_A::_0,
            true => DIVBYZERO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIVBYZERO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVBYZERO_A::_1
    }
}
#[doc = "Write proxy for field `DIVBYZERO`"]
pub struct DIVBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBYZERO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::_0)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> IACCVIOL_W {
        IACCVIOL_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> DACCVIOL_W {
        DACCVIOL_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W {
        MUNSTKERR_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> MSTKERR_W {
        MSTKERR_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn mlsperr(&mut self) -> MLSPERR_W {
        MLSPERR_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> MMARVALID_W {
        MMARVALID_W { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> IBUSERR_W {
        IBUSERR_W { w: self }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> PRECISERR_W {
        PRECISERR_W { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W {
        IMPRECISERR_W { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> UNSTKERR_W {
        UNSTKERR_W { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> STKERR_W {
        STKERR_W { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LSPERR_W {
        LSPERR_W { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> BFARVALID_W {
        BFARVALID_W { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W {
        UNDEFINSTR_W { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn invstate(&mut self) -> INVSTATE_W {
        INVSTATE_W { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn invpc(&mut self) -> INVPC_W {
        INVPC_W { w: self }
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> UNALIGNED_W {
        UNALIGNED_W { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W {
        DIVBYZERO_W { w: self }
    }
}
