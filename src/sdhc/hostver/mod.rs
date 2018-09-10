#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HOSTVER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SVN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVNR {
    #[doc = "SD host specification version 2.0, supports test event register and ADMA."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SVNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SVNR::_1 => 1,
            SVNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SVNR {
        match value {
            1 => SVNR::_1,
            i => SVNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SVNR::_1
    }
}
#[doc = "Possible values of the field `VVN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VVNR {
    #[doc = "Freescale SDHC version 1.0"]
    _0,
    #[doc = "Freescale SDHC version 2.0"]
    _10000,
    #[doc = "Freescale SDHC version 2.1"]
    _10001,
    #[doc = "Freescale SDHC version 2.2"]
    _10010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VVNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VVNR::_0 => 0,
            VVNR::_10000 => 16,
            VVNR::_10001 => 17,
            VVNR::_10010 => 18,
            VVNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VVNR {
        match value {
            0 => VVNR::_0,
            16 => VVNR::_10000,
            17 => VVNR::_10001,
            18 => VVNR::_10010,
            i => VVNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VVNR::_0
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == VVNR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == VVNR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == VVNR::_10010
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Specification Version Number"]
    #[inline]
    pub fn svn(&self) -> SVNR {
        SVNR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Vendor Version Number"]
    #[inline]
    pub fn vvn(&self) -> VVNR {
        VVNR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
