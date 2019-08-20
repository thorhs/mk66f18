#[doc = "Reader of register SC1%s"]
pub type R = crate::R<u32, super::SC1>;
#[doc = "Writer for register SC1%s"]
pub type W = crate::W<u32, super::SC1>;
#[doc = "Register SC1%s `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::SC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Input channel select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCH_A {
    #[doc = "0: When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    _00000,
    #[doc = "1: When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    _00001,
    #[doc = "2: When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    _00010,
    #[doc = "3: When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    _00011,
    #[doc = "4: When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    _00100,
    #[doc = "5: When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    _00101,
    #[doc = "6: When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    _00110,
    #[doc = "7: When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    _00111,
    #[doc = "8: When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    _01000,
    #[doc = "9: When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    _01001,
    #[doc = "10: When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    _01010,
    #[doc = "11: When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    _01011,
    #[doc = "12: When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    _01100,
    #[doc = "13: When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    _01101,
    #[doc = "14: When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    _01110,
    #[doc = "15: When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    _01111,
    #[doc = "16: When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    _10000,
    #[doc = "17: When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    _10001,
    #[doc = "18: When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    _10010,
    #[doc = "19: When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    _10011,
    #[doc = "20: When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    _10100,
    #[doc = "21: When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    _10101,
    #[doc = "22: When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    _10110,
    #[doc = "23: When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    _10111,
    #[doc = "26: When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    _11010,
    #[doc = "27: When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    _11011,
    #[doc = "29: When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11101,
    #[doc = "30: When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11110,
    #[doc = "31: Module is disabled."]
    _11111,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        match variant {
            ADCH_A::_00000 => 0,
            ADCH_A::_00001 => 1,
            ADCH_A::_00010 => 2,
            ADCH_A::_00011 => 3,
            ADCH_A::_00100 => 4,
            ADCH_A::_00101 => 5,
            ADCH_A::_00110 => 6,
            ADCH_A::_00111 => 7,
            ADCH_A::_01000 => 8,
            ADCH_A::_01001 => 9,
            ADCH_A::_01010 => 10,
            ADCH_A::_01011 => 11,
            ADCH_A::_01100 => 12,
            ADCH_A::_01101 => 13,
            ADCH_A::_01110 => 14,
            ADCH_A::_01111 => 15,
            ADCH_A::_10000 => 16,
            ADCH_A::_10001 => 17,
            ADCH_A::_10010 => 18,
            ADCH_A::_10011 => 19,
            ADCH_A::_10100 => 20,
            ADCH_A::_10101 => 21,
            ADCH_A::_10110 => 22,
            ADCH_A::_10111 => 23,
            ADCH_A::_11010 => 26,
            ADCH_A::_11011 => 27,
            ADCH_A::_11101 => 29,
            ADCH_A::_11110 => 30,
            ADCH_A::_11111 => 31,
        }
    }
}
#[doc = "Reader of field `ADCH`"]
pub type ADCH_R = crate::R<u8, ADCH_A>;
impl ADCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCH_A::_00000),
            1 => Val(ADCH_A::_00001),
            2 => Val(ADCH_A::_00010),
            3 => Val(ADCH_A::_00011),
            4 => Val(ADCH_A::_00100),
            5 => Val(ADCH_A::_00101),
            6 => Val(ADCH_A::_00110),
            7 => Val(ADCH_A::_00111),
            8 => Val(ADCH_A::_01000),
            9 => Val(ADCH_A::_01001),
            10 => Val(ADCH_A::_01010),
            11 => Val(ADCH_A::_01011),
            12 => Val(ADCH_A::_01100),
            13 => Val(ADCH_A::_01101),
            14 => Val(ADCH_A::_01110),
            15 => Val(ADCH_A::_01111),
            16 => Val(ADCH_A::_10000),
            17 => Val(ADCH_A::_10001),
            18 => Val(ADCH_A::_10010),
            19 => Val(ADCH_A::_10011),
            20 => Val(ADCH_A::_10100),
            21 => Val(ADCH_A::_10101),
            22 => Val(ADCH_A::_10110),
            23 => Val(ADCH_A::_10111),
            26 => Val(ADCH_A::_11010),
            27 => Val(ADCH_A::_11011),
            29 => Val(ADCH_A::_11101),
            30 => Val(ADCH_A::_11110),
            31 => Val(ADCH_A::_11111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == ADCH_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == ADCH_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == ADCH_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == ADCH_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == ADCH_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == ADCH_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == ADCH_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == ADCH_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == ADCH_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == ADCH_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == ADCH_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == ADCH_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == ADCH_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == ADCH_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == ADCH_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == ADCH_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == ADCH_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == ADCH_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == ADCH_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == ADCH_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == ADCH_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == ADCH_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == ADCH_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == ADCH_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == ADCH_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == ADCH_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == ADCH_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == ADCH_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == ADCH_A::_11111
    }
}
#[doc = "Write proxy for field `ADCH`"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(ADCH_A::_00000)
    }
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(ADCH_A::_00001)
    }
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(ADCH_A::_00010)
    }
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(ADCH_A::_00011)
    }
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(ADCH_A::_00100)
    }
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(ADCH_A::_00101)
    }
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(ADCH_A::_00110)
    }
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(ADCH_A::_00111)
    }
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(ADCH_A::_01000)
    }
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(ADCH_A::_01001)
    }
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(ADCH_A::_01010)
    }
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(ADCH_A::_01011)
    }
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(ADCH_A::_01100)
    }
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(ADCH_A::_01101)
    }
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(ADCH_A::_01110)
    }
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(ADCH_A::_01111)
    }
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(ADCH_A::_10000)
    }
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(ADCH_A::_10001)
    }
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(ADCH_A::_10010)
    }
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(ADCH_A::_10011)
    }
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(ADCH_A::_10100)
    }
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(ADCH_A::_10101)
    }
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(ADCH_A::_10110)
    }
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(ADCH_A::_10111)
    }
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(ADCH_A::_11010)
    }
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(ADCH_A::_11011)
    }
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(ADCH_A::_11101)
    }
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(ADCH_A::_11110)
    }
    #[doc = "Module is disabled."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(ADCH_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Differential Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFF_A {
    #[doc = "0: Single-ended conversions and input channels are selected."]
    _0,
    #[doc = "1: Differential conversions and input channels are selected."]
    _1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        match variant {
            DIFF_A::_0 => false,
            DIFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<bool, DIFF_A>;
impl DIFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::_0,
            true => DIFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIFF_A::_1
    }
}
#[doc = "Write proxy for field `DIFF`"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-ended conversions and input channels are selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFF_A::_0)
    }
    #[doc = "Differential conversions and input channels are selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFF_A::_1)
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
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt is disabled."]
    _0,
    #[doc = "1: Conversion complete interrupt is enabled."]
    _1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        match variant {
            AIEN_A::_0 => false,
            AIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AIEN`"]
pub type AIEN_R = crate::R<bool, AIEN_A>;
impl AIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::_0,
            true => AIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIEN_A::_1
    }
}
#[doc = "Write proxy for field `AIEN`"]
pub struct AIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIEN_A::_0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIEN_A::_1)
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
#[doc = "Conversion Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COCO_A {
    #[doc = "0: Conversion is not completed."]
    _0,
    #[doc = "1: Conversion is completed."]
    _1,
}
impl From<COCO_A> for bool {
    #[inline(always)]
    fn from(variant: COCO_A) -> Self {
        match variant {
            COCO_A::_0 => false,
            COCO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `COCO`"]
pub type COCO_R = crate::R<bool, COCO_A>;
impl COCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COCO_A {
        match self.bits {
            false => COCO_A::_0,
            true => COCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COCO_A::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco(&self) -> COCO_R {
        COCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&mut self) -> AIEN_W {
        AIEN_W { w: self }
    }
}
