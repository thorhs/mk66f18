#[doc = "Reader of register HOSTVER"]
pub type R = crate::R<u32, super::HOSTVER>;
#[doc = "Specification Version Number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVN_A {
    #[doc = "1: SD host specification version 2.0, supports test event register and ADMA."]
    _1,
}
impl From<SVN_A> for u8 {
    #[inline(always)]
    fn from(variant: SVN_A) -> Self {
        match variant {
            SVN_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `SVN`"]
pub type SVN_R = crate::R<u8, SVN_A>;
impl SVN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SVN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SVN_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVN_A::_1
    }
}
#[doc = "Vendor Version Number\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VVN_A {
    #[doc = "0: Freescale SDHC version 1.0"]
    _0,
    #[doc = "16: Freescale SDHC version 2.0"]
    _10000,
    #[doc = "17: Freescale SDHC version 2.1"]
    _10001,
    #[doc = "18: Freescale SDHC version 2.2"]
    _10010,
}
impl From<VVN_A> for u8 {
    #[inline(always)]
    fn from(variant: VVN_A) -> Self {
        match variant {
            VVN_A::_0 => 0,
            VVN_A::_10000 => 16,
            VVN_A::_10001 => 17,
            VVN_A::_10010 => 18,
        }
    }
}
#[doc = "Reader of field `VVN`"]
pub type VVN_R = crate::R<u8, VVN_A>;
impl VVN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VVN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VVN_A::_0),
            16 => Val(VVN_A::_10000),
            17 => Val(VVN_A::_10001),
            18 => Val(VVN_A::_10010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VVN_A::_0
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == VVN_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == VVN_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == VVN_A::_10010
    }
}
impl R {
    #[doc = "Bits 0:7 - Specification Version Number"]
    #[inline(always)]
    pub fn svn(&self) -> SVN_R {
        SVN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version Number"]
    #[inline(always)]
    pub fn vvn(&self) -> VVN_R {
        VVN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
