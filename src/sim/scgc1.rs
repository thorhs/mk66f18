#[doc = "Reader of register SCGC1"]
pub type R = crate::R<u32, super::SCGC1>;
#[doc = "Writer for register SCGC1"]
pub type W = crate::W<u32, super::SCGC1>;
#[doc = "Register SCGC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        match variant {
            I2C2_A::_0 => false,
            I2C2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `I2C2`"]
pub type I2C2_R = crate::R<bool, I2C2_A>;
impl I2C2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::_0,
            true => I2C2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C2_A::_1
    }
}
#[doc = "Write proxy for field `I2C2`"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "I2C3 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<I2C3_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_A) -> Self {
        match variant {
            I2C3_A::_0 => false,
            I2C3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `I2C3`"]
pub type I2C3_R = crate::R<bool, I2C3_A>;
impl I2C3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_A {
        match self.bits {
            false => I2C3_A::_0,
            true => I2C3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C3_A::_1
    }
}
#[doc = "Write proxy for field `I2C3`"]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C3_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "UART4 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<UART4_A> for bool {
    #[inline(always)]
    fn from(variant: UART4_A) -> Self {
        match variant {
            UART4_A::_0 => false,
            UART4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UART4`"]
pub type UART4_R = crate::R<bool, UART4_A>;
impl UART4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4_A {
        match self.bits {
            false => UART4_A::_0,
            true => UART4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART4_A::_1
    }
}
#[doc = "Write proxy for field `UART4`"]
pub struct UART4_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART4_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART4_A::_1)
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
    #[doc = "Bit 6 - I2C2 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C3 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - I2C2 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 7 - I2C3 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART4_W {
        UART4_W { w: self }
    }
}
