#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ESR1 {
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
#[doc = "Possible values of the field `WAKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKINTR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    _1,
}
impl WAKINTR {
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
            WAKINTR::_0 => false,
            WAKINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKINTR {
        match value {
            false => WAKINTR::_0,
            true => WAKINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKINTR::_1
    }
}
#[doc = "Possible values of the field `ERRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINTR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    _1,
}
impl ERRINTR {
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
            ERRINTR::_0 => false,
            ERRINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRINTR {
        match value {
            false => ERRINTR::_0,
            true => ERRINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRINTR::_1
    }
}
#[doc = "Possible values of the field `BOFFINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFINTR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "FlexCAN module entered Bus Off state."]
    _1,
}
impl BOFFINTR {
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
            BOFFINTR::_0 => false,
            BOFFINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFINTR {
        match value {
            false => BOFFINTR::_0,
            true => BOFFINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOFFINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOFFINTR::_1
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "FlexCAN is not receiving a message."]
    _0,
    #[doc = "FlexCAN is receiving a message."]
    _1,
}
impl RXR {
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
            RXR::_0 => false,
            RXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXR {
        match value {
            false => RXR::_0,
            true => RXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXR::_1
    }
}
#[doc = "Possible values of the field `FLTCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTCONFR {
    #[doc = "Error Active"]
    _00,
    #[doc = "Error Passive"]
    _01,
    #[doc = "Bus Off"]
    _1X,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLTCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLTCONFR::_00 => 0,
            FLTCONFR::_01 => 1,
            FLTCONFR::_1X => 2,
            FLTCONFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLTCONFR {
        match value {
            0 => FLTCONFR::_00,
            1 => FLTCONFR::_01,
            2 => FLTCONFR::_1X,
            i => FLTCONFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FLTCONFR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FLTCONFR::_01
    }
    #[doc = "Checks if the value of the field is `_1X`"]
    #[inline]
    pub fn is_1x(&self) -> bool {
        *self == FLTCONFR::_1X
    }
}
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "FlexCAN is not transmitting a message."]
    _0,
    #[doc = "FlexCAN is transmitting a message."]
    _1,
}
impl TXR {
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
            TXR::_0 => false,
            TXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXR {
        match value {
            false => TXR::_0,
            true => TXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXR::_1
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "CAN bus is now IDLE."]
    _1,
}
impl IDLER {
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
            IDLER::_0 => false,
            IDLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::_0,
            true => IDLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDLER::_1
    }
}
#[doc = "Possible values of the field `RXWRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWRNR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "RXERRCNT is greater than or equal to 96."]
    _1,
}
impl RXWRNR {
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
            RXWRNR::_0 => false,
            RXWRNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXWRNR {
        match value {
            false => RXWRNR::_0,
            true => RXWRNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXWRNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXWRNR::_1
    }
}
#[doc = "Possible values of the field `TXWRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXWRNR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "TXERRCNT is greater than or equal to 96."]
    _1,
}
impl TXWRNR {
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
            TXWRNR::_0 => false,
            TXWRNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXWRNR {
        match value {
            false => TXWRNR::_0,
            true => TXWRNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXWRNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXWRNR::_1
    }
}
#[doc = "Possible values of the field `STFERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "A Stuffing Error occurred since last read of this register."]
    _1,
}
impl STFERRR {
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
            STFERRR::_0 => false,
            STFERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STFERRR {
        match value {
            false => STFERRR::_0,
            true => STFERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STFERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STFERRR::_1
    }
}
#[doc = "Possible values of the field `FRMERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "A Form Error occurred since last read of this register."]
    _1,
}
impl FRMERRR {
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
            FRMERRR::_0 => false,
            FRMERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRMERRR {
        match value {
            false => FRMERRR::_0,
            true => FRMERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRMERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRMERRR::_1
    }
}
#[doc = "Possible values of the field `CRCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "A CRC error occurred since last read of this register."]
    _1,
}
impl CRCERRR {
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
            CRCERRR::_0 => false,
            CRCERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCERRR {
        match value {
            false => CRCERRR::_0,
            true => CRCERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCERRR::_1
    }
}
#[doc = "Possible values of the field `ACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "An ACK error occurred since last read of this register."]
    _1,
}
impl ACKERRR {
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
            ACKERRR::_0 => false,
            ACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKERRR {
        match value {
            false => ACKERRR::_0,
            true => ACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACKERRR::_1
    }
}
#[doc = "Possible values of the field `BIT0ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "At least one bit sent as dominant is received as recessive."]
    _1,
}
impl BIT0ERRR {
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
            BIT0ERRR::_0 => false,
            BIT0ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT0ERRR {
        match value {
            false => BIT0ERRR::_0,
            true => BIT0ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BIT0ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BIT0ERRR::_1
    }
}
#[doc = "Possible values of the field `BIT1ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERRR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "At least one bit sent as recessive is received as dominant."]
    _1,
}
impl BIT1ERRR {
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
            BIT1ERRR::_0 => false,
            BIT1ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT1ERRR {
        match value {
            false => BIT1ERRR::_0,
            true => BIT1ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BIT1ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BIT1ERRR::_1
    }
}
#[doc = "Possible values of the field `RWRNINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNINTR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl RWRNINTR {
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
            RWRNINTR::_0 => false,
            RWRNINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWRNINTR {
        match value {
            false => RWRNINTR::_0,
            true => RWRNINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWRNINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWRNINTR::_1
    }
}
#[doc = "Possible values of the field `TWRNINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNINTR {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl TWRNINTR {
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
            TWRNINTR::_0 => false,
            TWRNINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWRNINTR {
        match value {
            false => TWRNINTR::_0,
            true => TWRNINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWRNINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWRNINTR::_1
    }
}
#[doc = "Possible values of the field `SYNCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHR {
    #[doc = "FlexCAN is not synchronized to the CAN bus."]
    _0,
    #[doc = "FlexCAN is synchronized to the CAN bus."]
    _1,
}
impl SYNCHR {
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
            SYNCHR::_0 => false,
            SYNCHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCHR {
        match value {
            false => SYNCHR::_0,
            true => SYNCHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCHR::_1
    }
}
#[doc = "Values that can be written to the field `WAKINT`"]
pub enum WAKINTW {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    _1,
}
impl WAKINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKINTW::_0 => false,
            WAKINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKINTW::_0)
    }
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKINTW::_1)
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
#[doc = "Values that can be written to the field `ERRINT`"]
pub enum ERRINTW {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    _1,
}
impl ERRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRINTW::_0 => false,
            ERRINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRINTW::_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRINTW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOFFINT`"]
pub enum BOFFINTW {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "FlexCAN module entered Bus Off state."]
    _1,
}
impl BOFFINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFINTW::_0 => false,
            BOFFINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFINTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFINTW::_0)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFINTW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RWRNINT`"]
