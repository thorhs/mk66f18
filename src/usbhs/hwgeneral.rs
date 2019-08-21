#[doc = "Reader of register HWGENERAL"]
pub type R = crate::R<u32, super::HWGENERAL>;
#[doc = "PHY Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYW_A {
    #[doc = "1: 16 bit wide data bus"]
    _01,
}
impl From<PHYW_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYW_A) -> Self {
        match variant {
            PHYW_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `PHYW`"]
pub type PHYW_R = crate::R<u8, PHYW_A>;
impl PHYW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PHYW_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PHYW_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PHYW_A::_01
    }
}
#[doc = "PHY Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYM_A {
    #[doc = "0: Controller configured for UTMI/UTMI+ interface."]
    _000,
}
impl From<PHYM_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYM_A) -> Self {
        match variant {
            PHYM_A::_000 => 0,
        }
    }
}
#[doc = "Reader of field `PHYM`"]
pub type PHYM_R = crate::R<u8, PHYM_A>;
impl PHYM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PHYM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PHYM_A::_000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PHYM_A::_000
    }
}
#[doc = "Serial mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A {
    #[doc = "0: No Serial Engine, always use parallel signaling."]
    _00,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        match variant {
            SM_A::_00 => 0,
        }
    }
}
#[doc = "Reader of field `SM`"]
pub type SM_R = crate::R<u8, SM_A>;
impl SM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SM_A::_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SM_A::_00
    }
}
impl R {
    #[doc = "Bits 4:5 - PHY Width"]
    #[inline(always)]
    pub fn phyw(&self) -> PHYW_R {
        PHYW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:8 - PHY Mode"]
    #[inline(always)]
    pub fn phym(&self) -> PHYM_R {
        PHYM_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - Serial mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
