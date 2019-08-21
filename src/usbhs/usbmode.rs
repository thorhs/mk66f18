#[doc = "Reader of register USBMODE"]
pub type R = crate::R<u32, super::USBMODE>;
#[doc = "Writer for register USBMODE"]
pub type W = crate::W<u32, super::USBMODE>;
#[doc = "Register USBMODE `reset()`'s with value 0x5000"]
impl crate::ResetValue for super::USBMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000
    }
}
#[doc = "Controller Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM_A {
    #[doc = "0: Idle (default for the USBHS module)"]
    _00,
    #[doc = "2: Device controller"]
    _10,
    #[doc = "3: Host controller"]
    _11,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        match variant {
            CM_A::_00 => 0,
            CM_A::_10 => 2,
            CM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM_A::_00),
            2 => Val(CM_A::_10),
            3 => Val(CM_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CM_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CM_A::_11
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Idle (default for the USBHS module)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CM_A::_00)
    }
    #[doc = "Device controller"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CM_A::_10)
    }
    #[doc = "Host controller"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Endian Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ES_A {
    #[doc = "0: Little endian. First byte referenced in least significant byte of 32-bit word."]
    _0,
    #[doc = "1: Big endian. First byte referenced in most significant byte of 32-bit word."]
    _1,
}
impl From<ES_A> for bool {
    #[inline(always)]
    fn from(variant: ES_A) -> Self {
        match variant {
            ES_A::_0 => false,
            ES_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ES`"]
pub type ES_R = crate::R<bool, ES_A>;
impl ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ES_A {
        match self.bits {
            false => ES_A::_0,
            true => ES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ES_A::_1
    }
}
#[doc = "Write proxy for field `ES`"]
pub struct ES_W<'a> {
    w: &'a mut W,
}
impl<'a> ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little endian. First byte referenced in least significant byte of 32-bit word."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ES_A::_0)
    }
    #[doc = "Big endian. First byte referenced in most significant byte of 32-bit word."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ES_A::_1)
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
#[doc = "Reader of field `SLOM`"]
pub type SLOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOM`"]
pub struct SLOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOM_W<'a> {
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
#[doc = "Stream DISable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIS_A {
    #[doc = "0: Inactive"]
    _0,
    #[doc = "1: Active"]
    _1,
}
impl From<SDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SDIS_A) -> Self {
        match variant {
            SDIS_A::_0 => false,
            SDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SDIS`"]
pub type SDIS_R = crate::R<bool, SDIS_A>;
impl SDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIS_A {
        match self.bits {
            false => SDIS_A::_0,
            true => SDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDIS_A::_1
    }
}
#[doc = "Write proxy for field `SDIS`"]
pub struct SDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDIS_A::_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDIS_A::_1)
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
#[doc = "Tx to Tx HS Delay\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXHSD_A {
    #[doc = "0: 10"]
    _000,
    #[doc = "1: 11"]
    _001,
    #[doc = "2: 12"]
    _010,
    #[doc = "3: 13"]
    _011,
    #[doc = "4: 14"]
    _100,
    #[doc = "5: 15"]
    _101,
    #[doc = "6: 16"]
    _110,
    #[doc = "7: 17"]
    _111,
}
impl From<TXHSD_A> for u8 {
    #[inline(always)]
    fn from(variant: TXHSD_A) -> Self {
        match variant {
            TXHSD_A::_000 => 0,
            TXHSD_A::_001 => 1,
            TXHSD_A::_010 => 2,
            TXHSD_A::_011 => 3,
            TXHSD_A::_100 => 4,
            TXHSD_A::_101 => 5,
            TXHSD_A::_110 => 6,
            TXHSD_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `TXHSD`"]
pub type TXHSD_R = crate::R<u8, TXHSD_A>;
impl TXHSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXHSD_A {
        match self.bits {
            0 => TXHSD_A::_000,
            1 => TXHSD_A::_001,
            2 => TXHSD_A::_010,
            3 => TXHSD_A::_011,
            4 => TXHSD_A::_100,
            5 => TXHSD_A::_101,
            6 => TXHSD_A::_110,
            7 => TXHSD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TXHSD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TXHSD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TXHSD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TXHSD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TXHSD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TXHSD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TXHSD_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TXHSD_A::_111
    }
}
#[doc = "Write proxy for field `TXHSD`"]
pub struct TXHSD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXHSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXHSD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "10"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TXHSD_A::_000)
    }
    #[doc = "11"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TXHSD_A::_001)
    }
    #[doc = "12"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(TXHSD_A::_010)
    }
    #[doc = "13"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TXHSD_A::_011)
    }
    #[doc = "14"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TXHSD_A::_100)
    }
    #[doc = "15"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TXHSD_A::_101)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TXHSD_A::_110)
    }
    #[doc = "17"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(TXHSD_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Controller Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Endian Select"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Setup Lock-Out Mode"]
    #[inline(always)]
    pub fn slom(&self) -> SLOM_R {
        SLOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream DISable"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Tx to Tx HS Delay"]
    #[inline(always)]
    pub fn txhsd(&self) -> TXHSD_R {
        TXHSD_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controller Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 2 - Endian Select"]
    #[inline(always)]
    pub fn es(&mut self) -> ES_W {
        ES_W { w: self }
    }
    #[doc = "Bit 3 - Setup Lock-Out Mode"]
    #[inline(always)]
    pub fn slom(&mut self) -> SLOM_W {
        SLOM_W { w: self }
    }
    #[doc = "Bit 4 - Stream DISable"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W {
        SDIS_W { w: self }
    }
    #[doc = "Bits 12:14 - Tx to Tx HS Delay"]
    #[inline(always)]
    pub fn txhsd(&mut self) -> TXHSD_W {
        TXHSD_W { w: self }
    }
}
