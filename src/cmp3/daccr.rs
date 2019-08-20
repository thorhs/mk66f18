#[doc = "Reader of register DACCR"]
pub type R = crate::R<u8, super::DACCR>;
#[doc = "Writer for register DACCR"]
pub type W = crate::W<u8, super::DACCR>;
#[doc = "Register DACCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DACCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VOSEL`"]
pub type VOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOSEL`"]
pub struct VOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSEL_A {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference."]
    _0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference."]
    _1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        match variant {
            VRSEL_A::_0 => false,
            VRSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VRSEL`"]
pub type VRSEL_R = crate::R<bool, VRSEL_A>;
impl VRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::_0,
            true => VRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRSEL_A::_1
    }
}
#[doc = "Write proxy for field `VRSEL`"]
pub struct VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRSEL_A::_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRSEL_A::_1)
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
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    _0,
    #[doc = "1: DAC is enabled."]
    _1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        match variant {
            DACEN_A::_0 => false,
            DACEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACEN_A::_1
    }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&mut self) -> VOSEL_W {
        VOSEL_W { w: self }
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VRSEL_W {
        VRSEL_W { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
}
