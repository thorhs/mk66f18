#[doc = "Reader of register FPROT%s"]
pub type R = crate::R<u8, super::FPROT>;
#[doc = "Writer for register FPROT%s"]
pub type W = crate::W<u8, super::FPROT>;
#[doc = "Register FPROT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::FPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Program Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROT_A {
    #[doc = "0: Program flash region is protected."]
    _0,
    #[doc = "1: Program flash region is not protected"]
    _1,
}
impl From<PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROT_A) -> Self {
        match variant {
            PROT_A::_0 => 0,
            PROT_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<u8, PROT_A>;
impl PROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROT_A::_0),
            1 => Val(PROT_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROT_A::_1
    }
}
#[doc = "Write proxy for field `PROT`"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Program flash region is protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROT_A::_0)
    }
    #[doc = "Program flash region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROT_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
}
