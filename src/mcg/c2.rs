#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Internal Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCS_A {
    #[doc = "0: Slow internal reference clock selected."]
    _0,
    #[doc = "1: Fast internal reference clock selected."]
    _1,
}
impl From<IRCS_A> for bool {
    #[inline(always)]
    fn from(variant: IRCS_A) -> Self {
        match variant {
            IRCS_A::_0 => false,
            IRCS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IRCS`"]
pub type IRCS_R = crate::R<bool, IRCS_A>;
impl IRCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCS_A {
        match self.bits {
            false => IRCS_A::_0,
            true => IRCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCS_A::_1
    }
}
#[doc = "Write proxy for field `IRCS`"]
pub struct IRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slow internal reference clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCS_A::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCS_A::_1)
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
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_A {
    #[doc = "0: FLL or PLL is not disabled in bypass modes."]
    _0,
    #[doc = "1: FLL or PLL is disabled in bypass modes (lower power)"]
    _1,
}
impl From<LP_A> for bool {
    #[inline(always)]
    fn from(variant: LP_A) -> Self {
        match variant {
            LP_A::_0 => false,
            LP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LP`"]
pub type LP_R = crate::R<bool, LP_A>;
impl LP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_A {
        match self.bits {
            false => LP_A::_0,
            true => LP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LP_A::_1
    }
}
#[doc = "Write proxy for field `LP`"]
pub struct LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLL or PLL is not disabled in bypass modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LP_A::_0)
    }
    #[doc = "FLL or PLL is disabled in bypass modes (lower power)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFS_A {
    #[doc = "0: External reference clock requested."]
    _0,
    #[doc = "1: Oscillator requested."]
    _1,
}
impl From<EREFS_A> for bool {
    #[inline(always)]
    fn from(variant: EREFS_A) -> Self {
        match variant {
            EREFS_A::_0 => false,
            EREFS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EREFS`"]
pub type EREFS_R = crate::R<bool, EREFS_A>;
impl EREFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFS_A {
        match self.bits {
            false => EREFS_A::_0,
            true => EREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EREFS_A::_1
    }
}
#[doc = "Write proxy for field `EREFS`"]
pub struct EREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EREFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External reference clock requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS_A::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO_A {
    #[doc = "0: Configure crystal oscillator for low-power operation."]
    _0,
    #[doc = "1: Configure crystal oscillator for high-gain operation."]
    _1,
}
impl From<HGO_A> for bool {
    #[inline(always)]
    fn from(variant: HGO_A) -> Self {
        match variant {
            HGO_A::_0 => false,
            HGO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HGO`"]
pub type HGO_R = crate::R<bool, HGO_A>;
impl HGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO_A {
        match self.bits {
            false => HGO_A::_0,
            true => HGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HGO_A::_1
    }
}
#[doc = "Write proxy for field `HGO`"]
pub struct HGO_W<'a> {
    w: &'a mut W,
}
impl<'a> HGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HGO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO_A::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_A {
    #[doc = "0: Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00,
    #[doc = "1: Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        match variant {
            RANGE_A::_00 => 0,
            RANGE_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `RANGE`"]
pub type RANGE_R = crate::R<u8, RANGE_A>;
impl RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RANGE_A::_00),
            1 => Val(RANGE_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RANGE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RANGE_A::_01
    }
}
#[doc = "Write proxy for field `RANGE`"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGE_A::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `FCFTRIM`"]
pub type FCFTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCFTRIM`"]
pub struct FCFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FCFTRIM_W<'a> {
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
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE0_A {
    #[doc = "0: Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0,
    #[doc = "1: Generate a reset request on a loss of OSC0 external reference clock."]
    _1,
}
impl From<LOCRE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRE0_A) -> Self {
        match variant {
            LOCRE0_A::_0 => false,
            LOCRE0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCRE0`"]
pub type LOCRE0_R = crate::R<bool, LOCRE0_A>;
impl LOCRE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRE0_A {
        match self.bits {
            false => LOCRE0_A::_0,
            true => LOCRE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCRE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCRE0_A::_1
    }
}
#[doc = "Write proxy for field `LOCRE0`"]
pub struct LOCRE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCRE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCRE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE0_A::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE0_A::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IRCS_R {
        IRCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&self) -> EREFS_R {
        EREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&self) -> HGO_R {
        HGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn fcftrim(&self) -> FCFTRIM_R {
        FCFTRIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&self) -> LOCRE0_R {
        LOCRE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&mut self) -> IRCS_W {
        IRCS_W { w: self }
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&mut self) -> LP_W {
        LP_W { w: self }
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&mut self) -> EREFS_W {
        EREFS_W { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&mut self) -> HGO_W {
        HGO_W { w: self }
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn fcftrim(&mut self) -> FCFTRIM_W {
        FCFTRIM_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&mut self) -> LOCRE0_W {
        LOCRE0_W { w: self }
    }
}
