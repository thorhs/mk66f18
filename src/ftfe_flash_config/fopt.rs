#[doc = "Reader of register FOPT"]
pub type R = crate::R<u8, super::FOPT>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT_A {
    #[doc = "0: Low-power boot"]
    _00,
    #[doc = "1: Normal boot"]
    _01,
}
impl From<LPBOOT_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT_A) -> Self {
        match variant {
            LPBOOT_A::_00 => false,
            LPBOOT_A::_01 => true,
        }
    }
}
#[doc = "Reader of field `LPBOOT`"]
pub type LPBOOT_R = crate::R<bool, LPBOOT_A>;
impl LPBOOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT_A {
        match self.bits {
            false => LPBOOT_A::_00,
            true => LPBOOT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZPORT_DIS_A {
    #[doc = "0: EzPort operation is disabled"]
    _00,
    #[doc = "1: EzPort operation is enabled"]
    _01,
}
impl From<EZPORT_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: EZPORT_DIS_A) -> Self {
        match variant {
            EZPORT_DIS_A::_00 => false,
            EZPORT_DIS_A::_01 => true,
        }
    }
}
#[doc = "Reader of field `EZPORT_DIS`"]
pub type EZPORT_DIS_R = crate::R<bool, EZPORT_DIS_A>;
impl EZPORT_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZPORT_DIS_A {
        match self.bits {
            false => EZPORT_DIS_A::_00,
            true => EZPORT_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EZPORT_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EZPORT_DIS_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DIS_A {
    #[doc = "0: NMI interrupts are always blocked"]
    _00,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01,
}
impl From<NMI_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_DIS_A) -> Self {
        match variant {
            NMI_DIS_A::_00 => false,
            NMI_DIS_A::_01 => true,
        }
    }
}
#[doc = "Reader of field `NMI_DIS`"]
pub type NMI_DIS_R = crate::R<bool, NMI_DIS_A>;
impl NMI_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_DIS_A {
        match self.bits {
            false => NMI_DIS_A::_00,
            true => NMI_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NMI_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NMI_DIS_A::_01
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot(&self) -> LPBOOT_R {
        LPBOOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn ezport_dis(&self) -> EZPORT_DIS_R {
        EZPORT_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NMI_DIS_R {
        NMI_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
