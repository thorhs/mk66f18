#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCSR {
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
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "DMA request is disabled"]
    _0,
    #[doc = "DMA request is enabled"]
    _1,
}
impl TDRER {
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
            TDRER::_0 => false,
            TDRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::_0,
            true => TDRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDRER::_1
    }
}
#[doc = "Possible values of the field `TMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMODER {
    #[doc = "Timer Channel is disabled."]
    _0000,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    _0001,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    _0010,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    _0011,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    _0100,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    _0101,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    _0110,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    _0111,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    _1010,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    _1110,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMODER::_0000 => 0,
            TMODER::_0001 => 1,
            TMODER::_0010 => 2,
            TMODER::_0011 => 3,
            TMODER::_0100 => 4,
            TMODER::_0101 => 5,
            TMODER::_0110 => 6,
            TMODER::_0111 => 7,
            TMODER::_1010 => 10,
            TMODER::_1110 => 14,
            TMODER::_1111 => 15,
            TMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMODER {
        match value {
            0 => TMODER::_0000,
            1 => TMODER::_0001,
            2 => TMODER::_0010,
            3 => TMODER::_0011,
            4 => TMODER::_0100,
            5 => TMODER::_0101,
            6 => TMODER::_0110,
            7 => TMODER::_0111,
            10 => TMODER::_1010,
            14 => TMODER::_1110,
            15 => TMODER::_1111,
            i => TMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == TMODER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == TMODER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == TMODER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == TMODER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == TMODER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == TMODER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == TMODER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == TMODER::_0111
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == TMODER::_1010
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == TMODER::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == TMODER::_1111
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Interrupt is disabled"]
    _0,
    #[doc = "Interrupt is enabled"]
    _1,
}
impl TIER {
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
            TIER::_0 => false,
            TIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::_0,
            true => TIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIER::_1
    }
}
#[doc = "Possible values of the field `TF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFR {
    #[doc = "Input Capture or Output Compare has not occurred."]
    _0,
    #[doc = "Input Capture or Output Compare has occurred."]
    _1,
}
impl TFR {
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
            TFR::_0 => false,
            TFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFR {
        match value {
            false => TFR::_0,
            true => TFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFR::_1
    }
}
#[doc = "Values that can be written to the field `TDRE`"]
pub enum TDREW {
    #[doc = "DMA request is disabled"]
    _0,
    #[doc = "DMA request is enabled"]
    _1,
}
impl TDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDREW::_0 => false,
            TDREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDREW<'a> {
    w: &'a mut W,
}
impl<'a> _TDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDREW::_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDREW::_1)
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
#[doc = "Values that can be written to the field `TMODE`"]
pub enum TMODEW {
    #[doc = "Timer Channel is disabled."]
    _0000,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    _0001,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    _0010,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    _0011,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    _0100,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    _0101,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    _0110,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    _0111,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    _1010,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    _1110,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    _1111,
}
impl TMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMODEW::_0000 => 0,
            TMODEW::_0001 => 1,
            TMODEW::_0010 => 2,
            TMODEW::_0011 => 3,
            TMODEW::_0100 => 4,
            TMODEW::_0101 => 5,
            TMODEW::_0110 => 6,
            TMODEW::_0111 => 7,
            TMODEW::_1010 => 10,
            TMODEW::_1110 => 14,
            TMODEW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer Channel is disabled."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TMODEW::_0000)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TMODEW::_0001)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TMODEW::_0010)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TMODEW::_0011)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TMODEW::_0100)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TMODEW::_0101)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TMODEW::_0110)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TMODEW::_0111)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TMODEW::_1010)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TMODEW::_1110)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TMODEW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Interrupt is disabled"]
    _0,
    #[doc = "Interrupt is enabled"]
    _1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::_0 => false,
            TIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIEW::_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TF`"]
pub enum TFW {
    #[doc = "Input Capture or Output Compare has not occurred."]
    _0,
    #[doc = "Input Capture or Output Compare has occurred."]
    _1,
}
impl TFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFW::_0 => false,
            TFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFW::_0)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFW::_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline]
    pub fn tmode(&self) -> TMODER {
        TMODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline]
    pub fn tf(&self) -> TFR {
        TFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&mut self) -> _TDREW {
        _TDREW { w: self }
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline]
    pub fn tmode(&mut self) -> _TMODEW {
        _TMODEW { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline]
    pub fn tf(&mut self) -> _TFW {
        _TFW { w: self }
    }
}
