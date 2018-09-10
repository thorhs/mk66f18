#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::REGSC {
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
#[doc = "Possible values of the field `BGBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBER {
    #[doc = "Bandgap buffer not enabled"]
    _0,
    #[doc = "Bandgap buffer enabled"]
    _1,
}
impl BGBER {
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
            BGBER::_0 => false,
            BGBER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGBER {
        match value {
            false => BGBER::_0,
            true => BGBER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGBER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGBER::_1
    }
}
#[doc = "Possible values of the field `REGONS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGONSR {
    #[doc = "Regulator is in stop regulation or in transition to/from it"]
    _0,
    #[doc = "Regulator is in run regulation"]
    _1,
}
impl REGONSR {
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
            REGONSR::_0 => false,
            REGONSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGONSR {
        match value {
            false => REGONSR::_0,
            true => REGONSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REGONSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REGONSR::_1
    }
}
#[doc = "Possible values of the field `ACKISO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKISOR {
    #[doc = "Peripherals and I/O pads are in normal run state."]
    _0,
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    _1,
}
impl ACKISOR {
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
            ACKISOR::_0 => false,
            ACKISOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKISOR {
        match value {
            false => ACKISOR::_0,
            true => ACKISOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACKISOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACKISOR::_1
    }
}
#[doc = "Possible values of the field `BGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGENR {
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    _0,
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    _1,
}
impl BGENR {
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
            BGENR::_0 => false,
            BGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGENR {
        match value {
            false => BGENR::_0,
            true => BGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGENR::_1
    }
}
#[doc = "Values that can be written to the field `BGBE`"]
pub enum BGBEW {
    #[doc = "Bandgap buffer not enabled"]
    _0,
    #[doc = "Bandgap buffer enabled"]
    _1,
}
impl BGBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGBEW::_0 => false,
            BGBEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGBEW<'a> {
    w: &'a mut W,
}
impl<'a> _BGBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bandgap buffer not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGBEW::_0)
    }
    #[doc = "Bandgap buffer enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGBEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACKISO`"]
pub enum ACKISOW {
    #[doc = "Peripherals and I/O pads are in normal run state."]
    _0,
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    _1,
}
impl ACKISOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKISOW::_0 => false,
            ACKISOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKISOW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKISOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKISOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripherals and I/O pads are in normal run state."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKISOW::_0)
    }
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKISOW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BGEN`"]
pub enum BGENW {
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    _0,
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    _1,
}
impl BGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGENW::_0 => false,
            BGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGENW<'a> {
    w: &'a mut W,
}
impl<'a> _BGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGENW::_0)
    }
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGENW::_1)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline]
    pub fn bgbe(&self) -> BGBER {
        BGBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Regulator In Run Regulation Status"]
    #[inline]
    pub fn regons(&self) -> REGONSR {
        REGONSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline]
    pub fn ackiso(&self) -> ACKISOR {
        ACKISOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline]
    pub fn bgen(&self) -> BGENR {
        BGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 36 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline]
    pub fn bgbe(&mut self) -> _BGBEW {
        _BGBEW { w: self }
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline]
    pub fn ackiso(&mut self) -> _ACKISOW {
        _ACKISOW { w: self }
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline]
    pub fn bgen(&mut self) -> _BGENW {
        _BGENW { w: self }
    }
}
