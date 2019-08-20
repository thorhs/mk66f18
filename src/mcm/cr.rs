#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SRAM_U arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMUAP_A {
    #[doc = "0: Round robin"]
    _00,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl From<SRAMUAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMUAP_A) -> Self {
        match variant {
            SRAMUAP_A::_00 => 0,
            SRAMUAP_A::_01 => 1,
            SRAMUAP_A::_10 => 2,
            SRAMUAP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `SRAMUAP`"]
pub type SRAMUAP_R = crate::R<u8, SRAMUAP_A>;
impl SRAMUAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMUAP_A {
        match self.bits {
            0 => SRAMUAP_A::_00,
            1 => SRAMUAP_A::_01,
            2 => SRAMUAP_A::_10,
            3 => SRAMUAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMUAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMUAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMUAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMUAP_A::_11
    }
}
#[doc = "Write proxy for field `SRAMUAP`"]
pub struct SRAMUAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMUAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SRAMUWP`"]
pub type SRAMUWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMUWP`"]
pub struct SRAMUWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMUWP_W<'a> {
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
#[doc = "SRAM_L arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMLAP_A {
    #[doc = "0: Round robin"]
    _00,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl From<SRAMLAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMLAP_A) -> Self {
        match variant {
            SRAMLAP_A::_00 => 0,
            SRAMLAP_A::_01 => 1,
            SRAMLAP_A::_10 => 2,
            SRAMLAP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `SRAMLAP`"]
pub type SRAMLAP_R = crate::R<u8, SRAMLAP_A>;
impl SRAMLAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMLAP_A {
        match self.bits {
            0 => SRAMLAP_A::_00,
            1 => SRAMLAP_A::_01,
            2 => SRAMLAP_A::_10,
            3 => SRAMLAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMLAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMLAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMLAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMLAP_A::_11
    }
}
#[doc = "Write proxy for field `SRAMLAP`"]
pub struct SRAMLAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMLAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SRAMLWP`"]
pub type SRAMLWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMLWP`"]
pub struct SRAMLWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLWP_W<'a> {
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
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    pub fn sramuap(&self) -> SRAMUAP_R {
        SRAMUAP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    pub fn sramuwp(&self) -> SRAMUWP_R {
        SRAMUWP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    pub fn sramlap(&self) -> SRAMLAP_R {
        SRAMLAP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&self) -> SRAMLWP_R {
        SRAMLWP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    pub fn sramuap(&mut self) -> SRAMUAP_W {
        SRAMUAP_W { w: self }
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    pub fn sramuwp(&mut self) -> SRAMUWP_W {
        SRAMUWP_W { w: self }
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    pub fn sramlap(&mut self) -> SRAMLAP_W {
        SRAMLAP_W { w: self }
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&mut self) -> SRAMLWP_W {
        SRAMLWP_W { w: self }
    }
}
