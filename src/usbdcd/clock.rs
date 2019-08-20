#[doc = "Reader of register CLOCK"]
pub type R = crate::R<u32, super::CLOCK>;
#[doc = "Writer for register CLOCK"]
pub type W = crate::W<u32, super::CLOCK>;
#[doc = "Register CLOCK `reset()`'s with value 0xc1"]
impl crate::ResetValue for super::CLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc1
    }
}
#[doc = "Unit of Measurement Encoding for Clock Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_UNIT_A {
    #[doc = "0: kHz Speed (between 1 kHz and 1023 kHz)"]
    _0,
    #[doc = "1: MHz Speed (between 1 MHz and 1023 MHz)"]
    _1,
}
impl From<CLOCK_UNIT_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_UNIT_A) -> Self {
        match variant {
            CLOCK_UNIT_A::_0 => false,
            CLOCK_UNIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CLOCK_UNIT`"]
pub type CLOCK_UNIT_R = crate::R<bool, CLOCK_UNIT_A>;
impl CLOCK_UNIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_UNIT_A {
        match self.bits {
            false => CLOCK_UNIT_A::_0,
            true => CLOCK_UNIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLOCK_UNIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLOCK_UNIT_A::_1
    }
}
#[doc = "Write proxy for field `CLOCK_UNIT`"]
pub struct CLOCK_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_UNIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_UNIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOCK_UNIT_A::_0)
    }
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOCK_UNIT_A::_1)
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
#[doc = "Reader of field `CLOCK_SPEED`"]
pub type CLOCK_SPEED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLOCK_SPEED`"]
pub struct CLOCK_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub fn clock_unit(&self) -> CLOCK_UNIT_R {
        CLOCK_UNIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub fn clock_speed(&self) -> CLOCK_SPEED_R {
        CLOCK_SPEED_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub fn clock_unit(&mut self) -> CLOCK_UNIT_W {
        CLOCK_UNIT_W { w: self }
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub fn clock_speed(&mut self) -> CLOCK_SPEED_W {
        CLOCK_SPEED_W { w: self }
    }
}
