#[doc = "Reader of register C4"]
pub type R = crate::R<u8, super::C4>;
#[doc = "Writer for register C4"]
pub type W = crate::W<u8, super::C4>;
#[doc = "Register C4 `reset()`'s with value 0"]
impl crate::ResetValue for super::C4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRFA`"]
pub type BRFA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRFA`"]
pub struct BRFA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10_A {
    #[doc = "0: The parity bit is the ninth bit in the serial transmission."]
    _0,
    #[doc = "1: The parity bit is the tenth bit in the serial transmission."]
    _1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        match variant {
            M10_A::_0 => false,
            M10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M10`"]
pub type M10_R = crate::R<bool, M10_A>;
impl M10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::_0,
            true => M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M10_A::_1
    }
}
#[doc = "Write proxy for field `M10`"]
pub struct M10_W<'a> {
    w: &'a mut W,
}
impl<'a> M10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10_A::_0)
    }
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        match variant {
            MAEN2_A::_0 => false,
            MAEN2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MAEN2`"]
pub type MAEN2_R = crate::R<bool, MAEN2_A>;
impl MAEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::_0,
            true => MAEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN2_A::_1
    }
}
#[doc = "Write proxy for field `MAEN2`"]
pub struct MAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        match variant {
            MAEN1_A::_0 => false,
            MAEN1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MAEN1`"]
pub type MAEN1_R = crate::R<bool, MAEN1_A>;
impl MAEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::_0,
            true => MAEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN1_A::_1
    }
}
#[doc = "Write proxy for field `MAEN1`"]
pub struct MAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&self) -> BRFA_R {
        BRFA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&mut self) -> BRFA_W {
        BRFA_W { w: self }
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&mut self) -> M10_W {
        M10_W { w: self }
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&mut self) -> MAEN2_W {
        MAEN2_W { w: self }
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&mut self) -> MAEN1_W {
        MAEN1_W { w: self }
    }
}
