#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u32, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Divide by 1"]
    _000,
    #[doc = "1: Divide by 2"]
    _001,
    #[doc = "2: Divide by 4"]
    _010,
    #[doc = "3: Divide by 8"]
    _011,
    #[doc = "4: Divide by 16"]
    _100,
    #[doc = "5: Divide by 32"]
    _101,
    #[doc = "6: Divide by 64"]
    _110,
    #[doc = "7: Divide by 128"]
    _111,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::_000 => 0,
            PS_A::_001 => 1,
            PS_A::_010 => 2,
            PS_A::_011 => 3,
            PS_A::_100 => 4,
            PS_A::_101 => 5,
            PS_A::_110 => 6,
            PS_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PS_A::_111
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Clock Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOD_A {
    #[doc = "0: TPM counter is disabled"]
    _00,
    #[doc = "1: TPM counter increments on every TPM counter clock"]
    _01,
    #[doc = "2: TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    _10,
}
impl From<CMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        match variant {
            CMOD_A::_00 => 0,
            CMOD_A::_01 => 1,
            CMOD_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<u8, CMOD_A>;
impl CMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMOD_A::_00),
            1 => Val(CMOD_A::_01),
            2 => Val(CMOD_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMOD_A::_10
    }
}
#[doc = "Write proxy for field `CMOD`"]
pub struct CMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TPM counter is disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMOD_A::_00)
    }
    #[doc = "TPM counter increments on every TPM counter clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMOD_A::_01)
    }
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMOD_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMS_A {
    #[doc = "0: TPM counter operates in up counting mode."]
    _0,
    #[doc = "1: TPM counter operates in up-down counting mode."]
    _1,
}
impl From<CPWMS_A> for bool {
    #[inline(always)]
    fn from(variant: CPWMS_A) -> Self {
        match variant {
            CPWMS_A::_0 => false,
            CPWMS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CPWMS`"]
pub type CPWMS_R = crate::R<bool, CPWMS_A>;
impl CPWMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPWMS_A {
        match self.bits {
            false => CPWMS_A::_0,
            true => CPWMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPWMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPWMS_A::_1
    }
}
#[doc = "Write proxy for field `CPWMS`"]
pub struct CPWMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPWMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPWMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM counter operates in up counting mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMS_A::_0)
    }
    #[doc = "TPM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMS_A::_1)
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
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
    #[doc = "0: Disable TOF interrupts. Use software polling or DMA request."]
    _0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        match variant {
            TOIE_A::_0 => false,
            TOIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOIE`"]
pub type TOIE_R = crate::R<bool, TOIE_A>;
impl TOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::_0,
            true => TOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIE_A::_1
    }
}
#[doc = "Write proxy for field `TOIE`"]
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIE_A::_1)
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
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: TPM counter has not overflowed."]
    _0,
    #[doc = "1: TPM counter has overflowed."]
    _1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        match variant {
            TOF_A::_0 => false,
            TOF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOF`"]
pub type TOF_R = crate::R<bool, TOF_A>;
impl TOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOF_A::_1
    }
}
#[doc = "Write proxy for field `TOF`"]
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOF_A::_0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Disables DMA transfers."]
    _0,
    #[doc = "1: Enables DMA transfers."]
    _1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        match variant {
            DMA_A::_0 => false,
            DMA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables DMA transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Enables DMA transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CPWMS_R {
        CPWMS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    pub fn cmod(&mut self) -> CMOD_W {
        CMOD_W { w: self }
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&mut self) -> CPWMS_W {
        CPWMS_W { w: self }
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
}
