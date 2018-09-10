#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB_SBUSCFG {
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
#[doc = "Possible values of the field `BURSTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTMODER {
    #[doc = "INCR burst of unspecified length"]
    _000,
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into singles."]
    _001,
    #[doc = "INCR8, non-multiple transfers of INCR8, is decomposed into INCR4 or singles."]
    _010,
    #[doc = "INCR16, non-multiple transfers of INCR16, is decomposed into INCR8, INCR4 or singles."]
    _011,
    #[doc = "Reserved, do not use."]
    _100,
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into smaller unspecified length bursts."]
    _101,
    #[doc = "INCR8, non-multiple transfers of INCR8 is decomposed into smaller unspecified length bursts."]
    _110,
    #[doc = "INCR16, non-multiple transfers of INCR16 is decomposed into smaller unspecified length bursts."]
    _111,
}
impl BURSTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURSTMODER::_000 => 0,
            BURSTMODER::_001 => 1,
            BURSTMODER::_010 => 2,
            BURSTMODER::_011 => 3,
            BURSTMODER::_100 => 4,
            BURSTMODER::_101 => 5,
            BURSTMODER::_110 => 6,
            BURSTMODER::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURSTMODER {
        match value {
            0 => BURSTMODER::_000,
            1 => BURSTMODER::_001,
            2 => BURSTMODER::_010,
            3 => BURSTMODER::_011,
            4 => BURSTMODER::_100,
            5 => BURSTMODER::_101,
            6 => BURSTMODER::_110,
            7 => BURSTMODER::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == BURSTMODER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == BURSTMODER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == BURSTMODER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == BURSTMODER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == BURSTMODER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == BURSTMODER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == BURSTMODER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == BURSTMODER::_111
    }
}
#[doc = "Values that can be written to the field `BURSTMODE`"]
pub enum BURSTMODEW {
    #[doc = "INCR burst of unspecified length"]
    _000,
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into singles."]
    _001,
    #[doc = "INCR8, non-multiple transfers of INCR8, is decomposed into INCR4 or singles."]
    _010,
    #[doc = "INCR16, non-multiple transfers of INCR16, is decomposed into INCR8, INCR4 or singles."]
    _011,
    #[doc = "Reserved, do not use."]
    _100,
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into smaller unspecified length bursts."]
    _101,
    #[doc = "INCR8, non-multiple transfers of INCR8 is decomposed into smaller unspecified length bursts."]
    _110,
    #[doc = "INCR16, non-multiple transfers of INCR16 is decomposed into smaller unspecified length bursts."]
    _111,
}
impl BURSTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTMODEW::_000 => 0,
            BURSTMODEW::_001 => 1,
            BURSTMODEW::_010 => 2,
            BURSTMODEW::_011 => 3,
            BURSTMODEW::_100 => 4,
            BURSTMODEW::_101 => 5,
            BURSTMODEW::_110 => 6,
            BURSTMODEW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "INCR burst of unspecified length"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(BURSTMODEW::_000)
    }
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into singles."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(BURSTMODEW::_001)
    }
    #[doc = "INCR8, non-multiple transfers of INCR8, is decomposed into INCR4 or singles."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(BURSTMODEW::_010)
    }
    #[doc = "INCR16, non-multiple transfers of INCR16, is decomposed into INCR8, INCR4 or singles."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(BURSTMODEW::_011)
    }
    #[doc = "Reserved, do not use."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(BURSTMODEW::_100)
    }
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into smaller unspecified length bursts."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(BURSTMODEW::_101)
    }
    #[doc = "INCR8, non-multiple transfers of INCR8 is decomposed into smaller unspecified length bursts."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(BURSTMODEW::_110)
    }
    #[doc = "INCR16, non-multiple transfers of INCR16 is decomposed into smaller unspecified length bursts."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(BURSTMODEW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Burst mode"]
    #[inline]
    pub fn burstmode(&self) -> BURSTMODER {
        BURSTMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Burst mode"]
    #[inline]
    pub fn burstmode(&mut self) -> _BURSTMODEW {
        _BURSTMODEW { w: self }
    }
}
