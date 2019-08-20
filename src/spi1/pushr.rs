#[doc = "Reader of register PUSHR"]
pub type R = crate::R<u32, super::PUSHR>;
#[doc = "Writer for register PUSHR"]
pub type W = crate::W<u32, super::PUSHR>;
#[doc = "Register PUSHR `reset()`'s with value 0"]
impl crate::ResetValue for super::PUSHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS0_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS0_A> for bool {
    #[inline(always)]
    fn from(variant: PCS0_A) -> Self {
        match variant {
            PCS0_A::_0 => false,
            PCS0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS0`"]
pub type PCS0_R = crate::R<bool, PCS0_A>;
impl PCS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS0_A {
        match self.bits {
            false => PCS0_A::_0,
            true => PCS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS0_A::_1
    }
}
#[doc = "Write proxy for field `PCS0`"]
pub struct PCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS0_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS1_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS1_A> for bool {
    #[inline(always)]
    fn from(variant: PCS1_A) -> Self {
        match variant {
            PCS1_A::_0 => false,
            PCS1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS1`"]
pub type PCS1_R = crate::R<bool, PCS1_A>;
impl PCS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS1_A {
        match self.bits {
            false => PCS1_A::_0,
            true => PCS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS1_A::_1
    }
}
#[doc = "Write proxy for field `PCS1`"]
pub struct PCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS1_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS1_A::_1)
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
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS2_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS2_A> for bool {
    #[inline(always)]
    fn from(variant: PCS2_A) -> Self {
        match variant {
            PCS2_A::_0 => false,
            PCS2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS2`"]
pub type PCS2_R = crate::R<bool, PCS2_A>;
impl PCS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS2_A {
        match self.bits {
            false => PCS2_A::_0,
            true => PCS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS2_A::_1
    }
}
#[doc = "Write proxy for field `PCS2`"]
pub struct PCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS2_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS2_A::_1)
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
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS3_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS3_A> for bool {
    #[inline(always)]
    fn from(variant: PCS3_A) -> Self {
        match variant {
            PCS3_A::_0 => false,
            PCS3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS3`"]
pub type PCS3_R = crate::R<bool, PCS3_A>;
impl PCS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS3_A {
        match self.bits {
            false => PCS3_A::_0,
            true => PCS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS3_A::_1
    }
}
#[doc = "Write proxy for field `PCS3`"]
pub struct PCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS3_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS3_A::_1)
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
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS4_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS4_A> for bool {
    #[inline(always)]
    fn from(variant: PCS4_A) -> Self {
        match variant {
            PCS4_A::_0 => false,
            PCS4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS4`"]
pub type PCS4_R = crate::R<bool, PCS4_A>;
impl PCS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS4_A {
        match self.bits {
            false => PCS4_A::_0,
            true => PCS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS4_A::_1
    }
}
#[doc = "Write proxy for field `PCS4`"]
pub struct PCS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS4_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS4_A::_1)
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
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS5_A {
    #[doc = "0: Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "1: Assert the PCS\\[x\\] signal."]
    _1,
}
impl From<PCS5_A> for bool {
    #[inline(always)]
    fn from(variant: PCS5_A) -> Self {
        match variant {
            PCS5_A::_0 => false,
            PCS5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCS5`"]
pub type PCS5_R = crate::R<bool, PCS5_A>;
impl PCS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS5_A {
        match self.bits {
            false => PCS5_A::_0,
            true => PCS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS5_A::_1
    }
}
#[doc = "Write proxy for field `PCS5`"]
pub struct PCS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS5_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS5_A::_1)
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
#[doc = "Clear Transfer Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCNT_A {
    #[doc = "0: Do not clear the TCR\\[TCNT\\] field."]
    _0,
    #[doc = "1: Clear the TCR\\[TCNT\\] field."]
    _1,
}
impl From<CTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: CTCNT_A) -> Self {
        match variant {
            CTCNT_A::_0 => false,
            CTCNT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CTCNT`"]
pub type CTCNT_R = crate::R<bool, CTCNT_A>;
impl CTCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCNT_A {
        match self.bits {
            false => CTCNT_A::_0,
            true => CTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTCNT_A::_1
    }
}
#[doc = "Write proxy for field `CTCNT`"]
pub struct CTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear the TCR\\[TCNT\\] field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTCNT_A::_0)
    }
    #[doc = "Clear the TCR\\[TCNT\\] field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTCNT_A::_1)
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
#[doc = "End Of Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQ_A {
    #[doc = "0: The SPI data is not the last data to transfer."]
    _0,
    #[doc = "1: The SPI data is the last data to transfer."]
    _1,
}
impl From<EOQ_A> for bool {
    #[inline(always)]
    fn from(variant: EOQ_A) -> Self {
        match variant {
            EOQ_A::_0 => false,
            EOQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOQ`"]
pub type EOQ_R = crate::R<bool, EOQ_A>;
impl EOQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQ_A {
        match self.bits {
            false => EOQ_A::_0,
            true => EOQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOQ_A::_1
    }
}
#[doc = "Write proxy for field `EOQ`"]
pub struct EOQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EOQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SPI data is not the last data to transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQ_A::_0)
    }
    #[doc = "The SPI data is the last data to transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQ_A::_1)
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
#[doc = "Clock and Transfer Attributes Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTAS_A {
    #[doc = "0: CTAR0"]
    _000,
    #[doc = "1: CTAR1"]
    _001,
}
impl From<CTAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CTAS_A) -> Self {
        match variant {
            CTAS_A::_000 => 0,
            CTAS_A::_001 => 1,
        }
    }
}
#[doc = "Reader of field `CTAS`"]
pub type CTAS_R = crate::R<u8, CTAS_A>;
impl CTAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTAS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTAS_A::_000),
            1 => Val(CTAS_A::_001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTAS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTAS_A::_001
    }
}
#[doc = "Write proxy for field `CTAS`"]
pub struct CTAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CTAR0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CTAS_A::_000)
    }
    #[doc = "CTAR1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CTAS_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Continuous Peripheral Chip Select Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Return PCSn signals to their inactive state between transfers."]
    _0,
    #[doc = "1: Keep PCSn signals asserted between transfers."]
    _1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        match variant {
            CONT_A::_0 => false,
            CONT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, CONT_A>;
impl CONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::_0,
            true => CONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_A::_1
    }
}
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_A::_0)
    }
    #[doc = "Keep PCSn signals asserted between transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_A::_1)
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs0(&self) -> PCS0_R {
        PCS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs1(&self) -> PCS1_R {
        PCS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs2(&self) -> PCS2_R {
        PCS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs3(&self) -> PCS3_R {
        PCS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs4(&self) -> PCS4_R {
        PCS4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs5(&self) -> PCS5_R {
        PCS5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    pub fn ctcnt(&self) -> CTCNT_R {
        CTCNT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&self) -> EOQ_R {
        EOQ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    pub fn ctas(&self) -> CTAS_R {
        CTAS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Bit 16 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs0(&mut self) -> PCS0_W {
        PCS0_W { w: self }
    }
    #[doc = "Bit 17 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs1(&mut self) -> PCS1_W {
        PCS1_W { w: self }
    }
    #[doc = "Bit 18 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs2(&mut self) -> PCS2_W {
        PCS2_W { w: self }
    }
    #[doc = "Bit 19 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs3(&mut self) -> PCS3_W {
        PCS3_W { w: self }
    }
    #[doc = "Bit 20 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs4(&mut self) -> PCS4_W {
        PCS4_W { w: self }
    }
    #[doc = "Bit 21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs5(&mut self) -> PCS5_W {
        PCS5_W { w: self }
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    pub fn ctcnt(&mut self) -> CTCNT_W {
        CTCNT_W { w: self }
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&mut self) -> EOQ_W {
        EOQ_W { w: self }
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    pub fn ctas(&mut self) -> CTAS_W {
        CTAS_W { w: self }
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
}
