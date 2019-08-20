#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINID_A {
    #[doc = "2: 32-pin"]
    _0010,
    #[doc = "4: 48-pin"]
    _0100,
    #[doc = "5: 64-pin"]
    _0101,
    #[doc = "6: 80-pin"]
    _0110,
    #[doc = "7: 81-pin or 121-pin"]
    _0111,
    #[doc = "8: 100-pin"]
    _1000,
    #[doc = "9: 121-pin"]
    _1001,
    #[doc = "10: 144-pin"]
    _1010,
    #[doc = "11: Custom pinout (WLCSP)"]
    _1011,
    #[doc = "12: 169-pin"]
    _1100,
    #[doc = "14: 256-pin"]
    _1110,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        match variant {
            PINID_A::_0010 => 2,
            PINID_A::_0100 => 4,
            PINID_A::_0101 => 5,
            PINID_A::_0110 => 6,
            PINID_A::_0111 => 7,
            PINID_A::_1000 => 8,
            PINID_A::_1001 => 9,
            PINID_A::_1010 => 10,
            PINID_A::_1011 => 11,
            PINID_A::_1100 => 12,
            PINID_A::_1110 => 14,
        }
    }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PINID_A::_0010),
            4 => Val(PINID_A::_0100),
            5 => Val(PINID_A::_0101),
            6 => Val(PINID_A::_0110),
            7 => Val(PINID_A::_0111),
            8 => Val(PINID_A::_1000),
            9 => Val(PINID_A::_1001),
            10 => Val(PINID_A::_1010),
            11 => Val(PINID_A::_1011),
            12 => Val(PINID_A::_1100),
            14 => Val(PINID_A::_1110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PINID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PINID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PINID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PINID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PINID_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PINID_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PINID_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PINID_A::_1110
    }
}
#[doc = "Kinetis family identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMID_A {
    #[doc = "0: K1x Family (without tamper)"]
    _000,
    #[doc = "1: K2x Family (without tamper)"]
    _001,
    #[doc = "2: K3x Family or K1x/K6x Family (with tamper)"]
    _010,
    #[doc = "3: K4x Family or K2x Family (with tamper)"]
    _011,
    #[doc = "4: K6x Family (without tamper)"]
    _100,
    #[doc = "5: K7x Family"]
    _101,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        match variant {
            FAMID_A::_000 => 0,
            FAMID_A::_001 => 1,
            FAMID_A::_010 => 2,
            FAMID_A::_011 => 3,
            FAMID_A::_100 => 4,
            FAMID_A::_101 => 5,
        }
    }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, FAMID_A>;
impl FAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMID_A::_000),
            1 => Val(FAMID_A::_001),
            2 => Val(FAMID_A::_010),
            3 => Val(FAMID_A::_011),
            4 => Val(FAMID_A::_100),
            5 => Val(FAMID_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FAMID_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FAMID_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FAMID_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FAMID_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FAMID_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FAMID_A::_101
    }
}
#[doc = "Reader of field `DIEID`"]
pub type DIEID_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
#[doc = "Kinetis Series ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERIESID_A {
    #[doc = "0: Kinetis K series"]
    _0000,
    #[doc = "1: Kinetis L series"]
    _0001,
    #[doc = "5: Kinetis W series"]
    _0101,
    #[doc = "6: Kinetis V series"]
    _0110,
}
impl From<SERIESID_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIESID_A) -> Self {
        match variant {
            SERIESID_A::_0000 => 0,
            SERIESID_A::_0001 => 1,
            SERIESID_A::_0101 => 5,
            SERIESID_A::_0110 => 6,
        }
    }
}
#[doc = "Reader of field `SERIESID`"]
pub type SERIESID_R = crate::R<u8, SERIESID_A>;
impl SERIESID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SERIESID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SERIESID_A::_0000),
            1 => Val(SERIESID_A::_0001),
            5 => Val(SERIESID_A::_0101),
            6 => Val(SERIESID_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SERIESID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SERIESID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SERIESID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SERIESID_A::_0110
    }
}
#[doc = "Kinetis Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMID_A {
    #[doc = "0: Kx0 Subfamily"]
    _0000,
    #[doc = "1: Kx1 Subfamily (tamper detect)"]
    _0001,
    #[doc = "2: Kx2 Subfamily"]
    _0010,
    #[doc = "3: Kx3 Subfamily (tamper detect)"]
    _0011,
    #[doc = "4: Kx4 Subfamily"]
    _0100,
    #[doc = "5: Kx5 Subfamily (tamper detect)"]
    _0101,
    #[doc = "6: Kx6 Subfamily"]
    _0110,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        match variant {
            SUBFAMID_A::_0000 => 0,
            SUBFAMID_A::_0001 => 1,
            SUBFAMID_A::_0010 => 2,
            SUBFAMID_A::_0011 => 3,
            SUBFAMID_A::_0100 => 4,
            SUBFAMID_A::_0101 => 5,
            SUBFAMID_A::_0110 => 6,
        }
    }
}
#[doc = "Reader of field `SUBFAMID`"]
pub type SUBFAMID_R = crate::R<u8, SUBFAMID_A>;
impl SUBFAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBFAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBFAMID_A::_0000),
            1 => Val(SUBFAMID_A::_0001),
            2 => Val(SUBFAMID_A::_0010),
            3 => Val(SUBFAMID_A::_0011),
            4 => Val(SUBFAMID_A::_0100),
            5 => Val(SUBFAMID_A::_0101),
            6 => Val(SUBFAMID_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMID_A::_0110
    }
}
#[doc = "Kinetis Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMILYID_A {
    #[doc = "0: K0x Family"]
    _0000,
    #[doc = "1: K1x Family"]
    _0001,
    #[doc = "2: K2x Family"]
    _0010,
    #[doc = "3: K3x Family"]
    _0011,
    #[doc = "4: K4x Family"]
    _0100,
    #[doc = "6: K6x Family"]
    _0110,
    #[doc = "7: K7x Family"]
    _0111,
    #[doc = "8: K8x Family"]
    _1000,
}
impl From<FAMILYID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILYID_A) -> Self {
        match variant {
            FAMILYID_A::_0000 => 0,
            FAMILYID_A::_0001 => 1,
            FAMILYID_A::_0010 => 2,
            FAMILYID_A::_0011 => 3,
            FAMILYID_A::_0100 => 4,
            FAMILYID_A::_0110 => 6,
            FAMILYID_A::_0111 => 7,
            FAMILYID_A::_1000 => 8,
        }
    }
}
#[doc = "Reader of field `FAMILYID`"]
pub type FAMILYID_R = crate::R<u8, FAMILYID_A>;
impl FAMILYID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMILYID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMILYID_A::_0000),
            1 => Val(FAMILYID_A::_0001),
            2 => Val(FAMILYID_A::_0010),
            3 => Val(FAMILYID_A::_0011),
            4 => Val(FAMILYID_A::_0100),
            6 => Val(FAMILYID_A::_0110),
            7 => Val(FAMILYID_A::_0111),
            8 => Val(FAMILYID_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FAMILYID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FAMILYID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == FAMILYID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FAMILYID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FAMILYID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == FAMILYID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == FAMILYID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == FAMILYID_A::_1000
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:11 - Device Die ID"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline(always)]
    pub fn seriesid(&self) -> SERIESID_R {
        SERIESID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis Family ID"]
    #[inline(always)]
    pub fn familyid(&self) -> FAMILYID_R {
        FAMILYID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
