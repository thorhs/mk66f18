#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NUMTOF`"]
pub type NUMTOF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMTOF`"]
pub struct NUMTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMTOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `BDMMODE`"]
pub type BDMMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDMMODE`"]
pub struct BDMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Global Time Base Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
    #[doc = "0: Use of an external global time base is disabled."]
    _0,
    #[doc = "1: Use of an external global time base is enabled."]
    _1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        match variant {
            GTBEEN_A::_0 => false,
            GTBEEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GTBEEN`"]
pub type GTBEEN_R = crate::R<bool, GTBEEN_A>;
impl GTBEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::_0,
            true => GTBEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTBEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTBEEN_A::_1
    }
}
#[doc = "Write proxy for field `GTBEEN`"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use of an external global time base is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEEN_A::_0)
    }
    #[doc = "Use of an external global time base is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Global Time Base Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEOUT_A {
    #[doc = "0: A global time base signal generation is disabled."]
    _0,
    #[doc = "1: A global time base signal generation is enabled."]
    _1,
}
impl From<GTBEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEOUT_A) -> Self {
        match variant {
            GTBEOUT_A::_0 => false,
            GTBEOUT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GTBEOUT`"]
pub type GTBEOUT_R = crate::R<bool, GTBEOUT_A>;
impl GTBEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEOUT_A {
        match self.bits {
            false => GTBEOUT_A::_0,
            true => GTBEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTBEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTBEOUT_A::_1
    }
}
#[doc = "Write proxy for field `GTBEOUT`"]
pub struct GTBEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A global time base signal generation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_0)
    }
    #[doc = "A global time base signal generation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    pub fn numtof(&self) -> NUMTOF_R {
        NUMTOF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline(always)]
    pub fn bdmmode(&self) -> BDMMODE_R {
        BDMMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&self) -> GTBEOUT_R {
        GTBEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    pub fn numtof(&mut self) -> NUMTOF_W {
        NUMTOF_W { w: self }
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline(always)]
    pub fn bdmmode(&mut self) -> BDMMODE_W {
        BDMMODE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&mut self) -> GTBEOUT_W {
        GTBEOUT_W { w: self }
    }
}
