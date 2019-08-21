#[doc = "Reader of register C8"]
pub type R = crate::R<u8, super::C8>;
#[doc = "Writer for register C8"]
pub type W = crate::W<u8, super::C8>;
#[doc = "Register C8 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::C8 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "RTC Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCS1_A {
    #[doc = "0: Loss of RTC has not occur."]
    _0,
    #[doc = "1: Loss of RTC has occur"]
    _1,
}
impl From<LOCS1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCS1_A) -> Self {
        match variant {
            LOCS1_A::_0 => false,
            LOCS1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCS1`"]
pub type LOCS1_R = crate::R<bool, LOCS1_A>;
impl LOCS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCS1_A {
        match self.bits {
            false => LOCS1_A::_0,
            true => LOCS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCS1_A::_1
    }
}
#[doc = "Write proxy for field `LOCS1`"]
pub struct LOCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loss of RTC has not occur."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS1_A::_0)
    }
    #[doc = "Loss of RTC has occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS1_A::_1)
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
#[doc = "Clock Monitor Enable1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME1_A {
    #[doc = "0: External clock monitor is disabled for RTC clock."]
    _0,
    #[doc = "1: External clock monitor is enabled for RTC clock."]
    _1,
}
impl From<CME1_A> for bool {
    #[inline(always)]
    fn from(variant: CME1_A) -> Self {
        match variant {
            CME1_A::_0 => false,
            CME1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CME1`"]
pub type CME1_R = crate::R<bool, CME1_A>;
impl CME1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME1_A {
        match self.bits {
            false => CME1_A::_0,
            true => CME1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CME1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CME1_A::_1
    }
}
#[doc = "Write proxy for field `CME1`"]
pub struct CME1_W<'a> {
    w: &'a mut W,
}
impl<'a> CME1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock monitor is disabled for RTC clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME1_A::_0)
    }
    #[doc = "External clock monitor is enabled for RTC clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME1_A::_1)
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
#[doc = "PLL Loss of Lock Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLRE_A {
    #[doc = "0: Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0,
    #[doc = "1: Generate a reset request on a PLL loss of lock indication."]
    _1,
}
impl From<LOLRE_A> for bool {
    #[inline(always)]
    fn from(variant: LOLRE_A) -> Self {
        match variant {
            LOLRE_A::_0 => false,
            LOLRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOLRE`"]
pub type LOLRE_R = crate::R<bool, LOLRE_A>;
impl LOLRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLRE_A {
        match self.bits {
            false => LOLRE_A::_0,
            true => LOLRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLRE_A::_1
    }
}
#[doc = "Write proxy for field `LOLRE`"]
pub struct LOLRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLRE_A::_0)
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLRE_A::_1)
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
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE1_A {
    #[doc = "0: Interrupt request is generated on a loss of RTC external reference clock."]
    _0,
    #[doc = "1: Generate a reset request on a loss of RTC external reference clock"]
    _1,
}
impl From<LOCRE1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRE1_A) -> Self {
        match variant {
            LOCRE1_A::_0 => false,
            LOCRE1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCRE1`"]
pub type LOCRE1_R = crate::R<bool, LOCRE1_A>;
impl LOCRE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRE1_A {
        match self.bits {
            false => LOCRE1_A::_0,
            true => LOCRE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCRE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCRE1_A::_1
    }
}
#[doc = "Write proxy for field `LOCRE1`"]
pub struct LOCRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCRE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCRE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE1_A::_0)
    }
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE1_A::_1)
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
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    pub fn locs1(&self) -> LOCS1_R {
        LOCS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    pub fn cme1(&self) -> CME1_R {
        CME1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&self) -> LOLRE_R {
        LOLRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre1(&self) -> LOCRE1_R {
        LOCRE1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    pub fn locs1(&mut self) -> LOCS1_W {
        LOCS1_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    pub fn cme1(&mut self) -> CME1_W {
        CME1_W { w: self }
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&mut self) -> LOLRE_W {
        LOLRE_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre1(&mut self) -> LOCRE1_W {
        LOCRE1_W { w: self }
    }
}
