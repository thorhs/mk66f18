#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AD`"]
pub type AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD`"]
pub struct AD_W<'a> {
    w: &'a mut W,
}
impl<'a> AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Range Address Matching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMEN_A {
    #[doc = "0: Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    _0,
    #[doc = "1: Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    _1,
}
impl From<RMEN_A> for bool {
    #[inline(always)]
    fn from(variant: RMEN_A) -> Self {
        match variant {
            RMEN_A::_0 => false,
            RMEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RMEN`"]
pub type RMEN_R = crate::R<bool, RMEN_A>;
impl RMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMEN_A {
        match self.bits {
            false => RMEN_A::_0,
            true => RMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMEN_A::_1
    }
}
#[doc = "Write proxy for field `RMEN`"]
pub struct RMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMEN_A::_0)
    }
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMEN_A::_1)
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
#[doc = "Slave Baud Rate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBRC_A {
    #[doc = "0: The slave baud rate follows the master baud rate and clock stretching may occur"]
    _0,
    #[doc = "1: Slave baud rate is independent of the master baud rate"]
    _1,
}
impl From<SBRC_A> for bool {
    #[inline(always)]
    fn from(variant: SBRC_A) -> Self {
        match variant {
            SBRC_A::_0 => false,
            SBRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SBRC`"]
pub type SBRC_R = crate::R<bool, SBRC_A>;
impl SBRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBRC_A {
        match self.bits {
            false => SBRC_A::_0,
            true => SBRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBRC_A::_1
    }
}
#[doc = "Write proxy for field `SBRC`"]
pub struct SBRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBRC_A::_0)
    }
    #[doc = "Slave baud rate is independent of the master baud rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBRC_A::_1)
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
#[doc = "High Drive Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRS_A {
    #[doc = "0: Normal drive mode"]
    _0,
    #[doc = "1: High drive mode"]
    _1,
}
impl From<HDRS_A> for bool {
    #[inline(always)]
    fn from(variant: HDRS_A) -> Self {
        match variant {
            HDRS_A::_0 => false,
            HDRS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HDRS`"]
pub type HDRS_R = crate::R<bool, HDRS_A>;
impl HDRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRS_A {
        match self.bits {
            false => HDRS_A::_0,
            true => HDRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HDRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HDRS_A::_1
    }
}
#[doc = "Write proxy for field `HDRS`"]
pub struct HDRS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal drive mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRS_A::_0)
    }
    #[doc = "High drive mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Address Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEXT_A {
    #[doc = "0: 7-bit address scheme"]
    _0,
    #[doc = "1: 10-bit address scheme"]
    _1,
}
impl From<ADEXT_A> for bool {
    #[inline(always)]
    fn from(variant: ADEXT_A) -> Self {
        match variant {
            ADEXT_A::_0 => false,
            ADEXT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADEXT`"]
pub type ADEXT_R = crate::R<bool, ADEXT_A>;
impl ADEXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEXT_A {
        match self.bits {
            false => ADEXT_A::_0,
            true => ADEXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEXT_A::_1
    }
}
#[doc = "Write proxy for field `ADEXT`"]
pub struct ADEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit address scheme"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEXT_A::_0)
    }
    #[doc = "10-bit address scheme"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEXT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "General Call Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCAEN_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<GCAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCAEN_A) -> Self {
        match variant {
            GCAEN_A::_0 => false,
            GCAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GCAEN`"]
pub type GCAEN_R = crate::R<bool, GCAEN_A>;
impl GCAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAEN_A {
        match self.bits {
            false => GCAEN_A::_0,
            true => GCAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAEN_A::_1
    }
}
#[doc = "Write proxy for field `GCAEN`"]
pub struct GCAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    pub fn rmen(&self) -> RMEN_R {
        RMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    pub fn sbrc(&self) -> SBRC_R {
        SBRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    pub fn hdrs(&self) -> HDRS_R {
        HDRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    pub fn adext(&self) -> ADEXT_R {
        ADEXT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    pub fn ad(&mut self) -> AD_W {
        AD_W { w: self }
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    pub fn rmen(&mut self) -> RMEN_W {
        RMEN_W { w: self }
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    pub fn sbrc(&mut self) -> SBRC_W {
        SBRC_W { w: self }
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    pub fn hdrs(&mut self) -> HDRS_W {
        HDRS_W { w: self }
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    pub fn adext(&mut self) -> ADEXT_W {
        ADEXT_W { w: self }
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcaen(&mut self) -> GCAEN_W {
        GCAEN_W { w: self }
    }
}
