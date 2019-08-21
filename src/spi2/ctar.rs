#[doc = "Reader of register CTAR%s"]
pub type R = crate::R<u32, super::CTAR>;
#[doc = "Writer for register CTAR%s"]
pub type W = crate::W<u32, super::CTAR>;
#[doc = "Register CTAR%s `reset()`'s with value 0x7800_0000"]
impl crate::ResetValue for super::CTAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7800_0000
    }
}
#[doc = "Reader of field `BR`"]
pub type BR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BR`"]
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT`"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASC`"]
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CSSCK`"]
pub type CSSCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSSCK`"]
pub struct CSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Baud Rate Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBR_A {
    #[doc = "0: Baud Rate Prescaler value is 2."]
    _00,
    #[doc = "1: Baud Rate Prescaler value is 3."]
    _01,
    #[doc = "2: Baud Rate Prescaler value is 5."]
    _10,
    #[doc = "3: Baud Rate Prescaler value is 7."]
    _11,
}
impl From<PBR_A> for u8 {
    #[inline(always)]
    fn from(variant: PBR_A) -> Self {
        match variant {
            PBR_A::_00 => 0,
            PBR_A::_01 => 1,
            PBR_A::_10 => 2,
            PBR_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PBR`"]
pub type PBR_R = crate::R<u8, PBR_A>;
impl PBR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBR_A {
        match self.bits {
            0 => PBR_A::_00,
            1 => PBR_A::_01,
            2 => PBR_A::_10,
            3 => PBR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PBR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PBR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PBR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PBR_A::_11
    }
}
#[doc = "Write proxy for field `PBR`"]
pub struct PBR_W<'a> {
    w: &'a mut W,
}
impl<'a> PBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Baud Rate Prescaler value is 2."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PBR_A::_00)
    }
    #[doc = "Baud Rate Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PBR_A::_01)
    }
    #[doc = "Baud Rate Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PBR_A::_10)
    }
    #[doc = "Baud Rate Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PBR_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Delay after Transfer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDT_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11,
}
impl From<PDT_A> for u8 {
    #[inline(always)]
    fn from(variant: PDT_A) -> Self {
        match variant {
            PDT_A::_00 => 0,
            PDT_A::_01 => 1,
            PDT_A::_10 => 2,
            PDT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PDT`"]
pub type PDT_R = crate::R<u8, PDT_A>;
impl PDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDT_A {
        match self.bits {
            0 => PDT_A::_00,
            1 => PDT_A::_01,
            2 => PDT_A::_10,
            3 => PDT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PDT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PDT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PDT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PDT_A::_11
    }
}
#[doc = "Write proxy for field `PDT`"]
pub struct PDT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PDT_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PDT_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PDT_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PDT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "After SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PASC_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11,
}
impl From<PASC_A> for u8 {
    #[inline(always)]
    fn from(variant: PASC_A) -> Self {
        match variant {
            PASC_A::_00 => 0,
            PASC_A::_01 => 1,
            PASC_A::_10 => 2,
            PASC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PASC`"]
pub type PASC_R = crate::R<u8, PASC_A>;
impl PASC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PASC_A {
        match self.bits {
            0 => PASC_A::_00,
            1 => PASC_A::_01,
            2 => PASC_A::_10,
            3 => PASC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PASC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PASC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PASC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PASC_A::_11
    }
}
#[doc = "Write proxy for field `PASC`"]
pub struct PASC_W<'a> {
    w: &'a mut W,
}
impl<'a> PASC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PASC_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PASC_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PASC_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PASC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "PCS to SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSSCK_A {
    #[doc = "0: PCS to SCK Prescaler value is 1."]
    _00,
    #[doc = "1: PCS to SCK Prescaler value is 3."]
    _01,
    #[doc = "2: PCS to SCK Prescaler value is 5."]
    _10,
    #[doc = "3: PCS to SCK Prescaler value is 7."]
    _11,
}
impl From<PCSSCK_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSSCK_A) -> Self {
        match variant {
            PCSSCK_A::_00 => 0,
            PCSSCK_A::_01 => 1,
            PCSSCK_A::_10 => 2,
            PCSSCK_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PCSSCK`"]
pub type PCSSCK_R = crate::R<u8, PCSSCK_A>;
impl PCSSCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSCK_A {
        match self.bits {
            0 => PCSSCK_A::_00,
            1 => PCSSCK_A::_01,
            2 => PCSSCK_A::_10,
            3 => PCSSCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PCSSCK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PCSSCK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PCSSCK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PCSSCK_A::_11
    }
}
#[doc = "Write proxy for field `PCSSCK`"]
pub struct PCSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSSCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PCS to SCK Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSSCK_A::_00)
    }
    #[doc = "PCS to SCK Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSSCK_A::_01)
    }
    #[doc = "PCS to SCK Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSSCK_A::_10)
    }
    #[doc = "PCS to SCK Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSSCK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFE_A {
    #[doc = "0: Data is transferred MSB first."]
    _0,
    #[doc = "1: Data is transferred LSB first."]
    _1,
}
impl From<LSBFE_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFE_A) -> Self {
        match variant {
            LSBFE_A::_0 => false,
            LSBFE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LSBFE`"]
pub type LSBFE_R = crate::R<bool, LSBFE_A>;
impl LSBFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFE_A {
        match self.bits {
            false => LSBFE_A::_0,
            true => LSBFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBFE_A::_1
    }
}
#[doc = "Write proxy for field `LSBFE`"]
pub struct LSBFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is transferred MSB first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFE_A::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFE_A::_1)
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
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge."]
    _0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge."]
    _1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        match variant {
            CPHA_A::_0 => false,
            CPHA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
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
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low."]
    _0,
    #[doc = "1: The inactive state value of SCK is high."]
    _1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        match variant {
            CPOL_A::_0 => false,
            CPOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The inactive state value of SCK is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
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
#[doc = "Reader of field `FMSZ`"]
pub type FMSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMSZ`"]
pub struct FMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FMSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | (((value as u32) & 0x0f) << 27);
        self.w
    }
}
#[doc = "Double Baud Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBR_A {
    #[doc = "0: The baud rate is computed normally with a 50/50 duty cycle."]
    _0,
    #[doc = "1: The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    _1,
}
impl From<DBR_A> for bool {
    #[inline(always)]
    fn from(variant: DBR_A) -> Self {
        match variant {
            DBR_A::_0 => false,
            DBR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DBR`"]
pub type DBR_R = crate::R<bool, DBR_A>;
impl DBR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBR_A {
        match self.bits {
            false => DBR_A::_0,
            true => DBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBR_A::_1
    }
}
#[doc = "Write proxy for field `DBR`"]
pub struct DBR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBR_A::_0)
    }
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBR_A::_1)
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
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&self) -> PBR_R {
        PBR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&self) -> PDT_R {
        PDT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&self) -> PASC_R {
        PASC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&self) -> FMSZ_R {
        FMSZ_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&self) -> DBR_R {
        DBR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CSSCK_W {
        CSSCK_W { w: self }
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&mut self) -> PBR_W {
        PBR_W { w: self }
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&mut self) -> PDT_W {
        PDT_W { w: self }
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&mut self) -> PASC_W {
        PASC_W { w: self }
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&mut self) -> PCSSCK_W {
        PCSSCK_W { w: self }
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    pub fn lsbfe(&mut self) -> LSBFE_W {
        LSBFE_W { w: self }
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&mut self) -> FMSZ_W {
        FMSZ_W { w: self }
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&mut self) -> DBR_W {
        DBR_W { w: self }
    }
}
