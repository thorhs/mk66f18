#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SECV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECVR {
    #[doc = "No security violation"]
    _0,
    #[doc = "Security violation"]
    _1,
}
impl SECVR {
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
            SECVR::_0 => false,
            SECVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECVR {
        match value {
            false => SECVR::_0,
            true => SECVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SECVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SECVR::_1
    }
}
#[doc = "Possible values of the field `LRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRSR {
    #[doc = "No underflow"]
    _0,
    #[doc = "Underflow"]
    _1,
}
impl LRSR {
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
            LRSR::_0 => false,
            LRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRSR {
        match value {
            false => LRSR::_0,
            true => LRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRSR::_1
    }
}
#[doc = "Possible values of the field `ORU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORUR {
    #[doc = "No underflow"]
    _0,
    #[doc = "Underflow"]
    _1,
}
impl ORUR {
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
            ORUR::_0 => false,
            ORUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORUR {
        match value {
            false => ORUR::_0,
            true => ORUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ORUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ORUR::_1
    }
}
#[doc = "Possible values of the field `ERRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIR {
    #[doc = "No underflow"]
    _0,
    #[doc = "Underflow"]
    _1,
}
impl ERRIR {
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
            ERRIR::_0 => false,
            ERRIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIR {
        match value {
            false => ERRIR::_0,
            true => ERRIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRIR::_1
    }
}
#[doc = "Possible values of the field `SLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPR {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Sleep (low-power) mode"]
    _1,
}
impl SLPR {
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
            SLPR::_0 => false,
            SLPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLPR {
        match value {
            false => SLPR::_0,
            true => SLPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLPR::_1
    }
}
#[doc = "Possible values of the field `OREG_LVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OREG_LVLR {
    #[doc = "No words (empty)"]
    _0,
    #[doc = "One word (valid)"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OREG_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OREG_LVLR::_0 => 0,
            OREG_LVLR::_1 => 1,
            OREG_LVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OREG_LVLR {
        match value {
            0 => OREG_LVLR::_0,
            1 => OREG_LVLR::_1,
            i => OREG_LVLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OREG_LVLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OREG_LVLR::_1
    }
}
#[doc = "Possible values of the field `OREG_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OREG_SIZER {
    #[doc = "One word (this value is fixed)"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OREG_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OREG_SIZER::_1 => 1,
            OREG_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OREG_SIZER {
        match value {
            1 => OREG_SIZER::_1,
            i => OREG_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OREG_SIZER::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Security Violation"]
    #[inline]
    pub fn secv(&self) -> SECVR {
        SECVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Last Read Status"]
    #[inline]
    pub fn lrs(&self) -> LRSR {
        LRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Output Register Underflow"]
    #[inline]
    pub fn oru(&self) -> ORUR {
        ORUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline]
    pub fn erri(&self) -> ERRIR {
        ERRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline]
    pub fn slp(&self) -> SLPR {
        SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Output Register Level"]
    #[inline]
    pub fn oreg_lvl(&self) -> OREG_LVLR {
        OREG_LVLR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Output Register Size"]
    #[inline]
    pub fn oreg_size(&self) -> OREG_SIZER {
        OREG_SIZER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
