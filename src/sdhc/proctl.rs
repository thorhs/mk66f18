#[doc = "Reader of register PROCTL"]
pub type R = crate::R<u32, super::PROCTL>;
#[doc = "Writer for register PROCTL"]
pub type W = crate::W<u32, super::PROCTL>;
#[doc = "Register PROCTL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::PROCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTL_A {
    #[doc = "0: LED off."]
    _0,
    #[doc = "1: LED on."]
    _1,
}
impl From<LCTL_A> for bool {
    #[inline(always)]
    fn from(variant: LCTL_A) -> Self {
        match variant {
            LCTL_A::_0 => false,
            LCTL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LCTL`"]
pub type LCTL_R = crate::R<bool, LCTL_A>;
impl LCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCTL_A {
        match self.bits {
            false => LCTL_A::_0,
            true => LCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCTL_A::_1
    }
}
#[doc = "Write proxy for field `LCTL`"]
pub struct LCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LED off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCTL_A::_0)
    }
    #[doc = "LED on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCTL_A::_1)
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
#[doc = "Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTW_A {
    #[doc = "0: 1-bit mode"]
    _00,
    #[doc = "1: 4-bit mode"]
    _01,
    #[doc = "2: 8-bit mode"]
    _10,
}
impl From<DTW_A> for u8 {
    #[inline(always)]
    fn from(variant: DTW_A) -> Self {
        match variant {
            DTW_A::_00 => 0,
            DTW_A::_01 => 1,
            DTW_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `DTW`"]
pub type DTW_R = crate::R<u8, DTW_A>;
impl DTW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTW_A::_00),
            1 => Val(DTW_A::_01),
            2 => Val(DTW_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DTW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DTW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTW_A::_10
    }
}
#[doc = "Write proxy for field `DTW`"]
pub struct DTW_W<'a> {
    w: &'a mut W,
}
impl<'a> DTW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DTW_A::_00)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DTW_A::_01)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTW_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "DAT3 As Card Detection Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3CD_A {
    #[doc = "0: DAT3 does not monitor card Insertion."]
    _0,
    #[doc = "1: DAT3 as card detection pin."]
    _1,
}
impl From<D3CD_A> for bool {
    #[inline(always)]
    fn from(variant: D3CD_A) -> Self {
        match variant {
            D3CD_A::_0 => false,
            D3CD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `D3CD`"]
pub type D3CD_R = crate::R<bool, D3CD_A>;
impl D3CD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D3CD_A {
        match self.bits {
            false => D3CD_A::_0,
            true => D3CD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == D3CD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == D3CD_A::_1
    }
}
#[doc = "Write proxy for field `D3CD`"]
pub struct D3CD_W<'a> {
    w: &'a mut W,
}
impl<'a> D3CD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D3CD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAT3 does not monitor card Insertion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(D3CD_A::_0)
    }
    #[doc = "DAT3 as card detection pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(D3CD_A::_1)
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
#[doc = "Endian Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMODE_A {
    #[doc = "0: Big endian mode"]
    _00,
    #[doc = "1: Half word big endian mode"]
    _01,
    #[doc = "2: Little endian mode"]
    _10,
}
impl From<EMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        match variant {
            EMODE_A::_00 => 0,
            EMODE_A::_01 => 1,
            EMODE_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `EMODE`"]
pub type EMODE_R = crate::R<u8, EMODE_A>;
impl EMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EMODE_A::_00),
            1 => Val(EMODE_A::_01),
            2 => Val(EMODE_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EMODE_A::_10
    }
}
#[doc = "Write proxy for field `EMODE`"]
pub struct EMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Big endian mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMODE_A::_00)
    }
    #[doc = "Half word big endian mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMODE_A::_01)
    }
    #[doc = "Little endian mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMODE_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTL_A {
    #[doc = "0: Card detect test level is 0, no card inserted."]
    _0,
    #[doc = "1: Card detect test level is 1, card inserted."]
    _1,
}
impl From<CDTL_A> for bool {
    #[inline(always)]
    fn from(variant: CDTL_A) -> Self {
        match variant {
            CDTL_A::_0 => false,
            CDTL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CDTL`"]
pub type CDTL_R = crate::R<bool, CDTL_A>;
impl CDTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDTL_A {
        match self.bits {
            false => CDTL_A::_0,
            true => CDTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDTL_A::_1
    }
}
#[doc = "Write proxy for field `CDTL`"]
pub struct CDTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card detect test level is 0, no card inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDTL_A::_0)
    }
    #[doc = "Card detect test level is 1, card inserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDTL_A::_1)
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
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSS_A {
    #[doc = "0: Card detection level is selected for normal purpose."]
    _0,
    #[doc = "1: Card detection test level is selected for test purpose."]
    _1,
}
impl From<CDSS_A> for bool {
    #[inline(always)]
    fn from(variant: CDSS_A) -> Self {
        match variant {
            CDSS_A::_0 => false,
            CDSS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CDSS`"]
pub type CDSS_R = crate::R<bool, CDSS_A>;
impl CDSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSS_A {
        match self.bits {
            false => CDSS_A::_0,
            true => CDSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDSS_A::_1
    }
}
#[doc = "Write proxy for field `CDSS`"]
pub struct CDSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card detection level is selected for normal purpose."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDSS_A::_0)
    }
    #[doc = "Card detection test level is selected for test purpose."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDSS_A::_1)
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
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAS_A {
    #[doc = "0: No DMA or simple DMA is selected."]
    _00,
    #[doc = "1: ADMA1 is selected."]
    _01,
    #[doc = "2: ADMA2 is selected."]
    _10,
}
impl From<DMAS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        match variant {
            DMAS_A::_00 => 0,
            DMAS_A::_01 => 1,
            DMAS_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `DMAS`"]
pub type DMAS_R = crate::R<u8, DMAS_A>;
impl DMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMAS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAS_A::_00),
            1 => Val(DMAS_A::_01),
            2 => Val(DMAS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DMAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DMAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DMAS_A::_10
    }
}
#[doc = "Write proxy for field `DMAS`"]
pub struct DMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No DMA or simple DMA is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DMAS_A::_00)
    }
    #[doc = "ADMA1 is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DMAS_A::_01)
    }
    #[doc = "ADMA2 is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DMAS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABGREQ_A {
    #[doc = "0: Transfer"]
    _0,
    #[doc = "1: Stop"]
    _1,
}
impl From<SABGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SABGREQ_A) -> Self {
        match variant {
            SABGREQ_A::_0 => false,
            SABGREQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SABGREQ`"]
pub type SABGREQ_R = crate::R<bool, SABGREQ_A>;
impl SABGREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABGREQ_A {
        match self.bits {
            false => SABGREQ_A::_0,
            true => SABGREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SABGREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SABGREQ_A::_1
    }
}
#[doc = "Write proxy for field `SABGREQ`"]
pub struct SABGREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SABGREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SABGREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SABGREQ_A::_0)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SABGREQ_A::_1)
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
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREQ_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Restart"]
    _1,
}
impl From<CREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CREQ_A) -> Self {
        match variant {
            CREQ_A::_0 => false,
            CREQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CREQ`"]
pub type CREQ_R = crate::R<bool, CREQ_A>;
impl CREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREQ_A {
        match self.bits {
            false => CREQ_A::_0,
            true => CREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CREQ_A::_1
    }
}
#[doc = "Write proxy for field `CREQ`"]
pub struct CREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CREQ_A::_0)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CREQ_A::_1)
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
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTL_A {
    #[doc = "0: Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    _0,
    #[doc = "1: Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    _1,
}
impl From<RWCTL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTL_A) -> Self {
        match variant {
            RWCTL_A::_0 => false,
            RWCTL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWCTL`"]
pub type RWCTL_R = crate::R<bool, RWCTL_A>;
impl RWCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTL_A {
        match self.bits {
            false => RWCTL_A::_0,
            true => RWCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWCTL_A::_1
    }
}
#[doc = "Write proxy for field `RWCTL`"]
pub struct RWCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWCTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWCTL_A::_0)
    }
    #[doc = "Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWCTL_A::_1)
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
#[doc = "Interrupt At Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IABG_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<IABG_A> for bool {
    #[inline(always)]
    fn from(variant: IABG_A) -> Self {
        match variant {
            IABG_A::_0 => false,
            IABG_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IABG`"]
pub type IABG_R = crate::R<bool, IABG_A>;
impl IABG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IABG_A {
        match self.bits {
            false => IABG_A::_0,
            true => IABG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IABG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IABG_A::_1
    }
}
#[doc = "Write proxy for field `IABG`"]
pub struct IABG_W<'a> {
    w: &'a mut W,
}
impl<'a> IABG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IABG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IABG_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IABG_A::_1)
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
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINT_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<WECINT_A> for bool {
    #[inline(always)]
    fn from(variant: WECINT_A) -> Self {
        match variant {
            WECINT_A::_0 => false,
            WECINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WECINT`"]
pub type WECINT_R = crate::R<bool, WECINT_A>;
impl WECINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINT_A {
        match self.bits {
            false => WECINT_A::_0,
            true => WECINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECINT_A::_1
    }
}
#[doc = "Write proxy for field `WECINT`"]
pub struct WECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WECINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINT_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINT_A::_1)
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
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINS_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<WECINS_A> for bool {
    #[inline(always)]
    fn from(variant: WECINS_A) -> Self {
        match variant {
            WECINS_A::_0 => false,
            WECINS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WECINS`"]
pub type WECINS_R = crate::R<bool, WECINS_A>;
impl WECINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINS_A {
        match self.bits {
            false => WECINS_A::_0,
            true => WECINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECINS_A::_1
    }
}
#[doc = "Write proxy for field `WECINS`"]
pub struct WECINS_W<'a> {
    w: &'a mut W,
}
impl<'a> WECINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECINS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINS_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINS_A::_1)
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
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECRM_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<WECRM_A> for bool {
    #[inline(always)]
    fn from(variant: WECRM_A) -> Self {
        match variant {
            WECRM_A::_0 => false,
            WECRM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WECRM`"]
pub type WECRM_R = crate::R<bool, WECRM_A>;
impl WECRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECRM_A {
        match self.bits {
            false => WECRM_A::_0,
            true => WECRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WECRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WECRM_A::_1
    }
}
#[doc = "Write proxy for field `WECRM`"]
pub struct WECRM_W<'a> {
    w: &'a mut W,
}
impl<'a> WECRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECRM_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECRM_A::_1)
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
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn lctl(&self) -> LCTL_R {
        LCTL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&self) -> DTW_R {
        DTW_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&self) -> D3CD_R {
        D3CD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&self) -> CDTL_R {
        CDTL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&self) -> CDSS_R {
        CDSS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&self) -> SABGREQ_R {
        SABGREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&self) -> CREQ_R {
        CREQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&self) -> RWCTL_R {
        RWCTL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&self) -> IABG_R {
        IABG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&self) -> WECINT_R {
        WECINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&self) -> WECINS_R {
        WECINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&self) -> WECRM_R {
        WECRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn lctl(&mut self) -> LCTL_W {
        LCTL_W { w: self }
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&mut self) -> DTW_W {
        DTW_W { w: self }
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&mut self) -> D3CD_W {
        D3CD_W { w: self }
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&mut self) -> EMODE_W {
        EMODE_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&mut self) -> CDTL_W {
        CDTL_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&mut self) -> CDSS_W {
        CDSS_W { w: self }
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DMAS_W {
        DMAS_W { w: self }
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&mut self) -> SABGREQ_W {
        SABGREQ_W { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&mut self) -> CREQ_W {
        CREQ_W { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&mut self) -> RWCTL_W {
        RWCTL_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&mut self) -> IABG_W {
        IABG_W { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&mut self) -> WECINT_W {
        WECINT_W { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&mut self) -> WECINS_W {
        WECINS_W { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&mut self) -> WECRM_W {
        WECRM_W { w: self }
    }
}
