#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR {
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
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Loopback disabled."]
    _0,
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    _1,
}
impl LOOPR {
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
            LOOPR::_0 => false,
            LOOPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::_0,
            true => LOOPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOOPR::_1
    }
}
#[doc = "Possible values of the field `DRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRTR {
    #[doc = "Receive path operates independently of transmit. Used for full-duplex or to monitor transmit activity in half-duplex mode."]
    _0,
    #[doc = "Disable reception of frames while transmitting. Normally used for half-duplex mode."]
    _1,
}
impl DRTR {
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
            DRTR::_0 => false,
            DRTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRTR {
        match value {
            false => DRTR::_0,
            true => DRTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DRTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DRTR::_1
    }
}
#[doc = "Possible values of the field `MII_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MII_MODER {
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MII_MODER {
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
            MII_MODER::_1 => true,
            MII_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MII_MODER {
        match value {
            true => MII_MODER::_1,
            i => MII_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MII_MODER::_1
    }
}
#[doc = "Possible values of the field `PROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROMR {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enabled."]
    _1,
}
impl PROMR {
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
            PROMR::_0 => false,
            PROMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROMR {
        match value {
            false => PROMR::_0,
            true => PROMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PROMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PROMR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BC_REJR {
    bits: bool,
}
impl BC_REJR {
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
#[doc = r" Value of the field"]
pub struct FCER {
    bits: bool,
}
impl FCER {
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
#[doc = "Possible values of the field `RMII_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_MODER {
    #[doc = "MAC configured for MII mode."]
    _0,
    #[doc = "MAC configured for RMII operation."]
    _1,
}
impl RMII_MODER {
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
            RMII_MODER::_0 => false,
            RMII_MODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMII_MODER {
        match value {
            false => RMII_MODER::_0,
            true => RMII_MODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RMII_MODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RMII_MODER::_1
    }
}
#[doc = "Possible values of the field `RMII_10T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_10TR {
    #[doc = "100 Mbps operation."]
    _0,
    #[doc = "10 Mbps operation."]
    _1,
}
impl RMII_10TR {
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
            RMII_10TR::_0 => false,
            RMII_10TR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMII_10TR {
        match value {
            false => RMII_10TR::_0,
            true => RMII_10TR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RMII_10TR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RMII_10TR::_1
    }
}
#[doc = "Possible values of the field `PADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADENR {
    #[doc = "No padding is removed on receive by the MAC."]
    _0,
    #[doc = "Padding is removed from received frames."]
    _1,
}
impl PADENR {
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
            PADENR::_0 => false,
            PADENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PADENR {
        match value {
            false => PADENR::_0,
            true => PADENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PADENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PADENR::_1
    }
}
#[doc = "Possible values of the field `PAUFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUFWDR {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    _0,
    #[doc = "Pause frames are forwarded to the user application."]
    _1,
}
impl PAUFWDR {
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
            PAUFWDR::_0 => false,
            PAUFWDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAUFWDR {
        match value {
            false => PAUFWDR::_0,
            true => PAUFWDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PAUFWDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PAUFWDR::_1
    }
}
#[doc = "Possible values of the field `CRCFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWDR {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    _0,
    #[doc = "The CRC field is stripped from the frame."]
    _1,
}
impl CRCFWDR {
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
            CRCFWDR::_0 => false,
            CRCFWDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCFWDR {
        match value {
            false => CRCFWDR::_0,
            true => CRCFWDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCFWDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCFWDR::_1
    }
}
#[doc = "Possible values of the field `CFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFENR {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    _0,
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    _1,
}
impl CFENR {
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
            CFENR::_0 => false,
            CFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFENR {
        match value {
            false => CFENR::_0,
            true => CFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CFENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct MAX_FLR {
    bits: u16,
}
impl MAX_FLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `NLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NLCR {
    #[doc = "The payload length check is disabled."]
    _0,
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLC\\] field."]
    _1,
}
impl NLCR {
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
            NLCR::_0 => false,
            NLCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NLCR {
        match value {
            false => NLCR::_0,
            true => NLCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NLCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NLCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct GRSR {
    bits: bool,
}
impl GRSR {
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
#[doc = "Values that can be written to the field `LOOP`"]
pub enum LOOPW {
    #[doc = "Loopback disabled."]
    _0,
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    _1,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::_0 => false,
            LOOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loopback disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPW::_0)
    }
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPW::_1)
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
#[doc = "Values that can be written to the field `DRT`"]
pub enum DRTW {
    #[doc = "Receive path operates independently of transmit. Used for full-duplex or to monitor transmit activity in half-duplex mode."]
    _0,
    #[doc = "Disable reception of frames while transmitting. Normally used for half-duplex mode."]
    _1,
}
impl DRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRTW::_0 => false,
            DRTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive path operates independently of transmit. Used for full-duplex or to monitor transmit activity in half-duplex mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRTW::_0)
    }
    #[doc = "Disable reception of frames while transmitting. Normally used for half-duplex mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRTW::_1)
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
#[doc = "Values that can be written to the field `MII_MODE`"]
pub enum MII_MODEW {
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    _1,
}
impl MII_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MII_MODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MII_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MII_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MII_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MII_MODEW::_1)
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
#[doc = "Values that can be written to the field `PROM`"]
pub enum PROMW {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enabled."]
    _1,
}
impl PROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROMW::_0 => false,
            PROMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROMW<'a> {
    w: &'a mut W,
}
impl<'a> _PROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROMW::_0)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROMW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BC_REJW<'a> {
    w: &'a mut W,
}
impl<'a> _BC_REJW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCEW<'a> {
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
#[doc = "Values that can be written to the field `RMII_MODE`"]
pub enum RMII_MODEW {
    #[doc = "MAC configured for MII mode."]
    _0,
    #[doc = "MAC configured for RMII operation."]
    _1,
}
impl RMII_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMII_MODEW::_0 => false,
            RMII_MODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMII_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RMII_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMII_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC configured for MII mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMII_MODEW::_0)
    }
    #[doc = "MAC configured for RMII operation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMII_MODEW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMII_10T`"]
pub enum RMII_10TW {
    #[doc = "100 Mbps operation."]
    _0,
    #[doc = "10 Mbps operation."]
    _1,
}
impl RMII_10TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMII_10TW::_0 => false,
            RMII_10TW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMII_10TW<'a> {
    w: &'a mut W,
}
impl<'a> _RMII_10TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMII_10TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "100 Mbps operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMII_10TW::_0)
    }
    #[doc = "10 Mbps operation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMII_10TW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PADEN`"]
pub enum PADENW {
    #[doc = "No padding is removed on receive by the MAC."]
    _0,
    #[doc = "Padding is removed from received frames."]
    _1,
}
impl PADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PADENW::_0 => false,
            PADENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PADENW<'a> {
    w: &'a mut W,
}
impl<'a> _PADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No padding is removed on receive by the MAC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PADENW::_0)
    }
    #[doc = "Padding is removed from received frames."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PADENW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAUFWD`"]
pub enum PAUFWDW {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    _0,
    #[doc = "Pause frames are forwarded to the user application."]
    _1,
}
impl PAUFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAUFWDW::_0 => false,
            PAUFWDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAUFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAUFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUFWDW::_0)
    }
    #[doc = "Pause frames are forwarded to the user application."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUFWDW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCFWD`"]
pub enum CRCFWDW {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    _0,
    #[doc = "The CRC field is stripped from the frame."]
    _1,
}
impl CRCFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCFWDW::_0 => false,
            CRCFWDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWDW::_0)
    }
    #[doc = "The CRC field is stripped from the frame."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCFWDW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEN`"]
