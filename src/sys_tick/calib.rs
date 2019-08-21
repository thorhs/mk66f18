#[doc = "Reader of register CALIB"]
pub type R = crate::R<u32, super::CALIB>;
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "0: 10ms calibration value is exact"]
    _0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    _1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        match variant {
            SKEW_A::_0 => false,
            SKEW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, SKEW_A>;
impl SKEW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::_0,
            true => SKEW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SKEW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SKEW_A::_1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: The reference clock is provided"]
    _0,
    #[doc = "1: The reference clock is not provided"]
    _1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        match variant {
            NOREF_A::_0 => false,
            NOREF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, NOREF_A>;
impl NOREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::_0,
            true => NOREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOREF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
