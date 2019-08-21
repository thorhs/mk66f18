#[doc = "Reader of register SCGC6"]
pub type R = crate::R<u32, super::SCGC6>;
#[doc = "Writer for register SCGC6"]
pub type W = crate::W<u32, super::SCGC6>;
#[doc = "Register SCGC6 `reset()`'s with value 0x4000_0001"]
impl crate::ResetValue for super::SCGC6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0001
    }
}
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTF_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        match variant {
            FTF_A::_0 => false,
            FTF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTF`"]
pub type FTF_R = crate::R<bool, FTF_A>;
impl FTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTF_A {
        match self.bits {
            false => FTF_A::_0,
            true => FTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTF_A::_1
    }
}
#[doc = "Write proxy for field `FTF`"]
pub struct FTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTF_A::_1)
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
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUX_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<DMAMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX_A) -> Self {
        match variant {
            DMAMUX_A::_0 => false,
            DMAMUX_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAMUX`"]
pub type DMAMUX_R = crate::R<bool, DMAMUX_A>;
impl DMAMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX_A {
        match self.bits {
            false => DMAMUX_A::_0,
            true => DMAMUX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAMUX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAMUX_A::_1
    }
}
#[doc = "Write proxy for field `DMAMUX`"]
pub struct DMAMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAMUX_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAMUX_A::_1)
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
#[doc = "FlexCAN0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCAN0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FLEXCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCAN0_A) -> Self {
        match variant {
            FLEXCAN0_A::_0 => false,
            FLEXCAN0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXCAN0`"]
pub type FLEXCAN0_R = crate::R<bool, FLEXCAN0_A>;
impl FLEXCAN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCAN0_A {
        match self.bits {
            false => FLEXCAN0_A::_0,
            true => FLEXCAN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLEXCAN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLEXCAN0_A::_1
    }
}
#[doc = "Write proxy for field `FLEXCAN0`"]
pub struct FLEXCAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCAN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCAN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXCAN0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXCAN0_A::_1)
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
#[doc = "Reader of field `RNGA`"]
pub type RNGA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGA`"]
pub struct RNGA_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        match variant {
            SPI0_A::_0 => false,
            SPI0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SPI0`"]
pub type SPI0_R = crate::R<bool, SPI0_A>;
impl SPI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::_0,
            true => SPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_A::_1
    }
}
#[doc = "Write proxy for field `SPI0`"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SPI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        match variant {
            SPI1_A::_0 => false,
            SPI1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SPI1`"]
pub type SPI1_R = crate::R<bool, SPI1_A>;
impl SPI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::_0,
            true => SPI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_A::_1
    }
}
#[doc = "Write proxy for field `SPI1`"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "I2S Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<I2S_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_A) -> Self {
        match variant {
            I2S_A::_0 => false,
            I2S_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `I2S`"]
pub type I2S_R = crate::R<bool, I2S_A>;
impl I2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_A {
        match self.bits {
            false => I2S_A::_0,
            true => I2S_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2S_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2S_A::_1
    }
}
#[doc = "Write proxy for field `I2S`"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2S_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2S_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        match variant {
            CRC_A::_0 => false,
            CRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, CRC_A>;
impl CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC_A::_1
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "USB DCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDCD_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<USBDCD_A> for bool {
    #[inline(always)]
    fn from(variant: USBDCD_A) -> Self {
        match variant {
            USBDCD_A::_0 => false,
            USBDCD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBDCD`"]
pub type USBDCD_R = crate::R<bool, USBDCD_A>;
impl USBDCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDCD_A {
        match self.bits {
            false => USBDCD_A::_0,
            true => USBDCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBDCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBDCD_A::_1
    }
}
#[doc = "Write proxy for field `USBDCD`"]
pub struct USBDCD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDCD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDCD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "PDB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDB_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PDB_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_A) -> Self {
        match variant {
            PDB_A::_0 => false,
            PDB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDB`"]
pub type PDB_R = crate::R<bool, PDB_A>;
impl PDB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_A {
        match self.bits {
            false => PDB_A::_0,
            true => PDB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDB_A::_1
    }
}
#[doc = "Write proxy for field `PDB`"]
pub struct PDB_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        match variant {
            PIT_A::_0 => false,
            PIT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PIT`"]
pub type PIT_R = crate::R<bool, PIT_A>;
impl PIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT_A::_1
    }
}
#[doc = "Write proxy for field `PIT`"]
pub struct PIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FTM0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_A) -> Self {
        match variant {
            FTM0_A::_0 => false,
            FTM0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM0`"]
pub type FTM0_R = crate::R<bool, FTM0_A>;
impl FTM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_A {
        match self.bits {
            false => FTM0_A::_0,
            true => FTM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0_A::_1
    }
}
#[doc = "Write proxy for field `FTM0`"]
pub struct FTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_A::_1)
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
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FTM1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_A) -> Self {
        match variant {
            FTM1_A::_0 => false,
            FTM1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM1`"]
pub type FTM1_R = crate::R<bool, FTM1_A>;
impl FTM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_A {
        match self.bits {
            false => FTM1_A::_0,
            true => FTM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1_A::_1
    }
}
#[doc = "Write proxy for field `FTM1`"]
pub struct FTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_A::_1)
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
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        match variant {
            FTM2_A::_0 => false,
            FTM2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM2`"]
pub type FTM2_R = crate::R<bool, FTM2_A>;
impl FTM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Write proxy for field `FTM2`"]
pub struct FTM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
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
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        match variant {
            ADC0_A::_0 => false,
            ADC0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, ADC0_A>;
impl ADC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::_0,
            true => ADC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0_A::_1
    }
}
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "RTC Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Access and interrupts disabled"]
    _0,
    #[doc = "1: Access and interrupts enabled"]
    _1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        match variant {
            RTC_A::_0 => false,
            RTC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTC_A::_1
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access and interrupts disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Access and interrupts enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        match variant {
            DAC0_A::_0 => false,
            DAC0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DAC0`"]
pub type DAC0_R = crate::R<bool, DAC0_A>;
impl DAC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Write proxy for field `DAC0`"]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FlexCAN0 Clock Gate Control"]
    #[inline(always)]
    pub fn flexcan0(&self) -> FLEXCAN0_R {
        FLEXCAN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&self) -> RNGA_R {
        RNGA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbdcd(&self) -> USBDCD_R {
        USBDCD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> FTM0_R {
        FTM0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> FTM1_R {
        FTM1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W { w: self }
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&mut self) -> DMAMUX_W {
        DMAMUX_W { w: self }
    }
    #[doc = "Bit 4 - FlexCAN0 Clock Gate Control"]
    #[inline(always)]
    pub fn flexcan0(&mut self) -> FLEXCAN0_W {
        FLEXCAN0_W { w: self }
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&mut self) -> RNGA_W {
        RNGA_W { w: self }
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbdcd(&mut self) -> USBDCD_W {
        USBDCD_W { w: self }
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&mut self) -> PDB_W {
        PDB_W { w: self }
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&mut self) -> PIT_W {
        PIT_W { w: self }
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&mut self) -> FTM0_W {
        FTM0_W { w: self }
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&mut self) -> FTM1_W {
        FTM1_W { w: self }
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> FTM2_W {
        FTM2_W { w: self }
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
}
