#[doc = "Reader of register SC2"]
pub type R = crate::R<u32, super::SC2>;
#[doc = "Writer for register SC2"]
pub type W = crate::W<u32, super::SC2>;
#[doc = "Register SC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    _00,
    #[doc = "1: Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    _01,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        match variant {
            REFSEL_A::_00 => 0,
            REFSEL_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::_00),
            1 => Val(REFSEL_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == REFSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == REFSEL_A::_01
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(REFSEL_A::_00)
    }
    #[doc = "Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(REFSEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0,
    #[doc = "1: DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\] flags is asserted."]
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
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\] flags is asserted."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Compare Function Range Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACREN_A {
    #[doc = "0: Range function disabled. Only CV1 is compared."]
    _0,
    #[doc = "1: Range function enabled. Both CV1 and CV2 are compared."]
    _1,
}
impl From<ACREN_A> for bool {
    #[inline(always)]
    fn from(variant: ACREN_A) -> Self {
        match variant {
            ACREN_A::_0 => false,
            ACREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACREN`"]
pub type ACREN_R = crate::R<bool, ACREN_A>;
impl ACREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACREN_A {
        match self.bits {
            false => ACREN_A::_0,
            true => ACREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACREN_A::_1
    }
}
#[doc = "Write proxy for field `ACREN`"]
pub struct ACREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Range function disabled. Only CV1 is compared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACREN_A::_0)
    }
    #[doc = "Range function enabled. Both CV1 and CV2 are compared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACREN_A::_1)
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
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFGT_A {
    #[doc = "0: Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    _0,
    #[doc = "1: Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    _1,
}
impl From<ACFGT_A> for bool {
    #[inline(always)]
    fn from(variant: ACFGT_A) -> Self {
        match variant {
            ACFGT_A::_0 => false,
            ACFGT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACFGT`"]
pub type ACFGT_R = crate::R<bool, ACFGT_A>;
impl ACFGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFGT_A {
        match self.bits {
            false => ACFGT_A::_0,
            true => ACFGT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACFGT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACFGT_A::_1
    }
}
#[doc = "Write proxy for field `ACFGT`"]
pub struct ACFGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFGT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFGT_A::_0)
    }
    #[doc = "Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFGT_A::_1)
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
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled."]
    _0,
    #[doc = "1: Compare function enabled."]
    _1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        match variant {
            ACFE_A::_0 => false,
            ACFE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACFE`"]
pub type ACFE_R = crate::R<bool, ACFE_A>;
impl ACFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::_0,
            true => ACFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACFE_A::_1
    }
}
#[doc = "Write proxy for field `ACFE`"]
pub struct ACFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFE_A::_0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFE_A::_1)
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
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected."]
    _0,
    #[doc = "1: Hardware trigger selected."]
    _1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        match variant {
            ADTRG_A::_0 => false,
            ADTRG_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADTRG`"]
pub type ADTRG_R = crate::R<bool, ADTRG_A>;
impl ADTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::_0,
            true => ADTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRG_A::_1
    }
}
#[doc = "Write proxy for field `ADTRG`"]
pub struct ADTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADTRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRG_A::_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    _0,
    #[doc = "1: Conversion in progress."]
    _1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        match variant {
            ADACT_A::_0 => false,
            ADACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADACT`"]
pub type ADACT_R = crate::R<bool, ADACT_A>;
impl ADACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::_0,
            true => ADACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADACT_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> ACREN_R {
        ACREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&mut self) -> ACREN_W {
        ACREN_W { w: self }
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&mut self) -> ACFGT_W {
        ACFGT_W { w: self }
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&mut self) -> ACFE_W {
        ACFE_W { w: self }
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&mut self) -> ADTRG_W {
        ADTRG_W { w: self }
    }
}
