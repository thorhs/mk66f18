#[doc = "Reader of register USB_SBUSCFG"]
pub type R = crate::R<u32, super::USB_SBUSCFG>;
#[doc = "Writer for register USB_SBUSCFG"]
pub type W = crate::W<u32, super::USB_SBUSCFG>;
#[doc = "Register USB_SBUSCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_SBUSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTMODE_A {
    #[doc = "0: INCR burst of unspecified length"]
    _000,
    #[doc = "1: INCR4, non-multiple transfers of INCR4 is decomposed into singles."]
    _001,
    #[doc = "2: INCR8, non-multiple transfers of INCR8, is decomposed into INCR4 or singles."]
    _010,
    #[doc = "3: INCR16, non-multiple transfers of INCR16, is decomposed into INCR8, INCR4 or singles."]
    _011,
    #[doc = "4: Reserved, do not use."]
    _100,
    #[doc = "5: INCR4, non-multiple transfers of INCR4 is decomposed into smaller unspecified length bursts."]
    _101,
    #[doc = "6: INCR8, non-multiple transfers of INCR8 is decomposed into smaller unspecified length bursts."]
    _110,
    #[doc = "7: INCR16, non-multiple transfers of INCR16 is decomposed into smaller unspecified length bursts."]
    _111,
}
impl From<BURSTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTMODE_A) -> Self {
        match variant {
            BURSTMODE_A::_000 => 0,
            BURSTMODE_A::_001 => 1,
            BURSTMODE_A::_010 => 2,
            BURSTMODE_A::_011 => 3,
            BURSTMODE_A::_100 => 4,
            BURSTMODE_A::_101 => 5,
            BURSTMODE_A::_110 => 6,
            BURSTMODE_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `BURSTMODE`"]
pub type BURSTMODE_R = crate::R<u8, BURSTMODE_A>;
impl BURSTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTMODE_A {
        match self.bits {
            0 => BURSTMODE_A::_000,
            1 => BURSTMODE_A::_001,
            2 => BURSTMODE_A::_010,
            3 => BURSTMODE_A::_011,
            4 => BURSTMODE_A::_100,
            5 => BURSTMODE_A::_101,
            6 => BURSTMODE_A::_110,
            7 => BURSTMODE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BURSTMODE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BURSTMODE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BURSTMODE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BURSTMODE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BURSTMODE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BURSTMODE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BURSTMODE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == BURSTMODE_A::_111
    }
}
#[doc = "Write proxy for field `BURSTMODE`"]
pub struct BURSTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "INCR burst of unspecified length"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_000)
    }
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into singles."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_001)
    }
    #[doc = "INCR8, non-multiple transfers of INCR8, is decomposed into INCR4 or singles."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_010)
    }
    #[doc = "INCR16, non-multiple transfers of INCR16, is decomposed into INCR8, INCR4 or singles."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_011)
    }
    #[doc = "Reserved, do not use."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_100)
    }
    #[doc = "INCR4, non-multiple transfers of INCR4 is decomposed into smaller unspecified length bursts."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_101)
    }
    #[doc = "INCR8, non-multiple transfers of INCR8 is decomposed into smaller unspecified length bursts."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_110)
    }
    #[doc = "INCR16, non-multiple transfers of INCR16 is decomposed into smaller unspecified length bursts."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(BURSTMODE_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Burst mode"]
    #[inline(always)]
    pub fn burstmode(&self) -> BURSTMODE_R {
        BURSTMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst mode"]
    #[inline(always)]
    pub fn burstmode(&mut self) -> BURSTMODE_W {
        BURSTMODE_W { w: self }
    }
}
