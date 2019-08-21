#[doc = "Reader of register EDR%s"]
pub type R = crate::R<u32, super::EDR>;
#[doc = "Error Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERW_A {
    #[doc = "0: Read"]
    _0,
    #[doc = "1: Write"]
    _1,
}
impl From<ERW_A> for bool {
    #[inline(always)]
    fn from(variant: ERW_A) -> Self {
        match variant {
            ERW_A::_0 => false,
            ERW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERW`"]
pub type ERW_R = crate::R<bool, ERW_A>;
impl ERW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERW_A {
        match self.bits {
            false => ERW_A::_0,
            true => ERW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERW_A::_1
    }
}
#[doc = "Error Attributes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EATTR_A {
    #[doc = "0: User mode, instruction access"]
    _000,
    #[doc = "1: User mode, data access"]
    _001,
    #[doc = "2: Supervisor mode, instruction access"]
    _010,
    #[doc = "3: Supervisor mode, data access"]
    _011,
}
impl From<EATTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EATTR_A) -> Self {
        match variant {
            EATTR_A::_000 => 0,
            EATTR_A::_001 => 1,
            EATTR_A::_010 => 2,
            EATTR_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `EATTR`"]
pub type EATTR_R = crate::R<u8, EATTR_A>;
impl EATTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EATTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EATTR_A::_000),
            1 => Val(EATTR_A::_001),
            2 => Val(EATTR_A::_010),
            3 => Val(EATTR_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == EATTR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == EATTR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == EATTR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == EATTR_A::_011
    }
}
#[doc = "Reader of field `EMN`"]
pub type EMN_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPID`"]
pub type EPID_R = crate::R<u8, u8>;
#[doc = "Reader of field `EACD`"]
pub type EACD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Error Read/Write"]
    #[inline(always)]
    pub fn erw(&self) -> ERW_R {
        ERW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Error Attributes"]
    #[inline(always)]
    pub fn eattr(&self) -> EATTR_R {
        EATTR_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Error Master Number"]
    #[inline(always)]
    pub fn emn(&self) -> EMN_R {
        EMN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Error Process Identification"]
    #[inline(always)]
    pub fn epid(&self) -> EPID_R {
        EPID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Error Access Control Detail"]
    #[inline(always)]
    pub fn eacd(&self) -> EACD_R {
        EACD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
