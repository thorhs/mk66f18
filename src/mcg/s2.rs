#[doc = "Reader of register S2"]
pub type R = crate::R<u8, super::S2>;
#[doc = "PLL Clock Select Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCST_A {
    #[doc = "0: Source of PLLCS is PLL clock."]
    _0,
    #[doc = "1: Source of PLLCS is EXT_PLL clock."]
    _1,
}
impl From<PLLCST_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCST_A) -> Self {
        match variant {
            PLLCST_A::_0 => false,
            PLLCST_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLLCST`"]
pub type PLLCST_R = crate::R<bool, PLLCST_A>;
impl PLLCST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCST_A {
        match self.bits {
            false => PLLCST_A::_0,
            true => PLLCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCST_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - PLL Clock Select Status"]
    #[inline(always)]
    pub fn pllcst(&self) -> PLLCST_R {
        PLLCST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
