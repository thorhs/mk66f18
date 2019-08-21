#[doc = "Reader of register SYSCTL"]
pub type R = crate::R<u32, super::SYSCTL>;
#[doc = "Writer for register SYSCTL"]
pub type W = crate::W<u32, super::SYSCTL>;
#[doc = "Register SYSCTL `reset()`'s with value 0x8008"]
impl crate::ResetValue for super::SYSCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8008
    }
}
#[doc = "IPG Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGEN_A {
    #[doc = "0: Bus clock will be internally gated off."]
    _0,
    #[doc = "1: Bus clock will not be automatically gated off."]
    _1,
}
impl From<IPGEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPGEN_A) -> Self {
        match variant {
            IPGEN_A::_0 => false,
            IPGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IPGEN`"]
pub type IPGEN_R = crate::R<bool, IPGEN_A>;
impl IPGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGEN_A {
        match self.bits {
            false => IPGEN_A::_0,
            true => IPGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPGEN_A::_1
    }
}
#[doc = "Write proxy for field `IPGEN`"]
pub struct IPGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPGEN_A::_0)
    }
    #[doc = "Bus clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPGEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "System Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKEN_A {
    #[doc = "0: System clock will be internally gated off."]
    _0,
    #[doc = "1: System clock will not be automatically gated off."]
    _1,
}
impl From<HCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: HCKEN_A) -> Self {
        match variant {
            HCKEN_A::_0 => false,
            HCKEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HCKEN`"]
pub type HCKEN_R = crate::R<bool, HCKEN_A>;
impl HCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKEN_A {
        match self.bits {
            false => HCKEN_A::_0,
            true => HCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCKEN_A::_1
    }
}
#[doc = "Write proxy for field `HCKEN`"]
pub struct HCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "System clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCKEN_A::_0)
    }
    #[doc = "System clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCKEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Peripheral Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEREN_A {
    #[doc = "0: SDHC clock will be internally gated off."]
    _0,
    #[doc = "1: SDHC clock will not be automatically gated off."]
    _1,
}
impl From<PEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PEREN_A) -> Self {
        match variant {
            PEREN_A::_0 => false,
            PEREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PEREN`"]
pub type PEREN_R = crate::R<bool, PEREN_A>;
impl PEREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEREN_A {
        match self.bits {
            false => PEREN_A::_0,
            true => PEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEREN_A::_1
    }
}
#[doc = "Write proxy for field `PEREN`"]
pub struct PEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDHC clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEREN_A::_0)
    }
    #[doc = "SDHC clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEREN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SDCLKEN`"]
pub type SDCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDCLKEN`"]
pub struct SDCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVS_A {
    #[doc = "0: Divisor by 1."]
    _0,
    #[doc = "1: Divisor by 2."]
    _1,
    #[doc = "14: Divisor by 15."]
    _1110,
    #[doc = "15: Divisor by 16."]
    _1111,
}
impl From<DVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DVS_A) -> Self {
        match variant {
            DVS_A::_0 => 0,
            DVS_A::_1 => 1,
            DVS_A::_1110 => 14,
            DVS_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `DVS`"]
pub type DVS_R = crate::R<u8, DVS_A>;
impl DVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DVS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DVS_A::_0),
            1 => Val(DVS_A::_1),
            14 => Val(DVS_A::_1110),
            15 => Val(DVS_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVS_A::_1
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DVS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DVS_A::_1111
    }
}
#[doc = "Write proxy for field `DVS`"]
pub struct DVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DVS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divisor by 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVS_A::_0)
    }
    #[doc = "Divisor by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVS_A::_1)
    }
    #[doc = "Divisor by 15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DVS_A::_1110)
    }
    #[doc = "Divisor by 16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DVS_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "SDCLK Frequency Select\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKFS_A {
    #[doc = "1: Base clock divided by 2."]
    _1,
    #[doc = "2: Base clock divided by 4."]
    _10,
    #[doc = "4: Base clock divided by 8."]
    _100,
    #[doc = "8: Base clock divided by 16."]
    _1000,
    #[doc = "16: Base clock divided by 32."]
    _10000,
    #[doc = "32: Base clock divided by 64."]
    _100000,
    #[doc = "64: Base clock divided by 128."]
    _1000000,
    #[doc = "128: Base clock divided by 256."]
    _10000000,
}
impl From<SDCLKFS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLKFS_A) -> Self {
        match variant {
            SDCLKFS_A::_1 => 1,
            SDCLKFS_A::_10 => 2,
            SDCLKFS_A::_100 => 4,
            SDCLKFS_A::_1000 => 8,
            SDCLKFS_A::_10000 => 16,
            SDCLKFS_A::_100000 => 32,
            SDCLKFS_A::_1000000 => 64,
            SDCLKFS_A::_10000000 => 128,
        }
    }
}
#[doc = "Reader of field `SDCLKFS`"]
pub type SDCLKFS_R = crate::R<u8, SDCLKFS_A>;
impl SDCLKFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDCLKFS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SDCLKFS_A::_1),
            2 => Val(SDCLKFS_A::_10),
            4 => Val(SDCLKFS_A::_100),
            8 => Val(SDCLKFS_A::_1000),
            16 => Val(SDCLKFS_A::_10000),
            32 => Val(SDCLKFS_A::_100000),
            64 => Val(SDCLKFS_A::_1000000),
            128 => Val(SDCLKFS_A::_10000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCLKFS_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SDCLKFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDCLKFS_A::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SDCLKFS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == SDCLKFS_A::_10000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == SDCLKFS_A::_100000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == SDCLKFS_A::_1000000
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline(always)]
    pub fn is_10000000(&self) -> bool {
        *self == SDCLKFS_A::_10000000
    }
}
#[doc = "Write proxy for field `SDCLKFS`"]
pub struct SDCLKFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLKFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Base clock divided by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1)
    }
    #[doc = "Base clock divided by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10)
    }
    #[doc = "Base clock divided by 8."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_100)
    }
    #[doc = "Base clock divided by 16."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1000)
    }
    #[doc = "Base clock divided by 32."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10000)
    }
    #[doc = "Base clock divided by 64."]
    #[inline(always)]
    pub fn _100000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_100000)
    }
    #[doc = "Base clock divided by 128."]
    #[inline(always)]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1000000)
    }
    #[doc = "Base clock divided by 256."]
    #[inline(always)]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10000000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCV_A {
    #[doc = "0: SDCLK x 2 13"]
    _0000,
    #[doc = "1: SDCLK x 2 14"]
    _0001,
    #[doc = "14: SDCLK x 2 27"]
    _1110,
}
impl From<DTOCV_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_A) -> Self {
        match variant {
            DTOCV_A::_0000 => 0,
            DTOCV_A::_0001 => 1,
            DTOCV_A::_1110 => 14,
        }
    }
}
#[doc = "Reader of field `DTOCV`"]
pub type DTOCV_R = crate::R<u8, DTOCV_A>;
impl DTOCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTOCV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTOCV_A::_0000),
            1 => Val(DTOCV_A::_0001),
            14 => Val(DTOCV_A::_1110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DTOCV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DTOCV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DTOCV_A::_1110
    }
}
#[doc = "Write proxy for field `DTOCV`"]
pub struct DTOCV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOCV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDCLK x 2 13"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCV_A::_0000)
    }
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCV_A::_0001)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCV_A::_1110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Software Reset For ALL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTA_AW {
    #[doc = "0: No reset."]
    _0,
    #[doc = "1: Reset."]
    _1,
}
impl From<RSTA_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTA_AW) -> Self {
        match variant {
            RSTA_AW::_0 => false,
            RSTA_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RSTA`"]
pub struct RSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTA_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTA_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTC_AW {
    #[doc = "0: No reset."]
    _0,
    #[doc = "1: Reset."]
    _1,
}
impl From<RSTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTC_AW) -> Self {
        match variant {
            RSTC_AW::_0 => false,
            RSTC_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RSTC`"]
pub struct RSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTC_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTC_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTD_AW {
    #[doc = "0: No reset."]
    _0,
    #[doc = "1: Reset."]
    _1,
}
impl From<RSTD_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTD_AW) -> Self {
        match variant {
            RSTD_AW::_0 => false,
            RSTD_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RSTD`"]
pub struct RSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTD_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTD_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `INITA`"]
pub type INITA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITA`"]
pub struct INITA_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline(always)]
    pub fn ipgen(&self) -> IPGEN_R {
        IPGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline(always)]
    pub fn hcken(&self) -> HCKEN_R {
        HCKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&self) -> SDCLKFS_R {
        SDCLKFS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&self) -> DTOCV_R {
        DTOCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&self) -> INITA_R {
        INITA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline(always)]
    pub fn ipgen(&mut self) -> IPGEN_W {
        IPGEN_W { w: self }
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline(always)]
    pub fn hcken(&mut self) -> HCKEN_W {
        HCKEN_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline(always)]
    pub fn peren(&mut self) -> PEREN_W {
        PEREN_W { w: self }
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SDCLKEN_W {
        SDCLKEN_W { w: self }
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&mut self) -> DVS_W {
        DVS_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&mut self) -> SDCLKFS_W {
        SDCLKFS_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&mut self) -> DTOCV_W {
        DTOCV_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    pub fn rsta(&mut self) -> RSTA_W {
        RSTA_W { w: self }
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RSTC_W {
        RSTC_W { w: self }
    }
    #[doc = "Bit 26 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn rstd(&mut self) -> RSTD_W {
        RSTD_W { w: self }
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&mut self) -> INITA_W {
        INITA_W { w: self }
    }
}