pub enum CFENW {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    _0,
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    _1,
}
impl CFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFENW::_0 => false,
            CFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFENW::_0)
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFENW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_FLW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_FLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NLC`"]
pub enum NLCW {
    #[doc = "The payload length check is disabled."]
    _0,
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLC\\] field."]
    _1,
}
impl NLCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NLCW::_0 => false,
            NLCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NLCW<'a> {
    w: &'a mut W,
}
impl<'a> _NLCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NLCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The payload length check is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NLCW::_0)
    }
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLC\\] field."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NLCW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline]
    pub fn drt(&self) -> DRTR {
        DRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline]
    pub fn mii_mode(&self) -> MII_MODER {
        MII_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline]
    pub fn prom(&self) -> PROMR {
        PROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline]
    pub fn bc_rej(&self) -> BC_REJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BC_REJR { bits }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline]
    pub fn fce(&self) -> FCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCER { bits }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline]
    pub fn rmii_mode(&self) -> RMII_MODER {
        RMII_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables 10-Mbps mode of the RMII ."]
    #[inline]
    pub fn rmii_10t(&self) -> RMII_10TR {
        RMII_10TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline]
    pub fn paden(&self) -> PADENR {
        PADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline]
    pub fn paufwd(&self) -> PAUFWDR {
        PAUFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline]
    pub fn crcfwd(&self) -> CRCFWDR {
        CRCFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline]
    pub fn cfen(&self) -> CFENR {
        CFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline]
    pub fn max_fl(&self) -> MAX_FLR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MAX_FLR { bits }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline]
    pub fn nlc(&self) -> NLCR {
        NLCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Graceful Receive Stopped"]
    #[inline]
    pub fn grs(&self) -> GRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 99483649 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline]
    pub fn drt(&mut self) -> _DRTW {
        _DRTW { w: self }
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline]
    pub fn mii_mode(&mut self) -> _MII_MODEW {
        _MII_MODEW { w: self }
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline]
    pub fn prom(&mut self) -> _PROMW {
        _PROMW { w: self }
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline]
    pub fn bc_rej(&mut self) -> _BC_REJW {
        _BC_REJW { w: self }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline]
    pub fn fce(&mut self) -> _FCEW {
        _FCEW { w: self }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline]
    pub fn rmii_mode(&mut self) -> _RMII_MODEW {
        _RMII_MODEW { w: self }
    }
    #[doc = "Bit 9 - Enables 10-Mbps mode of the RMII ."]
    #[inline]
    pub fn rmii_10t(&mut self) -> _RMII_10TW {
        _RMII_10TW { w: self }
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline]
    pub fn paden(&mut self) -> _PADENW {
        _PADENW { w: self }
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline]
    pub fn paufwd(&mut self) -> _PAUFWDW {
        _PAUFWDW { w: self }
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline]
    pub fn crcfwd(&mut self) -> _CRCFWDW {
        _CRCFWDW { w: self }
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline]
    pub fn cfen(&mut self) -> _CFENW {
        _CFENW { w: self }
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline]
    pub fn max_fl(&mut self) -> _MAX_FLW {
        _MAX_FLW { w: self }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline]
    pub fn nlc(&mut self) -> _NLCW {
        _NLCW { w: self }
    }
}
