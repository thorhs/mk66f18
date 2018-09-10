#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
pub struct POPNXTPTRR {
    bits: u8,
}
impl POPNXTPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXCTRR {
    bits: u8,
}
impl RXCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXNXTPTRR {
    bits: u8,
}
impl TXNXTPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCTRR {
    bits: u8,
}
impl TXCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RFDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDFR {
    #[doc = "RX FIFO is empty."]
    _0,
    #[doc = "RX FIFO is not empty."]
    _1,
}
impl RFDFR {
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
            RFDFR::_0 => false,
            RFDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFDFR {
        match value {
            false => RFDFR::_0,
            true => RFDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFDFR::_1
    }
}
#[doc = "Possible values of the field `RFOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOFR {
    #[doc = "No Rx FIFO overflow."]
    _0,
    #[doc = "Rx FIFO overflow has occurred."]
    _1,
}
impl RFOFR {
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
            RFOFR::_0 => false,
            RFOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFOFR {
        match value {
            false => RFOFR::_0,
            true => RFOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFOFR::_1
    }
}
#[doc = "Possible values of the field `TFFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFFR {
    #[doc = "TX FIFO is full."]
    _0,
    #[doc = "TX FIFO is not full."]
    _1,
}
impl TFFFR {
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
            TFFFR::_0 => false,
            TFFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFFR {
        match value {
            false => TFFFR::_0,
            true => TFFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFFFR::_1
    }
}
#[doc = "Possible values of the field `TFUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFUFR {
    #[doc = "No TX FIFO underflow."]
    _0,
    #[doc = "TX FIFO underflow has occurred."]
    _1,
}
impl TFUFR {
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
            TFUFR::_0 => false,
            TFUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFUFR {
        match value {
            false => TFUFR::_0,
            true => TFUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFUFR::_1
    }
}
#[doc = "Possible values of the field `EOQF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQFR {
    #[doc = "EOQ is not set in the executing command."]
    _0,
    #[doc = "EOQ is set in the executing SPI command."]
    _1,
}
impl EOQFR {
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
            EOQFR::_0 => false,
            EOQFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOQFR {
        match value {
            false => EOQFR::_0,
            true => EOQFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOQFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOQFR::_1
    }
}
#[doc = "Possible values of the field `TXRXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXSR {
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    _0,
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    _1,
}
impl TXRXSR {
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
            TXRXSR::_0 => false,
            TXRXSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRXSR {
        match value {
            false => TXRXSR::_0,
            true => TXRXSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXRXSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXRXSR::_1
    }
}
#[doc = "Possible values of the field `TCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFR {
    #[doc = "Transfer not complete."]
    _0,
    #[doc = "Transfer complete."]
    _1,
}
impl TCFR {
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
            TCFR::_0 => false,
            TCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCFR {
        match value {
            false => TCFR::_0,
            true => TCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCFR::_1
    }
}
#[doc = "Values that can be written to the field `RFDF`"]
pub enum RFDFW {
    #[doc = "RX FIFO is empty."]
    _0,
    #[doc = "RX FIFO is not empty."]
    _1,
}
impl RFDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFDFW::_0 => false,
            RFDFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFDFW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX FIFO is empty."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDFW::_0)
    }
    #[doc = "RX FIFO is not empty."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDFW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFOF`"]
pub enum RFOFW {
    #[doc = "No Rx FIFO overflow."]
    _0,
    #[doc = "Rx FIFO overflow has occurred."]
    _1,
}
impl RFOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFOFW::_0 => false,
            RFOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFOFW<'a> {
    w: &'a mut W,
}
impl<'a> _RFOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Rx FIFO overflow."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOFW::_0)
    }
    #[doc = "Rx FIFO overflow has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOFW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFFF`"]
pub enum TFFFW {
    #[doc = "TX FIFO is full."]
    _0,
    #[doc = "TX FIFO is not full."]
    _1,
}
impl TFFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFFW::_0 => false,
            TFFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX FIFO is full."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFFW::_0)
    }
    #[doc = "TX FIFO is not full."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFFW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFUF`"]
pub enum TFUFW {
    #[doc = "No TX FIFO underflow."]
    _0,
    #[doc = "TX FIFO underflow has occurred."]
    _1,
}
impl TFUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFUFW::_0 => false,
            TFUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFUFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No TX FIFO underflow."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUFW::_0)
    }
    #[doc = "TX FIFO underflow has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUFW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOQF`"]
pub enum EOQFW {
    #[doc = "EOQ is not set in the executing command."]
    _0,
    #[doc = "EOQ is set in the executing SPI command."]
    _1,
}
impl EOQFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOQFW::_0 => false,
            EOQFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOQFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOQFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOQFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EOQ is not set in the executing command."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQFW::_0)
    }
    #[doc = "EOQ is set in the executing SPI command."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQFW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXRXS`"]
pub enum TXRXSW {
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    _0,
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    _1,
}
impl TXRXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRXSW::_0 => false,
            TXRXSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRXSW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRXSW::_0)
    }
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRXSW::_1)
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
#[doc = "Values that can be written to the field `TCF`"]
pub enum TCFW {
    #[doc = "Transfer not complete."]
    _0,
    #[doc = "Transfer complete."]
    _1,
}
impl TCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCFW::_0 => false,
            TCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer not complete."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFW::_0)
    }
    #[doc = "Transfer complete."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:3 - Pop Next Pointer"]
    #[inline]
    pub fn popnxtptr(&self) -> POPNXTPTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POPNXTPTRR { bits }
    }
    #[doc = "Bits 4:7 - RX FIFO Counter"]
    #[inline]
    pub fn rxctr(&self) -> RXCTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXCTRR { bits }
    }
    #[doc = "Bits 8:11 - Transmit Next Pointer"]
    #[inline]
    pub fn txnxtptr(&self) -> TXNXTPTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXNXTPTRR { bits }
    }
    #[doc = "Bits 12:15 - TX FIFO Counter"]
    #[inline]
    pub fn txctr(&self) -> TXCTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCTRR { bits }
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline]
    pub fn rfdf(&self) -> RFDFR {
        RFDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline]
    pub fn rfof(&self) -> RFOFR {
        RFOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline]
    pub fn tfff(&self) -> TFFFR {
        TFFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline]
    pub fn tfuf(&self) -> TFUFR {
        TFUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline]
    pub fn eoqf(&self) -> EOQFR {
        EOQFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline]
    pub fn txrxs(&self) -> TXRXSR {
        TXRXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        TCFR::_from({
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
        W { bits: 33554432 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline]
    pub fn rfdf(&mut self) -> _RFDFW {
        _RFDFW { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline]
    pub fn rfof(&mut self) -> _RFOFW {
        _RFOFW { w: self }
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline]
    pub fn tfff(&mut self) -> _TFFFW {
        _TFFFW { w: self }
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline]
    pub fn tfuf(&mut self) -> _TFUFW {
        _TFUFW { w: self }
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline]
    pub fn eoqf(&mut self) -> _EOQFW {
        _EOQFW { w: self }
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline]
    pub fn txrxs(&mut self) -> _TXRXSW {
        _TXRXSW { w: self }
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
}
