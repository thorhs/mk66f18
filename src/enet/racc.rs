#[doc = "Reader of register RACC"]
pub type R = crate::R<u32, super::RACC>;
#[doc = "Writer for register RACC"]
pub type W = crate::W<u32, super::RACC>;
#[doc = "Register RACC `reset()`'s with value 0"]
impl crate::ResetValue for super::RACC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Padding Removal For Short IP Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADREM_A {
    #[doc = "0: Padding not removed."]
    _0,
    #[doc = "1: Any bytes following the IP payload section of the frame are removed from the frame."]
    _1,
}
impl From<PADREM_A> for bool {
    #[inline(always)]
    fn from(variant: PADREM_A) -> Self {
        match variant {
            PADREM_A::_0 => false,
            PADREM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PADREM`"]
pub type PADREM_R = crate::R<bool, PADREM_A>;
impl PADREM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADREM_A {
        match self.bits {
            false => PADREM_A::_0,
            true => PADREM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PADREM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PADREM_A::_1
    }
}
#[doc = "Write proxy for field `PADREM`"]
pub struct PADREM_W<'a> {
    w: &'a mut W,
}
impl<'a> PADREM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADREM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Padding not removed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PADREM_A::_0)
    }
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PADREM_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPDIS_A {
    #[doc = "0: Frames with wrong IPv4 header checksum are not discarded."]
    _0,
    #[doc = "1: If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl From<IPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: IPDIS_A) -> Self {
        match variant {
            IPDIS_A::_0 => false,
            IPDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IPDIS`"]
pub type IPDIS_R = crate::R<bool, IPDIS_A>;
impl IPDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPDIS_A {
        match self.bits {
            false => IPDIS_A::_0,
            true => IPDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPDIS_A::_1
    }
}
#[doc = "Write proxy for field `IPDIS`"]
pub struct IPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPDIS_A::_0)
    }
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPDIS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable Discard Of Frames With Wrong Protocol Checksum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRODIS_A {
    #[doc = "0: Frames with wrong checksum are not discarded."]
    _0,
    #[doc = "1: If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1,
}
impl From<PRODIS_A> for bool {
    #[inline(always)]
    fn from(variant: PRODIS_A) -> Self {
        match variant {
            PRODIS_A::_0 => false,
            PRODIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PRODIS`"]
pub type PRODIS_R = crate::R<bool, PRODIS_A>;
impl PRODIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRODIS_A {
        match self.bits {
            false => PRODIS_A::_0,
            true => PRODIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRODIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRODIS_A::_1
    }
}
#[doc = "Write proxy for field `PRODIS`"]
pub struct PRODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRODIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frames with wrong checksum are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRODIS_A::_0)
    }
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRODIS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable Discard Of Frames With MAC Layer Errors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINEDIS_A {
    #[doc = "0: Frames with errors are not discarded."]
    _0,
    #[doc = "1: Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    _1,
}
impl From<LINEDIS_A> for bool {
    #[inline(always)]
    fn from(variant: LINEDIS_A) -> Self {
        match variant {
            LINEDIS_A::_0 => false,
            LINEDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LINEDIS`"]
pub type LINEDIS_R = crate::R<bool, LINEDIS_A>;
impl LINEDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEDIS_A {
        match self.bits {
            false => LINEDIS_A::_0,
            true => LINEDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINEDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINEDIS_A::_1
    }
}
#[doc = "Write proxy for field `LINEDIS`"]
pub struct LINEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LINEDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINEDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frames with errors are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINEDIS_A::_0)
    }
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINEDIS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "RX FIFO Shift-16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIFT16_A {
    #[doc = "0: Disabled."]
    _0,
    #[doc = "1: Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    _1,
}
impl From<SHIFT16_A> for bool {
    #[inline(always)]
    fn from(variant: SHIFT16_A) -> Self {
        match variant {
            SHIFT16_A::_0 => false,
            SHIFT16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SHIFT16`"]
pub type SHIFT16_R = crate::R<bool, SHIFT16_A>;
impl SHIFT16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIFT16_A {
        match self.bits {
            false => SHIFT16_A::_0,
            true => SHIFT16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHIFT16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHIFT16_A::_1
    }
}
#[doc = "Write proxy for field `SHIFT16`"]
pub struct SHIFT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIFT16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHIFT16_A::_0)
    }
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHIFT16_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    pub fn padrem(&self) -> PADREM_R {
        PADREM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    pub fn ipdis(&self) -> IPDIS_R {
        IPDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    pub fn prodis(&self) -> PRODIS_R {
        PRODIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    pub fn linedis(&self) -> LINEDIS_R {
        LINEDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&self) -> SHIFT16_R {
        SHIFT16_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    pub fn padrem(&mut self) -> PADREM_W {
        PADREM_W { w: self }
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    pub fn ipdis(&mut self) -> IPDIS_W {
        IPDIS_W { w: self }
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    pub fn prodis(&mut self) -> PRODIS_W {
        PRODIS_W { w: self }
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    pub fn linedis(&mut self) -> LINEDIS_W {
        LINEDIS_W { w: self }
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&mut self) -> SHIFT16_W {
        SHIFT16_W { w: self }
    }
}
