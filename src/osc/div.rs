#[doc = "Reader of register DIV"]
pub type R = crate::R<u8, super::DIV>;
#[doc = "Writer for register DIV"]
pub type W = crate::W<u8, super::DIV>;
#[doc = "Register DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::DIV {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ERCLK prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERPS_A {
    #[doc = "0: The divisor ratio is 1."]
    _00,
    #[doc = "1: The divisor ratio is 2."]
    _01,
    #[doc = "2: The divisor ratio is 4."]
    _10,
    #[doc = "3: The divisor ratio is 8."]
    _11,
}
impl From<ERPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ERPS_A) -> Self {
        match variant {
            ERPS_A::_00 => 0,
            ERPS_A::_01 => 1,
            ERPS_A::_10 => 2,
            ERPS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `ERPS`"]
pub type ERPS_R = crate::R<u8, ERPS_A>;
impl ERPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERPS_A {
        match self.bits {
            0 => ERPS_A::_00,
            1 => ERPS_A::_01,
            2 => ERPS_A::_10,
            3 => ERPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ERPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ERPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ERPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ERPS_A::_11
    }
}
#[doc = "Write proxy for field `ERPS`"]
pub struct ERPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The divisor ratio is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ERPS_A::_00)
    }
    #[doc = "The divisor ratio is 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ERPS_A::_01)
    }
    #[doc = "The divisor ratio is 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ERPS_A::_10)
    }
    #[doc = "The divisor ratio is 8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ERPS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline(always)]
    pub fn erps(&self) -> ERPS_R {
        ERPS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline(always)]
    pub fn erps(&mut self) -> ERPS_W {
        ERPS_W { w: self }
    }
}
