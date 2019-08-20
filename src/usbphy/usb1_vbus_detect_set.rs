#[doc = "Reader of register USB1_VBUS_DETECT_SET"]
pub type R = crate::R<u32, super::USB1_VBUS_DETECT_SET>;
#[doc = "Writer for register USB1_VBUS_DETECT_SET"]
pub type W = crate::W<u32, super::USB1_VBUS_DETECT_SET>;
#[doc = "Register USB1_VBUS_DETECT_SET `reset()`'s with value 0x0070_0004"]
impl crate::ResetValue for super::USB1_VBUS_DETECT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0070_0004
    }
}
#[doc = "Sets the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_THRESH_A {
    #[doc = "0: 4.0 V"]
    _000,
    #[doc = "1: 4.1 V"]
    _001,
    #[doc = "2: 4.2 V"]
    _010,
    #[doc = "3: 4.3 V"]
    _011,
    #[doc = "4: 4.4 V (Default)"]
    _100,
    #[doc = "5: 4.5 V"]
    _101,
    #[doc = "6: 4.6 V"]
    _110,
    #[doc = "7: 4.7 V"]
    _111,
}
impl From<VBUSVALID_THRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUSVALID_THRESH_A) -> Self {
        match variant {
            VBUSVALID_THRESH_A::_000 => 0,
            VBUSVALID_THRESH_A::_001 => 1,
            VBUSVALID_THRESH_A::_010 => 2,
            VBUSVALID_THRESH_A::_011 => 3,
            VBUSVALID_THRESH_A::_100 => 4,
            VBUSVALID_THRESH_A::_101 => 5,
            VBUSVALID_THRESH_A::_110 => 6,
            VBUSVALID_THRESH_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `VBUSVALID_THRESH`"]
pub type VBUSVALID_THRESH_R = crate::R<u8, VBUSVALID_THRESH_A>;
impl VBUSVALID_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_THRESH_A {
        match self.bits {
            0 => VBUSVALID_THRESH_A::_000,
            1 => VBUSVALID_THRESH_A::_001,
            2 => VBUSVALID_THRESH_A::_010,
            3 => VBUSVALID_THRESH_A::_011,
            4 => VBUSVALID_THRESH_A::_100,
            5 => VBUSVALID_THRESH_A::_101,
            6 => VBUSVALID_THRESH_A::_110,
            7 => VBUSVALID_THRESH_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_111
    }
}
#[doc = "Write proxy for field `VBUSVALID_THRESH`"]
pub struct VBUSVALID_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_THRESH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4.0 V"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_000)
    }
    #[doc = "4.1 V"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_001)
    }
    #[doc = "4.2 V"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_010)
    }
    #[doc = "4.3 V"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_011)
    }
    #[doc = "4.4 V (Default)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_100)
    }
    #[doc = "4.5 V"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_101)
    }
    #[doc = "4.6 V"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_110)
    }
    #[doc = "4.7 V"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "VBUS detect signal override enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_OVERRIDE_EN_A {
    #[doc = "0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    _0,
    #[doc = "1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    _1,
}
impl From<VBUS_OVERRIDE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_OVERRIDE_EN_A) -> Self {
        match variant {
            VBUS_OVERRIDE_EN_A::_0 => false,
            VBUS_OVERRIDE_EN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VBUS_OVERRIDE_EN`"]
pub type VBUS_OVERRIDE_EN_R = crate::R<bool, VBUS_OVERRIDE_EN_A>;
impl VBUS_OVERRIDE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_OVERRIDE_EN_A {
        match self.bits {
            false => VBUS_OVERRIDE_EN_A::_0,
            true => VBUS_OVERRIDE_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::_1
    }
}
#[doc = "Write proxy for field `VBUS_OVERRIDE_EN`"]
pub struct VBUS_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_OVERRIDE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_OVERRIDE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::_0)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::_1)
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
#[doc = "Reader of field `SESSEND_OVERRIDE`"]
pub type SESSEND_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSEND_OVERRIDE`"]
pub struct SESSEND_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSEND_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `BVALID_OVERRIDE`"]
pub type BVALID_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVALID_OVERRIDE`"]
pub struct BVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALID_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `AVALID_OVERRIDE`"]
pub type AVALID_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALID_OVERRIDE`"]
pub struct AVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALID_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `VBUSVALID_OVERRIDE`"]
pub type VBUSVALID_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSVALID_OVERRIDE`"]
pub struct VBUSVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_OVERRIDE_W<'a> {
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
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    _0,
    #[doc = "1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    _1,
}
impl From<VBUSVALID_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_SEL_A) -> Self {
        match variant {
            VBUSVALID_SEL_A::_0 => false,
            VBUSVALID_SEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VBUSVALID_SEL`"]
pub type VBUSVALID_SEL_R = crate::R<bool, VBUSVALID_SEL_A>;
impl VBUSVALID_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_SEL_A {
        match self.bits {
            false => VBUSVALID_SEL_A::_0,
            true => VBUSVALID_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUSVALID_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUSVALID_SEL_A::_1
    }
}
#[doc = "Write proxy for field `VBUSVALID_SEL`"]
pub struct VBUSVALID_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::_0)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::_1)
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
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_SOURCE_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    _00,
    #[doc = "1: Use the Session Valid comparator results for signal reported to the USB controller"]
    _01,
    #[doc = "2: Use the Session Valid comparator results for signal reported to the USB controller"]
    _10,
    #[doc = "3: Reserved, do not use"]
    _11,
}
impl From<VBUS_SOURCE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUS_SOURCE_SEL_A) -> Self {
        match variant {
            VBUS_SOURCE_SEL_A::_00 => 0,
            VBUS_SOURCE_SEL_A::_01 => 1,
            VBUS_SOURCE_SEL_A::_10 => 2,
            VBUS_SOURCE_SEL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `VBUS_SOURCE_SEL`"]
pub type VBUS_SOURCE_SEL_R = crate::R<u8, VBUS_SOURCE_SEL_A>;
impl VBUS_SOURCE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_SOURCE_SEL_A {
        match self.bits {
            0 => VBUS_SOURCE_SEL_A::_00,
            1 => VBUS_SOURCE_SEL_A::_01,
            2 => VBUS_SOURCE_SEL_A::_10,
            3 => VBUS_SOURCE_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::_11
    }
}
#[doc = "Write proxy for field `VBUS_SOURCE_SEL`"]
pub struct VBUS_SOURCE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_SOURCE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_SOURCE_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::_00)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::_01)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::_10)
    }
    #[doc = "Reserved, do not use"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Selects the comparator used for VBUS_VALID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_TO_SESSVALID_A {
    #[doc = "0: Use the VBUS_VALID comparator for VBUS_VALID results"]
    _0,
    #[doc = "1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    _1,
}
impl From<VBUSVALID_TO_SESSVALID_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_TO_SESSVALID_A) -> Self {
        match variant {
            VBUSVALID_TO_SESSVALID_A::_0 => false,
            VBUSVALID_TO_SESSVALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VBUSVALID_TO_SESSVALID`"]
pub type VBUSVALID_TO_SESSVALID_R = crate::R<bool, VBUSVALID_TO_SESSVALID_A>;
impl VBUSVALID_TO_SESSVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_TO_SESSVALID_A {
        match self.bits {
            false => VBUSVALID_TO_SESSVALID_A::_0,
            true => VBUSVALID_TO_SESSVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::_1
    }
}
#[doc = "Write proxy for field `VBUSVALID_TO_SESSVALID`"]
pub struct VBUSVALID_TO_SESSVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_TO_SESSVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_TO_SESSVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::_0)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::_1)
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
#[doc = "Enables the VBUS_VALID comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUP_CMPS_A {
    #[doc = "0: Powers down the VBUS_VALID comparator"]
    _0,
    #[doc = "1: Enables the VBUS_VALID comparator (default)"]
    _1,
}
impl From<PWRUP_CMPS_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUP_CMPS_A) -> Self {
        match variant {
            PWRUP_CMPS_A::_0 => false,
            PWRUP_CMPS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PWRUP_CMPS`"]
pub type PWRUP_CMPS_R = crate::R<bool, PWRUP_CMPS_A>;
impl PWRUP_CMPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUP_CMPS_A {
        match self.bits {
            false => PWRUP_CMPS_A::_0,
            true => PWRUP_CMPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWRUP_CMPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWRUP_CMPS_A::_1
    }
}
#[doc = "Write proxy for field `PWRUP_CMPS`"]
pub struct PWRUP_CMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_CMPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUP_CMPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::_0)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::_1)
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
#[doc = "Controls VBUS discharge resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCHARGE_VBUS_A {
    #[doc = "0: VBUS discharge resistor is disabled (Default)"]
    _0,
    #[doc = "1: VBUS discharge resistor is enabled"]
    _1,
}
impl From<DISCHARGE_VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: DISCHARGE_VBUS_A) -> Self {
        match variant {
            DISCHARGE_VBUS_A::_0 => false,
            DISCHARGE_VBUS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DISCHARGE_VBUS`"]
pub type DISCHARGE_VBUS_R = crate::R<bool, DISCHARGE_VBUS_A>;
impl DISCHARGE_VBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCHARGE_VBUS_A {
        match self.bits {
            false => DISCHARGE_VBUS_A::_0,
            true => DISCHARGE_VBUS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DISCHARGE_VBUS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DISCHARGE_VBUS_A::_1
    }
}
#[doc = "Write proxy for field `DISCHARGE_VBUS`"]
pub struct DISCHARGE_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCHARGE_VBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCHARGE_VBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::_0)
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::_1)
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
#[doc = "Enables resistors used for an older method of resistive battery charger detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_CHARGER_RESISTOR_A {
    #[doc = "0: Disable resistive charger detection resistors on USB_DP and USB_DP"]
    _0,
    #[doc = "1: Enable resistive charger detection resistors on USB_DP and USB_DP"]
    _1,
}
impl From<EN_CHARGER_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CHARGER_RESISTOR_A) -> Self {
        match variant {
            EN_CHARGER_RESISTOR_A::_0 => false,
            EN_CHARGER_RESISTOR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN_CHARGER_RESISTOR`"]
pub type EN_CHARGER_RESISTOR_R = crate::R<bool, EN_CHARGER_RESISTOR_A>;
impl EN_CHARGER_RESISTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CHARGER_RESISTOR_A {
        match self.bits {
            false => EN_CHARGER_RESISTOR_A::_0,
            true => EN_CHARGER_RESISTOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_CHARGER_RESISTOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_CHARGER_RESISTOR_A::_1
    }
}
#[doc = "Write proxy for field `EN_CHARGER_RESISTOR`"]
pub struct EN_CHARGER_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CHARGER_RESISTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_CHARGER_RESISTOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTOR_A::_0)
    }
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTOR_A::_1)
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
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESH_R {
        VBUSVALID_THRESH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&self) -> VBUS_OVERRIDE_EN_R {
        VBUS_OVERRIDE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Override value for SESSEND"]
    #[inline(always)]
    pub fn sessend_override(&self) -> SESSEND_OVERRIDE_R {
        SESSEND_OVERRIDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid"]
    #[inline(always)]
    pub fn bvalid_override(&self) -> BVALID_OVERRIDE_R {
        BVALID_OVERRIDE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid"]
    #[inline(always)]
    pub fn avalid_override(&self) -> AVALID_OVERRIDE_R {
        AVALID_OVERRIDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller"]
    #[inline(always)]
    pub fn vbusvalid_override(&self) -> VBUSVALID_OVERRIDE_R {
        VBUSVALID_OVERRIDE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&self) -> VBUSVALID_SEL_R {
        VBUSVALID_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&self) -> VBUS_SOURCE_SEL_R {
        VBUS_SOURCE_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&self) -> VBUSVALID_TO_SESSVALID_R {
        VBUSVALID_TO_SESSVALID_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn pwrup_cmps(&self) -> PWRUP_CMPS_R {
        PWRUP_CMPS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor"]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUS_R {
        DISCHARGE_VBUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub fn en_charger_resistor(&self) -> EN_CHARGER_RESISTOR_R {
        EN_CHARGER_RESISTOR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&mut self) -> VBUSVALID_THRESH_W {
        VBUSVALID_THRESH_W { w: self }
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&mut self) -> VBUS_OVERRIDE_EN_W {
        VBUS_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Override value for SESSEND"]
    #[inline(always)]
    pub fn sessend_override(&mut self) -> SESSEND_OVERRIDE_W {
        SESSEND_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid"]
    #[inline(always)]
    pub fn bvalid_override(&mut self) -> BVALID_OVERRIDE_W {
        BVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid"]
    #[inline(always)]
    pub fn avalid_override(&mut self) -> AVALID_OVERRIDE_W {
        AVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller"]
    #[inline(always)]
    pub fn vbusvalid_override(&mut self) -> VBUSVALID_OVERRIDE_W {
        VBUSVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&mut self) -> VBUSVALID_SEL_W {
        VBUSVALID_SEL_W { w: self }
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&mut self) -> VBUS_SOURCE_SEL_W {
        VBUS_SOURCE_SEL_W { w: self }
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&mut self) -> VBUSVALID_TO_SESSVALID_W {
        VBUSVALID_TO_SESSVALID_W { w: self }
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn pwrup_cmps(&mut self) -> PWRUP_CMPS_W {
        PWRUP_CMPS_W { w: self }
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor"]
    #[inline(always)]
    pub fn discharge_vbus(&mut self) -> DISCHARGE_VBUS_W {
        DISCHARGE_VBUS_W { w: self }
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub fn en_charger_resistor(&mut self) -> EN_CHARGER_RESISTOR_W {
        EN_CHARGER_RESISTOR_W { w: self }
    }
}
