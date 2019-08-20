#[doc = "Reader of register RGD%s_WORD3"]
pub type R = crate::R<u32, super::RGD_WORD3>;
#[doc = "Writer for register RGD%s_WORD3"]
pub type W = crate::W<u32, super::RGD_WORD3>;
#[doc = "Register RGD%s_WORD3 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RGD_WORD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
    #[doc = "0: Region descriptor is invalid"]
    _0,
    #[doc = "1: Region descriptor is valid"]
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
    #[doc = "Region descriptor is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VLD_A::_0)
    }
    #[doc = "Region descriptor is valid"]
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
#[doc = "Reader of field `PIDMASK`"]
pub type PIDMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIDMASK`"]
pub struct PIDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Process Identifier Mask"]
    #[inline(always)]
    pub fn pidmask(&self) -> PIDMASK_R {
        PIDMASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Process Identifier"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&mut self) -> VLD_W {
        VLD_W { w: self }
    }
    #[doc = "Bits 16:23 - Process Identifier Mask"]
    #[inline(always)]
    pub fn pidmask(&mut self) -> PIDMASK_W {
        PIDMASK_W { w: self }
    }
    #[doc = "Bits 24:31 - Process Identifier"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
