#[doc = "Reader of register GENCS"]
pub type R = crate::R<u32, super::GENCS>;
#[doc = "Writer for register GENCS"]
pub type W = crate::W<u32, super::GENCS>;
#[doc = "Register GENCS `reset()`'s with value 0"]
impl crate::ResetValue for super::GENCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "End-of-Scan DMA Transfer Request Enable Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSDMEO_A {
    #[doc = "0: Do not enable the End-of-Scan DMA transfer request only. Depending on ESOR state, either Out-of-Range or End-of-Scan can trigger a DMA transfer request and interrupt."]
    _0,
    #[doc = "1: Only the End-of-Scan event can trigger a DMA transfer request. The Out-of-Range event only and always triggers an interrupt if TSIIE is set."]
    _1,
}
impl From<EOSDMEO_A> for bool {
    #[inline(always)]
    fn from(variant: EOSDMEO_A) -> Self {
        match variant {
            EOSDMEO_A::_0 => false,
            EOSDMEO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOSDMEO`"]
pub type EOSDMEO_R = crate::R<bool, EOSDMEO_A>;
impl EOSDMEO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSDMEO_A {
        match self.bits {
            false => EOSDMEO_A::_0,
            true => EOSDMEO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSDMEO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSDMEO_A::_1
    }
}
#[doc = "Write proxy for field `EOSDMEO`"]
pub struct EOSDMEO_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSDMEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOSDMEO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not enable the End-of-Scan DMA transfer request only. Depending on ESOR state, either Out-of-Range or End-of-Scan can trigger a DMA transfer request and interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSDMEO_A::_0)
    }
    #[doc = "Only the End-of-Scan event can trigger a DMA transfer request. The Out-of-Range event only and always triggers an interrupt if TSIIE is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSDMEO_A::_1)
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
#[doc = "CURSW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURSW_A {
    #[doc = "0: The current source pair are not swapped."]
    _0,
    #[doc = "1: The current source pair are swapped."]
    _1,
}
impl From<CURSW_A> for bool {
    #[inline(always)]
    fn from(variant: CURSW_A) -> Self {
        match variant {
            CURSW_A::_0 => false,
            CURSW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CURSW`"]
pub type CURSW_R = crate::R<bool, CURSW_A>;
impl CURSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURSW_A {
        match self.bits {
            false => CURSW_A::_0,
            true => CURSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CURSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CURSW_A::_1
    }
}
#[doc = "Write proxy for field `CURSW`"]
pub struct CURSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CURSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The current source pair are not swapped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CURSW_A::_0)
    }
    #[doc = "The current source pair are swapped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CURSW_A::_1)
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
#[doc = "End of Scan Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSF_A {
    #[doc = "0: Scan not complete."]
    _0,
    #[doc = "1: Scan complete."]
    _1,
}
impl From<EOSF_A> for bool {
    #[inline(always)]
    fn from(variant: EOSF_A) -> Self {
        match variant {
            EOSF_A::_0 => false,
            EOSF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOSF`"]
pub type EOSF_R = crate::R<bool, EOSF_A>;
impl EOSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSF_A {
        match self.bits {
            false => EOSF_A::_0,
            true => EOSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSF_A::_1
    }
}
#[doc = "Write proxy for field `EOSF`"]
pub struct EOSF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Scan not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSF_A::_0)
    }
    #[doc = "Scan complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSF_A::_1)
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
#[doc = "Scan In Progress Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCNIP_A {
    #[doc = "0: No scan in progress."]
    _0,
    #[doc = "1: Scan in progress."]
    _1,
}
impl From<SCNIP_A> for bool {
    #[inline(always)]
    fn from(variant: SCNIP_A) -> Self {
        match variant {
            SCNIP_A::_0 => false,
            SCNIP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SCNIP`"]
pub type SCNIP_R = crate::R<bool, SCNIP_A>;
impl SCNIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCNIP_A {
        match self.bits {
            false => SCNIP_A::_0,
            true => SCNIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCNIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCNIP_A::_1
    }
}
#[doc = "Scan Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STM_A {
    #[doc = "0: Software trigger scan."]
    _0,
    #[doc = "1: Hardware trigger scan."]
    _1,
}
impl From<STM_A> for bool {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        match variant {
            STM_A::_0 => false,
            STM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<bool, STM_A>;
impl STM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STM_A {
        match self.bits {
            false => STM_A::_0,
            true => STM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STM_A::_1
    }
}
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger scan."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STM_A::_0)
    }
    #[doc = "Hardware trigger scan."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STM_A::_1)
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
#[doc = "TSI STOP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPE_A {
    #[doc = "0: TSI is disabled when MCU goes into low power mode."]
    _0,
    #[doc = "1: Allows TSI to continue running in all low power modes."]
    _1,
}
impl From<STPE_A> for bool {
    #[inline(always)]
    fn from(variant: STPE_A) -> Self {
        match variant {
            STPE_A::_0 => false,
            STPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STPE`"]
pub type STPE_R = crate::R<bool, STPE_A>;
impl STPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPE_A {
        match self.bits {
            false => STPE_A::_0,
            true => STPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STPE_A::_1
    }
}
#[doc = "Write proxy for field `STPE`"]
pub struct STPE_W<'a> {
    w: &'a mut W,
}
impl<'a> STPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TSI is disabled when MCU goes into low power mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPE_A::_0)
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPE_A::_1)
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
#[doc = "Touch Sensing Input Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIIEN_A {
    #[doc = "0: TSI interrupt is disabled."]
    _0,
    #[doc = "1: TSI interrupt is enabled."]
    _1,
}
impl From<TSIIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSIIEN_A) -> Self {
        match variant {
            TSIIEN_A::_0 => false,
            TSIIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TSIIEN`"]
pub type TSIIEN_R = crate::R<bool, TSIIEN_A>;
impl TSIIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIIEN_A {
        match self.bits {
            false => TSIIEN_A::_0,
            true => TSIIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIIEN_A::_1
    }
}
#[doc = "Write proxy for field `TSIIEN`"]
pub struct TSIIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TSI interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIIEN_A::_0)
    }
    #[doc = "TSI interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIIEN_A::_1)
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
#[doc = "Touch Sensing Input Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIEN_A {
    #[doc = "0: TSI module disabled."]
    _0,
    #[doc = "1: TSI module enabled."]
    _1,
}
impl From<TSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSIEN_A) -> Self {
        match variant {
            TSIEN_A::_0 => false,
            TSIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TSIEN`"]
pub type TSIEN_R = crate::R<bool, TSIEN_A>;
impl TSIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIEN_A {
        match self.bits {
            false => TSIEN_A::_0,
            true => TSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIEN_A::_1
    }
}
#[doc = "Write proxy for field `TSIEN`"]
pub struct TSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TSI module disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIEN_A::_0)
    }
    #[doc = "TSI module enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIEN_A::_1)
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
#[doc = "NSCN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSCN_A {
    #[doc = "0: Once per electrode"]
    _00000,
    #[doc = "1: Twice per electrode"]
    _00001,
    #[doc = "2: 3 times per electrode"]
    _00010,
    #[doc = "3: 4 times per electrode"]
    _00011,
    #[doc = "4: 5 times per electrode"]
    _00100,
    #[doc = "5: 6 times per electrode"]
    _00101,
    #[doc = "6: 7 times per electrode"]
    _00110,
    #[doc = "7: 8 times per electrode"]
    _00111,
    #[doc = "8: 9 times per electrode"]
    _01000,
    #[doc = "9: 10 times per electrode"]
    _01001,
    #[doc = "10: 11 times per electrode"]
    _01010,
    #[doc = "11: 12 times per electrode"]
    _01011,
    #[doc = "12: 13 times per electrode"]
    _01100,
    #[doc = "13: 14 times per electrode"]
    _01101,
    #[doc = "14: 15 times per electrode"]
    _01110,
    #[doc = "15: 16 times per electrode"]
    _01111,
    #[doc = "16: 17 times per electrode"]
    _10000,
    #[doc = "17: 18 times per electrode"]
    _10001,
    #[doc = "18: 19 times per electrode"]
    _10010,
    #[doc = "19: 20 times per electrode"]
    _10011,
    #[doc = "20: 21 times per electrode"]
    _10100,
    #[doc = "21: 22 times per electrode"]
    _10101,
    #[doc = "22: 23 times per electrode"]
    _10110,
    #[doc = "23: 24 times per electrode"]
    _10111,
    #[doc = "24: 25 times per electrode"]
    _11000,
    #[doc = "25: 26 times per electrode"]
    _11001,
    #[doc = "26: 27 times per electrode"]
    _11010,
    #[doc = "27: 28 times per electrode"]
    _11011,
    #[doc = "28: 29 times per electrode"]
    _11100,
    #[doc = "29: 30 times per electrode"]
    _11101,
    #[doc = "30: 31 times per electrode"]
    _11110,
    #[doc = "31: 32 times per electrode"]
    _11111,
}
impl From<NSCN_A> for u8 {
    #[inline(always)]
    fn from(variant: NSCN_A) -> Self {
        match variant {
            NSCN_A::_00000 => 0,
            NSCN_A::_00001 => 1,
            NSCN_A::_00010 => 2,
            NSCN_A::_00011 => 3,
            NSCN_A::_00100 => 4,
            NSCN_A::_00101 => 5,
            NSCN_A::_00110 => 6,
            NSCN_A::_00111 => 7,
            NSCN_A::_01000 => 8,
            NSCN_A::_01001 => 9,
            NSCN_A::_01010 => 10,
            NSCN_A::_01011 => 11,
            NSCN_A::_01100 => 12,
            NSCN_A::_01101 => 13,
            NSCN_A::_01110 => 14,
            NSCN_A::_01111 => 15,
            NSCN_A::_10000 => 16,
            NSCN_A::_10001 => 17,
            NSCN_A::_10010 => 18,
            NSCN_A::_10011 => 19,
            NSCN_A::_10100 => 20,
            NSCN_A::_10101 => 21,
            NSCN_A::_10110 => 22,
            NSCN_A::_10111 => 23,
            NSCN_A::_11000 => 24,
            NSCN_A::_11001 => 25,
            NSCN_A::_11010 => 26,
            NSCN_A::_11011 => 27,
            NSCN_A::_11100 => 28,
            NSCN_A::_11101 => 29,
            NSCN_A::_11110 => 30,
            NSCN_A::_11111 => 31,
        }
    }
}
#[doc = "Reader of field `NSCN`"]
pub type NSCN_R = crate::R<u8, NSCN_A>;
impl NSCN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSCN_A {
        match self.bits {
            0 => NSCN_A::_00000,
            1 => NSCN_A::_00001,
            2 => NSCN_A::_00010,
            3 => NSCN_A::_00011,
            4 => NSCN_A::_00100,
            5 => NSCN_A::_00101,
            6 => NSCN_A::_00110,
            7 => NSCN_A::_00111,
            8 => NSCN_A::_01000,
            9 => NSCN_A::_01001,
            10 => NSCN_A::_01010,
            11 => NSCN_A::_01011,
            12 => NSCN_A::_01100,
            13 => NSCN_A::_01101,
            14 => NSCN_A::_01110,
            15 => NSCN_A::_01111,
            16 => NSCN_A::_10000,
            17 => NSCN_A::_10001,
            18 => NSCN_A::_10010,
            19 => NSCN_A::_10011,
            20 => NSCN_A::_10100,
            21 => NSCN_A::_10101,
            22 => NSCN_A::_10110,
            23 => NSCN_A::_10111,
            24 => NSCN_A::_11000,
            25 => NSCN_A::_11001,
            26 => NSCN_A::_11010,
            27 => NSCN_A::_11011,
            28 => NSCN_A::_11100,
            29 => NSCN_A::_11101,
            30 => NSCN_A::_11110,
            31 => NSCN_A::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == NSCN_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == NSCN_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == NSCN_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == NSCN_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == NSCN_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == NSCN_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == NSCN_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == NSCN_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == NSCN_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == NSCN_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == NSCN_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == NSCN_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == NSCN_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == NSCN_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == NSCN_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == NSCN_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == NSCN_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == NSCN_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == NSCN_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == NSCN_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == NSCN_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == NSCN_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == NSCN_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == NSCN_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == NSCN_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == NSCN_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == NSCN_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == NSCN_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == NSCN_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == NSCN_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == NSCN_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == NSCN_A::_11111
    }
}
#[doc = "Write proxy for field `NSCN`"]
pub struct NSCN_W<'a> {
    w: &'a mut W,
}
impl<'a> NSCN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSCN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Once per electrode"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(NSCN_A::_00000)
    }
    #[doc = "Twice per electrode"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(NSCN_A::_00001)
    }
    #[doc = "3 times per electrode"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(NSCN_A::_00010)
    }
    #[doc = "4 times per electrode"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(NSCN_A::_00011)
    }
    #[doc = "5 times per electrode"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(NSCN_A::_00100)
    }
    #[doc = "6 times per electrode"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(NSCN_A::_00101)
    }
    #[doc = "7 times per electrode"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(NSCN_A::_00110)
    }
    #[doc = "8 times per electrode"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(NSCN_A::_00111)
    }
    #[doc = "9 times per electrode"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(NSCN_A::_01000)
    }
    #[doc = "10 times per electrode"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(NSCN_A::_01001)
    }
    #[doc = "11 times per electrode"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(NSCN_A::_01010)
    }
    #[doc = "12 times per electrode"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(NSCN_A::_01011)
    }
    #[doc = "13 times per electrode"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(NSCN_A::_01100)
    }
    #[doc = "14 times per electrode"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(NSCN_A::_01101)
    }
    #[doc = "15 times per electrode"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(NSCN_A::_01110)
    }
    #[doc = "16 times per electrode"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(NSCN_A::_01111)
    }
    #[doc = "17 times per electrode"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(NSCN_A::_10000)
    }
    #[doc = "18 times per electrode"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(NSCN_A::_10001)
    }
    #[doc = "19 times per electrode"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(NSCN_A::_10010)
    }
    #[doc = "20 times per electrode"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(NSCN_A::_10011)
    }
    #[doc = "21 times per electrode"]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(NSCN_A::_10100)
    }
    #[doc = "22 times per electrode"]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(NSCN_A::_10101)
    }
    #[doc = "23 times per electrode"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(NSCN_A::_10110)
    }
    #[doc = "24 times per electrode"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(NSCN_A::_10111)
    }
    #[doc = "25 times per electrode"]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(NSCN_A::_11000)
    }
    #[doc = "26 times per electrode"]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(NSCN_A::_11001)
    }
    #[doc = "27 times per electrode"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(NSCN_A::_11010)
    }
    #[doc = "28 times per electrode"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(NSCN_A::_11011)
    }
    #[doc = "29 times per electrode"]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(NSCN_A::_11100)
    }
    #[doc = "30 times per electrode"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(NSCN_A::_11101)
    }
    #[doc = "31 times per electrode"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(NSCN_A::_11110)
    }
    #[doc = "32 times per electrode"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(NSCN_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "PS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Electrode Oscillator Frequency divided by 1"]
    _000,
    #[doc = "1: Electrode Oscillator Frequency divided by 2"]
    _001,
    #[doc = "2: Electrode Oscillator Frequency divided by 4"]
    _010,
    #[doc = "3: Electrode Oscillator Frequency divided by 8"]
    _011,
    #[doc = "4: Electrode Oscillator Frequency divided by 16"]
    _100,
    #[doc = "5: Electrode Oscillator Frequency divided by 32"]
    _101,
    #[doc = "6: Electrode Oscillator Frequency divided by 64"]
    _110,
    #[doc = "7: Electrode Oscillator Frequency divided by 128"]
    _111,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::_000 => 0,
            PS_A::_001 => 1,
            PS_A::_010 => 2,
            PS_A::_011 => 3,
            PS_A::_100 => 4,
            PS_A::_101 => 5,
            PS_A::_110 => 6,
            PS_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PS_A::_111
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "EXTCHRG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCHRG_A {
    #[doc = "0: 500 nA."]
    _000,
    #[doc = "1: 1 uA."]
    _001,
    #[doc = "2: 2 uA."]
    _010,
    #[doc = "3: 4 uA."]
    _011,
    #[doc = "4: 8 uA."]
    _100,
    #[doc = "5: 16 uA."]
    _101,
    #[doc = "6: 32 uA."]
    _110,
    #[doc = "7: 64 uA."]
    _111,
}
impl From<EXTCHRG_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTCHRG_A) -> Self {
        match variant {
            EXTCHRG_A::_000 => 0,
            EXTCHRG_A::_001 => 1,
            EXTCHRG_A::_010 => 2,
            EXTCHRG_A::_011 => 3,
            EXTCHRG_A::_100 => 4,
            EXTCHRG_A::_101 => 5,
            EXTCHRG_A::_110 => 6,
            EXTCHRG_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `EXTCHRG`"]
pub type EXTCHRG_R = crate::R<u8, EXTCHRG_A>;
impl EXTCHRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCHRG_A {
        match self.bits {
            0 => EXTCHRG_A::_000,
            1 => EXTCHRG_A::_001,
            2 => EXTCHRG_A::_010,
            3 => EXTCHRG_A::_011,
            4 => EXTCHRG_A::_100,
            5 => EXTCHRG_A::_101,
            6 => EXTCHRG_A::_110,
            7 => EXTCHRG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == EXTCHRG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == EXTCHRG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == EXTCHRG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == EXTCHRG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == EXTCHRG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == EXTCHRG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == EXTCHRG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == EXTCHRG_A::_111
    }
}
#[doc = "Write proxy for field `EXTCHRG`"]
pub struct EXTCHRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCHRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCHRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "500 nA."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_000)
    }
    #[doc = "1 uA."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_001)
    }
    #[doc = "2 uA."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_010)
    }
    #[doc = "4 uA."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_011)
    }
    #[doc = "8 uA."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_100)
    }
    #[doc = "16 uA."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_101)
    }
    #[doc = "32 uA."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_110)
    }
    #[doc = "64 uA."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "DVOLT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVOLT_A {
    #[doc = "0: DV = 1.026 V; VP = 1.328 V; Vm = 0.302 V."]
    _00,
    #[doc = "1: DV = 0.592 V; VP = 1.111 V; Vm = 0.519 V."]
    _01,
    #[doc = "2: DV = 0.342 V; VP = 0.986 V; Vm = 0.644 V."]
    _10,
    #[doc = "3: DV = 0.197 V; VP = 0.914 V; Vm = 0.716 V."]
    _11,
}
impl From<DVOLT_A> for u8 {
    #[inline(always)]
    fn from(variant: DVOLT_A) -> Self {
        match variant {
            DVOLT_A::_00 => 0,
            DVOLT_A::_01 => 1,
            DVOLT_A::_10 => 2,
            DVOLT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `DVOLT`"]
pub type DVOLT_R = crate::R<u8, DVOLT_A>;
impl DVOLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVOLT_A {
        match self.bits {
            0 => DVOLT_A::_00,
            1 => DVOLT_A::_01,
            2 => DVOLT_A::_10,
            3 => DVOLT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DVOLT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DVOLT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DVOLT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DVOLT_A::_11
    }
}
#[doc = "Write proxy for field `DVOLT`"]
pub struct DVOLT_W<'a> {
    w: &'a mut W,
}
impl<'a> DVOLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DVOLT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DV = 1.026 V; VP = 1.328 V; Vm = 0.302 V."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVOLT_A::_00)
    }
    #[doc = "DV = 0.592 V; VP = 1.111 V; Vm = 0.519 V."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVOLT_A::_01)
    }
    #[doc = "DV = 0.342 V; VP = 0.986 V; Vm = 0.644 V."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVOLT_A::_10)
    }
    #[doc = "DV = 0.197 V; VP = 0.914 V; Vm = 0.716 V."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVOLT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "REFCHRG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCHRG_A {
    #[doc = "0: 500 nA."]
    _000,
    #[doc = "1: 1 uA."]
    _001,
    #[doc = "2: 2 uA."]
    _010,
    #[doc = "3: 4 uA."]
    _011,
    #[doc = "4: 8 uA."]
    _100,
    #[doc = "5: 16 uA."]
    _101,
    #[doc = "6: 32 uA."]
    _110,
    #[doc = "7: 64 uA."]
    _111,
}
impl From<REFCHRG_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCHRG_A) -> Self {
        match variant {
            REFCHRG_A::_000 => 0,
            REFCHRG_A::_001 => 1,
            REFCHRG_A::_010 => 2,
            REFCHRG_A::_011 => 3,
            REFCHRG_A::_100 => 4,
            REFCHRG_A::_101 => 5,
            REFCHRG_A::_110 => 6,
            REFCHRG_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `REFCHRG`"]
pub type REFCHRG_R = crate::R<u8, REFCHRG_A>;
impl REFCHRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCHRG_A {
        match self.bits {
            0 => REFCHRG_A::_000,
            1 => REFCHRG_A::_001,
            2 => REFCHRG_A::_010,
            3 => REFCHRG_A::_011,
            4 => REFCHRG_A::_100,
            5 => REFCHRG_A::_101,
            6 => REFCHRG_A::_110,
            7 => REFCHRG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REFCHRG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REFCHRG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == REFCHRG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == REFCHRG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == REFCHRG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == REFCHRG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REFCHRG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == REFCHRG_A::_111
    }
}
#[doc = "Write proxy for field `REFCHRG`"]
pub struct REFCHRG_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCHRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCHRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "500 nA."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(REFCHRG_A::_000)
    }
    #[doc = "1 uA."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(REFCHRG_A::_001)
    }
    #[doc = "2 uA."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(REFCHRG_A::_010)
    }
    #[doc = "4 uA."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(REFCHRG_A::_011)
    }
    #[doc = "8 uA."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(REFCHRG_A::_100)
    }
    #[doc = "16 uA."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(REFCHRG_A::_101)
    }
    #[doc = "32 uA."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(REFCHRG_A::_110)
    }
    #[doc = "64 uA."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(REFCHRG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "TSI analog modes setup and status bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Set TSI in capacitive sensing(non-noise detection) mode."]
    _0000,
    #[doc = "4: Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is disabled."]
    _0100,
    #[doc = "8: Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is enabled to work in higher frequencies operations."]
    _1000,
    #[doc = "12: Set TSI analog to work in automatic noise detection mode."]
    _1100,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::_0000 => 0,
            MODE_A::_0100 => 4,
            MODE_A::_1000 => 8,
            MODE_A::_1100 => 12,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::_0000),
            4 => Val(MODE_A::_0100),
            8 => Val(MODE_A::_1000),
            12 => Val(MODE_A::_1100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == MODE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == MODE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == MODE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == MODE_A::_1100
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set TSI in capacitive sensing(non-noise detection) mode."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(MODE_A::_0000)
    }
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is disabled."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(MODE_A::_0100)
    }
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is enabled to work in higher frequencies operations."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(MODE_A::_1000)
    }
    #[doc = "Set TSI analog to work in automatic noise detection mode."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(MODE_A::_1100)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "End-of-scan or Out-of-Range Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOR_A {
    #[doc = "0: Out-of-range interrupt is allowed."]
    _0,
    #[doc = "1: End-of-scan interrupt is allowed."]
    _1,
}
impl From<ESOR_A> for bool {
    #[inline(always)]
    fn from(variant: ESOR_A) -> Self {
        match variant {
            ESOR_A::_0 => false,
            ESOR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ESOR`"]
pub type ESOR_R = crate::R<bool, ESOR_A>;
impl ESOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESOR_A {
        match self.bits {
            false => ESOR_A::_0,
            true => ESOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESOR_A::_1
    }
}
#[doc = "Write proxy for field `ESOR`"]
pub struct ESOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Out-of-range interrupt is allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESOR_A::_0)
    }
    #[doc = "End-of-scan interrupt is allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESOR_A::_1)
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
#[doc = "Reader of field `OUTRGF`"]
pub type OUTRGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTRGF`"]
pub struct OUTRGF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTRGF_W<'a> {
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
    #[doc = "Bit 0 - End-of-Scan DMA Transfer Request Enable Only"]
    #[inline(always)]
    pub fn eosdmeo(&self) -> EOSDMEO_R {
        EOSDMEO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CURSW"]
    #[inline(always)]
    pub fn cursw(&self) -> CURSW_R {
        CURSW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Scan Flag"]
    #[inline(always)]
    pub fn eosf(&self) -> EOSF_R {
        EOSF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Scan In Progress Status"]
    #[inline(always)]
    pub fn scnip(&self) -> SCNIP_R {
        SCNIP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Scan Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSI STOP Enable"]
    #[inline(always)]
    pub fn stpe(&self) -> STPE_R {
        STPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Enable"]
    #[inline(always)]
    pub fn tsiien(&self) -> TSIIEN_R {
        TSIIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - NSCN"]
    #[inline(always)]
    pub fn nscn(&self) -> NSCN_R {
        NSCN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - PS"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - EXTCHRG"]
    #[inline(always)]
    pub fn extchrg(&self) -> EXTCHRG_R {
        EXTCHRG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - DVOLT"]
    #[inline(always)]
    pub fn dvolt(&self) -> DVOLT_R {
        DVOLT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - REFCHRG"]
    #[inline(always)]
    pub fn refchrg(&self) -> REFCHRG_R {
        REFCHRG_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - TSI analog modes setup and status bits."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - End-of-scan or Out-of-Range Interrupt Selection"]
    #[inline(always)]
    pub fn esor(&self) -> ESOR_R {
        ESOR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Out of Range Flag."]
    #[inline(always)]
    pub fn outrgf(&self) -> OUTRGF_R {
        OUTRGF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End-of-Scan DMA Transfer Request Enable Only"]
    #[inline(always)]
    pub fn eosdmeo(&mut self) -> EOSDMEO_W {
        EOSDMEO_W { w: self }
    }
    #[doc = "Bit 1 - CURSW"]
    #[inline(always)]
    pub fn cursw(&mut self) -> CURSW_W {
        CURSW_W { w: self }
    }
    #[doc = "Bit 2 - End of Scan Flag"]
    #[inline(always)]
    pub fn eosf(&mut self) -> EOSF_W {
        EOSF_W { w: self }
    }
    #[doc = "Bit 4 - Scan Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bit 5 - TSI STOP Enable"]
    #[inline(always)]
    pub fn stpe(&mut self) -> STPE_W {
        STPE_W { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Enable"]
    #[inline(always)]
    pub fn tsiien(&mut self) -> TSIIEN_W {
        TSIIEN_W { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TSIEN_W {
        TSIEN_W { w: self }
    }
    #[doc = "Bits 8:12 - NSCN"]
    #[inline(always)]
    pub fn nscn(&mut self) -> NSCN_W {
        NSCN_W { w: self }
    }
    #[doc = "Bits 13:15 - PS"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 16:18 - EXTCHRG"]
    #[inline(always)]
    pub fn extchrg(&mut self) -> EXTCHRG_W {
        EXTCHRG_W { w: self }
    }
    #[doc = "Bits 19:20 - DVOLT"]
    #[inline(always)]
    pub fn dvolt(&mut self) -> DVOLT_W {
        DVOLT_W { w: self }
    }
    #[doc = "Bits 21:23 - REFCHRG"]
    #[inline(always)]
    pub fn refchrg(&mut self) -> REFCHRG_W {
        REFCHRG_W { w: self }
    }
    #[doc = "Bits 24:27 - TSI analog modes setup and status bits."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 28 - End-of-scan or Out-of-Range Interrupt Selection"]
    #[inline(always)]
    pub fn esor(&mut self) -> ESOR_W {
        ESOR_W { w: self }
    }
    #[doc = "Bit 31 - Out of Range Flag."]
    #[inline(always)]
    pub fn outrgf(&mut self) -> OUTRGF_W {
        OUTRGF_W { w: self }
    }
}
