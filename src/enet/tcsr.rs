#[doc = "Reader of register TCSR%s"]
pub type R = crate::R<u32, super::TCSR>;
#[doc = "Writer for register TCSR%s"]
pub type W = crate::W<u32, super::TCSR>;
#[doc = "Register TCSR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: DMA request is disabled"]
    _0,
    #[doc = "1: DMA request is enabled"]
    _1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        match variant {
            TDRE_A::_0 => false,
            TDRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, TDRE_A>;
impl TDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Write proxy for field `TDRE`"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_A::_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMODE_A {
    #[doc = "0: Timer Channel is disabled."]
    _0000,
    #[doc = "1: Timer Channel is configured for Input Capture on rising edge."]
    _0001,
    #[doc = "2: Timer Channel is configured for Input Capture on falling edge."]
    _0010,
    #[doc = "3: Timer Channel is configured for Input Capture on both edges."]
    _0011,
    #[doc = "4: Timer Channel is configured for Output Compare - software only."]
    _0100,
    #[doc = "5: Timer Channel is configured for Output Compare - toggle output on compare."]
    _0101,
    #[doc = "6: Timer Channel is configured for Output Compare - clear output on compare."]
    _0110,
    #[doc = "7: Timer Channel is configured for Output Compare - set output on compare."]
    _0111,
    #[doc = "10: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    _1010,
    #[doc = "14: Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    _1110,
    #[doc = "15: Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    _1111,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        match variant {
            TMODE_A::_0000 => 0,
            TMODE_A::_0001 => 1,
            TMODE_A::_0010 => 2,
            TMODE_A::_0011 => 3,
            TMODE_A::_0100 => 4,
            TMODE_A::_0101 => 5,
            TMODE_A::_0110 => 6,
            TMODE_A::_0111 => 7,
            TMODE_A::_1010 => 10,
            TMODE_A::_1110 => 14,
            TMODE_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `TMODE`"]
pub type TMODE_R = crate::R<u8, TMODE_A>;
impl TMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMODE_A::_0000),
            1 => Val(TMODE_A::_0001),
            2 => Val(TMODE_A::_0010),
            3 => Val(TMODE_A::_0011),
            4 => Val(TMODE_A::_0100),
            5 => Val(TMODE_A::_0101),
            6 => Val(TMODE_A::_0110),
            7 => Val(TMODE_A::_0111),
            10 => Val(TMODE_A::_1010),
            14 => Val(TMODE_A::_1110),
            15 => Val(TMODE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TMODE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TMODE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TMODE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TMODE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TMODE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TMODE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TMODE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TMODE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TMODE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TMODE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TMODE_A::_1111
    }
}
#[doc = "Write proxy for field `TMODE`"]
pub struct TMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Channel is disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TMODE_A::_0000)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TMODE_A::_0001)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TMODE_A::_0010)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TMODE_A::_0011)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TMODE_A::_0100)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TMODE_A::_0101)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TMODE_A::_0110)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TMODE_A::_0111)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TMODE_A::_1010)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588-clock cycle."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TMODE_A::_1110)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588-clock cycle."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TMODE_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Interrupt is disabled"]
    _0,
    #[doc = "1: Interrupt is enabled"]
    _1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        match variant {
            TIE_A::_0 => false,
            TIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
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
#[doc = "Timer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_A {
    #[doc = "0: Input Capture or Output Compare has not occurred."]
    _0,
    #[doc = "1: Input Capture or Output Compare has occurred."]
    _1,
}
impl From<TF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_A) -> Self {
        match variant {
            TF_A::_0 => false,
            TF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TF`"]
pub type TF_R = crate::R<bool, TF_A>;
impl TF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_A {
        match self.bits {
            false => TF_A::_0,
            true => TF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TF_A::_1
    }
}
#[doc = "Write proxy for field `TF`"]
pub struct TF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF_A::_0)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&mut self) -> TMODE_W {
        TMODE_W { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W { w: self }
    }
}
