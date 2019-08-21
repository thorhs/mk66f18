#[doc = "Reader of register DEBUG1"]
pub type R = crate::R<u32, super::DEBUG1>;
#[doc = "Writer for register DEBUG1"]
pub type W = crate::W<u32, super::DEBUG1>;
#[doc = "Register DEBUG1 `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::DEBUG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Delay increment of the rise of squelch:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTAILADJVD_A {
    #[doc = "0: Delay is nominal"]
    _00,
    #[doc = "1: Delay is +20%"]
    _01,
    #[doc = "2: Delay is -20%"]
    _10,
    #[doc = "3: Delay is -40%"]
    _11,
}
impl From<ENTAILADJVD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENTAILADJVD_A) -> Self {
        match variant {
            ENTAILADJVD_A::_00 => 0,
            ENTAILADJVD_A::_01 => 1,
            ENTAILADJVD_A::_10 => 2,
            ENTAILADJVD_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `ENTAILADJVD`"]
pub type ENTAILADJVD_R = crate::R<u8, ENTAILADJVD_A>;
impl ENTAILADJVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTAILADJVD_A {
        match self.bits {
            0 => ENTAILADJVD_A::_00,
            1 => ENTAILADJVD_A::_01,
            2 => ENTAILADJVD_A::_10,
            3 => ENTAILADJVD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ENTAILADJVD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ENTAILADJVD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ENTAILADJVD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ENTAILADJVD_A::_11
    }
}
#[doc = "Write proxy for field `ENTAILADJVD`"]
pub struct ENTAILADJVD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTAILADJVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENTAILADJVD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay is nominal"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::_00)
    }
    #[doc = "Delay is +20%"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::_01)
    }
    #[doc = "Delay is -20%"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::_10)
    }
    #[doc = "Delay is -40%"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> ENTAILADJVD_R {
        ENTAILADJVD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub fn entailadjvd(&mut self) -> ENTAILADJVD_W {
        ENTAILADJVD_W { w: self }
    }
}
