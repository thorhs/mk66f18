#[doc = "Reader of register FCFG2"]
pub type R = crate::R<u32, super::FCFG2>;
#[doc = "Reader of field `MAXADDR1`"]
pub type MAXADDR1_R = crate::R<u8, u8>;
#[doc = "Program flash only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLSH_A {
    #[doc = "0: Device supports FlexNVM"]
    _0,
    #[doc = "1: Program Flash only, device does not support FlexNVM"]
    _1,
}
impl From<PFLSH_A> for bool {
    #[inline(always)]
    fn from(variant: PFLSH_A) -> Self {
        match variant {
            PFLSH_A::_0 => false,
            PFLSH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PFLSH`"]
pub type PFLSH_R = crate::R<bool, PFLSH_A>;
impl PFLSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFLSH_A {
        match self.bits {
            false => PFLSH_A::_0,
            true => PFLSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFLSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFLSH_A::_1
    }
}
#[doc = "Reader of field `MAXADDR0`"]
pub type MAXADDR0_R = crate::R<u8, u8>;
#[doc = "Swap program flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPPFLSH_A {
    #[doc = "0: Swap is not active."]
    _0,
    #[doc = "1: Swap is active."]
    _1,
}
impl From<SWAPPFLSH_A> for bool {
    #[inline(always)]
    fn from(variant: SWAPPFLSH_A) -> Self {
        match variant {
            SWAPPFLSH_A::_0 => false,
            SWAPPFLSH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SWAPPFLSH`"]
pub type SWAPPFLSH_R = crate::R<bool, SWAPPFLSH_A>;
impl SWAPPFLSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAPPFLSH_A {
        match self.bits {
            false => SWAPPFLSH_A::_0,
            true => SWAPPFLSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAPPFLSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAPPFLSH_A::_1
    }
}
impl R {
    #[doc = "Bits 16:22 - Max address block 1"]
    #[inline(always)]
    pub fn maxaddr1(&self) -> MAXADDR1_R {
        MAXADDR1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Program flash only"]
    #[inline(always)]
    pub fn pflsh(&self) -> PFLSH_R {
        PFLSH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - Max address block 0"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> MAXADDR0_R {
        MAXADDR0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Swap program flash"]
    #[inline(always)]
    pub fn swappflsh(&self) -> SWAPPFLSH_R {
        SWAPPFLSH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
