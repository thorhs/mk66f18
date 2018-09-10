#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SRS1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `JTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAGR {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl JTAGR {
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
            JTAGR::_0 => false,
            JTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JTAGR {
        match value {
            false => JTAGR::_0,
            true => JTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == JTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == JTAGR::_1
    }
}
#[doc = "Possible values of the field `LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUPR {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl LOCKUPR {
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
            LOCKUPR::_0 => false,
            LOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUPR {
        match value {
            false => LOCKUPR::_0,
            true => LOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCKUPR::_1
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SWR {
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
            SWR::_0 => false,
            SWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWR {
        match value {
            false => SWR::_0,
            true => SWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWR::_1
    }
}
#[doc = "Possible values of the field `MDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_APR {
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl MDM_APR {
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
            MDM_APR::_0 => false,
            MDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDM_APR {
        match value {
            false => MDM_APR::_0,
            true => MDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDM_APR::_1
    }
}
#[doc = "Possible values of the field `EZPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZPTR {
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0,
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1,
}
impl EZPTR {
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
            EZPTR::_0 => false,
            EZPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EZPTR {
        match value {
            false => EZPTR::_0,
            true => EZPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EZPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EZPTR::_1
    }
}
#[doc = "Possible values of the field `SACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERRR {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SACKERRR {
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
            SACKERRR::_0 => false,
            SACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SACKERRR {
        match value {
            false => SACKERRR::_0,
            true => SACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SACKERRR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - JTAG Generated Reset"]
    #[inline]
    pub fn jtag(&self) -> JTAGR {
        JTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Core Lockup"]
    #[inline]
    pub fn lockup(&self) -> LOCKUPR {
        LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Software"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - MDM-AP System Reset Request"]
    #[inline]
    pub fn mdm_ap(&self) -> MDM_APR {
        MDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - EzPort Reset"]
    #[inline]
    pub fn ezpt(&self) -> EZPTR {
        EZPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Stop Mode Acknowledge Error Reset"]
    #[inline]
    pub fn sackerr(&self) -> SACKERRR {
        SACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
