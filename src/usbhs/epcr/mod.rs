#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPCR {
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
#[doc = r" Value of the field"]
pub struct RXDR {
    bits: bool,
}
impl RXDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `RXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTR {
    #[doc = "Control"]
    _00,
    #[doc = "Isochronous"]
    _01,
    #[doc = "Bulk"]
    _10,
    #[doc = "Interrupt"]
    _11,
}
impl RXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXTR::_00 => 0,
            RXTR::_01 => 1,
            RXTR::_10 => 2,
            RXTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXTR {
        match value {
            0 => RXTR::_00,
            1 => RXTR::_01,
            2 => RXTR::_10,
            3 => RXTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RXTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RXTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RXTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RXTR::_11
    }
}
#[doc = "Possible values of the field `RXI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIR {
    #[doc = "PID sequencing enabled"]
    _0,
    #[doc = "PID sequencing disabled"]
    _1,
}
impl RXIR {
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
            RXIR::_0 => false,
            RXIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIR {
        match value {
            false => RXIR::_0,
            true => RXIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXIR::_1
    }
}
#[doc = "Possible values of the field `RXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXER {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
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
            RXER::_0 => false,
            RXER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXER {
        match value {
            false => RXER::_0,
            true => RXER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXER::_0
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
#[doc = r" Value of the field"]
pub struct TXDR {
    bits: bool,
}
impl TXDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTR {
    #[doc = "Control"]
    _00,
    #[doc = "Isochronous"]
    _01,
    #[doc = "Bulk"]
    _10,
    #[doc = "Interrupt"]
    _11,
}
impl TXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXTR::_00 => 0,
            TXTR::_01 => 1,
            TXTR::_10 => 2,
            TXTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXTR {
        match value {
            0 => TXTR::_00,
            1 => TXTR::_01,
            2 => TXTR::_10,
            3 => TXTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TXTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TXTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TXTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TXTR::_11
    }
}
#[doc = "Possible values of the field `TXI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIR {
    #[doc = "PID sequencing enabled"]
    _0,
    #[doc = "PID sequencing disabled"]
    _1,
}
impl TXIR {
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
            TXIR::_0 => false,
            TXIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIR {
        match value {
            false => TXIR::_0,
            true => TXIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXIR::_1
    }
}
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
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
            TXER::_0 => false,
            TXER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            false => TXER::_0,
            true => TXER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXER::_0
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
#[doc = r" Proxy"]
pub struct _RXDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXT`"]
pub enum RXTW {
    #[doc = "Control"]
    _00,
    #[doc = "Isochronous"]
    _01,
    #[doc = "Bulk"]
    _10,
    #[doc = "Interrupt"]
    _11,
}
impl RXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTW::_00 => 0,
            RXTW::_01 => 1,
            RXTW::_10 => 2,
            RXTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RXTW::_00)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RXTW::_01)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RXTW::_10)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RXTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXI`"]
pub enum RXIW {
    #[doc = "PID sequencing enabled"]
    _0,
    #[doc = "PID sequencing disabled"]
    _1,
}
impl RXIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIW::_0 => false,
            RXIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PID sequencing enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXIW::_0)
    }
    #[doc = "PID sequencing disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXIW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRW<'a> {
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
#[doc = "Values that can be written to the field `RXE`"]
pub enum RXEW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl RXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEW::_0 => false,
            RXEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEW::_1)
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
#[doc = r" Proxy"]
pub struct _TXDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDW<'a> {
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
#[doc = "Values that can be written to the field `TXT`"]
pub enum TXTW {
    #[doc = "Control"]
    _00,
    #[doc = "Isochronous"]
    _01,
    #[doc = "Bulk"]
    _10,
    #[doc = "Interrupt"]
    _11,
}
impl TXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXTW::_00 => 0,
            TXTW::_01 => 1,
            TXTW::_10 => 2,
            TXTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TXTW::_00)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TXTW::_01)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TXTW::_10)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TXTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXI`"]
pub enum TXIW {
    #[doc = "PID sequencing enabled"]
    _0,
    #[doc = "PID sequencing disabled"]
    _1,
}
impl TXIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIW::_0 => false,
            TXIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PID sequencing enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXIW::_0)
    }
    #[doc = "PID sequencing disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXIW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXE`"]
pub enum TXEW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl TXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEW::_0 => false,
            TXEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXEW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXEW::_1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 1 - RX endpoint Data sink"]
    #[inline]
    pub fn rxd(&self) -> RXDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDR { bits }
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
    #[doc = "Bit 5 - RX data toggle Inhibit"]
    #[inline]
    pub fn rxi(&self) -> RXIR {
        RXIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 16 - TX endpoint Stall"]
    #[inline]
    pub fn txs(&self) -> TXSR {
        TXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TX endpoint Data source"]
    #[inline]
    pub fn txd(&self) -> TXDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXDR { bits }
    }
    #[doc = "Bits 18:19 - TX endpoint Type"]
    #[inline]
    pub fn txt(&self) -> TXTR {
        TXTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - TX data toggle Inhibit"]
    #[inline]
    pub fn txi(&self) -> TXIR {
        TXIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - TX endpoint Enable"]
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
        W { bits: 0 }
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
    #[doc = "Bit 1 - RX endpoint Data sink"]
    #[inline]
    pub fn rxd(&mut self) -> _RXDW {
        _RXDW { w: self }
    }
    #[doc = "Bits 2:3 - RX endpoint Type"]
    #[inline]
    pub fn rxt(&mut self) -> _RXTW {
        _RXTW { w: self }
    }
    #[doc = "Bit 5 - RX data toggle Inhibit"]
    #[inline]
    pub fn rxi(&mut self) -> _RXIW {
        _RXIW { w: self }
    }
    #[doc = "Bit 6 - RX data toggle Reset"]
    #[inline]
    pub fn rxr(&mut self) -> _RXRW {
        _RXRW { w: self }
    }
    #[doc = "Bit 7 - RX endpoint Enable"]
    #[inline]
    pub fn rxe(&mut self) -> _RXEW {
        _RXEW { w: self }
    }
    #[doc = "Bit 16 - TX endpoint Stall"]
    #[inline]
    pub fn txs(&mut self) -> _TXSW {
        _TXSW { w: self }
    }
    #[doc = "Bit 17 - TX endpoint Data source"]
    #[inline]
    pub fn txd(&mut self) -> _TXDW {
        _TXDW { w: self }
    }
    #[doc = "Bits 18:19 - TX endpoint Type"]
    #[inline]
    pub fn txt(&mut self) -> _TXTW {
        _TXTW { w: self }
    }
    #[doc = "Bit 21 - TX data toggle Inhibit"]
    #[inline]
    pub fn txi(&mut self) -> _TXIW {
        _TXIW { w: self }
    }
    #[doc = "Bit 22 - TX data toggle Reset"]
    #[inline]
    pub fn txr(&mut self) -> _TXRW {
        _TXRW { w: self }
    }
    #[doc = "Bit 23 - TX endpoint Enable"]
    #[inline]
    pub fn txe(&mut self) -> _TXEW {
        _TXEW { w: self }
    }
}
