#[doc = "Reader of register CESR"]
pub type R = crate::R<u32, super::CESR>;
#[doc = "Writer for register CESR"]
pub type W = crate::W<u32, super::CESR>;
#[doc = "Register CESR `reset()`'s with value 0x0081_5101"]
impl crate::ResetValue for super::CESR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0081_5101
    }
}
#[doc = "Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
    #[doc = "0: MPU is disabled. All accesses from all bus masters are allowed."]
    _0,
    #[doc = "1: MPU is enabled"]
    _1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        match variant {
            VLD_A::_0 => false,
            VLD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, VLD_A>;
impl VLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::_0,
            true => VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLD_A::_1
    }
}
#[doc = "Write proxy for field `VLD`"]
pub struct VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> VLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MPU is disabled. All accesses from all bus masters are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VLD_A::_0)
    }
    #[doc = "MPU is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VLD_A::_1)
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
#[doc = "Number Of Region Descriptors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRGD_A {
    #[doc = "0: 8 region descriptors"]
    _0000,
    #[doc = "1: 12 region descriptors"]
    _0001,
    #[doc = "2: 16 region descriptors"]
    _0010,
}
impl From<NRGD_A> for u8 {
    #[inline(always)]
    fn from(variant: NRGD_A) -> Self {
        match variant {
            NRGD_A::_0000 => 0,
            NRGD_A::_0001 => 1,
            NRGD_A::_0010 => 2,
        }
    }
}
#[doc = "Reader of field `NRGD`"]
pub type NRGD_R = crate::R<u8, NRGD_A>;
impl NRGD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NRGD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NRGD_A::_0000),
            1 => Val(NRGD_A::_0001),
            2 => Val(NRGD_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == NRGD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == NRGD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == NRGD_A::_0010
    }
}
#[doc = "Reader of field `NSP`"]
pub type NSP_R = crate::R<u8, u8>;
#[doc = "Reader of field `HRL`"]
pub type HRL_R = crate::R<u8, u8>;
#[doc = "Slave Port n Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPERR_A {
    #[doc = "0: No error has occurred for slave port n."]
    _0,
    #[doc = "1: An error has occurred for slave port n."]
    _1,
}
impl From<SPERR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPERR_A) -> Self {
        match variant {
            SPERR_A::_0 => 0,
            SPERR_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `SPERR`"]
pub type SPERR_R = crate::R<u8, SPERR_A>;
impl SPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPERR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPERR_A::_0),
            1 => Val(SPERR_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPERR_A::_1
    }
}
#[doc = "Write proxy for field `SPERR`"]
pub struct SPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPERR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No error has occurred for slave port n."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPERR_A::_0)
    }
    #[doc = "An error has occurred for slave port n."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPERR_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Number Of Region Descriptors"]
    #[inline(always)]
    pub fn nrgd(&self) -> NRGD_R {
        NRGD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number Of Slave Ports"]
    #[inline(always)]
    pub fn nsp(&self) -> NSP_R {
        NSP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Hardware Revision Level"]
    #[inline(always)]
    pub fn hrl(&self) -> HRL_R {
        HRL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline(always)]
    pub fn sperr(&self) -> SPERR_R {
        SPERR_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&mut self) -> VLD_W {
        VLD_W { w: self }
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline(always)]
    pub fn sperr(&mut self) -> SPERR_W {
        SPERR_W { w: self }
    }
}
