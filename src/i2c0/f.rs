#[doc = "Reader of register F"]
pub type R = crate::R<u8, super::F>;
#[doc = "Writer for register F"]
pub type W = crate::W<u8, super::F>;
#[doc = "Register F `reset()`'s with value 0"]
impl crate::ResetValue for super::F {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICR`"]
pub type ICR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICR`"]
pub struct ICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "Multiplier Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULT_A {
    #[doc = "0: mul = 1"]
    _00,
    #[doc = "1: mul = 2"]
    _01,
    #[doc = "2: mul = 4"]
    _10,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        match variant {
            MULT_A::_00 => 0,
            MULT_A::_01 => 1,
            MULT_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `MULT`"]
pub type MULT_R = crate::R<u8, MULT_A>;
impl MULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULT_A::_00),
            1 => Val(MULT_A::_01),
            2 => Val(MULT_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MULT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MULT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MULT_A::_10
    }
}
#[doc = "Write proxy for field `MULT`"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "mul = 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULT_A::_00)
    }
    #[doc = "mul = 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULT_A::_01)
    }
    #[doc = "mul = 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULT_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    pub fn icr(&mut self) -> ICR_W {
        ICR_W { w: self }
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
}
