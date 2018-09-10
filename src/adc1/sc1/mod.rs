#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC1 {
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
#[doc = "Possible values of the field `ADCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHR {
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    _00000,
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    _00001,
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    _00010,
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    _00011,
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    _00100,
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    _00101,
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    _00110,
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    _00111,
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    _01000,
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    _01001,
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    _01010,
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    _01011,
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    _01100,
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    _01101,
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    _01110,
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    _01111,
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    _10000,
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    _10001,
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    _10010,
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    _10011,
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    _10100,
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    _10101,
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    _10110,
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    _10111,
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    _11010,
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    _11011,
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11101,
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11110,
    #[doc = "Module is disabled."]
    _11111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCHR::_00000 => 0,
            ADCHR::_00001 => 1,
            ADCHR::_00010 => 2,
            ADCHR::_00011 => 3,
            ADCHR::_00100 => 4,
            ADCHR::_00101 => 5,
            ADCHR::_00110 => 6,
            ADCHR::_00111 => 7,
            ADCHR::_01000 => 8,
            ADCHR::_01001 => 9,
            ADCHR::_01010 => 10,
            ADCHR::_01011 => 11,
            ADCHR::_01100 => 12,
            ADCHR::_01101 => 13,
            ADCHR::_01110 => 14,
            ADCHR::_01111 => 15,
            ADCHR::_10000 => 16,
            ADCHR::_10001 => 17,
            ADCHR::_10010 => 18,
            ADCHR::_10011 => 19,
            ADCHR::_10100 => 20,
            ADCHR::_10101 => 21,
            ADCHR::_10110 => 22,
            ADCHR::_10111 => 23,
            ADCHR::_11010 => 26,
            ADCHR::_11011 => 27,
            ADCHR::_11101 => 29,
            ADCHR::_11110 => 30,
            ADCHR::_11111 => 31,
            ADCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCHR {
        match value {
            0 => ADCHR::_00000,
            1 => ADCHR::_00001,
            2 => ADCHR::_00010,
            3 => ADCHR::_00011,
            4 => ADCHR::_00100,
            5 => ADCHR::_00101,
            6 => ADCHR::_00110,
            7 => ADCHR::_00111,
            8 => ADCHR::_01000,
            9 => ADCHR::_01001,
            10 => ADCHR::_01010,
            11 => ADCHR::_01011,
            12 => ADCHR::_01100,
            13 => ADCHR::_01101,
            14 => ADCHR::_01110,
            15 => ADCHR::_01111,
            16 => ADCHR::_10000,
            17 => ADCHR::_10001,
            18 => ADCHR::_10010,
            19 => ADCHR::_10011,
            20 => ADCHR::_10100,
            21 => ADCHR::_10101,
            22 => ADCHR::_10110,
            23 => ADCHR::_10111,
            26 => ADCHR::_11010,
            27 => ADCHR::_11011,
            29 => ADCHR::_11101,
            30 => ADCHR::_11110,
            31 => ADCHR::_11111,
            i => ADCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == ADCHR::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline]
    pub fn is_00001(&self) -> bool {
        *self == ADCHR::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline]
    pub fn is_00010(&self) -> bool {
        *self == ADCHR::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline]
    pub fn is_00011(&self) -> bool {
        *self == ADCHR::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline]
    pub fn is_00100(&self) -> bool {
        *self == ADCHR::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline]
    pub fn is_00101(&self) -> bool {
        *self == ADCHR::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline]
    pub fn is_00110(&self) -> bool {
        *self == ADCHR::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline]
    pub fn is_00111(&self) -> bool {
        *self == ADCHR::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline]
    pub fn is_01000(&self) -> bool {
        *self == ADCHR::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline]
    pub fn is_01001(&self) -> bool {
        *self == ADCHR::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline]
    pub fn is_01010(&self) -> bool {
        *self == ADCHR::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline]
    pub fn is_01011(&self) -> bool {
        *self == ADCHR::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline]
    pub fn is_01100(&self) -> bool {
        *self == ADCHR::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline]
    pub fn is_01101(&self) -> bool {
        *self == ADCHR::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline]
    pub fn is_01110(&self) -> bool {
        *self == ADCHR::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline]
    pub fn is_01111(&self) -> bool {
        *self == ADCHR::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == ADCHR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == ADCHR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == ADCHR::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline]
    pub fn is_10011(&self) -> bool {
        *self == ADCHR::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline]
    pub fn is_10100(&self) -> bool {
        *self == ADCHR::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == ADCHR::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline]
    pub fn is_10110(&self) -> bool {
        *self == ADCHR::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline]
    pub fn is_10111(&self) -> bool {
        *self == ADCHR::_10111
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline]
    pub fn is_11010(&self) -> bool {
        *self == ADCHR::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline]
    pub fn is_11011(&self) -> bool {
        *self == ADCHR::_11011
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline]
    pub fn is_11101(&self) -> bool {
        *self == ADCHR::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline]
    pub fn is_11110(&self) -> bool {
        *self == ADCHR::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == ADCHR::_11111
    }
}
#[doc = "Possible values of the field `DIFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFR {
    #[doc = "Single-ended conversions and input channels are selected."]
    _0,
    #[doc = "Differential conversions and input channels are selected."]
    _1,
}
impl DIFFR {
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
            DIFFR::_0 => false,
            DIFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFFR {
        match value {
            false => DIFFR::_0,
            true => DIFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIFFR::_1
    }
}
#[doc = "Possible values of the field `AIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIENR {
    #[doc = "Conversion complete interrupt is disabled."]
    _0,
    #[doc = "Conversion complete interrupt is enabled."]
    _1,
}
impl AIENR {
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
            AIENR::_0 => false,
            AIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIENR {
        match value {
            false => AIENR::_0,
            true => AIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AIENR::_1
    }
}
#[doc = "Possible values of the field `COCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COCOR {
    #[doc = "Conversion is not completed."]
    _0,
    #[doc = "Conversion is completed."]
    _1,
}
impl COCOR {
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
            COCOR::_0 => false,
            COCOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COCOR {
        match value {
            false => COCOR::_0,
            true => COCOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COCOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COCOR::_1
    }
}
#[doc = "Values that can be written to the field `ADCH`"]
pub enum ADCHW {
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    _00000,
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    _00001,
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    _00010,
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    _00011,
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    _00100,
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    _00101,
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    _00110,
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    _00111,
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    _01000,
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    _01001,
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    _01010,
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    _01011,
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    _01100,
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    _01101,
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    _01110,
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    _01111,
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    _10000,
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    _10001,
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    _10010,
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    _10011,
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    _10100,
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    _10101,
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    _10110,
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    _10111,
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    _11010,
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    _11011,
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11101,
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11110,
    #[doc = "Module is disabled."]
    _11111,
}
impl ADCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCHW::_00000 => 0,
            ADCHW::_00001 => 1,
            ADCHW::_00010 => 2,
            ADCHW::_00011 => 3,
            ADCHW::_00100 => 4,
            ADCHW::_00101 => 5,
            ADCHW::_00110 => 6,
            ADCHW::_00111 => 7,
            ADCHW::_01000 => 8,
            ADCHW::_01001 => 9,
            ADCHW::_01010 => 10,
            ADCHW::_01011 => 11,
            ADCHW::_01100 => 12,
            ADCHW::_01101 => 13,
            ADCHW::_01110 => 14,
            ADCHW::_01111 => 15,
            ADCHW::_10000 => 16,
            ADCHW::_10001 => 17,
            ADCHW::_10010 => 18,
            ADCHW::_10011 => 19,
            ADCHW::_10100 => 20,
            ADCHW::_10101 => 21,
            ADCHW::_10110 => 22,
            ADCHW::_10111 => 23,
            ADCHW::_11010 => 26,
            ADCHW::_11011 => 27,
            ADCHW::_11101 => 29,
            ADCHW::_11110 => 30,
            ADCHW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(ADCHW::_00000)
    }
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    #[inline]
    pub fn _00001(self) -> &'a mut W {
        self.variant(ADCHW::_00001)
    }
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    #[inline]
    pub fn _00010(self) -> &'a mut W {
        self.variant(ADCHW::_00010)
    }
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    #[inline]
    pub fn _00011(self) -> &'a mut W {
        self.variant(ADCHW::_00011)
    }
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _00100(self) -> &'a mut W {
        self.variant(ADCHW::_00100)
    }
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _00101(self) -> &'a mut W {
        self.variant(ADCHW::_00101)
    }
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _00110(self) -> &'a mut W {
        self.variant(ADCHW::_00110)
    }
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _00111(self) -> &'a mut W {
        self.variant(ADCHW::_00111)
    }
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01000(self) -> &'a mut W {
        self.variant(ADCHW::_01000)
    }
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01001(self) -> &'a mut W {
        self.variant(ADCHW::_01001)
    }
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01010(self) -> &'a mut W {
        self.variant(ADCHW::_01010)
    }
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01011(self) -> &'a mut W {
        self.variant(ADCHW::_01011)
    }
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01100(self) -> &'a mut W {
        self.variant(ADCHW::_01100)
    }
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01101(self) -> &'a mut W {
        self.variant(ADCHW::_01101)
    }
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01110(self) -> &'a mut W {
        self.variant(ADCHW::_01110)
    }
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _01111(self) -> &'a mut W {
        self.variant(ADCHW::_01111)
    }
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(ADCHW::_10000)
    }
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(ADCHW::_10001)
    }
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10010(self) -> &'a mut W {
        self.variant(ADCHW::_10010)
    }
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10011(self) -> &'a mut W {
        self.variant(ADCHW::_10011)
    }
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10100(self) -> &'a mut W {
        self.variant(ADCHW::_10100)
    }
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(ADCHW::_10101)
    }
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10110(self) -> &'a mut W {
        self.variant(ADCHW::_10110)
    }
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    #[inline]
    pub fn _10111(self) -> &'a mut W {
        self.variant(ADCHW::_10111)
    }
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    #[inline]
    pub fn _11010(self) -> &'a mut W {
        self.variant(ADCHW::_11010)
    }
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    #[inline]
    pub fn _11011(self) -> &'a mut W {
        self.variant(ADCHW::_11011)
    }
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline]
    pub fn _11101(self) -> &'a mut W {
        self.variant(ADCHW::_11101)
    }
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline]
    pub fn _11110(self) -> &'a mut W {
        self.variant(ADCHW::_11110)
    }
    #[doc = "Module is disabled."]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(ADCHW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIFF`"]
pub enum DIFFW {
    #[doc = "Single-ended conversions and input channels are selected."]
    _0,
    #[doc = "Differential conversions and input channels are selected."]
    _1,
}
impl DIFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFFW::_0 => false,
            DIFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single-ended conversions and input channels are selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFFW::_0)
    }
    #[doc = "Differential conversions and input channels are selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFFW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AIEN`"]
pub enum AIENW {
    #[doc = "Conversion complete interrupt is disabled."]
    _0,
    #[doc = "Conversion complete interrupt is enabled."]
    _1,
}
impl AIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIENW::_0 => false,
            AIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIENW<'a> {
    w: &'a mut W,
}
impl<'a> _AIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIENW::_0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline]
    pub fn adch(&self) -> ADCHR {
        ADCHR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        DIFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline]
    pub fn aien(&self) -> AIENR {
        AIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline]
    pub fn coco(&self) -> COCOR {
        COCOR::_from({
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
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline]
    pub fn adch(&mut self) -> _ADCHW {
        _ADCHW { w: self }
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline]
    pub fn aien(&mut self) -> _AIENW {
        _AIENW { w: self }
    }
}
