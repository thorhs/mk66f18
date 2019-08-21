#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRDIV_A {
    #[doc = "0: Divide Factor is 1"]
    _0,
    #[doc = "1: Divide Factor is 2"]
    _1,
    #[doc = "2: Divide Factor is 3"]
    _2,
    #[doc = "3: Divide Factor is 4"]
    _3,
    #[doc = "4: Divide Factor is 5"]
    _4,
    #[doc = "5: Divide Factor is 6"]
    _5,
    #[doc = "6: Divide Factor is 7"]
    _6,
    #[doc = "7: Divide Factor is 8"]
    _7,
}
impl From<PRDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDIV_A) -> Self {
        match variant {
            PRDIV_A::_0 => 0,
            PRDIV_A::_1 => 1,
            PRDIV_A::_2 => 2,
            PRDIV_A::_3 => 3,
            PRDIV_A::_4 => 4,
            PRDIV_A::_5 => 5,
            PRDIV_A::_6 => 6,
            PRDIV_A::_7 => 7,
        }
    }
}
#[doc = "Reader of field `PRDIV`"]
pub type PRDIV_R = crate::R<u8, PRDIV_A>;
impl PRDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDIV_A {
        match self.bits {
            0 => PRDIV_A::_0,
            1 => PRDIV_A::_1,
            2 => PRDIV_A::_2,
            3 => PRDIV_A::_3,
            4 => PRDIV_A::_4,
            5 => PRDIV_A::_5,
            6 => PRDIV_A::_6,
            7 => PRDIV_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PRDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PRDIV_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PRDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PRDIV_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == PRDIV_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PRDIV_A::_7
    }
}
#[doc = "Write proxy for field `PRDIV`"]
pub struct PRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide Factor is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDIV_A::_0)
    }
    #[doc = "Divide Factor is 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDIV_A::_1)
    }
    #[doc = "Divide Factor is 3"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDIV_A::_2)
    }
    #[doc = "Divide Factor is 4"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDIV_A::_3)
    }
    #[doc = "Divide Factor is 5"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PRDIV_A::_4)
    }
    #[doc = "Divide Factor is 6"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PRDIV_A::_5)
    }
    #[doc = "Divide Factor is 7"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(PRDIV_A::_6)
    }
    #[doc = "Divide Factor is 8"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PRDIV_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN_A {
    #[doc = "0: MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    _0,
    #[doc = "1: MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    _1,
}
impl From<PLLSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTEN_A) -> Self {
        match variant {
            PLLSTEN_A::_0 => false,
            PLLSTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLSTEN`"]
pub type PLLSTEN_R = crate::R<bool, PLLSTEN_A>;
impl PLLSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTEN_A {
        match self.bits {
            false => PLLSTEN_A::_0,
            true => PLLSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN_A::_1
    }
}
#[doc = "Write proxy for field `PLLSTEN`"]
pub struct PLLSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_0)
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_1)
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
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN_A {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0,
    #[doc = "1: MCGPLLCLK is active."]
    _1,
}
impl From<PLLCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCLKEN_A) -> Self {
        match variant {
            PLLCLKEN_A::_0 => false,
            PLLCLKEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLCLKEN`"]
pub type PLLCLKEN_R = crate::R<bool, PLLCLKEN_A>;
impl PLLCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCLKEN_A {
        match self.bits {
            false => PLLCLKEN_A::_0,
            true => PLLCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `PLLCLKEN`"]
pub struct PLLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_1)
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
impl R {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&self) -> PRDIV_R {
        PRDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&self) -> PLLSTEN_R {
        PLLSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&self) -> PLLCLKEN_R {
        PLLCLKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&mut self) -> PRDIV_W {
        PRDIV_W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&mut self) -> PLLSTEN_W {
        PLLSTEN_W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&mut self) -> PLLCLKEN_W {
        PLLCLKEN_W { w: self }
    }
}
