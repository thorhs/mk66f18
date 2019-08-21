#[doc = "Reader of register XFERTYP"]
pub type R = crate::R<u32, super::XFERTYP>;
#[doc = "Writer for register XFERTYP"]
pub type W = crate::W<u32, super::XFERTYP>;
#[doc = "Register XFERTYP `reset()`'s with value 0"]
impl crate::ResetValue for super::XFERTYP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        match variant {
            DMAEN_A::_0 => false,
            DMAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        match variant {
            BCEN_A::_0 => false,
            BCEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BCEN`"]
pub type BCEN_R = crate::R<bool, BCEN_A>;
impl BCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::_0,
            true => BCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCEN_A::_1
    }
}
#[doc = "Write proxy for field `BCEN`"]
pub struct BCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCEN_A::_1)
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
#[doc = "Auto CMD12 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EN_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<AC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EN_A) -> Self {
        match variant {
            AC12EN_A::_0 => false,
            AC12EN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12EN`"]
pub type AC12EN_R = crate::R<bool, AC12EN_A>;
impl AC12EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EN_A {
        match self.bits {
            false => AC12EN_A::_0,
            true => AC12EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EN_A::_1
    }
}
#[doc = "Write proxy for field `AC12EN`"]
pub struct AC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EN_A::_1)
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
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSEL_A {
    #[doc = "0: Write host to card."]
    _0,
    #[doc = "1: Read card to host."]
    _1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        match variant {
            DTDSEL_A::_0 => false,
            DTDSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DTDSEL`"]
pub type DTDSEL_R = crate::R<bool, DTDSEL_A>;
impl DTDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::_0,
            true => DTDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTDSEL_A::_1
    }
}
#[doc = "Write proxy for field `DTDSEL`"]
pub struct DTDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write host to card."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTDSEL_A::_0)
    }
    #[doc = "Read card to host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTDSEL_A::_1)
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
#[doc = "Multi/Single Block Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSEL_A {
    #[doc = "0: Single block."]
    _0,
    #[doc = "1: Multiple blocks."]
    _1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        match variant {
            MSBSEL_A::_0 => false,
            MSBSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MSBSEL`"]
pub type MSBSEL_R = crate::R<bool, MSBSEL_A>;
impl MSBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::_0,
            true => MSBSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSBSEL_A::_1
    }
}
#[doc = "Write proxy for field `MSBSEL`"]
pub struct MSBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single block."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBSEL_A::_0)
    }
    #[doc = "Multiple blocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTYP_A {
    #[doc = "0: No response."]
    _00,
    #[doc = "1: Response length 136."]
    _01,
    #[doc = "2: Response length 48."]
    _10,
    #[doc = "3: Response length 48, check busy after response."]
    _11,
}
impl From<RSPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYP_A) -> Self {
        match variant {
            RSPTYP_A::_00 => 0,
            RSPTYP_A::_01 => 1,
            RSPTYP_A::_10 => 2,
            RSPTYP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `RSPTYP`"]
pub type RSPTYP_R = crate::R<u8, RSPTYP_A>;
impl RSPTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTYP_A {
        match self.bits {
            0 => RSPTYP_A::_00,
            1 => RSPTYP_A::_01,
            2 => RSPTYP_A::_10,
            3 => RSPTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSPTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSPTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSPTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSPTYP_A::_11
    }
}
#[doc = "Write proxy for field `RSPTYP`"]
pub struct RSPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSPTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No response."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPTYP_A::_00)
    }
    #[doc = "Response length 136."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPTYP_A::_01)
    }
    #[doc = "Response length 48."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPTYP_A::_10)
    }
    #[doc = "Response length 48, check busy after response."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPTYP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCEN_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<CCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCCEN_A) -> Self {
        match variant {
            CCCEN_A::_0 => false,
            CCCEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCCEN`"]
pub type CCCEN_R = crate::R<bool, CCCEN_A>;
impl CCCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCEN_A {
        match self.bits {
            false => CCCEN_A::_0,
            true => CCCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCCEN_A::_1
    }
}
#[doc = "Write proxy for field `CCCEN`"]
pub struct CCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCCEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCCEN_A::_1)
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
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CICEN_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<CICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CICEN_A) -> Self {
        match variant {
            CICEN_A::_0 => false,
            CICEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CICEN`"]
pub type CICEN_R = crate::R<bool, CICEN_A>;
impl CICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CICEN_A {
        match self.bits {
            false => CICEN_A::_0,
            true => CICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CICEN_A::_1
    }
}
#[doc = "Write proxy for field `CICEN`"]
pub struct CICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CICEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CICEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSEL_A {
    #[doc = "0: No data present."]
    _0,
    #[doc = "1: Data present."]
    _1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        match variant {
            DPSEL_A::_0 => false,
            DPSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPSEL`"]
pub type DPSEL_R = crate::R<bool, DPSEL_A>;
impl DPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::_0,
            true => DPSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSEL_A::_1
    }
}
#[doc = "Write proxy for field `DPSEL`"]
pub struct DPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No data present."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSEL_A::_0)
    }
    #[doc = "Data present."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSEL_A::_1)
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
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYP_A {
    #[doc = "0: Normal other commands."]
    _00,
    #[doc = "1: Suspend CMD52 for writing bus suspend in CCCR."]
    _01,
    #[doc = "2: Resume CMD52 for writing function select in CCCR."]
    _10,
    #[doc = "3: Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    _11,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        match variant {
            CMDTYP_A::_00 => 0,
            CMDTYP_A::_01 => 1,
            CMDTYP_A::_10 => 2,
            CMDTYP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CMDTYP`"]
pub type CMDTYP_R = crate::R<u8, CMDTYP_A>;
impl CMDTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::_00,
            1 => CMDTYP_A::_01,
            2 => CMDTYP_A::_10,
            3 => CMDTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMDTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMDTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMDTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMDTYP_A::_11
    }
}
#[doc = "Write proxy for field `CMDTYP`"]
pub struct CMDTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Normal other commands."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMDTYP_A::_00)
    }
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMDTYP_A::_01)
    }
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMDTYP_A::_10)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMDTYP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CMDINX`"]
pub type CMDINX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDINX`"]
pub struct CMDINX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&self) -> AC12EN_R {
        AC12EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&self) -> RSPTYP_R {
        RSPTYP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&self) -> CCCEN_R {
        CCCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&self) -> CICEN_R {
        CICEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&self) -> CMDINX_R {
        CMDINX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&mut self) -> BCEN_W {
        BCEN_W { w: self }
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&mut self) -> AC12EN_W {
        AC12EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&mut self) -> DTDSEL_W {
        DTDSEL_W { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MSBSEL_W {
        MSBSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&mut self) -> RSPTYP_W {
        RSPTYP_W { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&mut self) -> CCCEN_W {
        CCCEN_W { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&mut self) -> CICEN_W {
        CICEN_W { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DPSEL_W {
        DPSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CMDTYP_W {
        CMDTYP_W { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&mut self) -> CMDINX_W {
        CMDINX_W { w: self }
    }
}
