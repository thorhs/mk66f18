#[doc = "Reader of register TCR4"]
pub type R = crate::R<u32, super::TCR4>;
#[doc = "Writer for register TCR4"]
pub type W = crate::W<u32, super::TCR4>;
#[doc = "Register TCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSD_A {
    #[doc = "0: Frame sync is generated externally in Slave mode."]
    _0,
    #[doc = "1: Frame sync is generated internally in Master mode."]
    _1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        match variant {
            FSD_A::_0 => false,
            FSD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSD`"]
pub type FSD_R = crate::R<bool, FSD_A>;
impl FSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::_0,
            true => FSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSD_A::_1
    }
}
#[doc = "Write proxy for field `FSD`"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSD_A::_0)
    }
    #[doc = "Frame sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSD_A::_1)
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
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    _0,
    #[doc = "1: Frame sync is active low."]
    _1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        match variant {
            FSP_A::_0 => false,
            FSP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSP`"]
pub type FSP_R = crate::R<bool, FSP_A>;
impl FSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::_0,
            true => FSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSP_A::_1
    }
}
#[doc = "Write proxy for field `FSP`"]
pub struct FSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSP_A::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSP_A::_1)
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
#[doc = "On Demand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEM_A {
    #[doc = "0: Internal frame sync is generated continuously."]
    _0,
    #[doc = "1: Internal frame sync is generated when the FIFO warning flag is clear."]
    _1,
}
impl From<ONDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ONDEM_A) -> Self {
        match variant {
            ONDEM_A::_0 => false,
            ONDEM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ONDEM`"]
pub type ONDEM_R = crate::R<bool, ONDEM_A>;
impl ONDEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONDEM_A {
        match self.bits {
            false => ONDEM_A::_0,
            true => ONDEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONDEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONDEM_A::_1
    }
}
#[doc = "Write proxy for field `ONDEM`"]
pub struct ONDEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONDEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONDEM_A::_0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONDEM_A::_1)
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
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        match variant {
            FSE_A::_0 => false,
            FSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSE`"]
pub type FSE_R = crate::R<bool, FSE_A>;
impl FSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::_0,
            true => FSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSE_A::_1
    }
}
#[doc = "Write proxy for field `FSE`"]
pub struct FSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSE_A::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSE_A::_1)
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
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MF_A {
    #[doc = "0: LSB is transmitted first."]
    _0,
    #[doc = "1: MSB is transmitted first."]
    _1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        match variant {
            MF_A::_0 => false,
            MF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MF`"]
pub type MF_R = crate::R<bool, MF_A>;
impl MF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::_0,
            true => MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MF_A::_1
    }
}
#[doc = "Write proxy for field `MF`"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MF_A::_0)
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MF_A::_1)
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
#[doc = "Reader of field `SYWD`"]
pub type SYWD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYWD`"]
pub struct SYWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FRSZ`"]
pub type FRSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRSZ`"]
pub struct FRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "FIFO Packing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPACK_A {
    #[doc = "0: FIFO packing is disabled"]
    _00,
    #[doc = "2: 8-bit FIFO packing is enabled"]
    _10,
    #[doc = "3: 16-bit FIFO packing is enabled"]
    _11,
}
impl From<FPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: FPACK_A) -> Self {
        match variant {
            FPACK_A::_00 => 0,
            FPACK_A::_10 => 2,
            FPACK_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FPACK`"]
pub type FPACK_R = crate::R<u8, FPACK_A>;
impl FPACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FPACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FPACK_A::_00),
            2 => Val(FPACK_A::_10),
            3 => Val(FPACK_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FPACK_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FPACK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FPACK_A::_11
    }
}
#[doc = "Write proxy for field `FPACK`"]
pub struct FPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FPACK_A::_00)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FPACK_A::_10)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FPACK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "FIFO Combine Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCOMB_A {
    #[doc = "0: FIFO combine mode disabled."]
    _00,
    #[doc = "1: FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    _01,
    #[doc = "2: FIFO combine mode enabled on FIFO writes (by software)."]
    _10,
    #[doc = "3: FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    _11,
}
impl From<FCOMB_A> for u8 {
    #[inline(always)]
    fn from(variant: FCOMB_A) -> Self {
        match variant {
            FCOMB_A::_00 => 0,
            FCOMB_A::_01 => 1,
            FCOMB_A::_10 => 2,
            FCOMB_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FCOMB`"]
pub type FCOMB_R = crate::R<u8, FCOMB_A>;
impl FCOMB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCOMB_A {
        match self.bits {
            0 => FCOMB_A::_00,
            1 => FCOMB_A::_01,
            2 => FCOMB_A::_10,
            3 => FCOMB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FCOMB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FCOMB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FCOMB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FCOMB_A::_11
    }
}
#[doc = "Write proxy for field `FCOMB`"]
pub struct FCOMB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCOMB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCOMB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FIFO combine mode disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCOMB_A::_00)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCOMB_A::_01)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (by software)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCOMB_A::_10)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCOMB_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "FIFO Continue on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCONT_A {
    #[doc = "0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    _0,
    #[doc = "1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    _1,
}
impl From<FCONT_A> for bool {
    #[inline(always)]
    fn from(variant: FCONT_A) -> Self {
        match variant {
            FCONT_A::_0 => false,
            FCONT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FCONT`"]
pub type FCONT_R = crate::R<bool, FCONT_A>;
impl FCONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCONT_A {
        match self.bits {
            false => FCONT_A::_0,
            true => FCONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCONT_A::_1
    }
}
#[doc = "Write proxy for field `FCONT`"]
pub struct FCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCONT_A::_0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCONT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&self) -> ONDEM_R {
        ONDEM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&self) -> FPACK_R {
        FPACK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&self) -> FCOMB_R {
        FCOMB_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&self) -> FCONT_R {
        FCONT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&mut self) -> FSP_W {
        FSP_W { w: self }
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&mut self) -> ONDEM_W {
        ONDEM_W { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&mut self) -> FSE_W {
        FSE_W { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&mut self) -> SYWD_W {
        SYWD_W { w: self }
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&mut self) -> FRSZ_W {
        FRSZ_W { w: self }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&mut self) -> FPACK_W {
        FPACK_W { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&mut self) -> FCOMB_W {
        FCOMB_W { w: self }
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&mut self) -> FCONT_W {
        FCONT_W { w: self }
    }
}
