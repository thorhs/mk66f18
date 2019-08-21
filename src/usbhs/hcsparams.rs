#[doc = "Reader of register HCSPARAMS"]
pub type R = crate::R<u32, super::HCSPARAMS>;
#[doc = "Reader of field `N_PORTS`"]
pub type N_PORTS_R = crate::R<u8, u8>;
#[doc = "Power Port Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPC_A {
    #[doc = "1: Ports have power port switches"]
    _1,
}
impl From<PPC_A> for bool {
    #[inline(always)]
    fn from(variant: PPC_A) -> Self {
        match variant {
            PPC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PPC`"]
pub type PPC_R = crate::R<bool, PPC_A>;
impl PPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PPC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PPC_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PPC_A::_1
    }
}
#[doc = "Reader of field `N_PCC`"]
pub type N_PCC_R = crate::R<u8, u8>;
#[doc = "Reader of field `N_CC`"]
pub type N_CC_R = crate::R<u8, u8>;
#[doc = "Port Indicators\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_A {
    #[doc = "0: No port indicator fields"]
    _0,
    #[doc = "1: The port status and control registers include a R/W field for controlling the state of the port indicator"]
    _1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        match variant {
            PI_A::_0 => false,
            PI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PI`"]
pub type PI_R = crate::R<bool, PI_A>;
impl PI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::_0,
            true => PI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PI_A::_1
    }
}
#[doc = "Reader of field `N_PTT`"]
pub type N_PTT_R = crate::R<u8, u8>;
#[doc = "Reader of field `N_TT`"]
pub type N_TT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of Ports"]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Power Port Control"]
    #[inline(always)]
    pub fn ppc(&self) -> PPC_R {
        PPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Number Ports per CC"]
    #[inline(always)]
    pub fn n_pcc(&self) -> N_PCC_R {
        N_PCC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Companion Controllers"]
    #[inline(always)]
    pub fn n_cc(&self) -> N_CC_R {
        N_CC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Port Indicators"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Ports per Transaction Translator"]
    #[inline(always)]
    pub fn n_ptt(&self) -> N_PTT_R {
        N_PTT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of Transaction Translators."]
    #[inline(always)]
    pub fn n_tt(&self) -> N_TT_R {
        N_TT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
