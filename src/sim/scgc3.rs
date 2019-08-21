#[doc = "Reader of register SCGC3"]
pub type R = crate::R<u32, super::SCGC3>;
#[doc = "Writer for register SCGC3"]
pub type W = crate::W<u32, super::SCGC3>;
#[doc = "Register SCGC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RNGA Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGA_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<RNGA_A> for bool {
    #[inline(always)]
    fn from(variant: RNGA_A) -> Self {
        match variant {
            RNGA_A::_0 => false,
            RNGA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RNGA`"]
pub type RNGA_R = crate::R<bool, RNGA_A>;
impl RNGA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGA_A {
        match self.bits {
            false => RNGA_A::_0,
            true => RNGA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNGA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNGA_A::_1
    }
}
#[doc = "Write proxy for field `RNGA`"]
pub struct RNGA_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNGA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNGA_A::_1)
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
#[doc = "USBHS Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHS_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<USBHS_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_A) -> Self {
        match variant {
            USBHS_A::_0 => false,
            USBHS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBHS`"]
pub type USBHS_R = crate::R<bool, USBHS_A>;
impl USBHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_A {
        match self.bits {
            false => USBHS_A::_0,
            true => USBHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBHS_A::_1
    }
}
#[doc = "Write proxy for field `USBHS`"]
pub struct USBHS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBHS_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBHS_A::_1)
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
#[doc = "USBHS PHY Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHSPHY_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<USBHSPHY_A> for bool {
    #[inline(always)]
    fn from(variant: USBHSPHY_A) -> Self {
        match variant {
            USBHSPHY_A::_0 => false,
            USBHSPHY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBHSPHY`"]
pub type USBHSPHY_R = crate::R<bool, USBHSPHY_A>;
impl USBHSPHY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHSPHY_A {
        match self.bits {
            false => USBHSPHY_A::_0,
            true => USBHSPHY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBHSPHY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBHSPHY_A::_1
    }
}
#[doc = "Write proxy for field `USBHSPHY`"]
pub struct USBHSPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHSPHY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHSPHY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBHSPHY_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBHSPHY_A::_1)
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
#[doc = "USBHS DCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHSDCD_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<USBHSDCD_A> for bool {
    #[inline(always)]
    fn from(variant: USBHSDCD_A) -> Self {
        match variant {
            USBHSDCD_A::_0 => false,
            USBHSDCD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBHSDCD`"]
pub type USBHSDCD_R = crate::R<bool, USBHSDCD_A>;
impl USBHSDCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHSDCD_A {
        match self.bits {
            false => USBHSDCD_A::_0,
            true => USBHSDCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBHSDCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBHSDCD_A::_1
    }
}
#[doc = "Write proxy for field `USBHSDCD`"]
pub struct USBHSDCD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHSDCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHSDCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBHSDCD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBHSDCD_A::_1)
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
#[doc = "FlexCAN1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCAN1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FLEXCAN1_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCAN1_A) -> Self {
        match variant {
            FLEXCAN1_A::_0 => false,
            FLEXCAN1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXCAN1`"]
pub type FLEXCAN1_R = crate::R<bool, FLEXCAN1_A>;
impl FLEXCAN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCAN1_A {
        match self.bits {
            false => FLEXCAN1_A::_0,
            true => FLEXCAN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLEXCAN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLEXCAN1_A::_1
    }
}
#[doc = "Write proxy for field `FLEXCAN1`"]
pub struct FLEXCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCAN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCAN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXCAN1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXCAN1_A::_1)
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
#[doc = "SPI2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<SPI2_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2_A) -> Self {
        match variant {
            SPI2_A::_0 => false,
            SPI2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SPI2`"]
pub type SPI2_R = crate::R<bool, SPI2_A>;
impl SPI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2_A {
        match self.bits {
            false => SPI2_A::_0,
            true => SPI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI2_A::_1
    }
}
#[doc = "Write proxy for field `SPI2`"]
pub struct SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI2_A::_1)
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
#[doc = "SDHC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHC_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<SDHC_A> for bool {
    #[inline(always)]
    fn from(variant: SDHC_A) -> Self {
        match variant {
            SDHC_A::_0 => false,
            SDHC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SDHC`"]
pub type SDHC_R = crate::R<bool, SDHC_A>;
impl SDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHC_A {
        match self.bits {
            false => SDHC_A::_0,
            true => SDHC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDHC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDHC_A::_1
    }
}
#[doc = "Write proxy for field `SDHC`"]
pub struct SDHC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDHC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDHC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDHC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "FTM3 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FTM3_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3_A) -> Self {
        match variant {
            FTM3_A::_0 => false,
            FTM3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FTM3`"]
pub type FTM3_R = crate::R<bool, FTM3_A>;
impl FTM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3_A {
        match self.bits {
            false => FTM3_A::_0,
            true => FTM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3_A::_1
    }
}
#[doc = "Write proxy for field `FTM3`"]
pub struct FTM3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3_A::_1)
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
#[doc = "ADC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<ADC1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_A) -> Self {
        match variant {
            ADC1_A::_0 => false,
            ADC1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, ADC1_A>;
impl ADC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_A {
        match self.bits {
            false => ADC1_A::_0,
            true => ADC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1_A::_1
    }
}
#[doc = "Write proxy for field `ADC1`"]
pub struct ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1_A::_1)
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
impl R {
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&self) -> RNGA_R {
        RNGA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USBHS Clock Gate Control"]
    #[inline(always)]
    pub fn usbhs(&self) -> USBHS_R {
        USBHS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USBHS PHY Clock Gate Control"]
    #[inline(always)]
    pub fn usbhsphy(&self) -> USBHSPHY_R {
        USBHSPHY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USBHS DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbhsdcd(&self) -> USBHSDCD_R {
        USBHSDCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FlexCAN1 Clock Gate Control"]
    #[inline(always)]
    pub fn flexcan1(&self) -> FLEXCAN1_R {
        FLEXCAN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline(always)]
    pub fn sdhc(&self) -> SDHC_R {
        SDHC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm3(&self) -> FTM3_R {
        FTM3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&mut self) -> RNGA_W {
        RNGA_W { w: self }
    }
    #[doc = "Bit 1 - USBHS Clock Gate Control"]
    #[inline(always)]
    pub fn usbhs(&mut self) -> USBHS_W {
        USBHS_W { w: self }
    }
    #[doc = "Bit 2 - USBHS PHY Clock Gate Control"]
    #[inline(always)]
    pub fn usbhsphy(&mut self) -> USBHSPHY_W {
        USBHSPHY_W { w: self }
    }
    #[doc = "Bit 3 - USBHS DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbhsdcd(&mut self) -> USBHSDCD_W {
        USBHSDCD_W { w: self }
    }
    #[doc = "Bit 4 - FlexCAN1 Clock Gate Control"]
    #[inline(always)]
    pub fn flexcan1(&mut self) -> FLEXCAN1_W {
        FLEXCAN1_W { w: self }
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W {
        SPI2_W { w: self }
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline(always)]
    pub fn sdhc(&mut self) -> SDHC_W {
        SDHC_W { w: self }
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> FTM2_W {
        FTM2_W { w: self }
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm3(&mut self) -> FTM3_W {
        FTM3_W { w: self }
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W { w: self }
    }
}