pub enum RWRNINTW {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl RWRNINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWRNINTW::_0 => false,
            RWRNINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWRNINTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWRNINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWRNINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWRNINTW::_0)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWRNINTW::_1)
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
#[doc = "Values that can be written to the field `TWRNINT`"]
pub enum TWRNINTW {
    #[doc = "No such occurrence."]
    _0,
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl TWRNINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWRNINTW::_0 => false,
            TWRNINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWRNINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRNINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWRNINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRNINTW::_0)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRNINTW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline]
    pub fn wakint(&self) -> WAKINTR {
        WAKINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline]
    pub fn errint(&self) -> ERRINTR {
        ERRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline]
    pub fn boffint(&self) -> BOFFINTR {
        BOFFINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline]
    pub fn fltconf(&self) -> FLTCONFR {
        FLTCONFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - This bit indicates when CAN bus is in IDLE state"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline]
    pub fn rxwrn(&self) -> RXWRNR {
        RXWRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline]
    pub fn txwrn(&self) -> TXWRNR {
        TXWRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline]
    pub fn stferr(&self) -> STFERRR {
        STFERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline]
    pub fn frmerr(&self) -> FRMERRR {
        FRMERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline]
    pub fn crcerr(&self) -> CRCERRR {
        CRCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline]
    pub fn ackerr(&self) -> ACKERRR {
        ACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline]
    pub fn bit0err(&self) -> BIT0ERRR {
        BIT0ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline]
    pub fn bit1err(&self) -> BIT1ERRR {
        BIT1ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline]
    pub fn rwrnint(&self) -> RWRNINTR {
        RWRNINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline]
    pub fn twrnint(&self) -> TWRNINTR {
        TWRNINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline]
    pub fn synch(&self) -> SYNCHR {
        SYNCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline]
    pub fn wakint(&mut self) -> _WAKINTW {
        _WAKINTW { w: self }
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline]
    pub fn errint(&mut self) -> _ERRINTW {
        _ERRINTW { w: self }
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline]
    pub fn boffint(&mut self) -> _BOFFINTW {
        _BOFFINTW { w: self }
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline]
    pub fn rwrnint(&mut self) -> _RWRNINTW {
        _RWRNINTW { w: self }
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline]
    pub fn twrnint(&mut self) -> _TWRNINTW {
        _TWRNINTW { w: self }
    }
}
