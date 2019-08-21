#[doc = "Reader of register REGSC"]
pub type R = crate::R<u8, super::REGSC>;
#[doc = "Writer for register REGSC"]
pub type W = crate::W<u8, super::REGSC>;
#[doc = "Register REGSC `reset()`'s with value 0x24"]
impl crate::ResetValue for super::REGSC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x24
    }
}
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBE_A {
    #[doc = "0: Bandgap buffer not enabled"]
    _0,
    #[doc = "1: Bandgap buffer enabled"]
    _1,
}
impl From<BGBE_A> for bool {
    #[inline(always)]
    fn from(variant: BGBE_A) -> Self {
        match variant {
            BGBE_A::_0 => false,
            BGBE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BGBE`"]
pub type BGBE_R = crate::R<bool, BGBE_A>;
impl BGBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGBE_A {
        match self.bits {
            false => BGBE_A::_0,
            true => BGBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGBE_A::_1
    }
}
#[doc = "Write proxy for field `BGBE`"]
pub struct BGBE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bandgap buffer not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGBE_A::_0)
    }
    #[doc = "Bandgap buffer enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGBE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Regulator In Run Regulation Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGONS_A {
    #[doc = "0: Regulator is in stop regulation or in transition to/from it"]
    _0,
    #[doc = "1: Regulator is in run regulation"]
    _1,
}
impl From<REGONS_A> for bool {
    #[inline(always)]
    fn from(variant: REGONS_A) -> Self {
        match variant {
            REGONS_A::_0 => false,
            REGONS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `REGONS`"]
pub type REGONS_R = crate::R<bool, REGONS_A>;
impl REGONS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGONS_A {
        match self.bits {
            false => REGONS_A::_0,
            true => REGONS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REGONS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REGONS_A::_1
    }
}
#[doc = "Acknowledge Isolation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKISO_A {
    #[doc = "0: Peripherals and I/O pads are in normal run state."]
    _0,
    #[doc = "1: Certain peripherals and I/O pads are in an isolated and latched state."]
    _1,
}
impl From<ACKISO_A> for bool {
    #[inline(always)]
    fn from(variant: ACKISO_A) -> Self {
        match variant {
            ACKISO_A::_0 => false,
            ACKISO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACKISO`"]
pub type ACKISO_R = crate::R<bool, ACKISO_A>;
impl ACKISO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKISO_A {
        match self.bits {
            false => ACKISO_A::_0,
            true => ACKISO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKISO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKISO_A::_1
    }
}
#[doc = "Write proxy for field `ACKISO`"]
pub struct ACKISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKISO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripherals and I/O pads are in normal run state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKISO_A::_0)
    }
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKISO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Bandgap Enable In VLPx Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEN_A {
    #[doc = "0: Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    _0,
    #[doc = "1: Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    _1,
}
impl From<BGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGEN_A) -> Self {
        match variant {
            BGEN_A::_0 => false,
            BGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BGEN`"]
pub type BGEN_R = crate::R<bool, BGEN_A>;
impl BGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGEN_A {
        match self.bits {
            false => BGEN_A::_0,
            true => BGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGEN_A::_1
    }
}
#[doc = "Write proxy for field `BGEN`"]
pub struct BGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEN_A::_0)
    }
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&self) -> BGBE_R {
        BGBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Regulator In Run Regulation Status"]
    #[inline(always)]
    pub fn regons(&self) -> REGONS_R {
        REGONS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    pub fn ackiso(&self) -> ACKISO_R {
        ACKISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    pub fn bgen(&self) -> BGEN_R {
        BGEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&mut self) -> BGBE_W {
        BGBE_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    pub fn ackiso(&mut self) -> ACKISO_W {
        ACKISO_W { w: self }
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    pub fn bgen(&mut self) -> BGEN_W {
        BGEN_W { w: self }
    }
}
