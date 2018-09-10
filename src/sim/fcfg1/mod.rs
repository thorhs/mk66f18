#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFG1 {
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
#[doc = "Possible values of the field `FLASHDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDISR {
    #[doc = "Flash is enabled"]
    _0,
    #[doc = "Flash is disabled"]
    _1,
}
impl FLASHDISR {
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
            FLASHDISR::_0 => false,
            FLASHDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHDISR {
        match value {
            false => FLASHDISR::_0,
            true => FLASHDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLASHDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLASHDISR::_1
    }
}
#[doc = "Possible values of the field `FLASHDOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDOZER {
    #[doc = "Flash remains enabled during Wait mode"]
    _0,
    #[doc = "Flash is disabled for the duration of Wait mode"]
    _1,
}
impl FLASHDOZER {
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
            FLASHDOZER::_0 => false,
            FLASHDOZER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHDOZER {
        match value {
            false => FLASHDOZER::_0,
            true => FLASHDOZER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLASHDOZER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLASHDOZER::_1
    }
}
#[doc = r" Value of the field"]
pub struct DEPARTR {
    bits: u8,
}
impl DEPARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EESIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EESIZER {
    #[doc = "16 KB"]
    _0000,
    #[doc = "8 KB"]
    _0001,
    #[doc = "4 KB"]
    _0010,
    #[doc = "2 KB"]
    _0011,
    #[doc = "1 KB"]
    _0100,
    #[doc = "512 Bytes"]
    _0101,
    #[doc = "256 Bytes"]
    _0110,
    #[doc = "128 Bytes"]
    _0111,
    #[doc = "64 Bytes"]
    _1000,
    #[doc = "32 Bytes"]
    _1001,
    #[doc = "0 Bytes"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EESIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EESIZER::_0000 => 0,
            EESIZER::_0001 => 1,
            EESIZER::_0010 => 2,
            EESIZER::_0011 => 3,
            EESIZER::_0100 => 4,
            EESIZER::_0101 => 5,
            EESIZER::_0110 => 6,
            EESIZER::_0111 => 7,
            EESIZER::_1000 => 8,
            EESIZER::_1001 => 9,
            EESIZER::_1111 => 15,
            EESIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EESIZER {
        match value {
            0 => EESIZER::_0000,
            1 => EESIZER::_0001,
            2 => EESIZER::_0010,
            3 => EESIZER::_0011,
            4 => EESIZER::_0100,
            5 => EESIZER::_0101,
            6 => EESIZER::_0110,
            7 => EESIZER::_0111,
            8 => EESIZER::_1000,
            9 => EESIZER::_1001,
            15 => EESIZER::_1111,
            i => EESIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == EESIZER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == EESIZER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == EESIZER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == EESIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == EESIZER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == EESIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == EESIZER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == EESIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == EESIZER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == EESIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == EESIZER::_1111
    }
}
#[doc = "Possible values of the field `PFSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSIZER {
    #[doc = "32 KB of program flash memory"]
    _0011,
    #[doc = "64 KB of program flash memory"]
    _0101,
    #[doc = "128 KB of program flash memory"]
    _0111,
    #[doc = "256 KB of program flash memory"]
    _1001,
    #[doc = "512 KB of program flash memory"]
    _1011,
    #[doc = "1024 KB of program flash memory"]
    _1101,
    #[doc = "2048 KB of program flash memory"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PFSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PFSIZER::_0011 => 3,
            PFSIZER::_0101 => 5,
            PFSIZER::_0111 => 7,
            PFSIZER::_1001 => 9,
            PFSIZER::_1011 => 11,
            PFSIZER::_1101 => 13,
            PFSIZER::_1111 => 15,
            PFSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PFSIZER {
        match value {
            3 => PFSIZER::_0011,
            5 => PFSIZER::_0101,
            7 => PFSIZER::_0111,
            9 => PFSIZER::_1001,
            11 => PFSIZER::_1011,
            13 => PFSIZER::_1101,
            15 => PFSIZER::_1111,
            i => PFSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PFSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PFSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PFSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PFSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PFSIZER::_1011
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == PFSIZER::_1101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == PFSIZER::_1111
    }
}
#[doc = "Possible values of the field `NVMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVMSIZER {
    #[doc = "0 KB of FlexNVM"]
    _0000,
    #[doc = "32 KB of FlexNVM"]
    _0011,
    #[doc = "64 KB of FlexNVM"]
    _0101,
    #[doc = "128 KB of FlexNVM"]
    _0111,
    #[doc = "256 KB of FlexNVM"]
    _1001,
    #[doc = "512 KB of FlexNVM"]
    _1011,
    #[doc = "256 KB of FlexNVM"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVMSIZER::_0000 => 0,
            NVMSIZER::_0011 => 3,
            NVMSIZER::_0101 => 5,
            NVMSIZER::_0111 => 7,
            NVMSIZER::_1001 => 9,
            NVMSIZER::_1011 => 11,
            NVMSIZER::_1111 => 15,
            NVMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVMSIZER {
        match value {
            0 => NVMSIZER::_0000,
            3 => NVMSIZER::_0011,
            5 => NVMSIZER::_0101,
            7 => NVMSIZER::_0111,
            9 => NVMSIZER::_1001,
            11 => NVMSIZER::_1011,
            15 => NVMSIZER::_1111,
            i => NVMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == NVMSIZER::_0000
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == NVMSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == NVMSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == NVMSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == NVMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == NVMSIZER::_1011
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == NVMSIZER::_1111
    }
}
#[doc = "Values that can be written to the field `FLASHDIS`"]
pub enum FLASHDISW {
    #[doc = "Flash is enabled"]
    _0,
    #[doc = "Flash is disabled"]
    _1,
}
impl FLASHDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHDISW::_0 => false,
            FLASHDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHDISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDISW::_0)
    }
    #[doc = "Flash is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDISW::_1)
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
#[doc = "Values that can be written to the field `FLASHDOZE`"]
pub enum FLASHDOZEW {
    #[doc = "Flash remains enabled during Wait mode"]
    _0,
    #[doc = "Flash is disabled for the duration of Wait mode"]
    _1,
}
impl FLASHDOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHDOZEW::_0 => false,
            FLASHDOZEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHDOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHDOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHDOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash remains enabled during Wait mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDOZEW::_0)
    }
    #[doc = "Flash is disabled for the duration of Wait mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDOZEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flash Disable"]
    #[inline]
    pub fn flashdis(&self) -> FLASHDISR {
        FLASHDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline]
    pub fn flashdoze(&self) -> FLASHDOZER {
        FLASHDOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - FlexNVM partition"]
    #[inline]
    pub fn depart(&self) -> DEPARTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEPARTR { bits }
    }
    #[doc = "Bits 16:19 - EEPROM size"]
    #[inline]
    pub fn eesize(&self) -> EESIZER {
        EESIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Program flash size"]
    #[inline]
    pub fn pfsize(&self) -> PFSIZER {
        PFSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - FlexNVM size"]
    #[inline]
    pub fn nvmsize(&self) -> NVMSIZER {
        NVMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4279176960 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flash Disable"]
    #[inline]
    pub fn flashdis(&mut self) -> _FLASHDISW {
        _FLASHDISW { w: self }
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline]
    pub fn flashdoze(&mut self) -> _FLASHDOZEW {
        _FLASHDOZEW { w: self }
    }
}
