#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CIHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIHBR {
    #[doc = "Can issue command using only CMD line."]
    _0,
    #[doc = "Cannot issue command."]
    _1,
}
impl CIHBR {
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
            CIHBR::_0 => false,
            CIHBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIHBR {
        match value {
            false => CIHBR::_0,
            true => CIHBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CIHBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CIHBR::_1
    }
}
#[doc = "Possible values of the field `CDIHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIHBR {
    #[doc = "Can issue command which uses the DAT line."]
    _0,
    #[doc = "Cannot issue command which uses the DAT line."]
    _1,
}
impl CDIHBR {
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
            CDIHBR::_0 => false,
            CDIHBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDIHBR {
        match value {
            false => CDIHBR::_0,
            true => CDIHBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CDIHBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CDIHBR::_1
    }
}
#[doc = "Possible values of the field `DLA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAR {
    #[doc = "DAT line inactive."]
    _0,
    #[doc = "DAT line active."]
    _1,
}
impl DLAR {
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
            DLAR::_0 => false,
            DLAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLAR {
        match value {
            false => DLAR::_0,
            true => DLAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DLAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DLAR::_1
    }
}
#[doc = "Possible values of the field `SDSTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDSTBR {
    #[doc = "Clock is changing frequency and not stable."]
    _0,
    #[doc = "Clock is stable."]
    _1,
}
impl SDSTBR {
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
            SDSTBR::_0 => false,
            SDSTBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDSTBR {
        match value {
            false => SDSTBR::_0,
            true => SDSTBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDSTBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDSTBR::_1
    }
}
#[doc = "Possible values of the field `IPGOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGOFFR {
    #[doc = "Bus clock is active."]
    _0,
    #[doc = "Bus clock is gated off."]
    _1,
}
impl IPGOFFR {
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
            IPGOFFR::_0 => false,
            IPGOFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPGOFFR {
        match value {
            false => IPGOFFR::_0,
            true => IPGOFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPGOFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPGOFFR::_1
    }
}
#[doc = "Possible values of the field `HCKOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKOFFR {
    #[doc = "System clock is active."]
    _0,
    #[doc = "System clock is gated off."]
    _1,
}
impl HCKOFFR {
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
            HCKOFFR::_0 => false,
            HCKOFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCKOFFR {
        match value {
            false => HCKOFFR::_0,
            true => HCKOFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HCKOFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HCKOFFR::_1
    }
}
#[doc = "Possible values of the field `PEROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEROFFR {
    #[doc = "SDHC clock is active."]
    _0,
    #[doc = "SDHC clock is gated off."]
    _1,
}
impl PEROFFR {
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
            PEROFFR::_0 => false,
            PEROFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEROFFR {
        match value {
            false => PEROFFR::_0,
            true => PEROFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEROFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEROFFR::_1
    }
}
#[doc = "Possible values of the field `SDOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDOFFR {
    #[doc = "SD clock is active."]
    _0,
    #[doc = "SD clock is gated off."]
    _1,
}
impl SDOFFR {
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
            SDOFFR::_0 => false,
            SDOFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDOFFR {
        match value {
            false => SDOFFR::_0,
            true => SDOFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDOFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDOFFR::_1
    }
}
#[doc = "Possible values of the field `WTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTAR {
    #[doc = "No valid data."]
    _0,
    #[doc = "Transferring data."]
    _1,
}
impl WTAR {
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
            WTAR::_0 => false,
            WTAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTAR {
        match value {
            false => WTAR::_0,
            true => WTAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WTAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WTAR::_1
    }
}
#[doc = "Possible values of the field `RTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTAR {
    #[doc = "No valid data."]
    _0,
    #[doc = "Transferring data."]
    _1,
}
impl RTAR {
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
            RTAR::_0 => false,
            RTAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTAR {
        match value {
            false => RTAR::_0,
            true => RTAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTAR::_1
    }
}
#[doc = "Possible values of the field `BWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWENR {
    #[doc = "Write disable, the buffer can hold valid data less than the write watermark level."]
    _0,
    #[doc = "Write enable, the buffer can hold valid data greater than the write watermark level."]
    _1,
}
impl BWENR {
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
            BWENR::_0 => false,
            BWENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWENR {
        match value {
            false => BWENR::_0,
            true => BWENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWENR::_1
    }
}
#[doc = "Possible values of the field `BREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRENR {
    #[doc = "Read disable, valid data less than the watermark level exist in the buffer."]
    _0,
    #[doc = "Read enable, valid data greater than the watermark level exist in the buffer."]
    _1,
}
impl BRENR {
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
            BRENR::_0 => false,
            BRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRENR {
        match value {
            false => BRENR::_0,
            true => BRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRENR::_1
    }
}
#[doc = "Possible values of the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSR {
    #[doc = "Power on reset or no card."]
    _0,
    #[doc = "Card inserted."]
    _1,
}
impl CINSR {
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
            CINSR::_0 => false,
            CINSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSR {
        match value {
            false => CINSR::_0,
            true => CINSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CLSLR {
    bits: bool,
}
impl CLSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DLSLR {
    bits: u8,
}
impl DLSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline]
    pub fn cihb(&self) -> CIHBR {
        CIHBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline]
    pub fn cdihb(&self) -> CDIHBR {
        CDIHBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline]
    pub fn dla(&self) -> DLAR {
        DLAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline]
    pub fn sdstb(&self) -> SDSTBR {
        SDSTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Bus Clock Gated Off Internally"]
    #[inline]
    pub fn ipgoff(&self) -> IPGOFFR {
        IPGOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - System Clock Gated Off Internally"]
    #[inline]
    pub fn hckoff(&self) -> HCKOFFR {
        HCKOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SDHC clock Gated Off Internally"]
    #[inline]
    pub fn peroff(&self) -> PEROFFR {
        PEROFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline]
    pub fn sdoff(&self) -> SDOFFR {
        SDOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline]
    pub fn wta(&self) -> WTAR {
        WTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline]
    pub fn rta(&self) -> RTAR {
        RTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline]
    pub fn bwen(&self) -> BWENR {
        BWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline]
    pub fn bren(&self) -> BRENR {
        BRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline]
    pub fn cins(&self) -> CINSR {
        CINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline]
    pub fn clsl(&self) -> CLSLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLSLR { bits }
    }
    #[doc = "Bits 24:31 - DAT Line Signal Level"]
    #[inline]
    pub fn dlsl(&self) -> DLSLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLSLR { bits }
    }
}
