#[doc = "Reader of register FILT1"]
pub type R = crate::R<u8, super::FILT1>;
#[doc = "Writer for register FILT1"]
pub type W = crate::W<u8, super::FILT1>;
#[doc = "Register FILT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FILT1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Filter Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSEL_A {
    #[doc = "0: Select LLWU_P0 for filter"]
    _00000,
    #[doc = "31: Select LLWU_P31 for filter"]
    _11111,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        match variant {
            FILTSEL_A::_00000 => 0,
            FILTSEL_A::_11111 => 31,
        }
    }
}
#[doc = "Reader of field `FILTSEL`"]
pub type FILTSEL_R = crate::R<u8, FILTSEL_A>;
impl FILTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTSEL_A::_00000),
            31 => Val(FILTSEL_A::_11111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == FILTSEL_A::_00000
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == FILTSEL_A::_11111
    }
}
#[doc = "Write proxy for field `FILTSEL`"]
pub struct FILTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select LLWU_P0 for filter"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(FILTSEL_A::_00000)
    }
    #[doc = "Select LLWU_P31 for filter"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(FILTSEL_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Digital Filter On External Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTE_A {
    #[doc = "0: Filter disabled"]
    _00,
    #[doc = "1: Filter posedge detect enabled"]
    _01,
    #[doc = "2: Filter negedge detect enabled"]
    _10,
    #[doc = "3: Filter any edge detect enabled"]
    _11,
}
impl From<FILTE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTE_A) -> Self {
        match variant {
            FILTE_A::_00 => 0,
            FILTE_A::_01 => 1,
            FILTE_A::_10 => 2,
            FILTE_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FILTE`"]
pub type FILTE_R = crate::R<u8, FILTE_A>;
impl FILTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTE_A {
        match self.bits {
            0 => FILTE_A::_00,
            1 => FILTE_A::_01,
            2 => FILTE_A::_10,
            3 => FILTE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FILTE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FILTE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FILTE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FILTE_A::_11
    }
}
#[doc = "Write proxy for field `FILTE`"]
pub struct FILTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FILTE_A::_00)
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FILTE_A::_01)
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FILTE_A::_10)
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FILTE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u8) & 0x03) << 5);
        self.w
    }
}
#[doc = "Filter Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTF_A {
    #[doc = "0: Pin Filter 1 was not a wakeup source"]
    _0,
    #[doc = "1: Pin Filter 1 was a wakeup source"]
    _1,
}
impl From<FILTF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTF_A) -> Self {
        match variant {
            FILTF_A::_0 => false,
            FILTF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FILTF`"]
pub type FILTF_R = crate::R<bool, FILTF_A>;
impl FILTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTF_A {
        match self.bits {
            false => FILTF_A::_0,
            true => FILTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILTF_A::_1
    }
}
#[doc = "Write proxy for field `FILTF`"]
pub struct FILTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Filter 1 was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTF_A::_0)
    }
    #[doc = "Pin Filter 1 was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTF_A::_1)
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
    #[doc = "Bits 0:4 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&self) -> FILTE_R {
        FILTE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&self) -> FILTF_R {
        FILTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FILTSEL_W {
        FILTSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&mut self) -> FILTE_W {
        FILTE_W { w: self }
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&mut self) -> FILTF_W {
        FILTF_W { w: self }
    }
}
