#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HTCAPBLT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBLR {
    #[doc = "512 bytes"]
    _000,
    #[doc = "1024 bytes"]
    _001,
    #[doc = "2048 bytes"]
    _010,
    #[doc = "4096 bytes"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MBLR::_000 => 0,
            MBLR::_001 => 1,
            MBLR::_010 => 2,
            MBLR::_011 => 3,
            MBLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MBLR {
        match value {
            0 => MBLR::_000,
            1 => MBLR::_001,
            2 => MBLR::_010,
            3 => MBLR::_011,
            i => MBLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MBLR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == MBLR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MBLR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MBLR::_011
    }
}
#[doc = "Possible values of the field `ADMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMASR {
    #[doc = "Advanced DMA not supported."]
    _0,
    #[doc = "Advanced DMA supported."]
    _1,
}
impl ADMASR {
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
            ADMASR::_0 => false,
            ADMASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMASR {
        match value {
            false => ADMASR::_0,
            true => ADMASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADMASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADMASR::_1
    }
}
#[doc = "Possible values of the field `HSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSSR {
    #[doc = "High speed not supported."]
    _0,
    #[doc = "High speed supported."]
    _1,
}
impl HSSR {
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
            HSSR::_0 => false,
            HSSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSSR {
        match value {
            false => HSSR::_0,
            true => HSSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSSR::_1
    }
}
#[doc = "Possible values of the field `DMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASR {
    #[doc = "DMA not supported."]
    _0,
    #[doc = "DMA supported."]
    _1,
}
impl DMASR {
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
            DMASR::_0 => false,
            DMASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMASR {
        match value {
            false => DMASR::_0,
            true => DMASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMASR::_1
    }
}
#[doc = "Possible values of the field `SRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSR {
    #[doc = "Not supported."]
    _0,
    #[doc = "Supported."]
    _1,
}
impl SRSR {
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
            SRSR::_0 => false,
            SRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSR {
        match value {
            false => SRSR::_0,
            true => SRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRSR::_1
    }
}
#[doc = "Possible values of the field `VS33`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS33R {
    #[doc = "3.3 V not supported."]
    _0,
    #[doc = "3.3 V supported."]
    _1,
}
impl VS33R {
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
            VS33R::_0 => false,
            VS33R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VS33R {
        match value {
            false => VS33R::_0,
            true => VS33R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VS33R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VS33R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:18 - Max Block Length"]
    #[inline]
    pub fn mbl(&self) -> MBLR {
        MBLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - ADMA Support"]
    #[inline]
    pub fn admas(&self) -> ADMASR {
        ADMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline]
    pub fn hss(&self) -> HSSR {
        HSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA Support"]
    #[inline]
    pub fn dmas(&self) -> DMASR {
        DMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline]
    pub fn srs(&self) -> SRSR {
        SRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Voltage Support 3.3 V"]
    #[inline]
    pub fn vs33(&self) -> VS33R {
        VS33R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
