#[doc = "Reader of register CR0"]
pub type R = crate::R<u8, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u8, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator hard block hysteresis control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTCTR_A {
    #[doc = "0: Level 0"]
    _00,
    #[doc = "1: Level 1"]
    _01,
    #[doc = "2: Level 2"]
    _10,
    #[doc = "3: Level 3"]
    _11,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        match variant {
            HYSTCTR_A::_00 => 0,
            HYSTCTR_A::_01 => 1,
            HYSTCTR_A::_10 => 2,
            HYSTCTR_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `HYSTCTR`"]
pub type HYSTCTR_R = crate::R<u8, HYSTCTR_A>;
impl HYSTCTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::_00,
            1 => HYSTCTR_A::_01,
            2 => HYSTCTR_A::_10,
            3 => HYSTCTR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HYSTCTR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HYSTCTR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HYSTCTR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HYSTCTR_A::_11
    }
}
#[doc = "Write proxy for field `HYSTCTR`"]
pub struct HYSTCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSTCTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_00)
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_01)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_10)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    _000,
    #[doc = "1: One sample must agree. The comparator output is simply sampled."]
    _001,
    #[doc = "2: 2 consecutive samples must agree."]
    _010,
    #[doc = "3: 3 consecutive samples must agree."]
    _011,
    #[doc = "4: 4 consecutive samples must agree."]
    _100,
    #[doc = "5: 5 consecutive samples must agree."]
    _101,
    #[doc = "6: 6 consecutive samples must agree."]
    _110,
    #[doc = "7: 7 consecutive samples must agree."]
    _111,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        match variant {
            FILTER_CNT_A::_000 => 0,
            FILTER_CNT_A::_001 => 1,
            FILTER_CNT_A::_010 => 2,
            FILTER_CNT_A::_011 => 3,
            FILTER_CNT_A::_100 => 4,
            FILTER_CNT_A::_101 => 5,
            FILTER_CNT_A::_110 => 6,
            FILTER_CNT_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `FILTER_CNT`"]
pub type FILTER_CNT_R = crate::R<u8, FILTER_CNT_A>;
impl FILTER_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::_000,
            1 => FILTER_CNT_A::_001,
            2 => FILTER_CNT_A::_010,
            3 => FILTER_CNT_A::_011,
            4 => FILTER_CNT_A::_100,
            5 => FILTER_CNT_A::_101,
            6 => FILTER_CNT_A::_110,
            7 => FILTER_CNT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FILTER_CNT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FILTER_CNT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FILTER_CNT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FILTER_CNT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FILTER_CNT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FILTER_CNT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FILTER_CNT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FILTER_CNT_A::_111
    }
}
#[doc = "Write proxy for field `FILTER_CNT`"]
pub struct FILTER_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_CNT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_000)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_001)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_010)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_011)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_100)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_101)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_110)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&mut self) -> HYSTCTR_W {
        HYSTCTR_W { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W {
        FILTER_CNT_W { w: self }
    }
}
