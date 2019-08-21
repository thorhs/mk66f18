#[doc = "Writer for register CAU_RADR_CASR"]
pub type W = crate::W<u32, super::CAU_RADR_CASR>;
#[doc = "Register CAU_RADR_CASR `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::CAU_RADR_CASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_AW {
    #[doc = "0: No illegal commands issued"]
    _0,
    #[doc = "1: Illegal command issued"]
    _1,
}
impl From<IC_AW> for bool {
    #[inline(always)]
    fn from(variant: IC_AW) -> Self {
        match variant {
            IC_AW::_0 => false,
            IC_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `IC`"]
pub struct IC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No illegal commands issued"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IC_AW::_0)
    }
    #[doc = "Illegal command issued"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IC_AW::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPE_AW {
    #[doc = "0: No error detected"]
    _0,
    #[doc = "1: DES key parity error detected"]
    _1,
}
impl From<DPE_AW> for bool {
    #[inline(always)]
    fn from(variant: DPE_AW) -> Self {
        match variant {
            DPE_AW::_0 => false,
            DPE_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `DPE`"]
pub struct DPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPE_AW::_0)
    }
    #[doc = "DES key parity error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPE_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "CAU version\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_AW {
    #[doc = "1: Initial CAU version"]
    _0001,
    #[doc = "2: Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010,
}
impl From<VER_AW> for u8 {
    #[inline(always)]
    fn from(variant: VER_AW) -> Self {
        match variant {
            VER_AW::_0001 => 1,
            VER_AW::_0010 => 2,
        }
    }
}
#[doc = "Write proxy for field `VER`"]
pub struct VER_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VER_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Initial CAU version"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(VER_AW::_0001)
    }
    #[doc = "Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(VER_AW::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ic(&mut self) -> IC_W {
        IC_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn dpe(&mut self) -> DPE_W {
        DPE_W { w: self }
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline(always)]
    pub fn ver(&mut self) -> VER_W {
        VER_W { w: self }
    }
}
