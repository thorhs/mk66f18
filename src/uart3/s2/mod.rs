#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::S2 {
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
#[doc = "Possible values of the field `RAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAFR {
    #[doc = "UART receiver idle/inactive waiting for a start bit."]
    _0,
    #[doc = "UART receiver active, RxD input not idle."]
    _1,
}
impl RAFR {
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
            RAFR::_0 => false,
            RAFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAFR {
        match value {
            false => RAFR::_0,
            true => RAFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAFR::_1
    }
}
#[doc = "Possible values of the field `LBKDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDER {
    #[doc = "Break character detection is disabled."]
    _0,
    #[doc = "Break character is detected at length of 11 bit times if C1\\[M\\] = 0 or 12 bits time if C1\\[M\\] = 1."]
    _1,
}
impl LBKDER {
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
            LBKDER::_0 => false,
            LBKDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDER {
        match value {
            false => LBKDER::_0,
            true => LBKDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDER::_1
    }
}
#[doc = "Possible values of the field `BRK13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13R {
    #[doc = "Break character is 10, 11, or 12 bits long."]
    _0,
    #[doc = "Break character is 13 or 14 bits long."]
    _1,
}
impl BRK13R {
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
            BRK13R::_0 => false,
            BRK13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRK13R {
        match value {
            false => BRK13R::_0,
            true => BRK13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRK13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRK13R::_1
    }
}
#[doc = "Possible values of the field `RWUID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUIDR {
    #[doc = "S1\\[IDLE\\] is not set upon detection of an idle character."]
    _0,
    #[doc = "S1\\[IDLE\\] is set upon detection of an idle character."]
    _1,
}
impl RWUIDR {
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
            RWUIDR::_0 => false,
            RWUIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUIDR {
        match value {
            false => RWUIDR::_0,
            true => RWUIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWUIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWUIDR::_1
    }
}
#[doc = "Possible values of the field `RXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINVR {
    #[doc = "Receive data is not inverted."]
    _0,
    #[doc = "Receive data is inverted."]
    _1,
}
impl RXINVR {
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
            RXINVR::_0 => false,
            RXINVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXINVR {
        match value {
            false => RXINVR::_0,
            true => RXINVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXINVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXINVR::_1
    }
}
#[doc = "Possible values of the field `MSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFR {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0,
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\] and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\] and C1\\[PE\\]."]
    _1,
}
impl MSBFR {
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
            MSBFR::_0 => false,
            MSBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBFR {
        match value {
            false => MSBFR::_0,
            true => MSBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSBFR::_1
    }
}
#[doc = "Possible values of the field `RXEDGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIFR {
    #[doc = "No active edge on the receive pin has occurred."]
    _0,
    #[doc = "An active edge on the receive pin has occurred."]
    _1,
}
impl RXEDGIFR {
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
            RXEDGIFR::_0 => false,
            RXEDGIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIFR {
        match value {
            false => RXEDGIFR::_0,
            true => RXEDGIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIFR::_1
    }
}
#[doc = "Possible values of the field `LBKDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIFR {
    #[doc = "No LIN break character detected."]
    _0,
    #[doc = "LIN break character detected."]
    _1,
}
impl LBKDIFR {
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
            LBKDIFR::_0 => false,
            LBKDIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIFR {
        match value {
            false => LBKDIFR::_0,
            true => LBKDIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDIFR::_1
    }
}
#[doc = "Values that can be written to the field `LBKDE`"]
pub enum LBKDEW {
    #[doc = "Break character detection is disabled."]
    _0,
    #[doc = "Break character is detected at length of 11 bit times if C1\\[M\\] = 0 or 12 bits time if C1\\[M\\] = 1."]
    _1,
}
impl LBKDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDEW::_0 => false,
            LBKDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break character detection is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDEW::_0)
    }
    #[doc = "Break character is detected at length of 11 bit times if C1\\[M\\] = 0 or 12 bits time if C1\\[M\\] = 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDEW::_1)
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
#[doc = "Values that can be written to the field `BRK13`"]
pub enum BRK13W {
    #[doc = "Break character is 10, 11, or 12 bits long."]
    _0,
    #[doc = "Break character is 13 or 14 bits long."]
    _1,
}
impl BRK13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRK13W::_0 => false,
            BRK13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRK13W<'a> {
    w: &'a mut W,
}
impl<'a> _BRK13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRK13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break character is 10, 11, or 12 bits long."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13W::_0)
    }
    #[doc = "Break character is 13 or 14 bits long."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13W::_1)
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
#[doc = "Values that can be written to the field `RWUID`"]
pub enum RWUIDW {
    #[doc = "S1\\[IDLE\\] is not set upon detection of an idle character."]
    _0,
    #[doc = "S1\\[IDLE\\] is set upon detection of an idle character."]
    _1,
}
impl RWUIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUIDW::_0 => false,
            RWUIDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUIDW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "S1\\[IDLE\\] is not set upon detection of an idle character."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUIDW::_0)
    }
    #[doc = "S1\\[IDLE\\] is set upon detection of an idle character."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUIDW::_1)
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
#[doc = "Values that can be written to the field `RXINV`"]
pub enum RXINVW {
    #[doc = "Receive data is not inverted."]
    _0,
    #[doc = "Receive data is inverted."]
    _1,
}
impl RXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXINVW::_0 => false,
            RXINVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive data is not inverted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINVW::_0)
    }
    #[doc = "Receive data is inverted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINVW::_1)
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
#[doc = "Values that can be written to the field `MSBF`"]
pub enum MSBFW {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0,
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\] and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\] and C1\\[PE\\]."]
    _1,
}
impl MSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBFW::_0 => false,
            MSBFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBFW::_0)
    }
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\] and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\] and C1\\[PE\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBFW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXEDGIF`"]
pub enum RXEDGIFW {
    #[doc = "No active edge on the receive pin has occurred."]
    _0,
    #[doc = "An active edge on the receive pin has occurred."]
    _1,
}
impl RXEDGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIFW::_0 => false,
            RXEDGIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIFW::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIFW::_1)
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
#[doc = "Values that can be written to the field `LBKDIF`"]
pub enum LBKDIFW {
    #[doc = "No LIN break character detected."]
    _0,
    #[doc = "LIN break character detected."]
    _1,
}
impl LBKDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIFW::_0 => false,
            LBKDIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No LIN break character detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIFW::_0)
    }
    #[doc = "LIN break character detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIFW::_1)
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
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline]
    pub fn raf(&self) -> RAFR {
        RAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&self) -> LBKDER {
        LBKDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline]
    pub fn brk13(&self) -> BRK13R {
        BRK13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline]
    pub fn rwuid(&self) -> RWUIDR {
        RWUIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        RXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        MSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&self) -> RXEDGIFR {
        RXEDGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&self) -> LBKDIFR {
        LBKDIFR::_from({
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
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&mut self) -> _LBKDEW {
        _LBKDEW { w: self }
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline]
    pub fn brk13(&mut self) -> _BRK13W {
        _BRK13W { w: self }
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline]
    pub fn rwuid(&mut self) -> _RWUIDW {
        _RWUIDW { w: self }
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&mut self) -> _RXEDGIFW {
        _RXEDGIFW { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&mut self) -> _LBKDIFW {
        _LBKDIFW { w: self }
    }
}
