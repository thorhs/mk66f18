#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FOPT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LPBOOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOTR {
    #[doc = "Low-power boot"]
    _00,
    #[doc = "Normal boot"]
    _01,
}
impl LPBOOTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPBOOTR::_00 => false,
            LPBOOTR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBOOTR {
        match value {
            false => LPBOOTR::_00,
            true => LPBOOTR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPBOOTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPBOOTR::_01
    }
}
#[doc = "Possible values of the field `EZPORT_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZPORT_DISR {
    #[doc = "EzPort operation is disabled"]
    _00,
    #[doc = "EzPort operation is enabled"]
    _01,
}
impl EZPORT_DISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EZPORT_DISR::_00 => false,
            EZPORT_DISR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZPORT_DISR {
        match value {
            false => EZPORT_DISR::_00,
            true => EZPORT_DISR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == EZPORT_DISR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == EZPORT_DISR::_01
    }
}
#[doc = "Possible values of the field `NMI_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DISR {
    #[doc = "NMI interrupts are always blocked"]
    _00,
    #[doc = "NMI_b pin/interrupts reset default to enabled"]
    _01,
}
impl NMI_DISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NMI_DISR::_00 => false,
            NMI_DISR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMI_DISR {
        match value {
            false => NMI_DISR::_00,
            true => NMI_DISR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == NMI_DISR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == NMI_DISR::_01
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn lpboot(&self) -> LPBOOTR {
        LPBOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn ezport_dis(&self) -> EZPORT_DISR {
        EZPORT_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn nmi_dis(&self) -> NMI_DISR {
        NMI_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
