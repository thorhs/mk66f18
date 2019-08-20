#[doc = "Reader of register FDPROT"]
pub type R = crate::R<u8, super::FDPROT>;
#[doc = "Writer for register FDPROT"]
pub type W = crate::W<u8, super::FDPROT>;
#[doc = "Register FDPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::FDPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPROT_A {
    #[doc = "0: Data Flash region is protected"]
    _0,
    #[doc = "1: Data Flash region is not protected"]
    _1,
}
impl From<DPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: DPROT_A) -> Self {
        match variant {
            DPROT_A::_0 => 0,
            DPROT_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `DPROT`"]
pub type DPROT_R = crate::R<u8, DPROT_A>;
impl DPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DPROT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DPROT_A::_0),
            1 => Val(DPROT_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPROT_A::_1
    }
}
#[doc = "Write proxy for field `DPROT`"]
pub struct DPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPROT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data Flash region is protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPROT_A::_0)
    }
    #[doc = "Data Flash region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPROT_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&mut self) -> DPROT_W {
        DPROT_W { w: self }
    }
}
