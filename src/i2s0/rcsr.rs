#[doc = "Reader of register RCSR"]
pub type R = crate::R<u32, super::RCSR>;
#[doc = "Writer for register RCSR"]
pub type W = crate::W<u32, super::RCSR>;
#[doc = "Register RCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO Request DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDE_A {
    #[doc = "0: Disables the DMA request."]
    _0,
    #[doc = "1: Enables the DMA request."]
    _1,
}
impl From<FRDE_A> for bool {
    #[inline(always)]
    fn from(variant: FRDE_A) -> Self {
        match variant {
            FRDE_A::_0 => false,
            FRDE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRDE`"]
pub type FRDE_R = crate::R<bool, FRDE_A>;
impl FRDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDE_A {
        match self.bits {
            false => FRDE_A::_0,
            true => FRDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDE_A::_1
    }
}
#[doc = "Write proxy for field `FRDE`"]
pub struct FRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDE_A::_1)
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
#[doc = "FIFO Warning DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWDE_A {
    #[doc = "0: Disables the DMA request."]
    _0,
    #[doc = "1: Enables the DMA request."]
    _1,
}
impl From<FWDE_A> for bool {
    #[inline(always)]
    fn from(variant: FWDE_A) -> Self {
        match variant {
            FWDE_A::_0 => false,
            FWDE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FWDE`"]
pub type FWDE_R = crate::R<bool, FWDE_A>;
impl FWDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDE_A {
        match self.bits {
            false => FWDE_A::_0,
            true => FWDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWDE_A::_1
    }
}
#[doc = "Write proxy for field `FWDE`"]
pub struct FWDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWDE_A::_1)
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
#[doc = "FIFO Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIE_A {
    #[doc = "0: Disables the interrupt."]
    _0,
    #[doc = "1: Enables the interrupt."]
    _1,
}
impl From<FRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRIE_A) -> Self {
        match variant {
            FRIE_A::_0 => false,
            FRIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRIE`"]
pub type FRIE_R = crate::R<bool, FRIE_A>;
impl FRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIE_A {
        match self.bits {
            false => FRIE_A::_0,
            true => FRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRIE_A::_1
    }
}
#[doc = "Write proxy for field `FRIE`"]
pub struct FRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIE_A::_1)
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
#[doc = "FIFO Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWIE_A {
    #[doc = "0: Disables the interrupt."]
    _0,
    #[doc = "1: Enables the interrupt."]
    _1,
}
impl From<FWIE_A> for bool {
    #[inline(always)]
    fn from(variant: FWIE_A) -> Self {
        match variant {
            FWIE_A::_0 => false,
            FWIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FWIE`"]
pub type FWIE_R = crate::R<bool, FWIE_A>;
impl FWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWIE_A {
        match self.bits {
            false => FWIE_A::_0,
            true => FWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWIE_A::_1
    }
}
#[doc = "Write proxy for field `FWIE`"]
pub struct FWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Disables the interrupt."]
    _0,
    #[doc = "1: Enables the interrupt."]
    _1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        match variant {
            FEIE_A::_0 => false,
            FEIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Sync Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIE_A {
    #[doc = "0: Disables interrupt."]
    _0,
    #[doc = "1: Enables interrupt."]
    _1,
}
impl From<SEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        match variant {
            SEIE_A::_0 => false,
            SEIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SEIE`"]
pub type SEIE_R = crate::R<bool, SEIE_A>;
impl SEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIE_A {
        match self.bits {
            false => SEIE_A::_0,
            true => SEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEIE_A::_1
    }
}
#[doc = "Write proxy for field `SEIE`"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Word Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSIE_A {
    #[doc = "0: Disables interrupt."]
    _0,
    #[doc = "1: Enables interrupt."]
    _1,
}
impl From<WSIE_A> for bool {
    #[inline(always)]
    fn from(variant: WSIE_A) -> Self {
        match variant {
            WSIE_A::_0 => false,
            WSIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WSIE`"]
pub type WSIE_R = crate::R<bool, WSIE_A>;
impl WSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSIE_A {
        match self.bits {
            false => WSIE_A::_0,
            true => WSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WSIE_A::_1
    }
}
#[doc = "Write proxy for field `WSIE`"]
pub struct WSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSIE_A::_1)
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
#[doc = "FIFO Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: Receive FIFO watermark not reached."]
    _0,
    #[doc = "1: Receive FIFO watermark has been reached."]
    _1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        match variant {
            FRF_A::_0 => false,
            FRF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<bool, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::_0,
            true => FRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRF_A::_1
    }
}
#[doc = "FIFO Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWF_A {
    #[doc = "0: No enabled receive FIFO is full."]
    _0,
    #[doc = "1: Enabled receive FIFO is full."]
    _1,
}
impl From<FWF_A> for bool {
    #[inline(always)]
    fn from(variant: FWF_A) -> Self {
        match variant {
            FWF_A::_0 => false,
            FWF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FWF`"]
pub type FWF_R = crate::R<bool, FWF_A>;
impl FWF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWF_A {
        match self.bits {
            false => FWF_A::_0,
            true => FWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWF_A::_1
    }
}
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: Receive overflow not detected."]
    _0,
    #[doc = "1: Receive overflow detected."]
    _1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        match variant {
            FEF_A::_0 => false,
            FEF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, FEF_A>;
impl FEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive overflow not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Receive overflow detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
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
#[doc = "Sync Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEF_A {
    #[doc = "0: Sync error not detected."]
    _0,
    #[doc = "1: Frame sync error detected."]
    _1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        match variant {
            SEF_A::_0 => false,
            SEF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SEF`"]
pub type SEF_R = crate::R<bool, SEF_A>;
impl SEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::_0,
            true => SEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEF_A::_1
    }
}
#[doc = "Write proxy for field `SEF`"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEF_A::_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEF_A::_1)
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
#[doc = "Word Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSF_A {
    #[doc = "0: Start of word not detected."]
    _0,
    #[doc = "1: Start of word detected."]
    _1,
}
impl From<WSF_A> for bool {
    #[inline(always)]
    fn from(variant: WSF_A) -> Self {
        match variant {
            WSF_A::_0 => false,
            WSF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WSF`"]
pub type WSF_R = crate::R<bool, WSF_A>;
impl WSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSF_A {
        match self.bits {
            false => WSF_A::_0,
            true => WSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WSF_A::_1
    }
}
#[doc = "Write proxy for field `WSF`"]
pub struct WSF_W<'a> {
    w: &'a mut W,
}
impl<'a> WSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSF_A::_0)
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSF_A::_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Software reset."]
    _1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        match variant {
            SR_A::_0 => false,
            SR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, SR_A>;
impl SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::_0,
            true => SR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SR_A::_1
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SR_A::_0)
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SR_A::_1)
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
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FR_AW {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: FIFO reset."]
    _1,
}
impl From<FR_AW> for bool {
    #[inline(always)]
    fn from(variant: FR_AW) -> Self {
        match variant {
            FR_AW::_0 => false,
            FR_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `FR`"]
pub struct FR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FR_AW::_0)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FR_AW::_1)
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
#[doc = "Bit Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCE_A {
    #[doc = "0: Receive bit clock is disabled."]
    _0,
    #[doc = "1: Receive bit clock is enabled."]
    _1,
}
impl From<BCE_A> for bool {
    #[inline(always)]
    fn from(variant: BCE_A) -> Self {
        match variant {
            BCE_A::_0 => false,
            BCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BCE`"]
pub type BCE_R = crate::R<bool, BCE_A>;
impl BCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCE_A {
        match self.bits {
            false => BCE_A::_0,
            true => BCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCE_A::_1
    }
}
#[doc = "Write proxy for field `BCE`"]
pub struct BCE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive bit clock is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCE_A::_0)
    }
    #[doc = "Receive bit clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCE_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGE_A {
    #[doc = "0: Receiver is disabled in Debug mode, after completing the current frame."]
    _0,
    #[doc = "1: Receiver is enabled in Debug mode."]
    _1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        match variant {
            DBGE_A::_0 => false,
            DBGE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DBGE`"]
pub type DBGE_R = crate::R<bool, DBGE_A>;
impl DBGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::_0,
            true => DBGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGE_A::_1
    }
}
#[doc = "Write proxy for field `DBGE`"]
pub struct DBGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGE_A::_0)
    }
    #[doc = "Receiver is enabled in Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGE_A::_1)
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
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPE_A {
    #[doc = "0: Receiver disabled in Stop mode."]
    _0,
    #[doc = "1: Receiver enabled in Stop mode."]
    _1,
}
impl From<STOPE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPE_A) -> Self {
        match variant {
            STOPE_A::_0 => false,
            STOPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STOPE`"]
pub type STOPE_R = crate::R<bool, STOPE_A>;
impl STOPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPE_A {
        match self.bits {
            false => STOPE_A::_0,
            true => STOPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPE_A::_1
    }
}
#[doc = "Write proxy for field `STOPE`"]
pub struct STOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPE_A::_0)
    }
    #[doc = "Receiver enabled in Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled."]
    _0,
    #[doc = "1: Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    _1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        match variant {
            RE_A::_0 => false,
            RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
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
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&self) -> FRDE_R {
        FRDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&self) -> FWDE_R {
        FWDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&self) -> FRIE_R {
        FRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&self) -> FWIE_R {
        FWIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&self) -> WSIE_R {
        WSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline(always)]
    pub fn fwf(&self) -> FWF_R {
        FWF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&self) -> WSF_R {
        WSF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&self) -> BCE_R {
        BCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&self) -> STOPE_R {
        STOPE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&mut self) -> FRDE_W {
        FRDE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&mut self) -> FWDE_W {
        FWDE_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&mut self) -> FRIE_W {
        FRIE_W { w: self }
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&mut self) -> FWIE_W {
        FWIE_W { w: self }
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&mut self) -> WSIE_W {
        WSIE_W { w: self }
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&mut self) -> WSF_W {
        WSF_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W {
        FR_W { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&mut self) -> BCE_W {
        BCE_W { w: self }
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&mut self) -> DBGE_W {
        DBGE_W { w: self }
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&mut self) -> STOPE_W {
        STOPE_W { w: self }
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
}
