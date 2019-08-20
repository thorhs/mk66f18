#[doc = "Reader of register PLL_SIC"]
pub type R = crate::R<u32, super::PLL_SIC>;
#[doc = "Writer for register PLL_SIC"]
pub type W = crate::W<u32, super::PLL_SIC>;
#[doc = "Register PLL_SIC `reset()`'s with value 0x0001_2000"]
impl crate::ResetValue for super::PLL_SIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_2000
    }
}
#[doc = "This field controls the USB PLL feedback loop divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_DIV_SEL_A {
    #[doc = "0: PLL reference frequency = 24MHz"]
    _00,
    #[doc = "1: PLL reference frequency = 16MHz"]
    _01,
}
impl From<PLL_DIV_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_DIV_SEL_A) -> Self {
        match variant {
            PLL_DIV_SEL_A::_00 => 0,
            PLL_DIV_SEL_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `PLL_DIV_SEL`"]
pub type PLL_DIV_SEL_R = crate::R<u8, PLL_DIV_SEL_A>;
impl PLL_DIV_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_DIV_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_DIV_SEL_A::_00),
            1 => Val(PLL_DIV_SEL_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLL_DIV_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLL_DIV_SEL_A::_01
    }
}
#[doc = "Write proxy for field `PLL_DIV_SEL`"]
pub struct PLL_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_DIV_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_DIV_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL reference frequency = 24MHz"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::_00)
    }
    #[doc = "PLL reference frequency = 16MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLL_EN_USB_CLKS`"]
pub type PLL_EN_USB_CLKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_EN_USB_CLKS`"]
pub struct PLL_EN_USB_CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_EN_USB_CLKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PLL_HOLD_RING_OFF`"]
pub type PLL_HOLD_RING_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_HOLD_RING_OFF`"]
pub struct PLL_HOLD_RING_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_HOLD_RING_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PLL_POWER`"]
pub type PLL_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_POWER`"]
pub struct PLL_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_POWER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PLL_ENABLE`"]
pub type PLL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_ENABLE`"]
pub struct PLL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PLL_BYPASS`"]
pub type PLL_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_BYPASS`"]
pub struct PLL_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "USB PLL lock status indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_A {
    #[doc = "0: PLL is not currently locked"]
    _0,
    #[doc = "1: PLL is currently locked"]
    _1,
}
impl From<PLL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_A) -> Self {
        match variant {
            PLL_LOCK_A::_0 => false,
            PLL_LOCK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLL_LOCK`"]
pub type PLL_LOCK_R = crate::R<bool, PLL_LOCK_A>;
impl PLL_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_A {
        match self.bits {
            false => PLL_LOCK_A::_0,
            true => PLL_LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLL_LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLL_LOCK_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PLL_DIV_SEL_R {
        PLL_DIV_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 6 - Enable the USB clock output from the USB PHY PLL."]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKS_R {
        PLL_EN_USB_CLKS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Analog debug bit"]
    #[inline(always)]
    pub fn pll_hold_ring_off(&self) -> PLL_HOLD_RING_OFF_R {
        PLL_HOLD_RING_OFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Power up the USB PLL."]
    #[inline(always)]
    pub fn pll_power(&self) -> PLL_POWER_R {
        PLL_POWER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable the clock output from the USB PLL."]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PLL_BYPASS_R {
        PLL_BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&mut self) -> PLL_DIV_SEL_W {
        PLL_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Enable the USB clock output from the USB PHY PLL."]
    #[inline(always)]
    pub fn pll_en_usb_clks(&mut self) -> PLL_EN_USB_CLKS_W {
        PLL_EN_USB_CLKS_W { w: self }
    }
    #[doc = "Bit 11 - Analog debug bit"]
    #[inline(always)]
    pub fn pll_hold_ring_off(&mut self) -> PLL_HOLD_RING_OFF_W {
        PLL_HOLD_RING_OFF_W { w: self }
    }
    #[doc = "Bit 12 - Power up the USB PLL."]
    #[inline(always)]
    pub fn pll_power(&mut self) -> PLL_POWER_W {
        PLL_POWER_W { w: self }
    }
    #[doc = "Bit 13 - Enable the clock output from the USB PLL."]
    #[inline(always)]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W {
        PLL_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline(always)]
    pub fn pll_bypass(&mut self) -> PLL_BYPASS_W {
        PLL_BYPASS_W { w: self }
    }
}
