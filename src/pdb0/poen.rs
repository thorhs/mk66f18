#[doc = "Reader of register POEN"]
pub type R = crate::R<u32, super::POEN>;
#[doc = "Writer for register POEN"]
pub type W = crate::W<u32, super::POEN>;
#[doc = "Register POEN `reset()`'s with value 0"]
impl crate::ResetValue for super::POEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN0_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN0_A> for bool {
    #[inline(always)]
    fn from(variant: POEN0_A) -> Self {
        match variant {
            POEN0_A::_0 => false,
            POEN0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN0`"]
pub type POEN0_R = crate::R<bool, POEN0_A>;
impl POEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN0_A {
        match self.bits {
            false => POEN0_A::_0,
            true => POEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN0_A::_1
    }
}
#[doc = "Write proxy for field `POEN0`"]
pub struct POEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN0_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN0_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN1_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN1_A> for bool {
    #[inline(always)]
    fn from(variant: POEN1_A) -> Self {
        match variant {
            POEN1_A::_0 => false,
            POEN1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN1`"]
pub type POEN1_R = crate::R<bool, POEN1_A>;
impl POEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN1_A {
        match self.bits {
            false => POEN1_A::_0,
            true => POEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN1_A::_1
    }
}
#[doc = "Write proxy for field `POEN1`"]
pub struct POEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN1_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN1_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN2_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN2_A> for bool {
    #[inline(always)]
    fn from(variant: POEN2_A) -> Self {
        match variant {
            POEN2_A::_0 => false,
            POEN2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN2`"]
pub type POEN2_R = crate::R<bool, POEN2_A>;
impl POEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN2_A {
        match self.bits {
            false => POEN2_A::_0,
            true => POEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN2_A::_1
    }
}
#[doc = "Write proxy for field `POEN2`"]
pub struct POEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN2_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN2_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN3_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN3_A> for bool {
    #[inline(always)]
    fn from(variant: POEN3_A) -> Self {
        match variant {
            POEN3_A::_0 => false,
            POEN3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN3`"]
pub type POEN3_R = crate::R<bool, POEN3_A>;
impl POEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN3_A {
        match self.bits {
            false => POEN3_A::_0,
            true => POEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN3_A::_1
    }
}
#[doc = "Write proxy for field `POEN3`"]
pub struct POEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN3_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN3_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN4_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN4_A> for bool {
    #[inline(always)]
    fn from(variant: POEN4_A) -> Self {
        match variant {
            POEN4_A::_0 => false,
            POEN4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN4`"]
pub type POEN4_R = crate::R<bool, POEN4_A>;
impl POEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN4_A {
        match self.bits {
            false => POEN4_A::_0,
            true => POEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN4_A::_1
    }
}
#[doc = "Write proxy for field `POEN4`"]
pub struct POEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN4_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN4_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN5_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN5_A> for bool {
    #[inline(always)]
    fn from(variant: POEN5_A) -> Self {
        match variant {
            POEN5_A::_0 => false,
            POEN5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN5`"]
pub type POEN5_R = crate::R<bool, POEN5_A>;
impl POEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN5_A {
        match self.bits {
            false => POEN5_A::_0,
            true => POEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN5_A::_1
    }
}
#[doc = "Write proxy for field `POEN5`"]
pub struct POEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN5_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN5_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN6_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN6_A> for bool {
    #[inline(always)]
    fn from(variant: POEN6_A) -> Self {
        match variant {
            POEN6_A::_0 => false,
            POEN6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN6`"]
pub type POEN6_R = crate::R<bool, POEN6_A>;
impl POEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN6_A {
        match self.bits {
            false => POEN6_A::_0,
            true => POEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN6_A::_1
    }
}
#[doc = "Write proxy for field `POEN6`"]
pub struct POEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN6_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN6_A::_1)
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
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN7_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1,
}
impl From<POEN7_A> for bool {
    #[inline(always)]
    fn from(variant: POEN7_A) -> Self {
        match variant {
            POEN7_A::_0 => false,
            POEN7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POEN7`"]
pub type POEN7_R = crate::R<bool, POEN7_A>;
impl POEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN7_A {
        match self.bits {
            false => POEN7_A::_0,
            true => POEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN7_A::_1
    }
}
#[doc = "Write proxy for field `POEN7`"]
pub struct POEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN7_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN7_A::_1)
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
impl R {
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen0(&self) -> POEN0_R {
        POEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen1(&self) -> POEN1_R {
        POEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen2(&self) -> POEN2_R {
        POEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen3(&self) -> POEN3_R {
        POEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen4(&self) -> POEN4_R {
        POEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen5(&self) -> POEN5_R {
        POEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen6(&self) -> POEN6_R {
        POEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen7(&self) -> POEN7_R {
        POEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen0(&mut self) -> POEN0_W {
        POEN0_W { w: self }
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen1(&mut self) -> POEN1_W {
        POEN1_W { w: self }
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen2(&mut self) -> POEN2_W {
        POEN2_W { w: self }
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen3(&mut self) -> POEN3_W {
        POEN3_W { w: self }
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen4(&mut self) -> POEN4_W {
        POEN4_W { w: self }
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen5(&mut self) -> POEN5_W {
        POEN5_W { w: self }
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen6(&mut self) -> POEN6_W {
        POEN6_W { w: self }
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen7(&mut self) -> POEN7_W {
        POEN7_W { w: self }
    }
}
