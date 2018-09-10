#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FSEC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECR {
    #[doc = "MCU security status is unsecure"]
    _10,
    #[doc = "MCU security status is secure"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECR::_10 => 2,
            SECR::_11 => 3,
            SECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECR {
        match value {
            2 => SECR::_10,
            3 => SECR::_11,
            i => SECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SECR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SECR::_11
    }
}
#[doc = "Possible values of the field `FSLACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLACCR {
    #[doc = "Freescale factory access denied"]
    _10,
    #[doc = "Freescale factory access granted"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSLACCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSLACCR::_10 => 2,
            FSLACCR::_11 => 3,
            FSLACCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSLACCR {
        match value {
            2 => FSLACCR::_10,
            3 => FSLACCR::_11,
            i => FSLACCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FSLACCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FSLACCR::_11
    }
}
#[doc = "Possible values of the field `MEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEENR {
    #[doc = "Mass erase is disabled"]
    _10,
    #[doc = "Mass erase is enabled"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MEENR::_10 => 2,
            MEENR::_11 => 3,
            MEENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MEENR {
        match value {
            2 => MEENR::_10,
            3 => MEENR::_11,
            i => MEENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MEENR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MEENR::_11
    }
}
#[doc = "Possible values of the field `KEYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYENR {
    #[doc = "Backdoor key access enabled"]
    _10,
    #[doc = "Backdoor key access disabled"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYENR::_10 => 2,
            KEYENR::_11 => 3,
            KEYENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYENR {
        match value {
            2 => KEYENR::_10,
            3 => KEYENR::_11,
            i => KEYENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == KEYENR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == KEYENR::_11
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline]
    pub fn sec(&self) -> SECR {
        SECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Freescale Failure Analysis Access Code"]
    #[inline]
    pub fn fslacc(&self) -> FSLACCR {
        FSLACCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline]
    pub fn meen(&self) -> MEENR {
        MEENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline]
    pub fn keyen(&self) -> KEYENR {
        KEYENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
