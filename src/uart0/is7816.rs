#[doc = "Reader of register IS7816"]
pub type R = crate::R<u8, super::IS7816>;
#[doc = "Writer for register IS7816"]
pub type W = crate::W<u8, super::IS7816>;
#[doc = "Register IS7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::IS7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXT_A {
    #[doc = "0: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    _0,
    #[doc = "1: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    _1,
}
impl From<RXT_A> for bool {
    #[inline(always)]
    fn from(variant: RXT_A) -> Self {
        match variant {
            RXT_A::_0 => false,
            RXT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXT`"]
pub type RXT_R = crate::R<bool, RXT_A>;
impl RXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXT_A {
        match self.bits {
            false => RXT_A::_0,
            true => RXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXT_A::_1
    }
}
#[doc = "Write proxy for field `RXT`"]
pub struct RXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXT_A::_0)
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXT_A::_1)
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
#[doc = "Transmit Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXT_A {
    #[doc = "0: The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    _0,
    #[doc = "1: The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    _1,
}
impl From<TXT_A> for bool {
    #[inline(always)]
    fn from(variant: TXT_A) -> Self {
        match variant {
            TXT_A::_0 => false,
            TXT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXT`"]
pub type TXT_R = crate::R<bool, TXT_A>;
impl TXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXT_A {
        match self.bits {
            false => TXT_A::_0,
            true => TXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXT_A::_1
    }
}
#[doc = "Write proxy for field `TXT`"]
pub struct TXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXT_A::_0)
    }
    #[doc = "The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXT_A::_1)
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
#[doc = "Guard Timer Violated Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTV_A {
    #[doc = "0: A guard time (GT, CGT, or BGT) has not been violated."]
    _0,
    #[doc = "1: A guard time (GT, CGT, or BGT) has been violated."]
    _1,
}
impl From<GTV_A> for bool {
    #[inline(always)]
    fn from(variant: GTV_A) -> Self {
        match variant {
            GTV_A::_0 => false,
            GTV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GTV`"]
pub type GTV_R = crate::R<bool, GTV_A>;
impl GTV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTV_A {
        match self.bits {
            false => GTV_A::_0,
            true => GTV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTV_A::_1
    }
}
#[doc = "Write proxy for field `GTV`"]
pub struct GTV_W<'a> {
    w: &'a mut W,
}
impl<'a> GTV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A guard time (GT, CGT, or BGT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTV_A::_0)
    }
    #[doc = "A guard time (GT, CGT, or BGT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTV_A::_1)
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
#[doc = "ATR Duration Time Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADT_A {
    #[doc = "0: ATR Duration time (ADT) has not been violated."]
    _0,
    #[doc = "1: ATR Duration time (ADT) has been violated."]
    _1,
}
impl From<ADT_A> for bool {
    #[inline(always)]
    fn from(variant: ADT_A) -> Self {
        match variant {
            ADT_A::_0 => false,
            ADT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADT`"]
pub type ADT_R = crate::R<bool, ADT_A>;
impl ADT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADT_A {
        match self.bits {
            false => ADT_A::_0,
            true => ADT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADT_A::_1
    }
}
#[doc = "Write proxy for field `ADT`"]
pub struct ADT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ATR Duration time (ADT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADT_A::_0)
    }
    #[doc = "ATR Duration time (ADT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADT_A::_1)
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
#[doc = "Initial Character Detected Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITD_A {
    #[doc = "0: A valid initial character has not been received."]
    _0,
    #[doc = "1: A valid initial character has been received."]
    _1,
}
impl From<INITD_A> for bool {
    #[inline(always)]
    fn from(variant: INITD_A) -> Self {
        match variant {
            INITD_A::_0 => false,
            INITD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INITD`"]
pub type INITD_R = crate::R<bool, INITD_A>;
impl INITD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITD_A {
        match self.bits {
            false => INITD_A::_0,
            true => INITD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITD_A::_1
    }
}
#[doc = "Write proxy for field `INITD`"]
pub struct INITD_W<'a> {
    w: &'a mut W,
}
impl<'a> INITD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A valid initial character has not been received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITD_A::_0)
    }
    #[doc = "A valid initial character has been received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITD_A::_1)
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
#[doc = "Block Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_A {
    #[doc = "0: Block wait time (BWT) has not been violated."]
    _0,
    #[doc = "1: Block wait time (BWT) has been violated."]
    _1,
}
impl From<BWT_A> for bool {
    #[inline(always)]
    fn from(variant: BWT_A) -> Self {
        match variant {
            BWT_A::_0 => false,
            BWT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BWT`"]
pub type BWT_R = crate::R<bool, BWT_A>;
impl BWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWT_A {
        match self.bits {
            false => BWT_A::_0,
            true => BWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWT_A::_1
    }
}
#[doc = "Write proxy for field `BWT`"]
pub struct BWT_W<'a> {
    w: &'a mut W,
}
impl<'a> BWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Block wait time (BWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_A::_0)
    }
    #[doc = "Block wait time (BWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_A::_1)
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
#[doc = "Character Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_A {
    #[doc = "0: Character wait time (CWT) has not been violated."]
    _0,
    #[doc = "1: Character wait time (CWT) has been violated."]
    _1,
}
impl From<CWT_A> for bool {
    #[inline(always)]
    fn from(variant: CWT_A) -> Self {
        match variant {
            CWT_A::_0 => false,
            CWT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CWT`"]
pub type CWT_R = crate::R<bool, CWT_A>;
impl CWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWT_A {
        match self.bits {
            false => CWT_A::_0,
            true => CWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWT_A::_1
    }
}
#[doc = "Write proxy for field `CWT`"]
pub struct CWT_W<'a> {
    w: &'a mut W,
}
impl<'a> CWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Character wait time (CWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_A::_0)
    }
    #[doc = "Character wait time (CWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_A::_1)
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
#[doc = "Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WT_A {
    #[doc = "0: Wait time (WT) has not been violated."]
    _0,
    #[doc = "1: Wait time (WT) has been violated."]
    _1,
}
impl From<WT_A> for bool {
    #[inline(always)]
    fn from(variant: WT_A) -> Self {
        match variant {
            WT_A::_0 => false,
            WT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<bool, WT_A>;
impl WT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WT_A {
        match self.bits {
            false => WT_A::_0,
            true => WT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WT_A::_1
    }
}
#[doc = "Write proxy for field `WT`"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wait time (WT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WT_A::_0)
    }
    #[doc = "Wait time (WT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WT_A::_1)
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
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    pub fn gtv(&self) -> GTV_R {
        GTV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ATR Duration Time Interrupt"]
    #[inline(always)]
    pub fn adt(&self) -> ADT_R {
        ADT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    pub fn bwt(&self) -> BWT_R {
        BWT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    pub fn cwt(&self) -> CWT_R {
        CWT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn rxt(&mut self) -> RXT_W {
        RXT_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn txt(&mut self) -> TXT_W {
        TXT_W { w: self }
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    pub fn gtv(&mut self) -> GTV_W {
        GTV_W { w: self }
    }
    #[doc = "Bit 3 - ATR Duration Time Interrupt"]
    #[inline(always)]
    pub fn adt(&mut self) -> ADT_W {
        ADT_W { w: self }
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    pub fn initd(&mut self) -> INITD_W {
        INITD_W { w: self }
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    pub fn bwt(&mut self) -> BWT_W {
        BWT_W { w: self }
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    pub fn cwt(&mut self) -> CWT_W {
        CWT_W { w: self }
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
}
