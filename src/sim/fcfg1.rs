#[doc = "Reader of register FCFG1"]
pub type R = crate::R<u32, super::FCFG1>;
#[doc = "Writer for register FCFG1"]
pub type W = crate::W<u32, super::FCFG1>;
#[doc = "Register FCFG1 `reset()`'s with value 0xff0f_0f00"]
impl crate::ResetValue for super::FCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff0f_0f00
    }
}
#[doc = "Flash Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDIS_A {
    #[doc = "0: Flash is enabled"]
    _0,
    #[doc = "1: Flash is disabled"]
    _1,
}
impl From<FLASHDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDIS_A) -> Self {
        match variant {
            FLASHDIS_A::_0 => false,
            FLASHDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLASHDIS`"]
pub type FLASHDIS_R = crate::R<bool, FLASHDIS_A>;
impl FLASHDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDIS_A {
        match self.bits {
            false => FLASHDIS_A::_0,
            true => FLASHDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDIS_A::_1
    }
}
#[doc = "Write proxy for field `FLASHDIS`"]
pub struct FLASHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_0)
    }
    #[doc = "Flash is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_1)
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
#[doc = "Flash Doze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDOZE_A {
    #[doc = "0: Flash remains enabled during Wait mode"]
    _0,
    #[doc = "1: Flash is disabled for the duration of Wait mode"]
    _1,
}
impl From<FLASHDOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDOZE_A) -> Self {
        match variant {
            FLASHDOZE_A::_0 => false,
            FLASHDOZE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLASHDOZE`"]
pub type FLASHDOZE_R = crate::R<bool, FLASHDOZE_A>;
impl FLASHDOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDOZE_A {
        match self.bits {
            false => FLASHDOZE_A::_0,
            true => FLASHDOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDOZE_A::_1
    }
}
#[doc = "Write proxy for field `FLASHDOZE`"]
pub struct FLASHDOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHDOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHDOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash remains enabled during Wait mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_0)
    }
    #[doc = "Flash is disabled for the duration of Wait mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_1)
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
#[doc = "Reader of field `DEPART`"]
pub type DEPART_R = crate::R<u8, u8>;
#[doc = "EEPROM size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EESIZE_A {
    #[doc = "0: 16 KB"]
    _0000,
    #[doc = "1: 8 KB"]
    _0001,
    #[doc = "2: 4 KB"]
    _0010,
    #[doc = "3: 2 KB"]
    _0011,
    #[doc = "4: 1 KB"]
    _0100,
    #[doc = "5: 512 Bytes"]
    _0101,
    #[doc = "6: 256 Bytes"]
    _0110,
    #[doc = "7: 128 Bytes"]
    _0111,
    #[doc = "8: 64 Bytes"]
    _1000,
    #[doc = "9: 32 Bytes"]
    _1001,
    #[doc = "15: 0 Bytes"]
    _1111,
}
impl From<EESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EESIZE_A) -> Self {
        match variant {
            EESIZE_A::_0000 => 0,
            EESIZE_A::_0001 => 1,
            EESIZE_A::_0010 => 2,
            EESIZE_A::_0011 => 3,
            EESIZE_A::_0100 => 4,
            EESIZE_A::_0101 => 5,
            EESIZE_A::_0110 => 6,
            EESIZE_A::_0111 => 7,
            EESIZE_A::_1000 => 8,
            EESIZE_A::_1001 => 9,
            EESIZE_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `EESIZE`"]
pub type EESIZE_R = crate::R<u8, EESIZE_A>;
impl EESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EESIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EESIZE_A::_0000),
            1 => Val(EESIZE_A::_0001),
            2 => Val(EESIZE_A::_0010),
            3 => Val(EESIZE_A::_0011),
            4 => Val(EESIZE_A::_0100),
            5 => Val(EESIZE_A::_0101),
            6 => Val(EESIZE_A::_0110),
            7 => Val(EESIZE_A::_0111),
            8 => Val(EESIZE_A::_1000),
            9 => Val(EESIZE_A::_1001),
            15 => Val(EESIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == EESIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == EESIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == EESIZE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == EESIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == EESIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == EESIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == EESIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == EESIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == EESIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == EESIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == EESIZE_A::_1111
    }
}
#[doc = "Program flash size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSIZE_A {
    #[doc = "3: 32 KB of program flash memory"]
    _0011,
    #[doc = "5: 64 KB of program flash memory"]
    _0101,
    #[doc = "7: 128 KB of program flash memory"]
    _0111,
    #[doc = "9: 256 KB of program flash memory"]
    _1001,
    #[doc = "11: 512 KB of program flash memory"]
    _1011,
    #[doc = "13: 1024 KB of program flash memory"]
    _1101,
    #[doc = "15: 2048 KB of program flash memory"]
    _1111,
}
impl From<PFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PFSIZE_A) -> Self {
        match variant {
            PFSIZE_A::_0011 => 3,
            PFSIZE_A::_0101 => 5,
            PFSIZE_A::_0111 => 7,
            PFSIZE_A::_1001 => 9,
            PFSIZE_A::_1011 => 11,
            PFSIZE_A::_1101 => 13,
            PFSIZE_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `PFSIZE`"]
pub type PFSIZE_R = crate::R<u8, PFSIZE_A>;
impl PFSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PFSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(PFSIZE_A::_0011),
            5 => Val(PFSIZE_A::_0101),
            7 => Val(PFSIZE_A::_0111),
            9 => Val(PFSIZE_A::_1001),
            11 => Val(PFSIZE_A::_1011),
            13 => Val(PFSIZE_A::_1101),
            15 => Val(PFSIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PFSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PFSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PFSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PFSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PFSIZE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PFSIZE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PFSIZE_A::_1111
    }
}
#[doc = "FlexNVM size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVMSIZE_A {
    #[doc = "0: 0 KB of FlexNVM"]
    _0000,
    #[doc = "3: 32 KB of FlexNVM"]
    _0011,
    #[doc = "5: 64 KB of FlexNVM"]
    _0101,
    #[doc = "7: 128 KB of FlexNVM"]
    _0111,
    #[doc = "9: 256 KB of FlexNVM"]
    _1001,
    #[doc = "11: 512 KB of FlexNVM"]
    _1011,
    #[doc = "15: 256 KB of FlexNVM"]
    _1111,
}
impl From<NVMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NVMSIZE_A) -> Self {
        match variant {
            NVMSIZE_A::_0000 => 0,
            NVMSIZE_A::_0011 => 3,
            NVMSIZE_A::_0101 => 5,
            NVMSIZE_A::_0111 => 7,
            NVMSIZE_A::_1001 => 9,
            NVMSIZE_A::_1011 => 11,
            NVMSIZE_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `NVMSIZE`"]
pub type NVMSIZE_R = crate::R<u8, NVMSIZE_A>;
impl NVMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NVMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NVMSIZE_A::_0000),
            3 => Val(NVMSIZE_A::_0011),
            5 => Val(NVMSIZE_A::_0101),
            7 => Val(NVMSIZE_A::_0111),
            9 => Val(NVMSIZE_A::_1001),
            11 => Val(NVMSIZE_A::_1011),
            15 => Val(NVMSIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == NVMSIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == NVMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == NVMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == NVMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == NVMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == NVMSIZE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == NVMSIZE_A::_1111
    }
}
impl R {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&self) -> FLASHDIS_R {
        FLASHDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&self) -> FLASHDOZE_R {
        FLASHDOZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - FlexNVM partition"]
    #[inline(always)]
    pub fn depart(&self) -> DEPART_R {
        DEPART_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEPROM size"]
    #[inline(always)]
    pub fn eesize(&self) -> EESIZE_R {
        EESIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Program flash size"]
    #[inline(always)]
    pub fn pfsize(&self) -> PFSIZE_R {
        PFSIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - FlexNVM size"]
    #[inline(always)]
    pub fn nvmsize(&self) -> NVMSIZE_R {
        NVMSIZE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&mut self) -> FLASHDIS_W {
        FLASHDIS_W { w: self }
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&mut self) -> FLASHDOZE_W {
        FLASHDOZE_W { w: self }
    }
}
