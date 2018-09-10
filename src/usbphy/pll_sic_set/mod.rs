#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_SIC_SET {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `PLL_DIV_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_DIV_SELR {
    #[doc = "PLL reference frequency = 24MHz"]
    _00,
    #[doc = "PLL reference frequency = 16MHz"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLL_DIV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLL_DIV_SELR::_00 => 0,
            PLL_DIV_SELR::_01 => 1,
            PLL_DIV_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLL_DIV_SELR {
        match value {
            0 => PLL_DIV_SELR::_00,
            1 => PLL_DIV_SELR::_01,
            i => PLL_DIV_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PLL_DIV_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PLL_DIV_SELR::_01
    }
}
#[doc = r" Value of the field"]
pub struct PLL_EN_USB_CLKSR {
    bits: bool,
}
impl PLL_EN_USB_CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLL_HOLD_RING_OFFR {
    bits: bool,
}
impl PLL_HOLD_RING_OFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLL_POWERR {
    bits: bool,
}
impl PLL_POWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLL_ENABLER {
    bits: bool,
}
impl PLL_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLL_BYPASSR {
    bits: bool,
}
impl PLL_BYPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PLL_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCKR {
    #[doc = "PLL is not currently locked"]
    _0,
    #[doc = "PLL is currently locked"]
    _1,
}
impl PLL_LOCKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLL_LOCKR::_0 => false,
            PLL_LOCKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_LOCKR {
        match value {
            false => PLL_LOCKR::_0,
            true => PLL_LOCKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_LOCKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_LOCKR::_1
    }
}
#[doc = "Values that can be written to the field `PLL_DIV_SEL`"]
pub enum PLL_DIV_SELW {
    #[doc = "PLL reference frequency = 24MHz"]
    _00,
    #[doc = "PLL reference frequency = 16MHz"]
    _01,
}
impl PLL_DIV_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLL_DIV_SELW::_00 => 0,
            PLL_DIV_SELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_DIV_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_DIV_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL reference frequency = 24MHz"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::_00)
    }
    #[doc = "PLL reference frequency = 16MHz"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_EN_USB_CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_EN_USB_CLKSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_HOLD_RING_OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_HOLD_RING_OFFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_POWERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_ENABLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_BYPASSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - This field controls the USB PLL feedback loop divider"]
    #[inline]
    pub fn pll_div_sel(&self) -> PLL_DIV_SELR {
        PLL_DIV_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Enable the USB clock output from the USB PHY PLL."]
    #[inline]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_EN_USB_CLKSR { bits }
    }
    #[doc = "Bit 11 - Analog debug bit"]
    #[inline]
    pub fn pll_hold_ring_off(&self) -> PLL_HOLD_RING_OFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_HOLD_RING_OFFR { bits }
    }
    #[doc = "Bit 12 - Power up the USB PLL."]
    #[inline]
    pub fn pll_power(&self) -> PLL_POWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_POWERR { bits }
    }
    #[doc = "Bit 13 - Enable the clock output from the USB PLL."]
    #[inline]
    pub fn pll_enable(&self) -> PLL_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_ENABLER { bits }
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline]
    pub fn pll_bypass(&self) -> PLL_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_BYPASSR { bits }
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline]
    pub fn pll_lock(&self) -> PLL_LOCKR {
        PLL_LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 73728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - This field controls the USB PLL feedback loop divider"]
    #[inline]
    pub fn pll_div_sel(&mut self) -> _PLL_DIV_SELW {
        _PLL_DIV_SELW { w: self }
    }
    #[doc = "Bit 6 - Enable the USB clock output from the USB PHY PLL."]
    #[inline]
    pub fn pll_en_usb_clks(&mut self) -> _PLL_EN_USB_CLKSW {
        _PLL_EN_USB_CLKSW { w: self }
    }
    #[doc = "Bit 11 - Analog debug bit"]
    #[inline]
    pub fn pll_hold_ring_off(&mut self) -> _PLL_HOLD_RING_OFFW {
        _PLL_HOLD_RING_OFFW { w: self }
    }
    #[doc = "Bit 12 - Power up the USB PLL."]
    #[inline]
    pub fn pll_power(&mut self) -> _PLL_POWERW {
        _PLL_POWERW { w: self }
    }
    #[doc = "Bit 13 - Enable the clock output from the USB PLL."]
    #[inline]
    pub fn pll_enable(&mut self) -> _PLL_ENABLEW {
        _PLL_ENABLEW { w: self }
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline]
    pub fn pll_bypass(&mut self) -> _PLL_BYPASSW {
        _PLL_BYPASSW { w: self }
    }
}
