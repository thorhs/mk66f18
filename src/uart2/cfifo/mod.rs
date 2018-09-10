#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CFIFO {
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
#[doc = "Possible values of the field `RXUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFER {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    _1,
}
impl RXUFER {
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
            RXUFER::_0 => false,
            RXUFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFER {
        match value {
            false => RXUFER::_0,
            true => RXUFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXUFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXUFER::_1
    }
}
#[doc = "Possible values of the field `TXOFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFER {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    _1,
}
impl TXOFER {
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
            TXOFER::_0 => false,
            TXOFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFER {
        match value {
            false => TXOFER::_0,
            true => TXOFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXOFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXOFER::_1
    }
}
#[doc = "Possible values of the field `RXOFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFER {
    #[doc = "RXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXOF flag generates an interrupt to the host."]
    _1,
}
impl RXOFER {
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
            RXOFER::_0 => false,
            RXOFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOFER {
        match value {
            false => RXOFER::_0,
            true => RXOFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXOFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXOFER::_1
    }
}
#[doc = "Values that can be written to the field `RXUFE`"]
pub enum RXUFEW {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    _1,
}
impl RXUFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFEW::_0 => false,
            RXUFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFEW::_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFEW::_1)
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
#[doc = "Values that can be written to the field `TXOFE`"]
pub enum TXOFEW {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    _1,
}
impl TXOFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFEW::_0 => false,
            TXOFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFEW::_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFEW::_1)
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
#[doc = "Values that can be written to the field `RXOFE`"]
pub enum RXOFEW {
    #[doc = "RXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXOF flag generates an interrupt to the host."]
    _1,
}
impl RXOFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOFEW::_0 => false,
            RXOFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXOF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFEW::_0)
    }
    #[doc = "RXOF flag generates an interrupt to the host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFEW::_1)
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
#[doc = "Values that can be written to the field `RXFLUSH`"]
pub enum RXFLUSHW {
    #[doc = "No flush operation occurs."]
    _0,
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    _1,
}
impl RXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFLUSHW::_0 => false,
            RXFLUSHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFLUSHW::_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFLUSHW::_1)
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
#[doc = "Values that can be written to the field `TXFLUSH`"]
pub enum TXFLUSHW {
    #[doc = "No flush operation occurs."]
    _0,
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    _1,
}
impl TXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFLUSHW::_0 => false,
            TXFLUSHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFLUSHW::_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFLUSHW::_1)
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
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&self) -> RXUFER {
        RXUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&self) -> TXOFER {
        TXOFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn rxofe(&self) -> RXOFER {
        RXOFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&mut self) -> _RXUFEW {
        _RXUFEW { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&mut self) -> _TXOFEW {
        _TXOFEW { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn rxofe(&mut self) -> _RXOFEW {
        _RXOFEW { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO/Buffer Flush"]
    #[inline]
    pub fn rxflush(&mut self) -> _RXFLUSHW {
        _RXFLUSHW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO/Buffer Flush"]
    #[inline]
    pub fn txflush(&mut self) -> _TXFLUSHW {
        _TXFLUSHW { w: self }
    }
}
