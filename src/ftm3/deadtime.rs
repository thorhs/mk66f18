#[doc = "Reader of register DEADTIME"]
pub type R = crate::R<u32, super::DEADTIME>;
#[doc = "Writer for register DEADTIME"]
pub type W = crate::W<u32, super::DEADTIME>;
#[doc = "Register DEADTIME `reset()`'s with value 0"]
impl crate::ResetValue for super::DEADTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTVAL`"]
pub type DTVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTVAL`"]
pub struct DTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Deadtime Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPS_A {
    #[doc = "2: Divide the system clock by 4."]
    _10,
    #[doc = "3: Divide the system clock by 16."]
    _11,
}
impl From<DTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS_A) -> Self {
        match variant {
            DTPS_A::_10 => 2,
            DTPS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `DTPS`"]
pub type DTPS_R = crate::R<u8, DTPS_A>;
impl DTPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPS_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(DTPS_A::_10),
            3 => Val(DTPS_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DTPS_A::_11
    }
}
#[doc = "Write proxy for field `DTPS`"]
pub struct DTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide the system clock by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTPS_A::_10)
    }
    #[doc = "Divide the system clock by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DTPS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&self) -> DTVAL_R {
        DTVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&self) -> DTPS_R {
        DTPS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&mut self) -> DTVAL_W {
        DTVAL_W { w: self }
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&mut self) -> DTPS_W {
        DTPS_W { w: self }
    }
}
