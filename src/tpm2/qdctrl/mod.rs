#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QDCTRL {
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
#[doc = "Possible values of the field `QUADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADENR {
    #[doc = "Quadrature decoder mode is disabled."]
    _0,
    #[doc = "Quadrature decoder mode is enabled."]
    _1,
}
impl QUADENR {
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
            QUADENR::_0 => false,
            QUADENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADENR {
        match value {
            false => QUADENR::_0,
            true => QUADENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADENR::_1
    }
}
#[doc = "Possible values of the field `TOFDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIRR {
    #[doc = "TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (zero) to its maximum value (MOD register)."]
    _0,
    #[doc = "TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (zero)."]
    _1,
}
impl TOFDIRR {
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
            TOFDIRR::_0 => false,
            TOFDIRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOFDIRR {
        match value {
            false => TOFDIRR::_0,
            true => TOFDIRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOFDIRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOFDIRR::_1
    }
}
#[doc = "Possible values of the field `QUADIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIRR {
    #[doc = "Counter direction is decreasing (counter decrement)."]
    _0,
    #[doc = "Counter direction is increasing (counter increment)."]
    _1,
}
impl QUADIRR {
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
            QUADIRR::_0 => false,
            QUADIRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADIRR {
        match value {
            false => QUADIRR::_0,
            true => QUADIRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADIRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADIRR::_1
    }
}
#[doc = "Possible values of the field `QUADMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODER {
    #[doc = "Phase encoding mode."]
    _0,
    #[doc = "Count and direction encoding mode."]
    _1,
}
impl QUADMODER {
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
            QUADMODER::_0 => false,
            QUADMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADMODER {
        match value {
            false => QUADMODER::_0,
            true => QUADMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADMODER::_1
    }
}
#[doc = "Values that can be written to the field `QUADEN`"]
pub enum QUADENW {
    #[doc = "Quadrature decoder mode is disabled."]
    _0,
    #[doc = "Quadrature decoder mode is enabled."]
    _1,
}
impl QUADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QUADENW::_0 => false,
            QUADENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUADENW<'a> {
    w: &'a mut W,
}
impl<'a> _QUADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature decoder mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADENW::_0)
    }
    #[doc = "Quadrature decoder mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADENW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QUADMODE`"]
pub enum QUADMODEW {
    #[doc = "Phase encoding mode."]
    _0,
    #[doc = "Count and direction encoding mode."]
    _1,
}
impl QUADMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QUADMODEW::_0 => false,
            QUADMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUADMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QUADMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUADMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase encoding mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADMODEW::_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADMODEW::_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Enables the quadrature decoder mode"]
    #[inline]
    pub fn quaden(&self) -> QUADENR {
        QUADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates if the TOF bit was set on the top or the bottom of counting."]
    #[inline]
    pub fn tofdir(&self) -> TOFDIRR {
        TOFDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Counter Direction in Quadrature Decode Mode"]
    #[inline]
    pub fn quadir(&self) -> QUADIRR {
        QUADIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline]
    pub fn quadmode(&self) -> QUADMODER {
        QUADMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Enables the quadrature decoder mode"]
    #[inline]
    pub fn quaden(&mut self) -> _QUADENW {
        _QUADENW { w: self }
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline]
    pub fn quadmode(&mut self) -> _QUADMODEW {
        _QUADMODEW { w: self }
    }
}
