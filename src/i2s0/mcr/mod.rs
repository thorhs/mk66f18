#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `MICS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MICSR {
    #[doc = "MCLK divider input clock 0 is selected."]
    _00,
    #[doc = "MCLK divider input clock 1 is selected."]
    _01,
    #[doc = "MCLK divider input clock 2 is selected."]
    _10,
    #[doc = "MCLK divider input clock 3 is selected."]
    _11,
}
impl MICSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MICSR::_00 => 0,
            MICSR::_01 => 1,
            MICSR::_10 => 2,
            MICSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MICSR {
        match value {
            0 => MICSR::_00,
            1 => MICSR::_01,
            2 => MICSR::_10,
            3 => MICSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MICSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MICSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MICSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MICSR::_11
    }
}
#[doc = "Possible values of the field `MOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOER {
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    _0,
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    _1,
}
impl MOER {
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
            MOER::_0 => false,
            MOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOER {
        match value {
            false => MOER::_0,
            true => MOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MOER::_1
    }
}
#[doc = "Possible values of the field `DUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUFR {
    #[doc = "MCLK divider ratio is not being updated currently."]
    _0,
    #[doc = "MCLK divider ratio is updating on-the-fly. Further updates to the MCLK divider ratio are blocked while this flag remains set."]
    _1,
}
impl DUFR {
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
            DUFR::_0 => false,
            DUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DUFR {
        match value {
            false => DUFR::_0,
            true => DUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DUFR::_1
    }
}
#[doc = "Values that can be written to the field `MICS`"]
pub enum MICSW {
    #[doc = "MCLK divider input clock 0 is selected."]
    _00,
    #[doc = "MCLK divider input clock 1 is selected."]
    _01,
    #[doc = "MCLK divider input clock 2 is selected."]
    _10,
    #[doc = "MCLK divider input clock 3 is selected."]
    _11,
}
impl MICSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MICSW::_00 => 0,
            MICSW::_01 => 1,
            MICSW::_10 => 2,
            MICSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MICSW<'a> {
    w: &'a mut W,
}
impl<'a> _MICSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MICSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCLK divider input clock 0 is selected."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MICSW::_00)
    }
    #[doc = "MCLK divider input clock 1 is selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MICSW::_01)
    }
    #[doc = "MCLK divider input clock 2 is selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MICSW::_10)
    }
    #[doc = "MCLK divider input clock 3 is selected."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MICSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MOE`"]
pub enum MOEW {
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    _0,
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    _1,
}
impl MOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOEW::_0 => false,
            MOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOEW<'a> {
    w: &'a mut W,
}
impl<'a> _MOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOEW::_0)
    }
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOEW::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline]
    pub fn mics(&self) -> MICSR {
        MICSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline]
    pub fn moe(&self) -> MOER {
        MOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Divider Update Flag"]
    #[inline]
    pub fn duf(&self) -> DUFR {
        DUFR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline]
    pub fn mics(&mut self) -> _MICSW {
        _MICSW { w: self }
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline]
    pub fn moe(&mut self) -> _MOEW {
        _MOEW { w: self }
    }
}
