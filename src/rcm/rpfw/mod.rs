#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RPFW {
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
#[doc = "Possible values of the field `RSTFLTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSELR {
    #[doc = "Bus clock filter count is 1"]
    _00000,
    #[doc = "Bus clock filter count is 2"]
    _00001,
    #[doc = "Bus clock filter count is 3"]
    _00010,
    #[doc = "Bus clock filter count is 4"]
    _00011,
    #[doc = "Bus clock filter count is 5"]
    _00100,
    #[doc = "Bus clock filter count is 6"]
    _00101,
    #[doc = "Bus clock filter count is 7"]
    _00110,
    #[doc = "Bus clock filter count is 8"]
    _00111,
    #[doc = "Bus clock filter count is 9"]
    _01000,
    #[doc = "Bus clock filter count is 10"]
    _01001,
    #[doc = "Bus clock filter count is 11"]
    _01010,
    #[doc = "Bus clock filter count is 12"]
    _01011,
    #[doc = "Bus clock filter count is 13"]
    _01100,
    #[doc = "Bus clock filter count is 14"]
    _01101,
    #[doc = "Bus clock filter count is 15"]
    _01110,
    #[doc = "Bus clock filter count is 16"]
    _01111,
    #[doc = "Bus clock filter count is 17"]
    _10000,
    #[doc = "Bus clock filter count is 18"]
    _10001,
    #[doc = "Bus clock filter count is 19"]
    _10010,
    #[doc = "Bus clock filter count is 20"]
    _10011,
    #[doc = "Bus clock filter count is 21"]
    _10100,
    #[doc = "Bus clock filter count is 22"]
    _10101,
    #[doc = "Bus clock filter count is 23"]
    _10110,
    #[doc = "Bus clock filter count is 24"]
    _10111,
    #[doc = "Bus clock filter count is 25"]
    _11000,
    #[doc = "Bus clock filter count is 26"]
    _11001,
    #[doc = "Bus clock filter count is 27"]
    _11010,
    #[doc = "Bus clock filter count is 28"]
    _11011,
    #[doc = "Bus clock filter count is 29"]
    _11100,
    #[doc = "Bus clock filter count is 30"]
    _11101,
    #[doc = "Bus clock filter count is 31"]
    _11110,
    #[doc = "Bus clock filter count is 32"]
    _11111,
}
impl RSTFLTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTFLTSELR::_00000 => 0,
            RSTFLTSELR::_00001 => 1,
            RSTFLTSELR::_00010 => 2,
            RSTFLTSELR::_00011 => 3,
            RSTFLTSELR::_00100 => 4,
            RSTFLTSELR::_00101 => 5,
            RSTFLTSELR::_00110 => 6,
            RSTFLTSELR::_00111 => 7,
            RSTFLTSELR::_01000 => 8,
            RSTFLTSELR::_01001 => 9,
            RSTFLTSELR::_01010 => 10,
            RSTFLTSELR::_01011 => 11,
            RSTFLTSELR::_01100 => 12,
            RSTFLTSELR::_01101 => 13,
            RSTFLTSELR::_01110 => 14,
            RSTFLTSELR::_01111 => 15,
            RSTFLTSELR::_10000 => 16,
            RSTFLTSELR::_10001 => 17,
            RSTFLTSELR::_10010 => 18,
            RSTFLTSELR::_10011 => 19,
            RSTFLTSELR::_10100 => 20,
            RSTFLTSELR::_10101 => 21,
            RSTFLTSELR::_10110 => 22,
            RSTFLTSELR::_10111 => 23,
            RSTFLTSELR::_11000 => 24,
            RSTFLTSELR::_11001 => 25,
            RSTFLTSELR::_11010 => 26,
            RSTFLTSELR::_11011 => 27,
            RSTFLTSELR::_11100 => 28,
            RSTFLTSELR::_11101 => 29,
            RSTFLTSELR::_11110 => 30,
            RSTFLTSELR::_11111 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTFLTSELR {
        match value {
            0 => RSTFLTSELR::_00000,
            1 => RSTFLTSELR::_00001,
            2 => RSTFLTSELR::_00010,
            3 => RSTFLTSELR::_00011,
            4 => RSTFLTSELR::_00100,
            5 => RSTFLTSELR::_00101,
            6 => RSTFLTSELR::_00110,
            7 => RSTFLTSELR::_00111,
            8 => RSTFLTSELR::_01000,
            9 => RSTFLTSELR::_01001,
            10 => RSTFLTSELR::_01010,
            11 => RSTFLTSELR::_01011,
            12 => RSTFLTSELR::_01100,
            13 => RSTFLTSELR::_01101,
            14 => RSTFLTSELR::_01110,
            15 => RSTFLTSELR::_01111,
            16 => RSTFLTSELR::_10000,
            17 => RSTFLTSELR::_10001,
            18 => RSTFLTSELR::_10010,
            19 => RSTFLTSELR::_10011,
            20 => RSTFLTSELR::_10100,
            21 => RSTFLTSELR::_10101,
            22 => RSTFLTSELR::_10110,
            23 => RSTFLTSELR::_10111,
            24 => RSTFLTSELR::_11000,
            25 => RSTFLTSELR::_11001,
            26 => RSTFLTSELR::_11010,
            27 => RSTFLTSELR::_11011,
            28 => RSTFLTSELR::_11100,
            29 => RSTFLTSELR::_11101,
            30 => RSTFLTSELR::_11110,
            31 => RSTFLTSELR::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == RSTFLTSELR::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline]
    pub fn is_00001(&self) -> bool {
        *self == RSTFLTSELR::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline]
    pub fn is_00010(&self) -> bool {
        *self == RSTFLTSELR::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline]
    pub fn is_00011(&self) -> bool {
        *self == RSTFLTSELR::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline]
    pub fn is_00100(&self) -> bool {
        *self == RSTFLTSELR::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline]
    pub fn is_00101(&self) -> bool {
        *self == RSTFLTSELR::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline]
    pub fn is_00110(&self) -> bool {
        *self == RSTFLTSELR::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline]
    pub fn is_00111(&self) -> bool {
        *self == RSTFLTSELR::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline]
    pub fn is_01000(&self) -> bool {
        *self == RSTFLTSELR::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline]
    pub fn is_01001(&self) -> bool {
        *self == RSTFLTSELR::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline]
    pub fn is_01010(&self) -> bool {
        *self == RSTFLTSELR::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline]
    pub fn is_01011(&self) -> bool {
        *self == RSTFLTSELR::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline]
    pub fn is_01100(&self) -> bool {
        *self == RSTFLTSELR::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline]
    pub fn is_01101(&self) -> bool {
        *self == RSTFLTSELR::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline]
    pub fn is_01110(&self) -> bool {
        *self == RSTFLTSELR::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline]
    pub fn is_01111(&self) -> bool {
        *self == RSTFLTSELR::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == RSTFLTSELR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == RSTFLTSELR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == RSTFLTSELR::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline]
    pub fn is_10011(&self) -> bool {
        *self == RSTFLTSELR::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline]
    pub fn is_10100(&self) -> bool {
        *self == RSTFLTSELR::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == RSTFLTSELR::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline]
    pub fn is_10110(&self) -> bool {
        *self == RSTFLTSELR::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline]
    pub fn is_10111(&self) -> bool {
        *self == RSTFLTSELR::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline]
    pub fn is_11000(&self) -> bool {
        *self == RSTFLTSELR::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline]
    pub fn is_11001(&self) -> bool {
        *self == RSTFLTSELR::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline]
    pub fn is_11010(&self) -> bool {
        *self == RSTFLTSELR::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline]
    pub fn is_11011(&self) -> bool {
        *self == RSTFLTSELR::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline]
    pub fn is_11100(&self) -> bool {
        *self == RSTFLTSELR::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline]
    pub fn is_11101(&self) -> bool {
        *self == RSTFLTSELR::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline]
    pub fn is_11110(&self) -> bool {
        *self == RSTFLTSELR::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == RSTFLTSELR::_11111
    }
}
#[doc = "Values that can be written to the field `RSTFLTSEL`"]
pub enum RSTFLTSELW {
    #[doc = "Bus clock filter count is 1"]
    _00000,
    #[doc = "Bus clock filter count is 2"]
    _00001,
    #[doc = "Bus clock filter count is 3"]
    _00010,
    #[doc = "Bus clock filter count is 4"]
    _00011,
    #[doc = "Bus clock filter count is 5"]
    _00100,
    #[doc = "Bus clock filter count is 6"]
    _00101,
    #[doc = "Bus clock filter count is 7"]
    _00110,
    #[doc = "Bus clock filter count is 8"]
    _00111,
    #[doc = "Bus clock filter count is 9"]
    _01000,
    #[doc = "Bus clock filter count is 10"]
    _01001,
    #[doc = "Bus clock filter count is 11"]
    _01010,
    #[doc = "Bus clock filter count is 12"]
    _01011,
    #[doc = "Bus clock filter count is 13"]
    _01100,
    #[doc = "Bus clock filter count is 14"]
    _01101,
    #[doc = "Bus clock filter count is 15"]
    _01110,
    #[doc = "Bus clock filter count is 16"]
    _01111,
    #[doc = "Bus clock filter count is 17"]
    _10000,
    #[doc = "Bus clock filter count is 18"]
    _10001,
    #[doc = "Bus clock filter count is 19"]
    _10010,
    #[doc = "Bus clock filter count is 20"]
    _10011,
    #[doc = "Bus clock filter count is 21"]
    _10100,
    #[doc = "Bus clock filter count is 22"]
    _10101,
    #[doc = "Bus clock filter count is 23"]
    _10110,
    #[doc = "Bus clock filter count is 24"]
    _10111,
    #[doc = "Bus clock filter count is 25"]
    _11000,
    #[doc = "Bus clock filter count is 26"]
    _11001,
    #[doc = "Bus clock filter count is 27"]
    _11010,
    #[doc = "Bus clock filter count is 28"]
    _11011,
    #[doc = "Bus clock filter count is 29"]
    _11100,
    #[doc = "Bus clock filter count is 30"]
    _11101,
    #[doc = "Bus clock filter count is 31"]
    _11110,
    #[doc = "Bus clock filter count is 32"]
    _11111,
}
impl RSTFLTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSTFLTSELW::_00000 => 0,
            RSTFLTSELW::_00001 => 1,
            RSTFLTSELW::_00010 => 2,
            RSTFLTSELW::_00011 => 3,
            RSTFLTSELW::_00100 => 4,
            RSTFLTSELW::_00101 => 5,
            RSTFLTSELW::_00110 => 6,
            RSTFLTSELW::_00111 => 7,
            RSTFLTSELW::_01000 => 8,
            RSTFLTSELW::_01001 => 9,
            RSTFLTSELW::_01010 => 10,
            RSTFLTSELW::_01011 => 11,
            RSTFLTSELW::_01100 => 12,
            RSTFLTSELW::_01101 => 13,
            RSTFLTSELW::_01110 => 14,
            RSTFLTSELW::_01111 => 15,
            RSTFLTSELW::_10000 => 16,
            RSTFLTSELW::_10001 => 17,
            RSTFLTSELW::_10010 => 18,
            RSTFLTSELW::_10011 => 19,
            RSTFLTSELW::_10100 => 20,
            RSTFLTSELW::_10101 => 21,
            RSTFLTSELW::_10110 => 22,
            RSTFLTSELW::_10111 => 23,
            RSTFLTSELW::_11000 => 24,
            RSTFLTSELW::_11001 => 25,
            RSTFLTSELW::_11010 => 26,
            RSTFLTSELW::_11011 => 27,
            RSTFLTSELW::_11100 => 28,
            RSTFLTSELW::_11101 => 29,
            RSTFLTSELW::_11110 => 30,
            RSTFLTSELW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFLTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFLTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFLTSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus clock filter count is 1"]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00000)
    }
    #[doc = "Bus clock filter count is 2"]
    #[inline]
    pub fn _00001(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00001)
    }
    #[doc = "Bus clock filter count is 3"]
    #[inline]
    pub fn _00010(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00010)
    }
    #[doc = "Bus clock filter count is 4"]
    #[inline]
    pub fn _00011(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00011)
    }
    #[doc = "Bus clock filter count is 5"]
    #[inline]
    pub fn _00100(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00100)
    }
    #[doc = "Bus clock filter count is 6"]
    #[inline]
    pub fn _00101(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00101)
    }
    #[doc = "Bus clock filter count is 7"]
    #[inline]
    pub fn _00110(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00110)
    }
    #[doc = "Bus clock filter count is 8"]
    #[inline]
    pub fn _00111(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_00111)
    }
    #[doc = "Bus clock filter count is 9"]
    #[inline]
    pub fn _01000(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01000)
    }
    #[doc = "Bus clock filter count is 10"]
    #[inline]
    pub fn _01001(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01001)
    }
    #[doc = "Bus clock filter count is 11"]
    #[inline]
    pub fn _01010(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01010)
    }
    #[doc = "Bus clock filter count is 12"]
    #[inline]
    pub fn _01011(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01011)
    }
    #[doc = "Bus clock filter count is 13"]
    #[inline]
    pub fn _01100(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01100)
    }
    #[doc = "Bus clock filter count is 14"]
    #[inline]
    pub fn _01101(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01101)
    }
    #[doc = "Bus clock filter count is 15"]
    #[inline]
    pub fn _01110(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01110)
    }
    #[doc = "Bus clock filter count is 16"]
    #[inline]
    pub fn _01111(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_01111)
    }
    #[doc = "Bus clock filter count is 17"]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10000)
    }
    #[doc = "Bus clock filter count is 18"]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10001)
    }
    #[doc = "Bus clock filter count is 19"]
    #[inline]
    pub fn _10010(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10010)
    }
    #[doc = "Bus clock filter count is 20"]
    #[inline]
    pub fn _10011(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10011)
    }
    #[doc = "Bus clock filter count is 21"]
    #[inline]
    pub fn _10100(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10100)
    }
    #[doc = "Bus clock filter count is 22"]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10101)
    }
    #[doc = "Bus clock filter count is 23"]
    #[inline]
    pub fn _10110(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10110)
    }
    #[doc = "Bus clock filter count is 24"]
    #[inline]
    pub fn _10111(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_10111)
    }
    #[doc = "Bus clock filter count is 25"]
    #[inline]
    pub fn _11000(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11000)
    }
    #[doc = "Bus clock filter count is 26"]
    #[inline]
    pub fn _11001(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11001)
    }
    #[doc = "Bus clock filter count is 27"]
    #[inline]
    pub fn _11010(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11010)
    }
    #[doc = "Bus clock filter count is 28"]
    #[inline]
    pub fn _11011(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11011)
    }
    #[doc = "Bus clock filter count is 29"]
    #[inline]
    pub fn _11100(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11100)
    }
    #[doc = "Bus clock filter count is 30"]
    #[inline]
    pub fn _11101(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11101)
    }
    #[doc = "Bus clock filter count is 31"]
    #[inline]
    pub fn _11110(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11110)
    }
    #[doc = "Bus clock filter count is 32"]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(RSTFLTSELW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline]
    pub fn rstfltsel(&self) -> RSTFLTSELR {
        RSTFLTSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline]
    pub fn rstfltsel(&mut self) -> _RSTFLTSELW {
        _RSTFLTSELW { w: self }
    }
}
