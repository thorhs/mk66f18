#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPCR0 {
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
#[doc = "Possible values of the field `RXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSR {
    #[doc = "Endpoint OK"]
    _0,
    #[doc = "Endpoint stalled"]
    _1,
}
impl RXSR {
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
            RXSR::_0 => false,
            RXSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSR {
        match value {
            false => RXSR::_0,
            true => RXSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXSR::_1
    }
}
#[doc = "Possible values of the field `RXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTR {
    #[doc = "Control"]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXTR::_00 => 0,
            RXTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXTR {
        match value {
            0 => RXTR::_00,
            i => RXTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RXTR::_00
    }
}
#[doc = "Possible values of the field `RXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXER {
    #[doc = "Enabled"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXER {
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
            RXER::_1 => true,
            RXER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXER {
        match value {
            true => RXER::_1,
            i => RXER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXER::_1
    }
}
#[doc = "Possible values of the field `TXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSR {
    #[doc = "Endpoint OK"]
    _0,
    #[doc = "Endpoint stalled"]
    _1,
}
impl TXSR {
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
            TXSR::_0 => false,
            TXSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSR {
        match value {
            false => TXSR::_0,
            true => TXSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXSR::_1
    }
}
#[doc = "Possible values of the field `TXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTR {
    #[doc = "Control"]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXTR::_00 => 0,
            TXTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXTR {
        match value {
            0 => TXTR::_00,
            i => TXTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TXTR::_00
    }
}
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Enable"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TXER {
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
            TXER::_1 => true,
            TXER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            true => TXER::_1,
            i => TXER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXER::_1
    }
}
#[doc = "Values that can be written to the field `RXS`"]
pub enum RXSW {
    #[doc = "Endpoint OK"]
    _0,
    #[doc = "Endpoint stalled"]
    _1,
}
impl RXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSW::_0 => false,
            RXSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSW::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXSW::_1)
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
#[doc = "Values that can be written to the field `TXS`"]
pub enum TXSW {
    #[doc = "Endpoint OK"]
    _0,
    #[doc = "Endpoint stalled"]
    _1,
}
impl TXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSW::_0 => false,
            TXSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXSW::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXSW::_1)
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
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline]
    pub fn rxs(&self) -> RXSR {
        RXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - RX endpoint Type"]
    #[inline]
    pub fn rxt(&self) -> RXTR {
        RXTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - RX endpoint Enable"]
    #[inline]
    pub fn rxe(&self) -> RXER {
        RXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TX Endpoint Stall"]
    #[inline]
    pub fn txs(&self) -> TXSR {
        TXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - TX Endpoint Type"]
    #[inline]
    pub fn txt(&self) -> TXTR {
        TXTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - TX Endpoint Enable"]
    #[inline]
    pub fn txe(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8388736 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline]
    pub fn rxs(&mut self) -> _RXSW {
        _RXSW { w: self }
    }
    #[doc = "Bit 16 - TX Endpoint Stall"]
    #[inline]
    pub fn txs(&mut self) -> _TXSW {
        _TXSW { w: self }
    }
}
