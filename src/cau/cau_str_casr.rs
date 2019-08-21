#[doc = "Reader of register CAU_STR_CASR"]
pub type R = crate::R<u32, super::CAU_STR_CASR>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_A {
    #[doc = "0: No illegal commands issued"]
    _0,
    #[doc = "1: Illegal command issued"]
    _1,
}
impl From<IC_A> for bool {
    #[inline(always)]
    fn from(variant: IC_A) -> Self {
        match variant {
            IC_A::_0 => false,
            IC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IC`"]
pub type IC_R = crate::R<bool, IC_A>;
impl IC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_A {
        match self.bits {
            false => IC_A::_0,
            true => IC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IC_A::_1
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPE_A {
    #[doc = "0: No error detected"]
    _0,
    #[doc = "1: DES key parity error detected"]
    _1,
}
impl From<DPE_A> for bool {
    #[inline(always)]
    fn from(variant: DPE_A) -> Self {
        match variant {
            DPE_A::_0 => false,
            DPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPE`"]
pub type DPE_R = crate::R<bool, DPE_A>;
impl DPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPE_A {
        match self.bits {
            false => DPE_A::_0,
            true => DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPE_A::_1
    }
}
#[doc = "CAU version\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_A {
    #[doc = "1: Initial CAU version"]
    _0001,
    #[doc = "2: Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010,
}
impl From<VER_A> for u8 {
    #[inline(always)]
    fn from(variant: VER_A) -> Self {
        match variant {
            VER_A::_0001 => 1,
            VER_A::_0010 => 2,
        }
    }
}
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, VER_A>;
impl VER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VER_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(VER_A::_0001),
            2 => Val(VER_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == VER_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == VER_A::_0010
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn dpe(&self) -> DPE_R {
        DPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
