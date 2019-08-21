#[doc = "Reader of register RPFW"]
pub type R = crate::R<u8, super::RPFW>;
#[doc = "Writer for register RPFW"]
pub type W = crate::W<u8, super::RPFW>;
#[doc = "Register RPFW `reset()`'s with value 0"]
impl crate::ResetValue for super::RPFW {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset Pin Filter Bus Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSEL_A {
    #[doc = "0: Bus clock filter count is 1"]
    _00000,
    #[doc = "1: Bus clock filter count is 2"]
    _00001,
    #[doc = "2: Bus clock filter count is 3"]
    _00010,
    #[doc = "3: Bus clock filter count is 4"]
    _00011,
    #[doc = "4: Bus clock filter count is 5"]
    _00100,
    #[doc = "5: Bus clock filter count is 6"]
    _00101,
    #[doc = "6: Bus clock filter count is 7"]
    _00110,
    #[doc = "7: Bus clock filter count is 8"]
    _00111,
    #[doc = "8: Bus clock filter count is 9"]
    _01000,
    #[doc = "9: Bus clock filter count is 10"]
    _01001,
    #[doc = "10: Bus clock filter count is 11"]
    _01010,
    #[doc = "11: Bus clock filter count is 12"]
    _01011,
    #[doc = "12: Bus clock filter count is 13"]
    _01100,
    #[doc = "13: Bus clock filter count is 14"]
    _01101,
    #[doc = "14: Bus clock filter count is 15"]
    _01110,
    #[doc = "15: Bus clock filter count is 16"]
    _01111,
    #[doc = "16: Bus clock filter count is 17"]
    _10000,
    #[doc = "17: Bus clock filter count is 18"]
    _10001,
    #[doc = "18: Bus clock filter count is 19"]
    _10010,
    #[doc = "19: Bus clock filter count is 20"]
    _10011,
    #[doc = "20: Bus clock filter count is 21"]
    _10100,
    #[doc = "21: Bus clock filter count is 22"]
    _10101,
    #[doc = "22: Bus clock filter count is 23"]
    _10110,
    #[doc = "23: Bus clock filter count is 24"]
    _10111,
    #[doc = "24: Bus clock filter count is 25"]
    _11000,
    #[doc = "25: Bus clock filter count is 26"]
    _11001,
    #[doc = "26: Bus clock filter count is 27"]
    _11010,
    #[doc = "27: Bus clock filter count is 28"]
    _11011,
    #[doc = "28: Bus clock filter count is 29"]
    _11100,
    #[doc = "29: Bus clock filter count is 30"]
    _11101,
    #[doc = "30: Bus clock filter count is 31"]
    _11110,
    #[doc = "31: Bus clock filter count is 32"]
    _11111,
}
impl From<RSTFLTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTFLTSEL_A) -> Self {
        match variant {
            RSTFLTSEL_A::_00000 => 0,
            RSTFLTSEL_A::_00001 => 1,
            RSTFLTSEL_A::_00010 => 2,
            RSTFLTSEL_A::_00011 => 3,
            RSTFLTSEL_A::_00100 => 4,
            RSTFLTSEL_A::_00101 => 5,
            RSTFLTSEL_A::_00110 => 6,
            RSTFLTSEL_A::_00111 => 7,
            RSTFLTSEL_A::_01000 => 8,
            RSTFLTSEL_A::_01001 => 9,
            RSTFLTSEL_A::_01010 => 10,
            RSTFLTSEL_A::_01011 => 11,
            RSTFLTSEL_A::_01100 => 12,
            RSTFLTSEL_A::_01101 => 13,
            RSTFLTSEL_A::_01110 => 14,
            RSTFLTSEL_A::_01111 => 15,
            RSTFLTSEL_A::_10000 => 16,
            RSTFLTSEL_A::_10001 => 17,
            RSTFLTSEL_A::_10010 => 18,
            RSTFLTSEL_A::_10011 => 19,
            RSTFLTSEL_A::_10100 => 20,
            RSTFLTSEL_A::_10101 => 21,
            RSTFLTSEL_A::_10110 => 22,
            RSTFLTSEL_A::_10111 => 23,
            RSTFLTSEL_A::_11000 => 24,
            RSTFLTSEL_A::_11001 => 25,
            RSTFLTSEL_A::_11010 => 26,
            RSTFLTSEL_A::_11011 => 27,
            RSTFLTSEL_A::_11100 => 28,
            RSTFLTSEL_A::_11101 => 29,
            RSTFLTSEL_A::_11110 => 30,
            RSTFLTSEL_A::_11111 => 31,
        }
    }
}
#[doc = "Reader of field `RSTFLTSEL`"]
pub type RSTFLTSEL_R = crate::R<u8, RSTFLTSEL_A>;
impl RSTFLTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSEL_A {
        match self.bits {
            0 => RSTFLTSEL_A::_00000,
            1 => RSTFLTSEL_A::_00001,
            2 => RSTFLTSEL_A::_00010,
            3 => RSTFLTSEL_A::_00011,
            4 => RSTFLTSEL_A::_00100,
            5 => RSTFLTSEL_A::_00101,
            6 => RSTFLTSEL_A::_00110,
            7 => RSTFLTSEL_A::_00111,
            8 => RSTFLTSEL_A::_01000,
            9 => RSTFLTSEL_A::_01001,
            10 => RSTFLTSEL_A::_01010,
            11 => RSTFLTSEL_A::_01011,
            12 => RSTFLTSEL_A::_01100,
            13 => RSTFLTSEL_A::_01101,
            14 => RSTFLTSEL_A::_01110,
            15 => RSTFLTSEL_A::_01111,
            16 => RSTFLTSEL_A::_10000,
            17 => RSTFLTSEL_A::_10001,
            18 => RSTFLTSEL_A::_10010,
            19 => RSTFLTSEL_A::_10011,
            20 => RSTFLTSEL_A::_10100,
            21 => RSTFLTSEL_A::_10101,
            22 => RSTFLTSEL_A::_10110,
            23 => RSTFLTSEL_A::_10111,
            24 => RSTFLTSEL_A::_11000,
            25 => RSTFLTSEL_A::_11001,
            26 => RSTFLTSEL_A::_11010,
            27 => RSTFLTSEL_A::_11011,
            28 => RSTFLTSEL_A::_11100,
            29 => RSTFLTSEL_A::_11101,
            30 => RSTFLTSEL_A::_11110,
            31 => RSTFLTSEL_A::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == RSTFLTSEL_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == RSTFLTSEL_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == RSTFLTSEL_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == RSTFLTSEL_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == RSTFLTSEL_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == RSTFLTSEL_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == RSTFLTSEL_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == RSTFLTSEL_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == RSTFLTSEL_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == RSTFLTSEL_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == RSTFLTSEL_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == RSTFLTSEL_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == RSTFLTSEL_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == RSTFLTSEL_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == RSTFLTSEL_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == RSTFLTSEL_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == RSTFLTSEL_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == RSTFLTSEL_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == RSTFLTSEL_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == RSTFLTSEL_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == RSTFLTSEL_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == RSTFLTSEL_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == RSTFLTSEL_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == RSTFLTSEL_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == RSTFLTSEL_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == RSTFLTSEL_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == RSTFLTSEL_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == RSTFLTSEL_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == RSTFLTSEL_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == RSTFLTSEL_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == RSTFLTSEL_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == RSTFLTSEL_A::_11111
    }
}
#[doc = "Write proxy for field `RSTFLTSEL`"]
pub struct RSTFLTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus clock filter count is 1"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00000)
    }
    #[doc = "Bus clock filter count is 2"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00001)
    }
    #[doc = "Bus clock filter count is 3"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00010)
    }
    #[doc = "Bus clock filter count is 4"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00011)
    }
    #[doc = "Bus clock filter count is 5"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00100)
    }
    #[doc = "Bus clock filter count is 6"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00101)
    }
    #[doc = "Bus clock filter count is 7"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00110)
    }
    #[doc = "Bus clock filter count is 8"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_00111)
    }
    #[doc = "Bus clock filter count is 9"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01000)
    }
    #[doc = "Bus clock filter count is 10"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01001)
    }
    #[doc = "Bus clock filter count is 11"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01010)
    }
    #[doc = "Bus clock filter count is 12"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01011)
    }
    #[doc = "Bus clock filter count is 13"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01100)
    }
    #[doc = "Bus clock filter count is 14"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01101)
    }
    #[doc = "Bus clock filter count is 15"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01110)
    }
    #[doc = "Bus clock filter count is 16"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_01111)
    }
    #[doc = "Bus clock filter count is 17"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10000)
    }
    #[doc = "Bus clock filter count is 18"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10001)
    }
    #[doc = "Bus clock filter count is 19"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10010)
    }
    #[doc = "Bus clock filter count is 20"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10011)
    }
    #[doc = "Bus clock filter count is 21"]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10100)
    }
    #[doc = "Bus clock filter count is 22"]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10101)
    }
    #[doc = "Bus clock filter count is 23"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10110)
    }
    #[doc = "Bus clock filter count is 24"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_10111)
    }
    #[doc = "Bus clock filter count is 25"]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11000)
    }
    #[doc = "Bus clock filter count is 26"]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11001)
    }
    #[doc = "Bus clock filter count is 27"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11010)
    }
    #[doc = "Bus clock filter count is 28"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11011)
    }
    #[doc = "Bus clock filter count is 29"]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11100)
    }
    #[doc = "Bus clock filter count is 30"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11101)
    }
    #[doc = "Bus clock filter count is 31"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11110)
    }
    #[doc = "Bus clock filter count is 32"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(RSTFLTSEL_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    pub fn rstfltsel(&self) -> RSTFLTSEL_R {
        RSTFLTSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    pub fn rstfltsel(&mut self) -> RSTFLTSEL_W {
        RSTFLTSEL_W { w: self }
    }
}
