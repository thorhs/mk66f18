#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PFIFO {
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
#[doc = "Possible values of the field `RXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOSIZER {
    #[doc = "Receive FIFO/Buffer depth = 1 dataword."]
    _000,
    #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
    _001,
    #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
    _010,
    #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
    _011,
    #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
    _100,
    #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
    _101,
    #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFIFOSIZER::_000 => 0,
            RXFIFOSIZER::_001 => 1,
            RXFIFOSIZER::_010 => 2,
            RXFIFOSIZER::_011 => 3,
            RXFIFOSIZER::_100 => 4,
            RXFIFOSIZER::_101 => 5,
            RXFIFOSIZER::_110 => 6,
            RXFIFOSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFIFOSIZER {
        match value {
            0 => RXFIFOSIZER::_000,
            1 => RXFIFOSIZER::_001,
            2 => RXFIFOSIZER::_010,
            3 => RXFIFOSIZER::_011,
            4 => RXFIFOSIZER::_100,
            5 => RXFIFOSIZER::_101,
            6 => RXFIFOSIZER::_110,
            i => RXFIFOSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RXFIFOSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RXFIFOSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RXFIFOSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RXFIFOSIZER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RXFIFOSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RXFIFOSIZER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == RXFIFOSIZER::_110
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1,
}
impl RXFER {
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
            RXFER::_0 => false,
            RXFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFER {
        match value {
            false => RXFER::_0,
            true => RXFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXFER::_1
    }
}
#[doc = "Possible values of the field `TXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOSIZER {
    #[doc = "Transmit FIFO/Buffer depth = 1 dataword."]
    _000,
    #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
    _001,
    #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
    _010,
    #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
    _011,
    #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
    _100,
    #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
    _101,
    #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFIFOSIZER::_000 => 0,
            TXFIFOSIZER::_001 => 1,
            TXFIFOSIZER::_010 => 2,
            TXFIFOSIZER::_011 => 3,
            TXFIFOSIZER::_100 => 4,
            TXFIFOSIZER::_101 => 5,
            TXFIFOSIZER::_110 => 6,
            TXFIFOSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFIFOSIZER {
        match value {
            0 => TXFIFOSIZER::_000,
            1 => TXFIFOSIZER::_001,
            2 => TXFIFOSIZER::_010,
            3 => TXFIFOSIZER::_011,
            4 => TXFIFOSIZER::_100,
            5 => TXFIFOSIZER::_101,
            6 => TXFIFOSIZER::_110,
            i => TXFIFOSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TXFIFOSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TXFIFOSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TXFIFOSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TXFIFOSIZER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TXFIFOSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TXFIFOSIZER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TXFIFOSIZER::_110
    }
}
#[doc = "Possible values of the field `TXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFER {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1,
}
impl TXFER {
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
            TXFER::_0 => false,
            TXFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFER {
        match value {
            false => TXFER::_0,
            true => TXFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXFER::_1
    }
}
#[doc = "Values that can be written to the field `RXFE`"]
pub enum RXFEW {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1,
}
impl RXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFEW::_0 => false,
            RXFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFEW::_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFEW::_1)
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
#[doc = "Values that can be written to the field `TXFE`"]
pub enum TXFEW {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1,
}
impl TXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFEW::_0 => false,
            TXFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFEW::_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFEW::_1)
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
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline]
    pub fn rxfifosize(&self) -> RXFIFOSIZER {
        RXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        RXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline]
    pub fn txfifosize(&self) -> TXFIFOSIZER {
        TXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        TXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&mut self) -> _RXFEW {
        _RXFEW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&mut self) -> _TXFEW {
        _TXFEW { w: self }
    }
}
