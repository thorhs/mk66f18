#[doc = "Reader of register FATR"]
pub type R = crate::R<u32, super::FATR>;
#[doc = "Bus error access type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEDA_A {
    #[doc = "0: Instruction"]
    _0,
    #[doc = "1: Data"]
    _1,
}
impl From<BEDA_A> for bool {
    #[inline(always)]
    fn from(variant: BEDA_A) -> Self {
        match variant {
            BEDA_A::_0 => false,
            BEDA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BEDA`"]
pub type BEDA_R = crate::R<bool, BEDA_A>;
impl BEDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEDA_A {
        match self.bits {
            false => BEDA_A::_0,
            true => BEDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEDA_A::_1
    }
}
#[doc = "Bus error privilege level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEMD_A {
    #[doc = "0: User mode"]
    _0,
    #[doc = "1: Supervisor/privileged mode"]
    _1,
}
impl From<BEMD_A> for bool {
    #[inline(always)]
    fn from(variant: BEMD_A) -> Self {
        match variant {
            BEMD_A::_0 => false,
            BEMD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BEMD`"]
pub type BEMD_R = crate::R<bool, BEMD_A>;
impl BEMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEMD_A {
        match self.bits {
            false => BEMD_A::_0,
            true => BEMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEMD_A::_1
    }
}
#[doc = "Bus error size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BESZ_A {
    #[doc = "0: 8-bit access"]
    _00,
    #[doc = "1: 16-bit access"]
    _01,
    #[doc = "2: 32-bit access"]
    _10,
}
impl From<BESZ_A> for u8 {
    #[inline(always)]
    fn from(variant: BESZ_A) -> Self {
        match variant {
            BESZ_A::_00 => 0,
            BESZ_A::_01 => 1,
            BESZ_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `BESZ`"]
pub type BESZ_R = crate::R<u8, BESZ_A>;
impl BESZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BESZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BESZ_A::_00),
            1 => Val(BESZ_A::_01),
            2 => Val(BESZ_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BESZ_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BESZ_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BESZ_A::_10
    }
}
#[doc = "Bus error write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEWT_A {
    #[doc = "0: Read access"]
    _0,
    #[doc = "1: Write access"]
    _1,
}
impl From<BEWT_A> for bool {
    #[inline(always)]
    fn from(variant: BEWT_A) -> Self {
        match variant {
            BEWT_A::_0 => false,
            BEWT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BEWT`"]
pub type BEWT_R = crate::R<bool, BEWT_A>;
impl BEWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEWT_A {
        match self.bits {
            false => BEWT_A::_0,
            true => BEWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEWT_A::_1
    }
}
#[doc = "Reader of field `BEMN`"]
pub type BEMN_R = crate::R<u8, u8>;
#[doc = "Bus error overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEOVR_A {
    #[doc = "0: No bus error overrun"]
    _0,
    #[doc = "1: Bus error overrun occurred. The FADR and FDR registers and the other FATR bits are not updated to reflect this new bus error."]
    _1,
}
impl From<BEOVR_A> for bool {
    #[inline(always)]
    fn from(variant: BEOVR_A) -> Self {
        match variant {
            BEOVR_A::_0 => false,
            BEOVR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BEOVR`"]
pub type BEOVR_R = crate::R<bool, BEOVR_A>;
impl BEOVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEOVR_A {
        match self.bits {
            false => BEOVR_A::_0,
            true => BEOVR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEOVR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEOVR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Bus error access type"]
    #[inline(always)]
    pub fn beda(&self) -> BEDA_R {
        BEDA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bus error privilege level"]
    #[inline(always)]
    pub fn bemd(&self) -> BEMD_R {
        BEMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Bus error size"]
    #[inline(always)]
    pub fn besz(&self) -> BESZ_R {
        BESZ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Bus error write"]
    #[inline(always)]
    pub fn bewt(&self) -> BEWT_R {
        BEWT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Bus error master number"]
    #[inline(always)]
    pub fn bemn(&self) -> BEMN_R {
        BEMN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Bus error overrun"]
    #[inline(always)]
    pub fn beovr(&self) -> BEOVR_R {
        BEOVR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
