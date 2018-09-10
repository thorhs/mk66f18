#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MAXADDR1R {
    bits: u8,
}
impl MAXADDR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PFLSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLSHR {
    #[doc = "Device supports FlexNVM"]
    _0,
    #[doc = "Program Flash only, device does not support FlexNVM"]
    _1,
}
impl PFLSHR {
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
            PFLSHR::_0 => false,
            PFLSHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFLSHR {
        match value {
            false => PFLSHR::_0,
            true => PFLSHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PFLSHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PFLSHR::_1
    }
}
#[doc = r" Value of the field"]
pub struct MAXADDR0R {
    bits: u8,
}
impl MAXADDR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SWAPPFLSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPPFLSHR {
    #[doc = "Swap is not active."]
    _0,
    #[doc = "Swap is active."]
    _1,
}
impl SWAPPFLSHR {
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
            SWAPPFLSHR::_0 => false,
            SWAPPFLSHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAPPFLSHR {
        match value {
            false => SWAPPFLSHR::_0,
            true => SWAPPFLSHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWAPPFLSHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWAPPFLSHR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:22 - Max address block 1"]
    #[inline]
    pub fn maxaddr1(&self) -> MAXADDR1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXADDR1R { bits }
    }
    #[doc = "Bit 23 - Program flash only"]
    #[inline]
    pub fn pflsh(&self) -> PFLSHR {
        PFLSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:30 - Max address block 0"]
    #[inline]
    pub fn maxaddr0(&self) -> MAXADDR0R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXADDR0R { bits }
    }
    #[doc = "Bit 31 - Swap program flash"]
    #[inline]
    pub fn swappflsh(&self) -> SWAPPFLSHR {
        SWAPPFLSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
