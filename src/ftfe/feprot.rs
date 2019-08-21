#[doc = "Reader of register FEPROT"]
pub type R = crate::R<u8, super::FEPROT>;
#[doc = "Writer for register FEPROT"]
pub type W = crate::W<u8, super::FEPROT>;
#[doc = "Register FEPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EEPROM Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPROT_A {
    #[doc = "0: For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    _0,
    #[doc = "1: For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    _1,
}
impl From<EPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROT_A) -> Self {
        match variant {
            EPROT_A::_0 => 0,
            EPROT_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `EPROT`"]
pub type EPROT_R = crate::R<u8, EPROT_A>;
impl EPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EPROT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EPROT_A::_0),
            1 => Val(EPROT_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPROT_A::_1
    }
}
#[doc = "Write proxy for field `EPROT`"]
pub struct EPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> EPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPROT_A::_0)
    }
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPROT_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&mut self) -> EPROT_W {
        EPROT_W { w: self }
    }
}
