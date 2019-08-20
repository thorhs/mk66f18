#[doc = "Reader of register MMCBOOT"]
pub type R = crate::R<u32, super::MMCBOOT>;
#[doc = "Writer for register MMCBOOT"]
pub type W = crate::W<u32, super::MMCBOOT>;
#[doc = "Register MMCBOOT `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCBOOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Boot ACK Time Out Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCVACK_A {
    #[doc = "0: SDCLK x 2^8"]
    _0000,
    #[doc = "1: SDCLK x 2^9"]
    _0001,
    #[doc = "2: SDCLK x 2^10"]
    _0010,
    #[doc = "3: SDCLK x 2^11"]
    _0011,
    #[doc = "4: SDCLK x 2^12"]
    _0100,
    #[doc = "5: SDCLK x 2^13"]
    _0101,
    #[doc = "6: SDCLK x 2^14"]
    _0110,
    #[doc = "7: SDCLK x 2^15"]
    _0111,
    #[doc = "14: SDCLK x 2^22"]
    _1110,
}
impl From<DTOCVACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCVACK_A) -> Self {
        match variant {
            DTOCVACK_A::_0000 => 0,
            DTOCVACK_A::_0001 => 1,
            DTOCVACK_A::_0010 => 2,
            DTOCVACK_A::_0011 => 3,
            DTOCVACK_A::_0100 => 4,
            DTOCVACK_A::_0101 => 5,
            DTOCVACK_A::_0110 => 6,
            DTOCVACK_A::_0111 => 7,
            DTOCVACK_A::_1110 => 14,
        }
    }
}
#[doc = "Reader of field `DTOCVACK`"]
pub type DTOCVACK_R = crate::R<u8, DTOCVACK_A>;
impl DTOCVACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTOCVACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTOCVACK_A::_0000),
            1 => Val(DTOCVACK_A::_0001),
            2 => Val(DTOCVACK_A::_0010),
            3 => Val(DTOCVACK_A::_0011),
            4 => Val(DTOCVACK_A::_0100),
            5 => Val(DTOCVACK_A::_0101),
            6 => Val(DTOCVACK_A::_0110),
            7 => Val(DTOCVACK_A::_0111),
            14 => Val(DTOCVACK_A::_1110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DTOCVACK_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DTOCVACK_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DTOCVACK_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DTOCVACK_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DTOCVACK_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DTOCVACK_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DTOCVACK_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DTOCVACK_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DTOCVACK_A::_1110
    }
}
#[doc = "Write proxy for field `DTOCVACK`"]
pub struct DTOCVACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCVACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOCVACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDCLK x 2^8"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0000)
    }
    #[doc = "SDCLK x 2^9"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0001)
    }
    #[doc = "SDCLK x 2^10"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0010)
    }
    #[doc = "SDCLK x 2^11"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0011)
    }
    #[doc = "SDCLK x 2^12"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0100)
    }
    #[doc = "SDCLK x 2^13"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0101)
    }
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0110)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0111)
    }
    #[doc = "SDCLK x 2^22"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_1110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Boot Ack Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTACK_A {
    #[doc = "0: No ack."]
    _0,
    #[doc = "1: Ack."]
    _1,
}
impl From<BOOTACK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTACK_A) -> Self {
        match variant {
            BOOTACK_A::_0 => false,
            BOOTACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BOOTACK`"]
pub type BOOTACK_R = crate::R<bool, BOOTACK_A>;
impl BOOTACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTACK_A {
        match self.bits {
            false => BOOTACK_A::_0,
            true => BOOTACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTACK_A::_1
    }
}
#[doc = "Write proxy for field `BOOTACK`"]
pub struct BOOTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ack."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTACK_A::_0)
    }
    #[doc = "Ack."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTACK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Boot Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTMODE_A {
    #[doc = "0: Normal boot."]
    _0,
    #[doc = "1: Alternative boot."]
    _1,
}
impl From<BOOTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTMODE_A) -> Self {
        match variant {
            BOOTMODE_A::_0 => false,
            BOOTMODE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BOOTMODE`"]
pub type BOOTMODE_R = crate::R<bool, BOOTMODE_A>;
impl BOOTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTMODE_A {
        match self.bits {
            false => BOOTMODE_A::_0,
            true => BOOTMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTMODE_A::_1
    }
}
#[doc = "Write proxy for field `BOOTMODE`"]
pub struct BOOTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal boot."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTMODE_A::_0)
    }
    #[doc = "Alternative boot."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTMODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Boot Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTEN_A {
    #[doc = "0: Fast boot disable."]
    _0,
    #[doc = "1: Fast boot enable."]
    _1,
}
impl From<BOOTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTEN_A) -> Self {
        match variant {
            BOOTEN_A::_0 => false,
            BOOTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BOOTEN`"]
pub type BOOTEN_R = crate::R<bool, BOOTEN_A>;
impl BOOTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTEN_A {
        match self.bits {
            false => BOOTEN_A::_0,
            true => BOOTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTEN_A::_1
    }
}
#[doc = "Write proxy for field `BOOTEN`"]
pub struct BOOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fast boot disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTEN_A::_0)
    }
    #[doc = "Fast boot enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTEN_A::_1)
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
#[doc = "Reader of field `AUTOSABGEN`"]
pub type AUTOSABGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSABGEN`"]
pub struct AUTOSABGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSABGEN_W<'a> {
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
#[doc = "Reader of field `BOOTBLKCNT`"]
pub type BOOTBLKCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOTBLKCNT`"]
pub struct BOOTBLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTBLKCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline(always)]
    pub fn dtocvack(&self) -> DTOCVACK_R {
        DTOCVACK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline(always)]
    pub fn bootack(&self) -> BOOTACK_R {
        BOOTACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline(always)]
    pub fn autosabgen(&self) -> AUTOSABGEN_R {
        AUTOSABGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline(always)]
    pub fn bootblkcnt(&self) -> BOOTBLKCNT_R {
        BOOTBLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline(always)]
    pub fn dtocvack(&mut self) -> DTOCVACK_W {
        DTOCVACK_W { w: self }
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline(always)]
    pub fn bootack(&mut self) -> BOOTACK_W {
        BOOTACK_W { w: self }
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W {
        BOOTMODE_W { w: self }
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W {
        BOOTEN_W { w: self }
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline(always)]
    pub fn autosabgen(&mut self) -> AUTOSABGEN_W {
        AUTOSABGEN_W { w: self }
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline(always)]
    pub fn bootblkcnt(&mut self) -> BOOTBLKCNT_W {
        BOOTBLKCNT_W { w: self }
    }
}
