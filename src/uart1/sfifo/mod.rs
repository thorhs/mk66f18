#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SFIFO {
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
#[doc = "Possible values of the field `RXUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFR {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXUFR {
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
            RXUFR::_0 => false,
            RXUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFR {
        match value {
            false => RXUFR::_0,
            true => RXUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXUFR::_1
    }
}
#[doc = "Possible values of the field `TXOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFR {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl TXOFR {
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
            TXOFR::_0 => false,
            TXOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFR {
        match value {
            false => TXOFR::_0,
            true => TXOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXOFR::_1
    }
}
#[doc = "Possible values of the field `RXOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFR {
    #[doc = "No receive buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXOFR {
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
            RXOFR::_0 => false,
            RXOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOFR {
        match value {
            false => RXOFR::_0,
            true => RXOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXOFR::_1
    }
}
#[doc = "Possible values of the field `RXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTR {
    #[doc = "Receive buffer is not empty."]
    _0,
    #[doc = "Receive buffer is empty."]
    _1,
}
impl RXEMPTR {
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
            RXEMPTR::_0 => false,
            RXEMPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEMPTR {
        match value {
            false => RXEMPTR::_0,
            true => RXEMPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEMPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEMPTR::_1
    }
}
#[doc = "Possible values of the field `TXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTR {
    #[doc = "Transmit buffer is not empty."]
    _0,
    #[doc = "Transmit buffer is empty."]
    _1,
}
impl TXEMPTR {
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
            TXEMPTR::_0 => false,
            TXEMPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTR {
        match value {
            false => TXEMPTR::_0,
            true => TXEMPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTR::_1
    }
}
#[doc = "Values that can be written to the field `RXUF`"]
pub enum RXUFW {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFW::_0 => false,
            RXUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFW::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFW::_1)
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
#[doc = "Values that can be written to the field `TXOF`"]
pub enum TXOFW {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl TXOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFW::_0 => false,
            TXOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFW::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFW::_1)
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
#[doc = "Values that can be written to the field `RXOF`"]
pub enum RXOFW {
    #[doc = "No receive buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOFW::_0 => false,
            RXOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFW::_0)
    }
    #[doc = "At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&self) -> RXUFR {
        RXUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&self) -> TXOFR {
        TXOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline]
    pub fn rxof(&self) -> RXOFR {
        RXOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Receive Buffer/FIFO Empty"]
    #[inline]
    pub fn rxempt(&self) -> RXEMPTR {
        RXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Buffer/FIFO Empty"]
    #[inline]
    pub fn txempt(&self) -> TXEMPTR {
        TXEMPTR::_from({
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
        W { bits: 192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&mut self) -> _RXUFW {
        _RXUFW { w: self }
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&mut self) -> _TXOFW {
        _TXOFW { w: self }
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline]
    pub fn rxof(&mut self) -> _RXOFW {
        _RXOFW { w: self }
    }
}
