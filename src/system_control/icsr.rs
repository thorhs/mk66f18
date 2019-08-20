#[doc = "Reader of register ICSR"]
pub type R = crate::R<u32, super::ICSR>;
#[doc = "Writer for register ICSR"]
pub type W = crate::W<u32, super::ICSR>;
#[doc = "Register ICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VECTACTIVE`"]
pub type VECTACTIVE_R = crate::R<u16, u16>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASE_A {
    #[doc = "0: there are preempted active exceptions to execute"]
    _0,
    #[doc = "1: there are no active exceptions, or the currently-executing exception is the only active exception"]
    _1,
}
impl From<RETTOBASE_A> for bool {
    #[inline(always)]
    fn from(variant: RETTOBASE_A) -> Self {
        match variant {
            RETTOBASE_A::_0 => false,
            RETTOBASE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RETTOBASE`"]
pub type RETTOBASE_R = crate::R<bool, RETTOBASE_A>;
impl RETTOBASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETTOBASE_A {
        match self.bits {
            false => RETTOBASE_A::_0,
            true => RETTOBASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RETTOBASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RETTOBASE_A::_1
    }
}
#[doc = "Reader of field `VECTPENDING`"]
pub type VECTPENDING_R = crate::R<u8, u8>;
#[doc = "Reader of field `ISRPENDING`"]
pub type ISRPENDING_R = crate::R<bool, bool>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPREEMPT_A {
    #[doc = "0: Will not service"]
    _0,
    #[doc = "1: Will service a pending exception"]
    _1,
}
impl From<ISRPREEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPREEMPT_A) -> Self {
        match variant {
            ISRPREEMPT_A::_0 => false,
            ISRPREEMPT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISRPREEMPT`"]
pub type ISRPREEMPT_R = crate::R<bool, ISRPREEMPT_A>;
impl ISRPREEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPREEMPT_A {
        match self.bits {
            false => ISRPREEMPT_A::_0,
            true => ISRPREEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISRPREEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISRPREEMPT_A::_1
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_AW {
    #[doc = "0: no effect"]
    _0,
    #[doc = "1: removes the pending state from the SysTick exception"]
    _1,
}
impl From<PENDSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_AW) -> Self {
        match variant {
            PENDSTCLR_AW::_0 => false,
            PENDSTCLR_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PENDSTCLR`"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: write: no effect; read: SysTick exception is not pending"]
    _0,
    #[doc = "1: write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    _1,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        match variant {
            PENDSTSET_A::_0 => false,
            PENDSTSET_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSTSET`"]
pub type PENDSTSET_R = crate::R<bool, PENDSTSET_A>;
impl PENDSTSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::_0,
            true => PENDSTSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PENDSTSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PENDSTSET_A::_1
    }
}
#[doc = "Write proxy for field `PENDSTSET`"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_AW {
    #[doc = "0: no effect"]
    _0,
    #[doc = "1: removes the pending state from the PendSV exception"]
    _1,
}
impl From<PENDSVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_AW) -> Self {
        match variant {
            PENDSVCLR_AW::_0 => false,
            PENDSVCLR_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PENDSVCLR`"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: write: no effect; read: PendSV exception is not pending"]
    _0,
    #[doc = "1: write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    _1,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        match variant {
            PENDSVSET_A::_0 => false,
            PENDSVSET_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PENDSVSET`"]
pub type PENDSVSET_R = crate::R<bool, PENDSVSET_A>;
impl PENDSVSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::_0,
            true => PENDSVSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PENDSVSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PENDSVSET_A::_1
    }
}
#[doc = "Write proxy for field `PENDSVSET`"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSET_A {
    #[doc = "0: write: no effect; read: NMI exception is not pending"]
    _0,
    #[doc = "1: write: changes NMI exception state to pending; read: NMI exception is pending"]
    _1,
}
impl From<NMIPENDSET_A> for bool {
    #[inline(always)]
    fn from(variant: NMIPENDSET_A) -> Self {
        match variant {
            NMIPENDSET_A::_0 => false,
            NMIPENDSET_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NMIPENDSET`"]
pub type NMIPENDSET_R = crate::R<bool, NMIPENDSET_A>;
impl NMIPENDSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIPENDSET_A {
        match self.bits {
            false => NMIPENDSET_A::_0,
            true => NMIPENDSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIPENDSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIPENDSET_A::_1
    }
}
#[doc = "Write proxy for field `NMIPENDSET`"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIPENDSET_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
}
