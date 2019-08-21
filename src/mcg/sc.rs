#[doc = "Reader of register SC"]
pub type R = crate::R<u8, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u8, super::SC>;
#[doc = "Register SC `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "OSC0 Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCS0_A {
    #[doc = "0: Loss of OSC0 has not occurred."]
    _0,
    #[doc = "1: Loss of OSC0 has occurred."]
    _1,
}
impl From<LOCS0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCS0_A) -> Self {
        match variant {
            LOCS0_A::_0 => false,
            LOCS0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCS0`"]
pub type LOCS0_R = crate::R<bool, LOCS0_A>;
impl LOCS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCS0_A {
        match self.bits {
            false => LOCS0_A::_0,
            true => LOCS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCS0_A::_1
    }
}
#[doc = "Write proxy for field `LOCS0`"]
pub struct LOCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loss of OSC0 has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS0_A::_0)
    }
    #[doc = "Loss of OSC0 has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS0_A::_1)
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
#[doc = "Fast Clock Internal Reference Divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRDIV_A {
    #[doc = "0: Divide Factor is 1"]
    _000,
    #[doc = "1: Divide Factor is 2."]
    _001,
    #[doc = "2: Divide Factor is 4."]
    _010,
    #[doc = "3: Divide Factor is 8."]
    _011,
    #[doc = "4: Divide Factor is 16"]
    _100,
    #[doc = "5: Divide Factor is 32"]
    _101,
    #[doc = "6: Divide Factor is 64"]
    _110,
    #[doc = "7: Divide Factor is 128."]
    _111,
}
impl From<FCRDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FCRDIV_A) -> Self {
        match variant {
            FCRDIV_A::_000 => 0,
            FCRDIV_A::_001 => 1,
            FCRDIV_A::_010 => 2,
            FCRDIV_A::_011 => 3,
            FCRDIV_A::_100 => 4,
            FCRDIV_A::_101 => 5,
            FCRDIV_A::_110 => 6,
            FCRDIV_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `FCRDIV`"]
pub type FCRDIV_R = crate::R<u8, FCRDIV_A>;
impl FCRDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRDIV_A {
        match self.bits {
            0 => FCRDIV_A::_000,
            1 => FCRDIV_A::_001,
            2 => FCRDIV_A::_010,
            3 => FCRDIV_A::_011,
            4 => FCRDIV_A::_100,
            5 => FCRDIV_A::_101,
            6 => FCRDIV_A::_110,
            7 => FCRDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FCRDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FCRDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FCRDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FCRDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FCRDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FCRDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FCRDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FCRDIV_A::_111
    }
}
#[doc = "Write proxy for field `FCRDIV`"]
pub struct FCRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide Factor is 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FCRDIV_A::_000)
    }
    #[doc = "Divide Factor is 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FCRDIV_A::_001)
    }
    #[doc = "Divide Factor is 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FCRDIV_A::_010)
    }
    #[doc = "Divide Factor is 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FCRDIV_A::_011)
    }
    #[doc = "Divide Factor is 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FCRDIV_A::_100)
    }
    #[doc = "Divide Factor is 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FCRDIV_A::_101)
    }
    #[doc = "Divide Factor is 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FCRDIV_A::_110)
    }
    #[doc = "Divide Factor is 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FCRDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
#[doc = "FLL Filter Preserve Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTPRSRV_A {
    #[doc = "0: FLL filter and FLL frequency will reset on changes to currect clock mode."]
    _0,
    #[doc = "1: Fll filter and FLL frequency retain their previous values during new clock mode change."]
    _1,
}
impl From<FLTPRSRV_A> for bool {
    #[inline(always)]
    fn from(variant: FLTPRSRV_A) -> Self {
        match variant {
            FLTPRSRV_A::_0 => false,
            FLTPRSRV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLTPRSRV`"]
pub type FLTPRSRV_R = crate::R<bool, FLTPRSRV_A>;
impl FLTPRSRV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPRSRV_A {
        match self.bits {
            false => FLTPRSRV_A::_0,
            true => FLTPRSRV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLTPRSRV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLTPRSRV_A::_1
    }
}
#[doc = "Write proxy for field `FLTPRSRV`"]
pub struct FLTPRSRV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTPRSRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTPRSRV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLL filter and FLL frequency will reset on changes to currect clock mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLTPRSRV_A::_0)
    }
    #[doc = "Fll filter and FLL frequency retain their previous values during new clock mode change."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLTPRSRV_A::_1)
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
#[doc = "Automatic Trim Machine Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATMF_A {
    #[doc = "0: Automatic Trim Machine completed normally."]
    _0,
    #[doc = "1: Automatic Trim Machine failed."]
    _1,
}
impl From<ATMF_A> for bool {
    #[inline(always)]
    fn from(variant: ATMF_A) -> Self {
        match variant {
            ATMF_A::_0 => false,
            ATMF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ATMF`"]
pub type ATMF_R = crate::R<bool, ATMF_A>;
impl ATMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATMF_A {
        match self.bits {
            false => ATMF_A::_0,
            true => ATMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATMF_A::_1
    }
}
#[doc = "Write proxy for field `ATMF`"]
pub struct ATMF_W<'a> {
    w: &'a mut W,
}
impl<'a> ATMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic Trim Machine completed normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMF_A::_0)
    }
    #[doc = "Automatic Trim Machine failed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMF_A::_1)
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
#[doc = "Automatic Trim Machine Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATMS_A {
    #[doc = "0: 32 kHz Internal Reference Clock selected."]
    _0,
    #[doc = "1: 4 MHz Internal Reference Clock selected."]
    _1,
}
impl From<ATMS_A> for bool {
    #[inline(always)]
    fn from(variant: ATMS_A) -> Self {
        match variant {
            ATMS_A::_0 => false,
            ATMS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ATMS`"]
pub type ATMS_R = crate::R<bool, ATMS_A>;
impl ATMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATMS_A {
        match self.bits {
            false => ATMS_A::_0,
            true => ATMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATMS_A::_1
    }
}
#[doc = "Write proxy for field `ATMS`"]
pub struct ATMS_W<'a> {
    w: &'a mut W,
}
impl<'a> ATMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "32 kHz Internal Reference Clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMS_A::_0)
    }
    #[doc = "4 MHz Internal Reference Clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMS_A::_1)
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
#[doc = "Automatic Trim Machine Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATME_A {
    #[doc = "0: Auto Trim Machine disabled."]
    _0,
    #[doc = "1: Auto Trim Machine enabled."]
    _1,
}
impl From<ATME_A> for bool {
    #[inline(always)]
    fn from(variant: ATME_A) -> Self {
        match variant {
            ATME_A::_0 => false,
            ATME_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ATME`"]
pub type ATME_R = crate::R<bool, ATME_A>;
impl ATME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATME_A {
        match self.bits {
            false => ATME_A::_0,
            true => ATME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATME_A::_1
    }
}
#[doc = "Write proxy for field `ATME`"]
pub struct ATME_W<'a> {
    w: &'a mut W,
}
impl<'a> ATME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto Trim Machine disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATME_A::_0)
    }
    #[doc = "Auto Trim Machine enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATME_A::_1)
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
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline(always)]
    pub fn locs0(&self) -> LOCS0_R {
        LOCS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline(always)]
    pub fn fcrdiv(&self) -> FCRDIV_R {
        FCRDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline(always)]
    pub fn fltprsrv(&self) -> FLTPRSRV_R {
        FLTPRSRV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline(always)]
    pub fn atmf(&self) -> ATMF_R {
        ATMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline(always)]
    pub fn atms(&self) -> ATMS_R {
        ATMS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline(always)]
    pub fn atme(&self) -> ATME_R {
        ATME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline(always)]
    pub fn locs0(&mut self) -> LOCS0_W {
        LOCS0_W { w: self }
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline(always)]
    pub fn fcrdiv(&mut self) -> FCRDIV_W {
        FCRDIV_W { w: self }
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline(always)]
    pub fn fltprsrv(&mut self) -> FLTPRSRV_W {
        FLTPRSRV_W { w: self }
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline(always)]
    pub fn atmf(&mut self) -> ATMF_W {
        ATMF_W { w: self }
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline(always)]
    pub fn atms(&mut self) -> ATMS_W {
        ATMS_W { w: self }
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline(always)]
    pub fn atme(&mut self) -> ATME_W {
        ATME_W { w: self }
    }
}
