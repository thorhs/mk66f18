#[doc = "Reader of register C9"]
pub type R = crate::R<u8, super::C9>;
#[doc = "Writer for register C9"]
pub type W = crate::W<u8, super::C9>;
#[doc = "Register C9 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::C9 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "External PLL Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_PLL_LOCS_A {
    #[doc = "0: Loss of MCG EXT_PLL has not occurred."]
    _0,
    #[doc = "1: Loss of MCG EXT_PLL has occurred."]
    _1,
}
impl From<EXT_PLL_LOCS_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_PLL_LOCS_A) -> Self {
        match variant {
            EXT_PLL_LOCS_A::_0 => false,
            EXT_PLL_LOCS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EXT_PLL_LOCS`"]
pub type EXT_PLL_LOCS_R = crate::R<bool, EXT_PLL_LOCS_A>;
impl EXT_PLL_LOCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_PLL_LOCS_A {
        match self.bits {
            false => EXT_PLL_LOCS_A::_0,
            true => EXT_PLL_LOCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXT_PLL_LOCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXT_PLL_LOCS_A::_1
    }
}
#[doc = "Write proxy for field `EXT_PLL_LOCS`"]
pub struct EXT_PLL_LOCS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_PLL_LOCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXT_PLL_LOCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loss of MCG EXT_PLL has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXT_PLL_LOCS_A::_0)
    }
    #[doc = "Loss of MCG EXT_PLL has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXT_PLL_LOCS_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "MCG External PLL Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCRE_A {
    #[doc = "0: Interrupt request is generated on a invalid or loss of the MCG external PLL clock."]
    _0,
    #[doc = "1: Generates a system reset request on a invalid or loss of the MCG external PLL clock."]
    _1,
}
impl From<PLL_LOCRE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCRE_A) -> Self {
        match variant {
            PLL_LOCRE_A::_0 => false,
            PLL_LOCRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLL_LOCRE`"]
pub type PLL_LOCRE_R = crate::R<bool, PLL_LOCRE_A>;
impl PLL_LOCRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCRE_A {
        match self.bits {
            false => PLL_LOCRE_A::_0,
            true => PLL_LOCRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLL_LOCRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLL_LOCRE_A::_1
    }
}
#[doc = "Write proxy for field `PLL_LOCRE`"]
pub struct PLL_LOCRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_LOCRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request is generated on a invalid or loss of the MCG external PLL clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_LOCRE_A::_0)
    }
    #[doc = "Generates a system reset request on a invalid or loss of the MCG external PLL clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_LOCRE_A::_1)
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
#[doc = "MCG External PLL Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CME_A {
    #[doc = "0: External clock monitor is disabled for EXT_PLL clock."]
    _0,
    #[doc = "1: External clock monitor is enabled for EXT_PLL clock."]
    _1,
}
impl From<PLL_CME_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CME_A) -> Self {
        match variant {
            PLL_CME_A::_0 => false,
            PLL_CME_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLL_CME`"]
pub type PLL_CME_R = crate::R<bool, PLL_CME_A>;
impl PLL_CME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_CME_A {
        match self.bits {
            false => PLL_CME_A::_0,
            true => PLL_CME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLL_CME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLL_CME_A::_1
    }
}
#[doc = "Write proxy for field `PLL_CME`"]
pub struct PLL_CME_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock monitor is disabled for EXT_PLL clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_CME_A::_0)
    }
    #[doc = "External clock monitor is enabled for EXT_PLL clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_CME_A::_1)
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
impl R {
    #[doc = "Bit 0 - External PLL Loss of Clock Status"]
    #[inline(always)]
    pub fn ext_pll_locs(&self) -> EXT_PLL_LOCS_R {
        EXT_PLL_LOCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - MCG External PLL Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn pll_locre(&self) -> PLL_LOCRE_R {
        PLL_LOCRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MCG External PLL Clock Monitor Enable"]
    #[inline(always)]
    pub fn pll_cme(&self) -> PLL_CME_R {
        PLL_CME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External PLL Loss of Clock Status"]
    #[inline(always)]
    pub fn ext_pll_locs(&mut self) -> EXT_PLL_LOCS_W {
        EXT_PLL_LOCS_W { w: self }
    }
    #[doc = "Bit 4 - MCG External PLL Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn pll_locre(&mut self) -> PLL_LOCRE_W {
        PLL_LOCRE_W { w: self }
    }
    #[doc = "Bit 5 - MCG External PLL Clock Monitor Enable"]
    #[inline(always)]
    pub fn pll_cme(&mut self) -> PLL_CME_W {
        PLL_CME_W { w: self }
    }
}
