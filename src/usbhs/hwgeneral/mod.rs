#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWGENERAL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PHYW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYWR {
    #[doc = "16 bit wide data bus"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PHYWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHYWR::_01 => 1,
            PHYWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHYWR {
        match value {
            1 => PHYWR::_01,
            i => PHYWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PHYWR::_01
    }
}
#[doc = "Possible values of the field `PHYM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYMR {
    #[doc = "Controller configured for UTMI/UTMI+ interface."]
    _000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PHYMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHYMR::_000 => 0,
            PHYMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHYMR {
        match value {
            0 => PHYMR::_000,
            i => PHYMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PHYMR::_000
    }
}
#[doc = "Possible values of the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMR {
    #[doc = "No Serial Engine, always use parallel signaling."]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMR::_00 => 0,
            SMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMR {
        match value {
            0 => SMR::_00,
            i => SMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SMR::_00
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:5 - PHY Width"]
    #[inline]
    pub fn phyw(&self) -> PHYWR {
        PHYWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - PHY Mode"]
    #[inline]
    pub fn phym(&self) -> PHYMR {
        PHYMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:10 - Serial mode"]
    #[inline]
    pub fn sm(&self) -> SMR {
        SMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
