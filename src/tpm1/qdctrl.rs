#[doc = "Reader of register QDCTRL"]
pub type R = crate::R<u32, super::QDCTRL>;
#[doc = "Writer for register QDCTRL"]
pub type W = crate::W<u32, super::QDCTRL>;
#[doc = "Register QDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::QDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables the quadrature decoder mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADEN_A {
    #[doc = "0: Quadrature decoder mode is disabled."]
    _0,
    #[doc = "1: Quadrature decoder mode is enabled."]
    _1,
}
impl From<QUADEN_A> for bool {
    #[inline(always)]
    fn from(variant: QUADEN_A) -> Self {
        match variant {
            QUADEN_A::_0 => false,
            QUADEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `QUADEN`"]
pub type QUADEN_R = crate::R<bool, QUADEN_A>;
impl QUADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADEN_A {
        match self.bits {
            false => QUADEN_A::_0,
            true => QUADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADEN_A::_1
    }
}
#[doc = "Write proxy for field `QUADEN`"]
pub struct QUADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature decoder mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADEN_A::_0)
    }
    #[doc = "Quadrature decoder mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADEN_A::_1)
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
#[doc = "Indicates if the TOF bit was set on the top or the bottom of counting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIR_A {
    #[doc = "0: TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (zero) to its maximum value (MOD register)."]
    _0,
    #[doc = "1: TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (zero)."]
    _1,
}
impl From<TOFDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TOFDIR_A) -> Self {
        match variant {
            TOFDIR_A::_0 => false,
            TOFDIR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOFDIR`"]
pub type TOFDIR_R = crate::R<bool, TOFDIR_A>;
impl TOFDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOFDIR_A {
        match self.bits {
            false => TOFDIR_A::_0,
            true => TOFDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOFDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOFDIR_A::_1
    }
}
#[doc = "Counter Direction in Quadrature Decode Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIR_A {
    #[doc = "0: Counter direction is decreasing (counter decrement)."]
    _0,
    #[doc = "1: Counter direction is increasing (counter increment)."]
    _1,
}
impl From<QUADIR_A> for bool {
    #[inline(always)]
    fn from(variant: QUADIR_A) -> Self {
        match variant {
            QUADIR_A::_0 => false,
            QUADIR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `QUADIR`"]
pub type QUADIR_R = crate::R<bool, QUADIR_A>;
impl QUADIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADIR_A {
        match self.bits {
            false => QUADIR_A::_0,
            true => QUADIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADIR_A::_1
    }
}
#[doc = "Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODE_A {
    #[doc = "0: Phase encoding mode."]
    _0,
    #[doc = "1: Count and direction encoding mode."]
    _1,
}
impl From<QUADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: QUADMODE_A) -> Self {
        match variant {
            QUADMODE_A::_0 => false,
            QUADMODE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `QUADMODE`"]
pub type QUADMODE_R = crate::R<bool, QUADMODE_A>;
impl QUADMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADMODE_A {
        match self.bits {
            false => QUADMODE_A::_0,
            true => QUADMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADMODE_A::_1
    }
}
#[doc = "Write proxy for field `QUADMODE`"]
pub struct QUADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase encoding mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADMODE_A::_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADMODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the quadrature decoder mode"]
    #[inline(always)]
    pub fn quaden(&self) -> QUADEN_R {
        QUADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the TOF bit was set on the top or the bottom of counting."]
    #[inline(always)]
    pub fn tofdir(&self) -> TOFDIR_R {
        TOFDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Direction in Quadrature Decode Mode"]
    #[inline(always)]
    pub fn quadir(&self) -> QUADIR_R {
        QUADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&self) -> QUADMODE_R {
        QUADMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the quadrature decoder mode"]
    #[inline(always)]
    pub fn quaden(&mut self) -> QUADEN_W {
        QUADEN_W { w: self }
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&mut self) -> QUADMODE_W {
        QUADMODE_W { w: self }
    }
}
