#[doc = "Reader of register C7816"]
pub type R = crate::R<u8, super::C7816>;
#[doc = "Writer for register C7816"]
pub type W = crate::W<u8, super::C7816>;
#[doc = "Register C7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::C7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ISO-7816 Functionality Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISO_7816E_A {
    #[doc = "0: ISO-7816 functionality is turned off/not enabled."]
    _0,
    #[doc = "1: ISO-7816 functionality is turned on/enabled."]
    _1,
}
impl From<ISO_7816E_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_7816E_A) -> Self {
        match variant {
            ISO_7816E_A::_0 => false,
            ISO_7816E_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISO_7816E`"]
pub type ISO_7816E_R = crate::R<bool, ISO_7816E_A>;
impl ISO_7816E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_7816E_A {
        match self.bits {
            false => ISO_7816E_A::_0,
            true => ISO_7816E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISO_7816E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISO_7816E_A::_1
    }
}
#[doc = "Write proxy for field `ISO_7816E`"]
pub struct ISO_7816E_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_7816E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO_7816E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISO_7816E_A::_0)
    }
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISO_7816E_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTYPE_A {
    #[doc = "0: T = 0 per the ISO-7816 specification."]
    _0,
    #[doc = "1: T = 1 per the ISO-7816 specification."]
    _1,
}
impl From<TTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TTYPE_A) -> Self {
        match variant {
            TTYPE_A::_0 => false,
            TTYPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TTYPE`"]
pub type TTYPE_R = crate::R<bool, TTYPE_A>;
impl TTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTYPE_A {
        match self.bits {
            false => TTYPE_A::_0,
            true => TTYPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TTYPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TTYPE_A::_1
    }
}
#[doc = "Write proxy for field `TTYPE`"]
pub struct TTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "T = 0 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTYPE_A::_0)
    }
    #[doc = "T = 1 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTYPE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Detect Initial Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal operating mode. Receiver does not seek to identify initial character."]
    _0,
    #[doc = "1: Receiver searches for initial character."]
    _1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        match variant {
            INIT_A::_0 => false,
            INIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::_0,
            true => INIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIT_A::_1
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INIT_A::_0)
    }
    #[doc = "Receiver searches for initial character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Generate NACK on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACK_A {
    #[doc = "0: No NACK is automatically generated."]
    _0,
    #[doc = "1: A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    _1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        match variant {
            ANACK_A::_0 => false,
            ANACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ANACK`"]
pub type ANACK_R = crate::R<bool, ANACK_A>;
impl ANACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::_0,
            true => ANACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANACK_A::_1
    }
}
#[doc = "Write proxy for field `ANACK`"]
pub struct ANACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No NACK is automatically generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANACK_A::_0)
    }
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANACK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Generate NACK on Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONACK_A {
    #[doc = "0: The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    _0,
    #[doc = "1: If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    _1,
}
impl From<ONACK_A> for bool {
    #[inline(always)]
    fn from(variant: ONACK_A) -> Self {
        match variant {
            ONACK_A::_0 => false,
            ONACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ONACK`"]
pub type ONACK_R = crate::R<bool, ONACK_A>;
impl ONACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONACK_A {
        match self.bits {
            false => ONACK_A::_0,
            true => ONACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONACK_A::_1
    }
}
#[doc = "Write proxy for field `ONACK`"]
pub struct ONACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ONACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONACK_A::_0)
    }
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONACK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    pub fn iso_7816e(&self) -> ISO_7816E_R {
        ISO_7816E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    pub fn onack(&self) -> ONACK_R {
        ONACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    pub fn iso_7816e(&mut self) -> ISO_7816E_W {
        ISO_7816E_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    pub fn ttype(&mut self) -> TTYPE_W {
        TTYPE_W { w: self }
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    pub fn anack(&mut self) -> ANACK_W {
        ANACK_W { w: self }
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    pub fn onack(&mut self) -> ONACK_W {
        ONACK_W { w: self }
    }
}
