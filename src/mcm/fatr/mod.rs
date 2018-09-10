#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FATR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BEDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEDAR {
    #[doc = "Instruction"]
    _0,
    #[doc = "Data"]
    _1,
}
impl BEDAR {
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
            BEDAR::_0 => false,
            BEDAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEDAR {
        match value {
            false => BEDAR::_0,
            true => BEDAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEDAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEDAR::_1
    }
}
#[doc = "Possible values of the field `BEMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEMDR {
    #[doc = "User mode"]
    _0,
    #[doc = "Supervisor/privileged mode"]
    _1,
}
impl BEMDR {
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
            BEMDR::_0 => false,
            BEMDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEMDR {
        match value {
            false => BEMDR::_0,
            true => BEMDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEMDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEMDR::_1
    }
}
#[doc = "Possible values of the field `BESZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BESZR {
    #[doc = "8-bit access"]
    _00,
    #[doc = "16-bit access"]
    _01,
    #[doc = "32-bit access"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BESZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BESZR::_00 => 0,
            BESZR::_01 => 1,
            BESZR::_10 => 2,
            BESZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BESZR {
        match value {
            0 => BESZR::_00,
            1 => BESZR::_01,
            2 => BESZR::_10,
            i => BESZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == BESZR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == BESZR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BESZR::_10
    }
}
#[doc = "Possible values of the field `BEWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEWTR {
    #[doc = "Read access"]
    _0,
    #[doc = "Write access"]
    _1,
}
impl BEWTR {
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
            BEWTR::_0 => false,
            BEWTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEWTR {
        match value {
            false => BEWTR::_0,
            true => BEWTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEWTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEWTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BEMNR {
    bits: u8,
}
impl BEMNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BEOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEOVRR {
    #[doc = "No bus error overrun"]
    _0,
    #[doc = "Bus error overrun occurred. The FADR and FDR registers and the other FATR bits are not updated to reflect this new bus error."]
    _1,
}
impl BEOVRR {
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
            BEOVRR::_0 => false,
            BEOVRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEOVRR {
        match value {
            false => BEOVRR::_0,
            true => BEOVRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEOVRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEOVRR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bus error access type"]
    #[inline]
    pub fn beda(&self) -> BEDAR {
        BEDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Bus error privilege level"]
    #[inline]
    pub fn bemd(&self) -> BEMDR {
        BEMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Bus error size"]
    #[inline]
    pub fn besz(&self) -> BESZR {
        BESZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Bus error write"]
    #[inline]
    pub fn bewt(&self) -> BEWTR {
        BEWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Bus error master number"]
    #[inline]
    pub fn bemn(&self) -> BEMNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BEMNR { bits }
    }
    #[doc = "Bit 31 - Bus error overrun"]
    #[inline]
    pub fn beovr(&self) -> BEOVRR {
        BEOVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
