#[doc = "Reader of register C7"]
pub type R = crate::R<u8, super::C7>;
#[doc = "Writer for register C7"]
pub type W = crate::W<u8, super::C7>;
#[doc = "Register C7 `reset()`'s with value 0"]
impl crate::ResetValue for super::C7 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCG OSC Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSEL_A {
    #[doc = "0: Selects Oscillator (OSCCLK0)."]
    _00,
    #[doc = "1: Selects 32 kHz RTC Oscillator."]
    _01,
    #[doc = "2: Selects Oscillator (OSCCLK1)."]
    _10,
}
impl From<OSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        match variant {
            OSCSEL_A::_00 => 0,
            OSCSEL_A::_01 => 1,
            OSCSEL_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `OSCSEL`"]
pub type OSCSEL_R = crate::R<u8, OSCSEL_A>;
impl OSCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSCSEL_A::_00),
            1 => Val(OSCSEL_A::_01),
            2 => Val(OSCSEL_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSCSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OSCSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSCSEL_A::_10
    }
}
#[doc = "Write proxy for field `OSCSEL`"]
pub struct OSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects Oscillator (OSCCLK0)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSCSEL_A::_00)
    }
    #[doc = "Selects 32 kHz RTC Oscillator."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSCSEL_A::_01)
    }
    #[doc = "Selects Oscillator (OSCCLK1)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSCSEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W {
        OSCSEL_W { w: self }
    }
}
