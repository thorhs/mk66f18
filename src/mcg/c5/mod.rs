#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C5 {
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
#[doc = r" Value of the field"]
pub struct PRDIVR {
    bits: u8,
}
impl PRDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PLLSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTENR {
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTENR {
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
            PLLSTENR::_0 => false,
            PLLSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSTENR {
        match value {
            false => PLLSTENR::_0,
            true => PLLSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSTENR::_1
    }
}
#[doc = "Possible values of the field `PLLCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKENR {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKENR {
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
            PLLCLKENR::_0 => false,
            PLLCLKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLCLKENR {
        match value {
            false => PLLCLKENR::_0,
            true => PLLCLKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKENR::_1
    }
}
#[doc = r" Proxy"]
pub struct _PRDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PRDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSTEN`"]
pub enum PLLSTENW {
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSTENW::_0 => false,
            PLLSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTENW::_0)
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTENW::_1)
    }
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLCLKEN`"]
pub enum PLLCLKENW {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLCLKENW::_0 => false,
            PLLCLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKENW::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKENW::_1)
    }
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv(&self) -> PRDIVR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        PRDIVR { bits }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten(&self) -> PLLSTENR {
        PLLSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken(&self) -> PLLCLKENR {
        PLLCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv(&mut self) -> _PRDIVW {
        _PRDIVW { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten(&mut self) -> _PLLSTENW {
        _PLLSTENW { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken(&mut self) -> _PLLCLKENW {
        _PLLCLKENW { w: self }
    }
}
