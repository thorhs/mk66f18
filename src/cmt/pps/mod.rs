#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PPS {
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
#[doc = "Possible values of the field `PPSDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPSDIVR {
    #[doc = "Bus clock * 1"]
    _0000,
    #[doc = "Bus clock * 2"]
    _0001,
    #[doc = "Bus clock * 3"]
    _0010,
    #[doc = "Bus clock * 4"]
    _0011,
    #[doc = "Bus clock * 5"]
    _0100,
    #[doc = "Bus clock * 6"]
    _0101,
    #[doc = "Bus clock * 7"]
    _0110,
    #[doc = "Bus clock * 8"]
    _0111,
    #[doc = "Bus clock * 9"]
    _1000,
    #[doc = "Bus clock * 10"]
    _1001,
    #[doc = "Bus clock * 11"]
    _1010,
    #[doc = "Bus clock * 12"]
    _1011,
    #[doc = "Bus clock * 13"]
    _1100,
    #[doc = "Bus clock * 14"]
    _1101,
    #[doc = "Bus clock * 15"]
    _1110,
    #[doc = "Bus clock * 16"]
    _1111,
}
impl PPSDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPSDIVR::_0000 => 0,
            PPSDIVR::_0001 => 1,
            PPSDIVR::_0010 => 2,
            PPSDIVR::_0011 => 3,
            PPSDIVR::_0100 => 4,
            PPSDIVR::_0101 => 5,
            PPSDIVR::_0110 => 6,
            PPSDIVR::_0111 => 7,
            PPSDIVR::_1000 => 8,
            PPSDIVR::_1001 => 9,
            PPSDIVR::_1010 => 10,
            PPSDIVR::_1011 => 11,
            PPSDIVR::_1100 => 12,
            PPSDIVR::_1101 => 13,
            PPSDIVR::_1110 => 14,
            PPSDIVR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPSDIVR {
        match value {
            0 => PPSDIVR::_0000,
            1 => PPSDIVR::_0001,
            2 => PPSDIVR::_0010,
            3 => PPSDIVR::_0011,
            4 => PPSDIVR::_0100,
            5 => PPSDIVR::_0101,
            6 => PPSDIVR::_0110,
            7 => PPSDIVR::_0111,
            8 => PPSDIVR::_1000,
            9 => PPSDIVR::_1001,
            10 => PPSDIVR::_1010,
            11 => PPSDIVR::_1011,
            12 => PPSDIVR::_1100,
            13 => PPSDIVR::_1101,
            14 => PPSDIVR::_1110,
            15 => PPSDIVR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PPSDIVR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PPSDIVR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PPSDIVR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PPSDIVR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PPSDIVR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PPSDIVR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PPSDIVR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PPSDIVR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PPSDIVR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PPSDIVR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == PPSDIVR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PPSDIVR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == PPSDIVR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == PPSDIVR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == PPSDIVR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == PPSDIVR::_1111
    }
}
#[doc = "Values that can be written to the field `PPSDIV`"]
pub enum PPSDIVW {
    #[doc = "Bus clock * 1"]
    _0000,
    #[doc = "Bus clock * 2"]
    _0001,
    #[doc = "Bus clock * 3"]
    _0010,
    #[doc = "Bus clock * 4"]
    _0011,
    #[doc = "Bus clock * 5"]
    _0100,
    #[doc = "Bus clock * 6"]
    _0101,
    #[doc = "Bus clock * 7"]
    _0110,
    #[doc = "Bus clock * 8"]
    _0111,
    #[doc = "Bus clock * 9"]
    _1000,
    #[doc = "Bus clock * 10"]
    _1001,
    #[doc = "Bus clock * 11"]
    _1010,
    #[doc = "Bus clock * 12"]
    _1011,
    #[doc = "Bus clock * 13"]
    _1100,
    #[doc = "Bus clock * 14"]
    _1101,
    #[doc = "Bus clock * 15"]
    _1110,
    #[doc = "Bus clock * 16"]
    _1111,
}
impl PPSDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPSDIVW::_0000 => 0,
            PPSDIVW::_0001 => 1,
            PPSDIVW::_0010 => 2,
            PPSDIVW::_0011 => 3,
            PPSDIVW::_0100 => 4,
            PPSDIVW::_0101 => 5,
            PPSDIVW::_0110 => 6,
            PPSDIVW::_0111 => 7,
            PPSDIVW::_1000 => 8,
            PPSDIVW::_1001 => 9,
            PPSDIVW::_1010 => 10,
            PPSDIVW::_1011 => 11,
            PPSDIVW::_1100 => 12,
            PPSDIVW::_1101 => 13,
            PPSDIVW::_1110 => 14,
            PPSDIVW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PPSDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPSDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus clock * 1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PPSDIVW::_0000)
    }
    #[doc = "Bus clock * 2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PPSDIVW::_0001)
    }
    #[doc = "Bus clock * 3"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PPSDIVW::_0010)
    }
    #[doc = "Bus clock * 4"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PPSDIVW::_0011)
    }
    #[doc = "Bus clock * 5"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PPSDIVW::_0100)
    }
    #[doc = "Bus clock * 6"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PPSDIVW::_0101)
    }
    #[doc = "Bus clock * 7"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PPSDIVW::_0110)
    }
    #[doc = "Bus clock * 8"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PPSDIVW::_0111)
    }
    #[doc = "Bus clock * 9"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PPSDIVW::_1000)
    }
    #[doc = "Bus clock * 10"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PPSDIVW::_1001)
    }
    #[doc = "Bus clock * 11"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PPSDIVW::_1010)
    }
    #[doc = "Bus clock * 12"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PPSDIVW::_1011)
    }
    #[doc = "Bus clock * 13"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PPSDIVW::_1100)
    }
    #[doc = "Bus clock * 14"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PPSDIVW::_1101)
    }
    #[doc = "Bus clock * 15"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PPSDIVW::_1110)
    }
    #[doc = "Bus clock * 16"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PPSDIVW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline]
    pub fn ppsdiv(&self) -> PPSDIVR {
        PPSDIVR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline]
    pub fn ppsdiv(&mut self) -> _PPSDIVW {
        _PPSDIVW { w: self }
    }
}
