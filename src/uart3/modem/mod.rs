#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MODEM {
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
#[doc = "Possible values of the field `TXCTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSER {
    #[doc = "CTS has no effect on the transmitter."]
    _0,
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1,
}
impl TXCTSER {
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
            TXCTSER::_0 => false,
            TXCTSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCTSER {
        match value {
            false => TXCTSER::_0,
            true => TXCTSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXCTSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXCTSER::_1
    }
}
#[doc = "Possible values of the field `TXRTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSER {
    #[doc = "The transmitter has no effect on RTS."]
    _0,
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    _1,
}
impl TXRTSER {
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
            TXRTSER::_0 => false,
            TXRTSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRTSER {
        match value {
            false => TXRTSER::_0,
            true => TXRTSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXRTSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXRTSER::_1
    }
}
#[doc = "Possible values of the field `TXRTSPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSPOLR {
    #[doc = "Transmitter RTS is active low."]
    _0,
    #[doc = "Transmitter RTS is active high."]
    _1,
}
impl TXRTSPOLR {
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
            TXRTSPOLR::_0 => false,
            TXRTSPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRTSPOLR {
        match value {
            false => TXRTSPOLR::_0,
            true => TXRTSPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXRTSPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXRTSPOLR::_1
    }
}
#[doc = "Possible values of the field `RXRTSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRTSER {
    #[doc = "The receiver has no effect on RTS."]
    _0,
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]. See Hardware flow control"]
    _1,
}
impl RXRTSER {
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
            RXRTSER::_0 => false,
            RXRTSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRTSER {
        match value {
            false => RXRTSER::_0,
            true => RXRTSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXRTSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXRTSER::_1
    }
}
#[doc = "Values that can be written to the field `TXCTSE`"]
pub enum TXCTSEW {
    #[doc = "CTS has no effect on the transmitter."]
    _0,
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1,
}
impl TXCTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCTSEW::_0 => false,
            TXCTSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CTS has no effect on the transmitter."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSEW::_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSEW::_1)
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
#[doc = "Values that can be written to the field `TXRTSE`"]
pub enum TXRTSEW {
    #[doc = "The transmitter has no effect on RTS."]
    _0,
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    _1,
}
impl TXRTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRTSEW::_0 => false,
            TXRTSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmitter has no effect on RTS."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSEW::_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSEW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXRTSPOL`"]
pub enum TXRTSPOLW {
    #[doc = "Transmitter RTS is active low."]
    _0,
    #[doc = "Transmitter RTS is active high."]
    _1,
}
impl TXRTSPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRTSPOLW::_0 => false,
            TXRTSPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRTSPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRTSPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRTSPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter RTS is active low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSPOLW::_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSPOLW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXRTSE`"]
pub enum RXRTSEW {
    #[doc = "The receiver has no effect on RTS."]
    _0,
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]. See Hardware flow control"]
    _1,
}
impl RXRTSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRTSEW::_0 => false,
            RXRTSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRTSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRTSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRTSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver has no effect on RTS."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRTSEW::_0)
    }
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]. See Hardware flow control"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRTSEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline]
    pub fn txctse(&self) -> TXCTSER {
        TXCTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline]
    pub fn txrtse(&self) -> TXRTSER {
        TXRTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline]
    pub fn txrtspol(&self) -> TXRTSPOLR {
        TXRTSPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline]
    pub fn rxrtse(&self) -> RXRTSER {
        RXRTSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline]
    pub fn txctse(&mut self) -> _TXCTSEW {
        _TXCTSEW { w: self }
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline]
    pub fn txrtse(&mut self) -> _TXRTSEW {
        _TXRTSEW { w: self }
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline]
    pub fn txrtspol(&mut self) -> _TXRTSPOLW {
        _TXRTSPOLW { w: self }
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline]
    pub fn rxrtse(&mut self) -> _RXRTSEW {
        _RXRTSEW { w: self }
    }
}
