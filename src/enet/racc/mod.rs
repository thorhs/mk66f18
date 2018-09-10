#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RACC {
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
#[doc = "Possible values of the field `PADREM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADREMR {
    #[doc = "Padding not removed."]
    _0,
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    _1,
}
impl PADREMR {
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
            PADREMR::_0 => false,
            PADREMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PADREMR {
        match value {
            false => PADREMR::_0,
            true => PADREMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PADREMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PADREMR::_1
    }
}
#[doc = "Possible values of the field `IPDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPDISR {
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    _0,
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl IPDISR {
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
            IPDISR::_0 => false,
            IPDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPDISR {
        match value {
            false => IPDISR::_0,
            true => IPDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPDISR::_1
    }
}
#[doc = "Possible values of the field `PRODIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRODISR {
    #[doc = "Frames with wrong checksum are not discarded."]
    _0,
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl PRODISR {
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
            PRODISR::_0 => false,
            PRODISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRODISR {
        match value {
            false => PRODISR::_0,
            true => PRODISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRODISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRODISR::_1
    }
}
#[doc = "Possible values of the field `LINEDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINEDISR {
    #[doc = "Frames with errors are not discarded."]
    _0,
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    _1,
}
impl LINEDISR {
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
            LINEDISR::_0 => false,
            LINEDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINEDISR {
        match value {
            false => LINEDISR::_0,
            true => LINEDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LINEDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LINEDISR::_1
    }
}
#[doc = "Possible values of the field `SHIFT16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIFT16R {
    #[doc = "Disabled."]
    _0,
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    _1,
}
impl SHIFT16R {
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
            SHIFT16R::_0 => false,
            SHIFT16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHIFT16R {
        match value {
            false => SHIFT16R::_0,
            true => SHIFT16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SHIFT16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SHIFT16R::_1
    }
}
#[doc = "Values that can be written to the field `PADREM`"]
pub enum PADREMW {
    #[doc = "Padding not removed."]
    _0,
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    _1,
}
impl PADREMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PADREMW::_0 => false,
            PADREMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PADREMW<'a> {
    w: &'a mut W,
}
impl<'a> _PADREMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PADREMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Padding not removed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PADREMW::_0)
    }
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PADREMW::_1)
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
#[doc = "Values that can be written to the field `IPDIS`"]
pub enum IPDISW {
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    _0,
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl IPDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPDISW::_0 => false,
            IPDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPDISW<'a> {
    w: &'a mut W,
}
impl<'a> _IPDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPDISW::_0)
    }
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPDISW::_1)
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
#[doc = "Values that can be written to the field `PRODIS`"]
pub enum PRODISW {
    #[doc = "Frames with wrong checksum are not discarded."]
    _0,
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl PRODISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRODISW::_0 => false,
            PRODISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRODISW<'a> {
    w: &'a mut W,
}
impl<'a> _PRODISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRODISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frames with wrong checksum are not discarded."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRODISW::_0)
    }
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRODISW::_1)
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
#[doc = "Values that can be written to the field `LINEDIS`"]
pub enum LINEDISW {
    #[doc = "Frames with errors are not discarded."]
    _0,
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    _1,
}
impl LINEDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINEDISW::_0 => false,
            LINEDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINEDISW<'a> {
    w: &'a mut W,
}
impl<'a> _LINEDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINEDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frames with errors are not discarded."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINEDISW::_0)
    }
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINEDISW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SHIFT16`"]
pub enum SHIFT16W {
    #[doc = "Disabled."]
    _0,
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    _1,
}
impl SHIFT16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHIFT16W::_0 => false,
            SHIFT16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHIFT16W<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFT16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHIFT16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHIFT16W::_0)
    }
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHIFT16W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline]
    pub fn padrem(&self) -> PADREMR {
        PADREMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline]
    pub fn ipdis(&self) -> IPDISR {
        IPDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline]
    pub fn prodis(&self) -> PRODISR {
        PRODISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline]
    pub fn linedis(&self) -> LINEDISR {
        LINEDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline]
    pub fn shift16(&self) -> SHIFT16R {
        SHIFT16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline]
    pub fn padrem(&mut self) -> _PADREMW {
        _PADREMW { w: self }
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline]
    pub fn ipdis(&mut self) -> _IPDISW {
        _IPDISW { w: self }
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline]
    pub fn prodis(&mut self) -> _PRODISW {
        _PRODISW { w: self }
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline]
    pub fn linedis(&mut self) -> _LINEDISW {
        _LINEDISW { w: self }
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline]
    pub fn shift16(&mut self) -> _SHIFT16W {
        _SHIFT16W { w: self }
    }
}
