#[doc = "Reader of register HTCAPBLT"]
pub type R = crate::R<u32, super::HTCAPBLT>;
#[doc = "Max Block Length\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBL_A {
    #[doc = "0: 512 bytes"]
    _000,
    #[doc = "1: 1024 bytes"]
    _001,
    #[doc = "2: 2048 bytes"]
    _010,
    #[doc = "3: 4096 bytes"]
    _011,
}
impl From<MBL_A> for u8 {
    #[inline(always)]
    fn from(variant: MBL_A) -> Self {
        match variant {
            MBL_A::_000 => 0,
            MBL_A::_001 => 1,
            MBL_A::_010 => 2,
            MBL_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `MBL`"]
pub type MBL_R = crate::R<u8, MBL_A>;
impl MBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MBL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MBL_A::_000),
            1 => Val(MBL_A::_001),
            2 => Val(MBL_A::_010),
            3 => Val(MBL_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MBL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MBL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MBL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MBL_A::_011
    }
}
#[doc = "ADMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMAS_A {
    #[doc = "0: Advanced DMA not supported."]
    _0,
    #[doc = "1: Advanced DMA supported."]
    _1,
}
impl From<ADMAS_A> for bool {
    #[inline(always)]
    fn from(variant: ADMAS_A) -> Self {
        match variant {
            ADMAS_A::_0 => false,
            ADMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADMAS`"]
pub type ADMAS_R = crate::R<bool, ADMAS_A>;
impl ADMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMAS_A {
        match self.bits {
            false => ADMAS_A::_0,
            true => ADMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADMAS_A::_1
    }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSS_A {
    #[doc = "0: High speed not supported."]
    _0,
    #[doc = "1: High speed supported."]
    _1,
}
impl From<HSS_A> for bool {
    #[inline(always)]
    fn from(variant: HSS_A) -> Self {
        match variant {
            HSS_A::_0 => false,
            HSS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HSS`"]
pub type HSS_R = crate::R<bool, HSS_A>;
impl HSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSS_A {
        match self.bits {
            false => HSS_A::_0,
            true => HSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSS_A::_1
    }
}
#[doc = "DMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAS_A {
    #[doc = "0: DMA not supported."]
    _0,
    #[doc = "1: DMA supported."]
    _1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        match variant {
            DMAS_A::_0 => false,
            DMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAS`"]
pub type DMAS_R = crate::R<bool, DMAS_A>;
impl DMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::_0,
            true => DMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAS_A::_1
    }
}
#[doc = "Suspend/Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS_A {
    #[doc = "0: Not supported."]
    _0,
    #[doc = "1: Supported."]
    _1,
}
impl From<SRS_A> for bool {
    #[inline(always)]
    fn from(variant: SRS_A) -> Self {
        match variant {
            SRS_A::_0 => false,
            SRS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SRS`"]
pub type SRS_R = crate::R<bool, SRS_A>;
impl SRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS_A {
        match self.bits {
            false => SRS_A::_0,
            true => SRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRS_A::_1
    }
}
#[doc = "Voltage Support 3.3 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS33_A {
    #[doc = "0: 3.3 V not supported."]
    _0,
    #[doc = "1: 3.3 V supported."]
    _1,
}
impl From<VS33_A> for bool {
    #[inline(always)]
    fn from(variant: VS33_A) -> Self {
        match variant {
            VS33_A::_0 => false,
            VS33_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VS33`"]
pub type VS33_R = crate::R<bool, VS33_A>;
impl VS33_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS33_A {
        match self.bits {
            false => VS33_A::_0,
            true => VS33_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VS33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VS33_A::_1
    }
}
impl R {
    #[doc = "Bits 16:18 - Max Block Length"]
    #[inline(always)]
    pub fn mbl(&self) -> MBL_R {
        MBL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - ADMA Support"]
    #[inline(always)]
    pub fn admas(&self) -> ADMAS_R {
        ADMAS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hss(&self) -> HSS_R {
        HSS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA Support"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn srs(&self) -> SRS_R {
        SRS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3 V"]
    #[inline(always)]
    pub fn vs33(&self) -> VS33_R {
        VS33_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
