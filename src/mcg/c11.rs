#[doc = "Reader of register C11"]
pub type R = crate::R<u8, super::C11>;
#[doc = "Writer for register C11"]
pub type W = crate::W<u8, super::C11>;
#[doc = "Register C11 `reset()`'s with value 0"]
impl crate::ResetValue for super::C11 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PLL Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCS_A {
    #[doc = "0: PLL0 output clock is selected."]
    _0,
    #[doc = "1: External PLL clock is selected."]
    _1,
}
impl From<PLLCS_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCS_A) -> Self {
        match variant {
            PLLCS_A::_0 => false,
            PLLCS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLCS`"]
pub type PLLCS_R = crate::R<bool, PLLCS_A>;
impl PLLCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCS_A {
        match self.bits {
            false => PLLCS_A::_0,
            true => PLLCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCS_A::_1
    }
}
#[doc = "Write proxy for field `PLLCS`"]
pub struct PLLCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL0 output clock is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCS_A::_0)
    }
    #[doc = "External PLL clock is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCS_A::_1)
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
impl R {
    #[doc = "Bit 4 - PLL Clock Select"]
    #[inline(always)]
    pub fn pllcs(&self) -> PLLCS_R {
        PLLCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PLL Clock Select"]
    #[inline(always)]
    pub fn pllcs(&mut self) -> PLLCS_W {
        PLLCS_W { w: self }
    }
}
