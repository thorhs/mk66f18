#[doc = "Reader of register S"]
pub type R = crate::R<u8, super::S>;
#[doc = "Writer for register S"]
pub type W = crate::W<u8, super::S>;
#[doc = "Register S `reset()`'s with value 0x10"]
impl crate::ResetValue for super::S {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Internal Reference Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCST_A {
    #[doc = "0: Source of internal reference clock is the slow clock (32 kHz IRC)."]
    _0,
    #[doc = "1: Source of internal reference clock is the fast clock (4 MHz IRC)."]
    _1,
}
impl From<IRCST_A> for bool {
    #[inline(always)]
    fn from(variant: IRCST_A) -> Self {
        match variant {
            IRCST_A::_0 => false,
            IRCST_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IRCST`"]
pub type IRCST_R = crate::R<bool, IRCST_A>;
impl IRCST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCST_A {
        match self.bits {
            false => IRCST_A::_0,
            true => IRCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCST_A::_1
    }
}
#[doc = "Reader of field `OSCINIT0`"]
pub type OSCINIT0_R = crate::R<bool, bool>;
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKST_A {
    #[doc = "0: Encoding 0 - Output of the FLL is selected (reset default)."]
    _00,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = "3: Encoding 3 - Output of the PLL is selected."]
    _11,
}
impl From<CLKST_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKST_A) -> Self {
        match variant {
            CLKST_A::_00 => 0,
            CLKST_A::_01 => 1,
            CLKST_A::_10 => 2,
            CLKST_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CLKST`"]
pub type CLKST_R = crate::R<u8, CLKST_A>;
impl CLKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKST_A {
        match self.bits {
            0 => CLKST_A::_00,
            1 => CLKST_A::_01,
            2 => CLKST_A::_10,
            3 => CLKST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKST_A::_11
    }
}
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFST_A {
    #[doc = "0: Source of FLL reference clock is the external reference clock."]
    _0,
    #[doc = "1: Source of FLL reference clock is the internal reference clock."]
    _1,
}
impl From<IREFST_A> for bool {
    #[inline(always)]
    fn from(variant: IREFST_A) -> Self {
        match variant {
            IREFST_A::_0 => false,
            IREFST_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IREFST`"]
pub type IREFST_R = crate::R<bool, IREFST_A>;
impl IREFST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFST_A {
        match self.bits {
            false => IREFST_A::_0,
            true => IREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFST_A::_1
    }
}
#[doc = "PLL Select Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLST_A {
    #[doc = "0: Source of PLLS clock is FLL clock."]
    _0,
    #[doc = "1: Source of PLLS clock is PLLCS output clock."]
    _1,
}
impl From<PLLST_A> for bool {
    #[inline(always)]
    fn from(variant: PLLST_A) -> Self {
        match variant {
            PLLST_A::_0 => false,
            PLLST_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLST`"]
pub type PLLST_R = crate::R<bool, PLLST_A>;
impl PLLST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLST_A {
        match self.bits {
            false => PLLST_A::_0,
            true => PLLST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLST_A::_1
    }
}
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK0_A {
    #[doc = "0: PLL is currently unlocked."]
    _0,
    #[doc = "1: PLL is currently locked."]
    _1,
}
impl From<LOCK0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK0_A) -> Self {
        match variant {
            LOCK0_A::_0 => false,
            LOCK0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK0`"]
pub type LOCK0_R = crate::R<bool, LOCK0_A>;
impl LOCK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK0_A {
        match self.bits {
            false => LOCK0_A::_0,
            true => LOCK0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCK0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCK0_A::_1
    }
}
#[doc = "Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLS0_A {
    #[doc = "0: PLL has not lost lock since LOLS 0 was last cleared."]
    _0,
    #[doc = "1: PLL has lost lock since LOLS 0 was last cleared."]
    _1,
}
impl From<LOLS0_A> for bool {
    #[inline(always)]
    fn from(variant: LOLS0_A) -> Self {
        match variant {
            LOLS0_A::_0 => false,
            LOLS0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOLS0`"]
pub type LOLS0_R = crate::R<bool, LOLS0_A>;
impl LOLS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLS0_A {
        match self.bits {
            false => LOLS0_A::_0,
            true => LOLS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLS0_A::_1
    }
}
#[doc = "Write proxy for field `LOLS0`"]
pub struct LOLS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLS0_A::_0)
    }
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLS0_A::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline(always)]
    pub fn ircst(&self) -> IRCST_R {
        IRCST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline(always)]
    pub fn oscinit0(&self) -> OSCINIT0_R {
        OSCINIT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> CLKST_R {
        CLKST_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IREFST_R {
        IREFST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL Select Status"]
    #[inline(always)]
    pub fn pllst(&self) -> PLLST_R {
        PLLST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols0(&self) -> LOLS0_R {
        LOLS0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols0(&mut self) -> LOLS0_W {
        LOLS0_W { w: self }
    }
}
