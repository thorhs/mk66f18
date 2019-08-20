#[doc = "Reader of register C0"]
pub type R = crate::R<u8, super::C0>;
#[doc = "Writer for register C0"]
pub type W = crate::W<u8, super::C0>;
#[doc = "Register C0 `reset()`'s with value 0"]
impl crate::ResetValue for super::C0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC Buffer Read Pointer Bottom Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBBIEN_A {
    #[doc = "0: The DAC buffer read pointer bottom flag interrupt is disabled."]
    _0,
    #[doc = "1: The DAC buffer read pointer bottom flag interrupt is enabled."]
    _1,
}
impl From<DACBBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBBIEN_A) -> Self {
        match variant {
            DACBBIEN_A::_0 => false,
            DACBBIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBBIEN`"]
pub type DACBBIEN_R = crate::R<bool, DACBBIEN_A>;
impl DACBBIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBBIEN_A {
        match self.bits {
            false => DACBBIEN_A::_0,
            true => DACBBIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBBIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBBIEN_A::_1
    }
}
#[doc = "Write proxy for field `DACBBIEN`"]
pub struct DACBBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBBIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "DAC Buffer Read Pointer Top Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBTIEN_A {
    #[doc = "0: The DAC buffer read pointer top flag interrupt is disabled."]
    _0,
    #[doc = "1: The DAC buffer read pointer top flag interrupt is enabled."]
    _1,
}
impl From<DACBTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBTIEN_A) -> Self {
        match variant {
            DACBTIEN_A::_0 => false,
            DACBTIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBTIEN`"]
pub type DACBTIEN_R = crate::R<bool, DACBTIEN_A>;
impl DACBTIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBTIEN_A {
        match self.bits {
            false => DACBTIEN_A::_0,
            true => DACBTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBTIEN_A::_1
    }
}
#[doc = "Write proxy for field `DACBTIEN`"]
pub struct DACBTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBTIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "DAC Buffer Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBWIEN_A {
    #[doc = "0: The DAC buffer watermark interrupt is disabled."]
    _0,
    #[doc = "1: The DAC buffer watermark interrupt is enabled."]
    _1,
}
impl From<DACBWIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBWIEN_A) -> Self {
        match variant {
            DACBWIEN_A::_0 => false,
            DACBWIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBWIEN`"]
pub type DACBWIEN_R = crate::R<bool, DACBWIEN_A>;
impl DACBWIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBWIEN_A {
        match self.bits {
            false => DACBWIEN_A::_0,
            true => DACBWIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBWIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBWIEN_A::_1
    }
}
#[doc = "Write proxy for field `DACBWIEN`"]
pub struct DACBWIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBWIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBWIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer watermark interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBWIEN_A::_0)
    }
    #[doc = "The DAC buffer watermark interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBWIEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "DAC Low Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN_A {
    #[doc = "0: High-Power mode"]
    _0,
    #[doc = "1: Low-Power mode"]
    _1,
}
impl From<LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPEN_A) -> Self {
        match variant {
            LPEN_A::_0 => false,
            LPEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LPEN`"]
pub type LPEN_R = crate::R<bool, LPEN_A>;
impl LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPEN_A {
        match self.bits {
            false => LPEN_A::_0,
            true => LPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPEN_A::_1
    }
}
#[doc = "Write proxy for field `LPEN`"]
pub struct LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High-Power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPEN_A::_0)
    }
    #[doc = "Low-Power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "DAC Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACSWTRG_AW {
    #[doc = "0: The DAC soft trigger is not valid."]
    _0,
    #[doc = "1: The DAC soft trigger is valid."]
    _1,
}
impl From<DACSWTRG_AW> for bool {
    #[inline(always)]
    fn from(variant: DACSWTRG_AW) -> Self {
        match variant {
            DACSWTRG_AW::_0 => false,
            DACSWTRG_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `DACSWTRG`"]
pub struct DACSWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSWTRG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC soft trigger is not valid."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_0)
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "DAC Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACTRGSEL_A {
    #[doc = "0: The DAC hardware trigger is selected."]
    _0,
    #[doc = "1: The DAC software trigger is selected."]
    _1,
}
impl From<DACTRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DACTRGSEL_A) -> Self {
        match variant {
            DACTRGSEL_A::_0 => false,
            DACTRGSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACTRGSEL`"]
pub type DACTRGSEL_R = crate::R<bool, DACTRGSEL_A>;
impl DACTRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACTRGSEL_A {
        match self.bits {
            false => DACTRGSEL_A::_0,
            true => DACTRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACTRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACTRGSEL_A::_1
    }
}
#[doc = "Write proxy for field `DACTRGSEL`"]
pub struct DACTRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACTRGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC hardware trigger is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_0)
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRFS_A {
    #[doc = "0: The DAC selects DACREF_1 as the reference voltage."]
    _0,
    #[doc = "1: The DAC selects DACREF_2 as the reference voltage."]
    _1,
}
impl From<DACRFS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRFS_A) -> Self {
        match variant {
            DACRFS_A::_0 => false,
            DACRFS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACRFS`"]
pub type DACRFS_R = crate::R<bool, DACRFS_A>;
impl DACRFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRFS_A {
        match self.bits {
            false => DACRFS_A::_0,
            true => DACRFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACRFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACRFS_A::_1
    }
}
#[doc = "Write proxy for field `DACRFS`"]
pub struct DACRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACRFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACRFS_A::_0)
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACRFS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: The DAC system is disabled."]
    _0,
    #[doc = "1: The DAC system is enabled."]
    _1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        match variant {
            DACEN_A::_0 => false,
            DACEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACEN_A::_1
    }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC system is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "The DAC system is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&self) -> DACBBIEN_R {
        DACBBIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&self) -> DACBTIEN_R {
        DACBTIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn dacbwien(&self) -> DACBWIEN_R {
        DACBWIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&self) -> DACTRGSEL_R {
        DACTRGSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&self) -> DACRFS_R {
        DACRFS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&mut self) -> DACBBIEN_W {
        DACBBIEN_W { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&mut self) -> DACBTIEN_W {
        DACBTIEN_W { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn dacbwien(&mut self) -> DACBWIEN_W {
        DACBWIEN_W { w: self }
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LPEN_W {
        LPEN_W { w: self }
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline(always)]
    pub fn dacswtrg(&mut self) -> DACSWTRG_W {
        DACSWTRG_W { w: self }
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&mut self) -> DACTRGSEL_W {
        DACTRGSEL_W { w: self }
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&mut self) -> DACRFS_W {
        DACRFS_W { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
}
