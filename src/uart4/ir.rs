#[doc = "Reader of register IR"]
pub type R = crate::R<u8, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u8, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmitter narrow pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNP_A {
    #[doc = "0: 3/16."]
    _00,
    #[doc = "1: 1/16."]
    _01,
    #[doc = "2: 1/32."]
    _10,
    #[doc = "3: 1/4."]
    _11,
}
impl From<TNP_A> for u8 {
    #[inline(always)]
    fn from(variant: TNP_A) -> Self {
        match variant {
            TNP_A::_00 => 0,
            TNP_A::_01 => 1,
            TNP_A::_10 => 2,
            TNP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TNP`"]
pub type TNP_R = crate::R<u8, TNP_A>;
impl TNP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNP_A {
        match self.bits {
            0 => TNP_A::_00,
            1 => TNP_A::_01,
            2 => TNP_A::_10,
            3 => TNP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TNP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TNP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TNP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TNP_A::_11
    }
}
#[doc = "Write proxy for field `TNP`"]
pub struct TNP_W<'a> {
    w: &'a mut W,
}
impl<'a> TNP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3/16."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TNP_A::_00)
    }
    #[doc = "1/16."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TNP_A::_01)
    }
    #[doc = "1/32."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TNP_A::_10)
    }
    #[doc = "1/4."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TNP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: IR disabled."]
    _0,
    #[doc = "1: IR enabled."]
    _1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        match variant {
            IREN_A::_0 => false,
            IREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IREN`"]
pub type IREN_R = crate::R<bool, IREN_A>;
impl IREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::_0,
            true => IREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREN_A::_1
    }
}
#[doc = "Write proxy for field `IREN`"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREN_A::_0)
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&self) -> TNP_R {
        TNP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&mut self) -> TNP_W {
        TNP_W { w: self }
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
}
