#[doc = "Reader of register LVDSC2"]
pub type R = crate::R<u8, super::LVDSC2>;
#[doc = "Writer for register LVDSC2"]
pub type W = crate::W<u8, super::LVDSC2>;
#[doc = "Register LVDSC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LVDSC2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWV_A {
    #[doc = "0: Low trip point selected (VLVW = VLVW1)"]
    _00,
    #[doc = "1: Mid 1 trip point selected (VLVW = VLVW2)"]
    _01,
    #[doc = "2: Mid 2 trip point selected (VLVW = VLVW3)"]
    _10,
    #[doc = "3: High trip point selected (VLVW = VLVW4)"]
    _11,
}
impl From<LVWV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVWV_A) -> Self {
        match variant {
            LVWV_A::_00 => 0,
            LVWV_A::_01 => 1,
            LVWV_A::_10 => 2,
            LVWV_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `LVWV`"]
pub type LVWV_R = crate::R<u8, LVWV_A>;
impl LVWV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWV_A {
        match self.bits {
            0 => LVWV_A::_00,
            1 => LVWV_A::_01,
            2 => LVWV_A::_10,
            3 => LVWV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LVWV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LVWV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LVWV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LVWV_A::_11
    }
}
#[doc = "Write proxy for field `LVWV`"]
pub struct LVWV_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVWV_A::_00)
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVWV_A::_01)
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LVWV_A::_10)
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LVWV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "1: Request a hardware interrupt when LVWF = 1"]
    _1,
}
impl From<LVWIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVWIE_A) -> Self {
        match variant {
            LVWIE_A::_0 => false,
            LVWIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVWIE`"]
pub type LVWIE_R = crate::R<bool, LVWIE_A>;
impl LVWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWIE_A {
        match self.bits {
            false => LVWIE_A::_0,
            true => LVWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWIE_A::_1
    }
}
#[doc = "Write proxy for field `LVWIE`"]
pub struct LVWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIE_A::_1)
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
#[doc = "Write proxy for field `LVWACK`"]
pub struct LVWACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWACK_W<'a> {
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
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWF_A {
    #[doc = "0: Low-voltage warning event not detected"]
    _0,
    #[doc = "1: Low-voltage warning event detected"]
    _1,
}
impl From<LVWF_A> for bool {
    #[inline(always)]
    fn from(variant: LVWF_A) -> Self {
        match variant {
            LVWF_A::_0 => false,
            LVWF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVWF`"]
pub type LVWF_R = crate::R<bool, LVWF_A>;
impl LVWF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWF_A {
        match self.bits {
            false => LVWF_A::_0,
            true => LVWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&self) -> LVWV_R {
        LVWV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LVWIE_R {
        LVWIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LVWF_R {
        LVWF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&mut self) -> LVWV_W {
        LVWV_W { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&mut self) -> LVWIE_W {
        LVWIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&mut self) -> LVWACK_W {
        LVWACK_W { w: self }
    }
}
