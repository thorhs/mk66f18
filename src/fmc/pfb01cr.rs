#[doc = "Reader of register PFB01CR"]
pub type R = crate::R<u32, super::PFB01CR>;
#[doc = "Writer for register PFB01CR"]
pub type W = crate::W<u32, super::PFB01CR>;
#[doc = "Register PFB01CR `reset()`'s with value 0x3004_001f"]
impl crate::ResetValue for super::PFB01CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3004_001f
    }
}
#[doc = "Reader of field `RFU`"]
pub type RFU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFU`"]
pub struct RFU_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU_W<'a> {
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
#[doc = "Bank 0 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0IPE_A {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1,
}
impl From<B0IPE_A> for bool {
    #[inline(always)]
    fn from(variant: B0IPE_A) -> Self {
        match variant {
            B0IPE_A::_0 => false,
            B0IPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `B0IPE`"]
pub type B0IPE_R = crate::R<bool, B0IPE_A>;
impl B0IPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0IPE_A {
        match self.bits {
            false => B0IPE_A::_0,
            true => B0IPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0IPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0IPE_A::_1
    }
}
#[doc = "Write proxy for field `B0IPE`"]
pub struct B0IPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0IPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B0IPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0IPE_A::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0IPE_A::_1)
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
#[doc = "Bank 0 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0DPE_A {
    #[doc = "0: Do not prefetch in response to data references."]
    _0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1,
}
impl From<B0DPE_A> for bool {
    #[inline(always)]
    fn from(variant: B0DPE_A) -> Self {
        match variant {
            B0DPE_A::_0 => false,
            B0DPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `B0DPE`"]
pub type B0DPE_R = crate::R<bool, B0DPE_A>;
impl B0DPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0DPE_A {
        match self.bits {
            false => B0DPE_A::_0,
            true => B0DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0DPE_A::_1
    }
}
#[doc = "Write proxy for field `B0DPE`"]
pub struct B0DPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0DPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B0DPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DPE_A::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DPE_A::_1)
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
#[doc = "Bank 0 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0ICE_A {
    #[doc = "0: Do not cache instruction fetches."]
    _0,
    #[doc = "1: Cache instruction fetches."]
    _1,
}
impl From<B0ICE_A> for bool {
    #[inline(always)]
    fn from(variant: B0ICE_A) -> Self {
        match variant {
            B0ICE_A::_0 => false,
            B0ICE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `B0ICE`"]
pub type B0ICE_R = crate::R<bool, B0ICE_A>;
impl B0ICE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0ICE_A {
        match self.bits {
            false => B0ICE_A::_0,
            true => B0ICE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0ICE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0ICE_A::_1
    }
}
#[doc = "Write proxy for field `B0ICE`"]
pub struct B0ICE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0ICE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B0ICE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0ICE_A::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0ICE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Bank 0 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0DCE_A {
    #[doc = "0: Do not cache data references."]
    _0,
    #[doc = "1: Cache data references."]
    _1,
}
impl From<B0DCE_A> for bool {
    #[inline(always)]
    fn from(variant: B0DCE_A) -> Self {
        match variant {
            B0DCE_A::_0 => false,
            B0DCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `B0DCE`"]
pub type B0DCE_R = crate::R<bool, B0DCE_A>;
impl B0DCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0DCE_A {
        match self.bits {
            false => B0DCE_A::_0,
            true => B0DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0DCE_A::_1
    }
}
#[doc = "Write proxy for field `B0DCE`"]
pub struct B0DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0DCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B0DCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DCE_A::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Cache Replacement Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: LRU replacement algorithm per set across all four ways"]
    _000,
    #[doc = "2: Independent LRU with ways \\[0-1\\] for ifetches, \\[2-3\\] for data"]
    _010,
    #[doc = "3: Independent LRU with ways \\[0-2\\] for ifetches, \\[3\\] for data"]
    _011,
}
impl From<CRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        match variant {
            CRC_A::_000 => 0,
            CRC_A::_010 => 2,
            CRC_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<u8, CRC_A>;
impl CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRC_A::_000),
            2 => Val(CRC_A::_010),
            3 => Val(CRC_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRC_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CRC_A::_011
    }
}
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LRU replacement algorithm per set across all four ways"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CRC_A::_000)
    }
    #[doc = "Independent LRU with ways \\[0-1\\] for ifetches, \\[2-3\\] for data"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CRC_A::_010)
    }
    #[doc = "Independent LRU with ways \\[0-2\\] for ifetches, \\[3\\] for data"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CRC_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Bank 0 Memory Width\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0MW_A {
    #[doc = "0: 32 bits"]
    _00,
    #[doc = "1: 64 bits"]
    _01,
    #[doc = "2: 128 bits"]
    _10,
}
impl From<B0MW_A> for u8 {
    #[inline(always)]
    fn from(variant: B0MW_A) -> Self {
        match variant {
            B0MW_A::_00 => 0,
            B0MW_A::_01 => 1,
            B0MW_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `B0MW`"]
pub type B0MW_R = crate::R<u8, B0MW_A>;
impl B0MW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, B0MW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(B0MW_A::_00),
            1 => Val(B0MW_A::_01),
            2 => Val(B0MW_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B0MW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B0MW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == B0MW_A::_10
    }
}
#[doc = "Invalidate Prefetch Speculation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_B_INV_AW {
    #[doc = "0: Speculation buffer is not affected"]
    _0,
    #[doc = "1: Invalidate (clear) speculation buffer"]
    _1,
}
impl From<S_B_INV_AW> for bool {
    #[inline(always)]
    fn from(variant: S_B_INV_AW) -> Self {
        match variant {
            S_B_INV_AW::_0 => false,
            S_B_INV_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `S_B_INV`"]
pub struct S_B_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> S_B_INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_B_INV_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Speculation buffer is not affected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(S_B_INV_AW::_0)
    }
    #[doc = "Invalidate (clear) speculation buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(S_B_INV_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Cache Invalidate Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINV_WAY_AW {
    #[doc = "0: No cache way invalidation for the corresponding cache"]
    _0,
    #[doc = "1: Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    _1,
}
impl From<CINV_WAY_AW> for u8 {
    #[inline(always)]
    fn from(variant: CINV_WAY_AW) -> Self {
        match variant {
            CINV_WAY_AW::_0 => 0,
            CINV_WAY_AW::_1 => 1,
        }
    }
}
#[doc = "Write proxy for field `CINV_WAY`"]
pub struct CINV_WAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CINV_WAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINV_WAY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No cache way invalidation for the corresponding cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINV_WAY_AW::_0)
    }
    #[doc = "Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINV_WAY_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Cache Lock Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLCK_WAY_A {
    #[doc = "0: Cache way is unlocked and may be displaced"]
    _0,
    #[doc = "1: Cache way is locked and its contents are not displaced"]
    _1,
}
impl From<CLCK_WAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CLCK_WAY_A) -> Self {
        match variant {
            CLCK_WAY_A::_0 => 0,
            CLCK_WAY_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `CLCK_WAY`"]
pub type CLCK_WAY_R = crate::R<u8, CLCK_WAY_A>;
impl CLCK_WAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLCK_WAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLCK_WAY_A::_0),
            1 => Val(CLCK_WAY_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLCK_WAY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLCK_WAY_A::_1
    }
}
#[doc = "Write proxy for field `CLCK_WAY`"]
pub struct CLCK_WAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLCK_WAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLCK_WAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Cache way is unlocked and may be displaced"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLCK_WAY_A::_0)
    }
    #[doc = "Cache way is locked and its contents are not displaced"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLCK_WAY_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `B0RWSC`"]
pub type B0RWSC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Reserved for future use"]
    #[inline(always)]
    pub fn rfu(&self) -> RFU_R {
        RFU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b0ipe(&self) -> B0IPE_R {
        B0IPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b0dpe(&self) -> B0DPE_R {
        B0DPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b0ice(&self) -> B0ICE_R {
        B0ICE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    pub fn b0dce(&self) -> B0DCE_R {
        B0DCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - Bank 0 Memory Width"]
    #[inline(always)]
    pub fn b0mw(&self) -> B0MW_R {
        B0MW_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    pub fn clck_way(&self) -> CLCK_WAY_R {
        CLCK_WAY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Bank 0 Read Wait State Control"]
    #[inline(always)]
    pub fn b0rwsc(&self) -> B0RWSC_R {
        B0RWSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved for future use"]
    #[inline(always)]
    pub fn rfu(&mut self) -> RFU_W {
        RFU_W { w: self }
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b0ipe(&mut self) -> B0IPE_W {
        B0IPE_W { w: self }
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b0dpe(&mut self) -> B0DPE_W {
        B0DPE_W { w: self }
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b0ice(&mut self) -> B0ICE_W {
        B0ICE_W { w: self }
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    pub fn b0dce(&mut self) -> B0DCE_W {
        B0DCE_W { w: self }
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 19 - Invalidate Prefetch Speculation Buffer"]
    #[inline(always)]
    pub fn s_b_inv(&mut self) -> S_B_INV_W {
        S_B_INV_W { w: self }
    }
    #[doc = "Bits 20:23 - Cache Invalidate Way x"]
    #[inline(always)]
    pub fn cinv_way(&mut self) -> CINV_WAY_W {
        CINV_WAY_W { w: self }
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    pub fn clck_way(&mut self) -> CLCK_WAY_W {
        CLCK_WAY_W { w: self }
    }
}
