#[doc = "Reader of register PMCTRL"]
pub type R = crate::R<u8, super::PMCTRL>;
#[doc = "Writer for register PMCTRL"]
pub type W = crate::W<u8, super::PMCTRL>;
#[doc = "Register PMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stop Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPM_A {
    #[doc = "0: Normal Stop (STOP)"]
    _000,
    #[doc = "2: Very-Low-Power Stop (VLPS)"]
    _010,
    #[doc = "3: Low-Leakage Stop (LLSx)"]
    _011,
    #[doc = "4: Very-Low-Leakage Stop (VLLSx)"]
    _100,
    #[doc = "6: Reseved"]
    _110,
}
impl From<STOPM_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPM_A) -> Self {
        match variant {
            STOPM_A::_000 => 0,
            STOPM_A::_010 => 2,
            STOPM_A::_011 => 3,
            STOPM_A::_100 => 4,
            STOPM_A::_110 => 6,
        }
    }
}
#[doc = "Reader of field `STOPM`"]
pub type STOPM_R = crate::R<u8, STOPM_A>;
impl STOPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STOPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STOPM_A::_000),
            2 => Val(STOPM_A::_010),
            3 => Val(STOPM_A::_011),
            4 => Val(STOPM_A::_100),
            6 => Val(STOPM_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == STOPM_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == STOPM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == STOPM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == STOPM_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == STOPM_A::_110
    }
}
#[doc = "Write proxy for field `STOPM`"]
pub struct STOPM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Stop (STOP)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(STOPM_A::_000)
    }
    #[doc = "Very-Low-Power Stop (VLPS)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(STOPM_A::_010)
    }
    #[doc = "Low-Leakage Stop (LLSx)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(STOPM_A::_011)
    }
    #[doc = "Very-Low-Leakage Stop (VLLSx)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(STOPM_A::_100)
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(STOPM_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Stop Aborted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPA_A {
    #[doc = "0: The previous stop mode entry was successsful."]
    _0,
    #[doc = "1: The previous stop mode entry was aborted."]
    _1,
}
impl From<STOPA_A> for bool {
    #[inline(always)]
    fn from(variant: STOPA_A) -> Self {
        match variant {
            STOPA_A::_0 => false,
            STOPA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STOPA`"]
pub type STOPA_R = crate::R<bool, STOPA_A>;
impl STOPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPA_A {
        match self.bits {
            false => STOPA_A::_0,
            true => STOPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPA_A::_1
    }
}
#[doc = "Run Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNM_A {
    #[doc = "0: Normal Run mode (RUN)"]
    _00,
    #[doc = "2: Very-Low-Power Run mode (VLPR)"]
    _10,
    #[doc = "3: High Speed Run mode (HSRUN)"]
    _11,
}
impl From<RUNM_A> for u8 {
    #[inline(always)]
    fn from(variant: RUNM_A) -> Self {
        match variant {
            RUNM_A::_00 => 0,
            RUNM_A::_10 => 2,
            RUNM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `RUNM`"]
pub type RUNM_R = crate::R<u8, RUNM_A>;
impl RUNM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RUNM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RUNM_A::_00),
            2 => Val(RUNM_A::_10),
            3 => Val(RUNM_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RUNM_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RUNM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RUNM_A::_11
    }
}
#[doc = "Write proxy for field `RUNM`"]
pub struct RUNM_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUNM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Run mode (RUN)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RUNM_A::_00)
    }
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RUNM_A::_10)
    }
    #[doc = "High Speed Run mode (HSRUN)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RUNM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u8) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&self) -> STOPM_R {
        STOPM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Stop Aborted"]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&self) -> RUNM_R {
        RUNM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&mut self) -> STOPM_W {
        STOPM_W { w: self }
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&mut self) -> RUNM_W {
        RUNM_W { w: self }
    }
}
