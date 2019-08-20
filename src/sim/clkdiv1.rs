#[doc = "Reader of register CLKDIV1"]
pub type R = crate::R<u32, super::CLKDIV1>;
#[doc = "Writer for register CLKDIV1"]
pub type W = crate::W<u32, super::CLKDIV1>;
#[doc = "Register CLKDIV1 `reset()`'s with value 0x0011_0000"]
impl crate::ResetValue for super::CLKDIV1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0000
    }
}
#[doc = "Clock 4 output divider value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV4_A {
    #[doc = "0: Divide-by-1."]
    _0000,
    #[doc = "1: Divide-by-2."]
    _0001,
    #[doc = "2: Divide-by-3."]
    _0010,
    #[doc = "3: Divide-by-4."]
    _0011,
    #[doc = "4: Divide-by-5."]
    _0100,
    #[doc = "5: Divide-by-6."]
    _0101,
    #[doc = "6: Divide-by-7."]
    _0110,
    #[doc = "7: Divide-by-8."]
    _0111,
    #[doc = "8: Divide-by-9."]
    _1000,
    #[doc = "9: Divide-by-10."]
    _1001,
    #[doc = "10: Divide-by-11."]
    _1010,
    #[doc = "11: Divide-by-12."]
    _1011,
    #[doc = "12: Divide-by-13."]
    _1100,
    #[doc = "13: Divide-by-14."]
    _1101,
    #[doc = "14: Divide-by-15."]
    _1110,
    #[doc = "15: Divide-by-16."]
    _1111,
}
impl From<OUTDIV4_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV4_A) -> Self {
        match variant {
            OUTDIV4_A::_0000 => 0,
            OUTDIV4_A::_0001 => 1,
            OUTDIV4_A::_0010 => 2,
            OUTDIV4_A::_0011 => 3,
            OUTDIV4_A::_0100 => 4,
            OUTDIV4_A::_0101 => 5,
            OUTDIV4_A::_0110 => 6,
            OUTDIV4_A::_0111 => 7,
            OUTDIV4_A::_1000 => 8,
            OUTDIV4_A::_1001 => 9,
            OUTDIV4_A::_1010 => 10,
            OUTDIV4_A::_1011 => 11,
            OUTDIV4_A::_1100 => 12,
            OUTDIV4_A::_1101 => 13,
            OUTDIV4_A::_1110 => 14,
            OUTDIV4_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `OUTDIV4`"]
pub type OUTDIV4_R = crate::R<u8, OUTDIV4_A>;
impl OUTDIV4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV4_A {
        match self.bits {
            0 => OUTDIV4_A::_0000,
            1 => OUTDIV4_A::_0001,
            2 => OUTDIV4_A::_0010,
            3 => OUTDIV4_A::_0011,
            4 => OUTDIV4_A::_0100,
            5 => OUTDIV4_A::_0101,
            6 => OUTDIV4_A::_0110,
            7 => OUTDIV4_A::_0111,
            8 => OUTDIV4_A::_1000,
            9 => OUTDIV4_A::_1001,
            10 => OUTDIV4_A::_1010,
            11 => OUTDIV4_A::_1011,
            12 => OUTDIV4_A::_1100,
            13 => OUTDIV4_A::_1101,
            14 => OUTDIV4_A::_1110,
            15 => OUTDIV4_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == OUTDIV4_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == OUTDIV4_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == OUTDIV4_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == OUTDIV4_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == OUTDIV4_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == OUTDIV4_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == OUTDIV4_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == OUTDIV4_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == OUTDIV4_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == OUTDIV4_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == OUTDIV4_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == OUTDIV4_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == OUTDIV4_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == OUTDIV4_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == OUTDIV4_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == OUTDIV4_A::_1111
    }
}
#[doc = "Write proxy for field `OUTDIV4`"]
pub struct OUTDIV4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_0111)
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1000)
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1001)
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1010)
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1011)
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1100)
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1101)
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1110)
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(OUTDIV4_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Clock 3 output divider value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV3_A {
    #[doc = "0: Divide-by-1."]
    _0000,
    #[doc = "1: Divide-by-2."]
    _0001,
    #[doc = "2: Divide-by-3."]
    _0010,
    #[doc = "3: Divide-by-4."]
    _0011,
    #[doc = "4: Divide-by-5."]
    _0100,
    #[doc = "5: Divide-by-6."]
    _0101,
    #[doc = "6: Divide-by-7."]
    _0110,
    #[doc = "7: Divide-by-8."]
    _0111,
    #[doc = "8: Divide-by-9."]
    _1000,
    #[doc = "9: Divide-by-10."]
    _1001,
    #[doc = "10: Divide-by-11."]
    _1010,
    #[doc = "11: Divide-by-12."]
    _1011,
    #[doc = "12: Divide-by-13."]
    _1100,
    #[doc = "13: Divide-by-14."]
    _1101,
    #[doc = "14: Divide-by-15."]
    _1110,
    #[doc = "15: Divide-by-16."]
    _1111,
}
impl From<OUTDIV3_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV3_A) -> Self {
        match variant {
            OUTDIV3_A::_0000 => 0,
            OUTDIV3_A::_0001 => 1,
            OUTDIV3_A::_0010 => 2,
            OUTDIV3_A::_0011 => 3,
            OUTDIV3_A::_0100 => 4,
            OUTDIV3_A::_0101 => 5,
            OUTDIV3_A::_0110 => 6,
            OUTDIV3_A::_0111 => 7,
            OUTDIV3_A::_1000 => 8,
            OUTDIV3_A::_1001 => 9,
            OUTDIV3_A::_1010 => 10,
            OUTDIV3_A::_1011 => 11,
            OUTDIV3_A::_1100 => 12,
            OUTDIV3_A::_1101 => 13,
            OUTDIV3_A::_1110 => 14,
            OUTDIV3_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `OUTDIV3`"]
pub type OUTDIV3_R = crate::R<u8, OUTDIV3_A>;
impl OUTDIV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV3_A {
        match self.bits {
            0 => OUTDIV3_A::_0000,
            1 => OUTDIV3_A::_0001,
            2 => OUTDIV3_A::_0010,
            3 => OUTDIV3_A::_0011,
            4 => OUTDIV3_A::_0100,
            5 => OUTDIV3_A::_0101,
            6 => OUTDIV3_A::_0110,
            7 => OUTDIV3_A::_0111,
            8 => OUTDIV3_A::_1000,
            9 => OUTDIV3_A::_1001,
            10 => OUTDIV3_A::_1010,
            11 => OUTDIV3_A::_1011,
            12 => OUTDIV3_A::_1100,
            13 => OUTDIV3_A::_1101,
            14 => OUTDIV3_A::_1110,
            15 => OUTDIV3_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == OUTDIV3_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == OUTDIV3_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == OUTDIV3_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == OUTDIV3_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == OUTDIV3_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == OUTDIV3_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == OUTDIV3_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == OUTDIV3_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == OUTDIV3_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == OUTDIV3_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == OUTDIV3_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == OUTDIV3_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == OUTDIV3_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == OUTDIV3_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == OUTDIV3_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == OUTDIV3_A::_1111
    }
}
#[doc = "Write proxy for field `OUTDIV3`"]
pub struct OUTDIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_0111)
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1000)
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1001)
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1010)
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1011)
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1100)
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1101)
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1110)
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(OUTDIV3_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Clock 2 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV2_A {
    #[doc = "0: Divide-by-1."]
    _0000,
    #[doc = "1: Divide-by-2."]
    _0001,
    #[doc = "2: Divide-by-3."]
    _0010,
    #[doc = "3: Divide-by-4."]
    _0011,
    #[doc = "4: Divide-by-5."]
    _0100,
    #[doc = "5: Divide-by-6."]
    _0101,
    #[doc = "6: Divide-by-7."]
    _0110,
    #[doc = "7: Divide-by-8."]
    _0111,
    #[doc = "8: Divide-by-9."]
    _1000,
    #[doc = "9: Divide-by-10."]
    _1001,
    #[doc = "10: Divide-by-11."]
    _1010,
    #[doc = "11: Divide-by-12."]
    _1011,
    #[doc = "12: Divide-by-13."]
    _1100,
    #[doc = "13: Divide-by-14."]
    _1101,
    #[doc = "14: Divide-by-15."]
    _1110,
    #[doc = "15: Divide-by-16."]
    _1111,
}
impl From<OUTDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV2_A) -> Self {
        match variant {
            OUTDIV2_A::_0000 => 0,
            OUTDIV2_A::_0001 => 1,
            OUTDIV2_A::_0010 => 2,
            OUTDIV2_A::_0011 => 3,
            OUTDIV2_A::_0100 => 4,
            OUTDIV2_A::_0101 => 5,
            OUTDIV2_A::_0110 => 6,
            OUTDIV2_A::_0111 => 7,
            OUTDIV2_A::_1000 => 8,
            OUTDIV2_A::_1001 => 9,
            OUTDIV2_A::_1010 => 10,
            OUTDIV2_A::_1011 => 11,
            OUTDIV2_A::_1100 => 12,
            OUTDIV2_A::_1101 => 13,
            OUTDIV2_A::_1110 => 14,
            OUTDIV2_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `OUTDIV2`"]
pub type OUTDIV2_R = crate::R<u8, OUTDIV2_A>;
impl OUTDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV2_A {
        match self.bits {
            0 => OUTDIV2_A::_0000,
            1 => OUTDIV2_A::_0001,
            2 => OUTDIV2_A::_0010,
            3 => OUTDIV2_A::_0011,
            4 => OUTDIV2_A::_0100,
            5 => OUTDIV2_A::_0101,
            6 => OUTDIV2_A::_0110,
            7 => OUTDIV2_A::_0111,
            8 => OUTDIV2_A::_1000,
            9 => OUTDIV2_A::_1001,
            10 => OUTDIV2_A::_1010,
            11 => OUTDIV2_A::_1011,
            12 => OUTDIV2_A::_1100,
            13 => OUTDIV2_A::_1101,
            14 => OUTDIV2_A::_1110,
            15 => OUTDIV2_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == OUTDIV2_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == OUTDIV2_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == OUTDIV2_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == OUTDIV2_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == OUTDIV2_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == OUTDIV2_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == OUTDIV2_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == OUTDIV2_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == OUTDIV2_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == OUTDIV2_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == OUTDIV2_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == OUTDIV2_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == OUTDIV2_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == OUTDIV2_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == OUTDIV2_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == OUTDIV2_A::_1111
    }
}
#[doc = "Write proxy for field `OUTDIV2`"]
pub struct OUTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_0111)
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1000)
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1001)
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1010)
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1011)
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1100)
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1101)
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1110)
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(OUTDIV2_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Clock 1 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDIV1_A {
    #[doc = "0: Divide-by-1."]
    _0000,
    #[doc = "1: Divide-by-2."]
    _0001,
    #[doc = "2: Divide-by-3."]
    _0010,
    #[doc = "3: Divide-by-4."]
    _0011,
    #[doc = "4: Divide-by-5."]
    _0100,
    #[doc = "5: Divide-by-6."]
    _0101,
    #[doc = "6: Divide-by-7."]
    _0110,
    #[doc = "7: Divide-by-8."]
    _0111,
    #[doc = "8: Divide-by-9."]
    _1000,
    #[doc = "9: Divide-by-10."]
    _1001,
    #[doc = "10: Divide-by-11."]
    _1010,
    #[doc = "11: Divide-by-12."]
    _1011,
    #[doc = "12: Divide-by-13."]
    _1100,
    #[doc = "13: Divide-by-14."]
    _1101,
    #[doc = "14: Divide-by-15."]
    _1110,
    #[doc = "15: Divide-by-16."]
    _1111,
}
impl From<OUTDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTDIV1_A) -> Self {
        match variant {
            OUTDIV1_A::_0000 => 0,
            OUTDIV1_A::_0001 => 1,
            OUTDIV1_A::_0010 => 2,
            OUTDIV1_A::_0011 => 3,
            OUTDIV1_A::_0100 => 4,
            OUTDIV1_A::_0101 => 5,
            OUTDIV1_A::_0110 => 6,
            OUTDIV1_A::_0111 => 7,
            OUTDIV1_A::_1000 => 8,
            OUTDIV1_A::_1001 => 9,
            OUTDIV1_A::_1010 => 10,
            OUTDIV1_A::_1011 => 11,
            OUTDIV1_A::_1100 => 12,
            OUTDIV1_A::_1101 => 13,
            OUTDIV1_A::_1110 => 14,
            OUTDIV1_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `OUTDIV1`"]
pub type OUTDIV1_R = crate::R<u8, OUTDIV1_A>;
impl OUTDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDIV1_A {
        match self.bits {
            0 => OUTDIV1_A::_0000,
            1 => OUTDIV1_A::_0001,
            2 => OUTDIV1_A::_0010,
            3 => OUTDIV1_A::_0011,
            4 => OUTDIV1_A::_0100,
            5 => OUTDIV1_A::_0101,
            6 => OUTDIV1_A::_0110,
            7 => OUTDIV1_A::_0111,
            8 => OUTDIV1_A::_1000,
            9 => OUTDIV1_A::_1001,
            10 => OUTDIV1_A::_1010,
            11 => OUTDIV1_A::_1011,
            12 => OUTDIV1_A::_1100,
            13 => OUTDIV1_A::_1101,
            14 => OUTDIV1_A::_1110,
            15 => OUTDIV1_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == OUTDIV1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == OUTDIV1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == OUTDIV1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == OUTDIV1_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == OUTDIV1_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == OUTDIV1_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == OUTDIV1_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == OUTDIV1_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == OUTDIV1_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == OUTDIV1_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == OUTDIV1_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == OUTDIV1_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == OUTDIV1_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == OUTDIV1_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == OUTDIV1_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == OUTDIV1_A::_1111
    }
}
#[doc = "Write proxy for field `OUTDIV1`"]
pub struct OUTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_0111)
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1000)
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1001)
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1010)
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1011)
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1100)
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1101)
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1110)
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(OUTDIV1_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Clock 4 output divider value"]
    #[inline(always)]
    pub fn outdiv4(&self) -> OUTDIV4_R {
        OUTDIV4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&self) -> OUTDIV3_R {
        OUTDIV3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&self) -> OUTDIV2_R {
        OUTDIV2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&self) -> OUTDIV1_R {
        OUTDIV1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Clock 4 output divider value"]
    #[inline(always)]
    pub fn outdiv4(&mut self) -> OUTDIV4_W {
        OUTDIV4_W { w: self }
    }
    #[doc = "Bits 20:23 - Clock 3 output divider value"]
    #[inline(always)]
    pub fn outdiv3(&mut self) -> OUTDIV3_W {
        OUTDIV3_W { w: self }
    }
    #[doc = "Bits 24:27 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&mut self) -> OUTDIV2_W {
        OUTDIV2_W { w: self }
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&mut self) -> OUTDIV1_W {
        OUTDIV1_W { w: self }
    }
}
