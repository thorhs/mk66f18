#[doc = "Reader of register OTGCTL"]
pub type R = crate::R<u8, super::OTGCTL>;
#[doc = "Writer for register OTGCTL"]
pub type W = crate::W<u8, super::OTGCTL>;
#[doc = "Register OTGCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "On-The-Go pullup/pulldown resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGEN_A {
    #[doc = "0: If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    _0,
    #[doc = "1: The pull-up and pull-down controls in this register are used."]
    _1,
}
impl From<OTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGEN_A) -> Self {
        match variant {
            OTGEN_A::_0 => false,
            OTGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `OTGEN`"]
pub type OTGEN_R = crate::R<bool, OTGEN_A>;
impl OTGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGEN_A {
        match self.bits {
            false => OTGEN_A::_0,
            true => OTGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OTGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OTGEN_A::_1
    }
}
#[doc = "Write proxy for field `OTGEN`"]
pub struct OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGEN_A::_0)
    }
    #[doc = "The pull-up and pull-down controls in this register are used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGEN_A::_1)
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
#[doc = "D- Data Line pull-down resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMLOW_A {
    #[doc = "0: D- pulldown resistor is not enabled."]
    _0,
    #[doc = "1: D- pulldown resistor is enabled."]
    _1,
}
impl From<DMLOW_A> for bool {
    #[inline(always)]
    fn from(variant: DMLOW_A) -> Self {
        match variant {
            DMLOW_A::_0 => false,
            DMLOW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMLOW`"]
pub type DMLOW_R = crate::R<bool, DMLOW_A>;
impl DMLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMLOW_A {
        match self.bits {
            false => DMLOW_A::_0,
            true => DMLOW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMLOW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMLOW_A::_1
    }
}
#[doc = "Write proxy for field `DMLOW`"]
pub struct DMLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "D- pulldown resistor is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOW_A::_0)
    }
    #[doc = "D- pulldown resistor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOW_A::_1)
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
#[doc = "D+ Data Line pull-down resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPLOW_A {
    #[doc = "0: D+ pulldown resistor is not enabled."]
    _0,
    #[doc = "1: D+ pulldown resistor is enabled."]
    _1,
}
impl From<DPLOW_A> for bool {
    #[inline(always)]
    fn from(variant: DPLOW_A) -> Self {
        match variant {
            DPLOW_A::_0 => false,
            DPLOW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPLOW`"]
pub type DPLOW_R = crate::R<bool, DPLOW_A>;
impl DPLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPLOW_A {
        match self.bits {
            false => DPLOW_A::_0,
            true => DPLOW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPLOW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPLOW_A::_1
    }
}
#[doc = "Write proxy for field `DPLOW`"]
pub struct DPLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "D+ pulldown resistor is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPLOW_A::_0)
    }
    #[doc = "D+ pulldown resistor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPLOW_A::_1)
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
#[doc = "D+ Data Line pullup resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHIGH_A {
    #[doc = "0: D+ pullup resistor is not enabled"]
    _0,
    #[doc = "1: D+ pullup resistor is enabled"]
    _1,
}
impl From<DPHIGH_A> for bool {
    #[inline(always)]
    fn from(variant: DPHIGH_A) -> Self {
        match variant {
            DPHIGH_A::_0 => false,
            DPHIGH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPHIGH`"]
pub type DPHIGH_R = crate::R<bool, DPHIGH_A>;
impl DPHIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPHIGH_A {
        match self.bits {
            false => DPHIGH_A::_0,
            true => DPHIGH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPHIGH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPHIGH_A::_1
    }
}
#[doc = "Write proxy for field `DPHIGH`"]
pub struct DPHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPHIGH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "D+ pullup resistor is not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPHIGH_A::_0)
    }
    #[doc = "D+ pullup resistor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPHIGH_A::_1)
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
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dmlow(&self) -> DMLOW_R {
        DMLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dplow(&self) -> DPLOW_R {
        DPLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline(always)]
    pub fn dphigh(&self) -> DPHIGH_R {
        DPHIGH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline(always)]
    pub fn otgen(&mut self) -> OTGEN_W {
        OTGEN_W { w: self }
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dmlow(&mut self) -> DMLOW_W {
        DMLOW_W { w: self }
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dplow(&mut self) -> DPLOW_W {
        DPLOW_W { w: self }
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline(always)]
    pub fn dphigh(&mut self) -> DPHIGH_W {
        DPHIGH_W { w: self }
    }
}
