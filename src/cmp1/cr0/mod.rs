#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `HYSTCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTCTRR {
    #[doc = "Level 0"]
    _00,
    #[doc = "Level 1"]
    _01,
    #[doc = "Level 2"]
    _10,
    #[doc = "Level 3"]
    _11,
}
impl HYSTCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSTCTRR::_00 => 0,
            HYSTCTRR::_01 => 1,
            HYSTCTRR::_10 => 2,
            HYSTCTRR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSTCTRR {
        match value {
            0 => HYSTCTRR::_00,
            1 => HYSTCTRR::_01,
            2 => HYSTCTRR::_10,
            3 => HYSTCTRR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == HYSTCTRR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == HYSTCTRR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == HYSTCTRR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == HYSTCTRR::_11
    }
}
#[doc = "Possible values of the field `FILTER_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_CNTR {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    _000,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    _001,
    #[doc = "2 consecutive samples must agree."]
    _010,
    #[doc = "3 consecutive samples must agree."]
    _011,
    #[doc = "4 consecutive samples must agree."]
    _100,
    #[doc = "5 consecutive samples must agree."]
    _101,
    #[doc = "6 consecutive samples must agree."]
    _110,
    #[doc = "7 consecutive samples must agree."]
    _111,
}
impl FILTER_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTER_CNTR::_000 => 0,
            FILTER_CNTR::_001 => 1,
            FILTER_CNTR::_010 => 2,
            FILTER_CNTR::_011 => 3,
            FILTER_CNTR::_100 => 4,
            FILTER_CNTR::_101 => 5,
            FILTER_CNTR::_110 => 6,
            FILTER_CNTR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTER_CNTR {
        match value {
            0 => FILTER_CNTR::_000,
            1 => FILTER_CNTR::_001,
            2 => FILTER_CNTR::_010,
            3 => FILTER_CNTR::_011,
            4 => FILTER_CNTR::_100,
            5 => FILTER_CNTR::_101,
            6 => FILTER_CNTR::_110,
            7 => FILTER_CNTR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FILTER_CNTR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FILTER_CNTR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FILTER_CNTR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FILTER_CNTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FILTER_CNTR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FILTER_CNTR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FILTER_CNTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FILTER_CNTR::_111
    }
}
#[doc = "Values that can be written to the field `HYSTCTR`"]
pub enum HYSTCTRW {
    #[doc = "Level 0"]
    _00,
    #[doc = "Level 1"]
    _01,
    #[doc = "Level 2"]
    _10,
    #[doc = "Level 3"]
    _11,
}
impl HYSTCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSTCTRW::_00 => 0,
            HYSTCTRW::_01 => 1,
            HYSTCTRW::_10 => 2,
            HYSTCTRW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTCTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(HYSTCTRW::_00)
    }
    #[doc = "Level 1"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(HYSTCTRW::_01)
    }
    #[doc = "Level 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(HYSTCTRW::_10)
    }
    #[doc = "Level 3"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(HYSTCTRW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTER_CNT`"]
pub enum FILTER_CNTW {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    _000,
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    _001,
    #[doc = "2 consecutive samples must agree."]
    _010,
    #[doc = "3 consecutive samples must agree."]
    _011,
    #[doc = "4 consecutive samples must agree."]
    _100,
    #[doc = "5 consecutive samples must agree."]
    _101,
    #[doc = "6 consecutive samples must agree."]
    _110,
    #[doc = "7 consecutive samples must agree."]
    _111,
}
impl FILTER_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTER_CNTW::_000 => 0,
            FILTER_CNTW::_001 => 1,
            FILTER_CNTW::_010 => 2,
            FILTER_CNTW::_011 => 3,
            FILTER_CNTW::_100 => 4,
            FILTER_CNTW::_101 => 5,
            FILTER_CNTW::_110 => 6,
            FILTER_CNTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTER_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTER_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTER_CNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_000)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_001)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_010)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_011)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_100)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_101)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_110)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline]
    pub fn hystctr(&self) -> HYSTCTRR {
        HYSTCTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&self) -> FILTER_CNTR {
        FILTER_CNTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline]
    pub fn hystctr(&mut self) -> _HYSTCTRW {
        _HYSTCTRW { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&mut self) -> _FILTER_CNTW {
        _FILTER_CNTW { w: self }
    }
}
