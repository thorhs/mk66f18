#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "32-pin"]
    _0010,
    #[doc = "48-pin"]
    _0100,
    #[doc = "64-pin"]
    _0101,
    #[doc = "80-pin"]
    _0110,
    #[doc = "81-pin or 121-pin"]
    _0111,
    #[doc = "100-pin"]
    _1000,
    #[doc = "121-pin"]
    _1001,
    #[doc = "144-pin"]
    _1010,
    #[doc = "Custom pinout (WLCSP)"]
    _1011,
    #[doc = "169-pin"]
    _1100,
    #[doc = "256-pin"]
    _1110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0010 => 2,
            PINIDR::_0100 => 4,
            PINIDR::_0101 => 5,
            PINIDR::_0110 => 6,
            PINIDR::_0111 => 7,
            PINIDR::_1000 => 8,
            PINIDR::_1001 => 9,
            PINIDR::_1010 => 10,
            PINIDR::_1011 => 11,
            PINIDR::_1100 => 12,
            PINIDR::_1110 => 14,
            PINIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            2 => PINIDR::_0010,
            4 => PINIDR::_0100,
            5 => PINIDR::_0101,
            6 => PINIDR::_0110,
            7 => PINIDR::_0111,
            8 => PINIDR::_1000,
            9 => PINIDR::_1001,
            10 => PINIDR::_1010,
            11 => PINIDR::_1011,
            12 => PINIDR::_1100,
            14 => PINIDR::_1110,
            i => PINIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PINIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PINIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PINIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PINIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PINIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PINIDR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PINIDR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == PINIDR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PINIDR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == PINIDR::_1100
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == PINIDR::_1110
    }
}
#[doc = "Possible values of the field `FAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMIDR {
    #[doc = "K1x Family (without tamper)"]
    _000,
    #[doc = "K2x Family (without tamper)"]
    _001,
    #[doc = "K3x Family or K1x/K6x Family (with tamper)"]
    _010,
    #[doc = "K4x Family or K2x Family (with tamper)"]
    _011,
    #[doc = "K6x Family (without tamper)"]
    _100,
    #[doc = "K7x Family"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMIDR::_000 => 0,
            FAMIDR::_001 => 1,
            FAMIDR::_010 => 2,
            FAMIDR::_011 => 3,
            FAMIDR::_100 => 4,
            FAMIDR::_101 => 5,
            FAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMIDR {
        match value {
            0 => FAMIDR::_000,
            1 => FAMIDR::_001,
            2 => FAMIDR::_010,
            3 => FAMIDR::_011,
            4 => FAMIDR::_100,
            5 => FAMIDR::_101,
            i => FAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FAMIDR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FAMIDR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FAMIDR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FAMIDR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FAMIDR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FAMIDR::_101
    }
}
#[doc = r" Value of the field"]
pub struct DIEIDR {
    bits: u8,
}
impl DIEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SERIESID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERIESIDR {
    #[doc = "Kinetis K series"]
    _0000,
    #[doc = "Kinetis L series"]
    _0001,
    #[doc = "Kinetis W series"]
    _0101,
    #[doc = "Kinetis V series"]
    _0110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SERIESIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SERIESIDR::_0000 => 0,
            SERIESIDR::_0001 => 1,
            SERIESIDR::_0101 => 5,
            SERIESIDR::_0110 => 6,
            SERIESIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SERIESIDR {
        match value {
            0 => SERIESIDR::_0000,
            1 => SERIESIDR::_0001,
            5 => SERIESIDR::_0101,
            6 => SERIESIDR::_0110,
            i => SERIESIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SERIESIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SERIESIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SERIESIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SERIESIDR::_0110
    }
}
#[doc = "Possible values of the field `SUBFAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMIDR {
    #[doc = "Kx0 Subfamily"]
    _0000,
    #[doc = "Kx1 Subfamily (tamper detect)"]
    _0001,
    #[doc = "Kx2 Subfamily"]
    _0010,
    #[doc = "Kx3 Subfamily (tamper detect)"]
    _0011,
    #[doc = "Kx4 Subfamily"]
    _0100,
    #[doc = "Kx5 Subfamily (tamper detect)"]
    _0101,
    #[doc = "Kx6 Subfamily"]
    _0110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUBFAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUBFAMIDR::_0000 => 0,
            SUBFAMIDR::_0001 => 1,
            SUBFAMIDR::_0010 => 2,
            SUBFAMIDR::_0011 => 3,
            SUBFAMIDR::_0100 => 4,
            SUBFAMIDR::_0101 => 5,
            SUBFAMIDR::_0110 => 6,
            SUBFAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUBFAMIDR {
        match value {
            0 => SUBFAMIDR::_0000,
            1 => SUBFAMIDR::_0001,
            2 => SUBFAMIDR::_0010,
            3 => SUBFAMIDR::_0011,
            4 => SUBFAMIDR::_0100,
            5 => SUBFAMIDR::_0101,
            6 => SUBFAMIDR::_0110,
            i => SUBFAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMIDR::_0110
    }
}
#[doc = "Possible values of the field `FAMILYID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMILYIDR {
    #[doc = "K0x Family"]
    _0000,
    #[doc = "K1x Family"]
    _0001,
    #[doc = "K2x Family"]
    _0010,
    #[doc = "K3x Family"]
    _0011,
    #[doc = "K4x Family"]
    _0100,
    #[doc = "K6x Family"]
    _0110,
    #[doc = "K7x Family"]
    _0111,
    #[doc = "K8x Family"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMILYIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMILYIDR::_0000 => 0,
            FAMILYIDR::_0001 => 1,
            FAMILYIDR::_0010 => 2,
            FAMILYIDR::_0011 => 3,
            FAMILYIDR::_0100 => 4,
            FAMILYIDR::_0110 => 6,
            FAMILYIDR::_0111 => 7,
            FAMILYIDR::_1000 => 8,
            FAMILYIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMILYIDR {
        match value {
            0 => FAMILYIDR::_0000,
            1 => FAMILYIDR::_0001,
            2 => FAMILYIDR::_0010,
            3 => FAMILYIDR::_0011,
            4 => FAMILYIDR::_0100,
            6 => FAMILYIDR::_0110,
            7 => FAMILYIDR::_0111,
            8 => FAMILYIDR::_1000,
            i => FAMILYIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == FAMILYIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == FAMILYIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == FAMILYIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == FAMILYIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FAMILYIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == FAMILYIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == FAMILYIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == FAMILYIDR::_1000
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        FAMIDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:11 - Device Die ID"]
    #[inline]
    pub fn dieid(&self) -> DIEIDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIEIDR { bits }
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline]
    pub fn seriesid(&self) -> SERIESIDR {
        SERIESIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline]
    pub fn subfamid(&self) -> SUBFAMIDR {
        SUBFAMIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Kinetis Family ID"]
    #[inline]
    pub fn familyid(&self) -> FAMILYIDR {
        FAMILYIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
