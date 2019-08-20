#[doc = "Reader of register FLT"]
pub type R = crate::R<u8, super::FLT>;
#[doc = "Writer for register FLT"]
pub type W = crate::W<u8, super::FLT>;
#[doc = "Register FLT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C Programmable Filter Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_A {
    #[doc = "0: No filter/bypass"]
    _0,
}
impl From<FLT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_A) -> Self {
        match variant {
            FLT_A::_0 => 0,
        }
    }
}
#[doc = "Reader of field `FLT`"]
pub type FLT_R = crate::R<u8, FLT_A>;
impl FLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLT_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT_A::_0
    }
}
#[doc = "Write proxy for field `FLT`"]
pub struct FLT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "I2C Bus Start Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTF_A {
    #[doc = "0: No start happens on I2C bus"]
    _0,
    #[doc = "1: Start detected on I2C bus"]
    _1,
}
impl From<STARTF_A> for bool {
    #[inline(always)]
    fn from(variant: STARTF_A) -> Self {
        match variant {
            STARTF_A::_0 => false,
            STARTF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STARTF`"]
pub type STARTF_R = crate::R<bool, STARTF_A>;
impl STARTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTF_A {
        match self.bits {
            false => STARTF_A::_0,
            true => STARTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STARTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STARTF_A::_1
    }
}
#[doc = "Write proxy for field `STARTF`"]
pub struct STARTF_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No start happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTF_A::_0)
    }
    #[doc = "Start detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTF_A::_1)
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
#[doc = "I2C Bus Stop or Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIE_A {
    #[doc = "0: Stop or start detection interrupt is disabled"]
    _0,
    #[doc = "1: Stop or start detection interrupt is enabled"]
    _1,
}
impl From<SSIE_A> for bool {
    #[inline(always)]
    fn from(variant: SSIE_A) -> Self {
        match variant {
            SSIE_A::_0 => false,
            SSIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SSIE`"]
pub type SSIE_R = crate::R<bool, SSIE_A>;
impl SSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIE_A {
        match self.bits {
            false => SSIE_A::_0,
            true => SSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIE_A::_1
    }
}
#[doc = "Write proxy for field `SSIE`"]
pub struct SSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIE_A::_0)
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIE_A::_1)
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
#[doc = "I2C Bus Stop Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    #[doc = "0: No stop happens on I2C bus"]
    _0,
    #[doc = "1: Stop detected on I2C bus"]
    _1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        match variant {
            STOPF_A::_0 => false,
            STOPF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STOPF`"]
pub type STOPF_R = crate::R<bool, STOPF_A>;
impl STOPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::_0,
            true => STOPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPF_A::_1
    }
}
#[doc = "Write proxy for field `STOPF`"]
pub struct STOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No stop happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPF_A::_0)
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPF_A::_1)
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
#[doc = "Stop Hold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHEN_A {
    #[doc = "0: Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    _0,
    #[doc = "1: Stop holdoff is enabled."]
    _1,
}
impl From<SHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN_A) -> Self {
        match variant {
            SHEN_A::_0 => false,
            SHEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SHEN`"]
pub type SHEN_R = crate::R<bool, SHEN_A>;
impl SHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN_A {
        match self.bits {
            false => SHEN_A::_0,
            true => SHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHEN_A::_1
    }
}
#[doc = "Write proxy for field `SHEN`"]
pub struct SHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN_A::_0)
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN_A::_1)
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
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&self) -> SHEN_R {
        SHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&mut self) -> FLT_W {
        FLT_W { w: self }
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> STARTF_W {
        STARTF_W { w: self }
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W {
        SSIE_W { w: self }
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&mut self) -> STOPF_W {
        STOPF_W { w: self }
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&mut self) -> SHEN_W {
        SHEN_W { w: self }
    }
}
