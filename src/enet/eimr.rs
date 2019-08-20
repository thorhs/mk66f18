#[doc = "Reader of register EIMR"]
pub type R = crate::R<u32, super::EIMR>;
#[doc = "Writer for register EIMR"]
pub type W = crate::W<u32, super::EIMR>;
#[doc = "Register EIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::EIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS_TIMER`"]
pub type TS_TIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_TIMER`"]
pub struct TS_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_TIMER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TS_AVAIL`"]
pub type TS_AVAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_AVAIL`"]
pub struct TS_AVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_AVAIL_W<'a> {
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
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP`"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
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
#[doc = "Reader of field `PLR`"]
pub type PLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLR`"]
pub struct PLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLR_W<'a> {
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
#[doc = "Reader of field `UN`"]
pub type UN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UN`"]
pub struct UN_W<'a> {
    w: &'a mut W,
}
impl<'a> UN_W<'a> {
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
#[doc = "Reader of field `RL`"]
pub type RL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RL`"]
pub struct RL_W<'a> {
    w: &'a mut W,
}
impl<'a> RL_W<'a> {
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
#[doc = "Reader of field `LC`"]
pub type LC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LC`"]
pub struct LC_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_W<'a> {
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
#[doc = "Reader of field `EBERR`"]
pub type EBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBERR`"]
pub struct EBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EBERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `MII`"]
pub type MII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MII`"]
pub struct MII_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RXB`"]
pub type RXB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXB`"]
pub struct RXB_W<'a> {
    w: &'a mut W,
}
impl<'a> RXB_W<'a> {
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
#[doc = "Reader of field `RXF`"]
pub type RXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXF`"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
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
#[doc = "TXB Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXB_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1,
}
impl From<TXB_A> for bool {
    #[inline(always)]
    fn from(variant: TXB_A) -> Self {
        match variant {
            TXB_A::_0 => false,
            TXB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXB`"]
pub type TXB_R = crate::R<bool, TXB_A>;
impl TXB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXB_A {
        match self.bits {
            false => TXB_A::_0,
            true => TXB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXB_A::_1
    }
}
#[doc = "Write proxy for field `TXB`"]
pub struct TXB_W<'a> {
    w: &'a mut W,
}
impl<'a> TXB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXB_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXB_A::_1)
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
#[doc = "TXF Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXF_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        match variant {
            TXF_A::_0 => false,
            TXF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXF`"]
pub type TXF_R = crate::R<bool, TXF_A>;
impl TXF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::_0,
            true => TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXF_A::_1
    }
}
#[doc = "Write proxy for field `TXF`"]
pub struct TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXF_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXF_A::_1)
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
#[doc = "GRA Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GRA_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1,
}
impl From<GRA_A> for bool {
    #[inline(always)]
    fn from(variant: GRA_A) -> Self {
        match variant {
            GRA_A::_0 => false,
            GRA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GRA`"]
pub type GRA_R = crate::R<bool, GRA_A>;
impl GRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRA_A {
        match self.bits {
            false => GRA_A::_0,
            true => GRA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRA_A::_1
    }
}
#[doc = "Write proxy for field `GRA`"]
pub struct GRA_W<'a> {
    w: &'a mut W,
}
impl<'a> GRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRA_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRA_A::_1)
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
#[doc = "BABT Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABT_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1,
}
impl From<BABT_A> for bool {
    #[inline(always)]
    fn from(variant: BABT_A) -> Self {
        match variant {
            BABT_A::_0 => false,
            BABT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BABT`"]
pub type BABT_R = crate::R<bool, BABT_A>;
impl BABT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABT_A {
        match self.bits {
            false => BABT_A::_0,
            true => BABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BABT_A::_1
    }
}
#[doc = "Write proxy for field `BABT`"]
pub struct BABT_W<'a> {
    w: &'a mut W,
}
impl<'a> BABT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BABT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABT_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABT_A::_1)
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
#[doc = "BABR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1,
}
impl From<BABR_A> for bool {
    #[inline(always)]
    fn from(variant: BABR_A) -> Self {
        match variant {
            BABR_A::_0 => false,
            BABR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BABR`"]
pub type BABR_R = crate::R<bool, BABR_A>;
impl BABR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABR_A {
        match self.bits {
            false => BABR_A::_0,
            true => BABR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BABR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BABR_A::_1
    }
}
#[doc = "Write proxy for field `BABR`"]
pub struct BABR_W<'a> {
    w: &'a mut W,
}
impl<'a> BABR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BABR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABR_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABR_A::_1)
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
impl R {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&mut self) -> TS_TIMER_W {
        TS_TIMER_W { w: self }
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W {
        TS_AVAIL_W { w: self }
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&mut self) -> PLR_W {
        PLR_W { w: self }
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&mut self) -> UN_W {
        UN_W { w: self }
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W { w: self }
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W {
        LC_W { w: self }
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&mut self) -> EBERR_W {
        EBERR_W { w: self }
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&mut self) -> RXB_W {
        RXB_W { w: self }
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&mut self) -> TXB_W {
        TXB_W { w: self }
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W {
        TXF_W { w: self }
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&mut self) -> GRA_W {
        GRA_W { w: self }
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&mut self) -> BABT_W {
        BABT_W { w: self }
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&mut self) -> BABR_W {
        BABR_W { w: self }
    }
}
