#[doc = "Reader of register IE7816"]
pub type R = crate::R<u8, super::IE7816>;
#[doc = "Writer for register IE7816"]
pub type W = crate::W<u8, super::IE7816>;
#[doc = "Register IE7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::IE7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTE_A {
    #[doc = "0: The assertion of IS7816\\[RXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[RXT\\] results in the generation of an interrupt."]
    _1,
}
impl From<RXTE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTE_A) -> Self {
        match variant {
            RXTE_A::_0 => false,
            RXTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXTE`"]
pub type RXTE_R = crate::R<bool, RXTE_A>;
impl RXTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTE_A {
        match self.bits {
            false => RXTE_A::_0,
            true => RXTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXTE_A::_1
    }
}
#[doc = "Write proxy for field `RXTE`"]
pub struct RXTE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[RXT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[RXT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTE_A::_1)
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
#[doc = "Transmit Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTE_A {
    #[doc = "0: The assertion of IS7816\\[TXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[TXT\\] results in the generation of an interrupt."]
    _1,
}
impl From<TXTE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTE_A) -> Self {
        match variant {
            TXTE_A::_0 => false,
            TXTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXTE`"]
pub type TXTE_R = crate::R<bool, TXTE_A>;
impl TXTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTE_A {
        match self.bits {
            false => TXTE_A::_0,
            true => TXTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXTE_A::_1
    }
}
#[doc = "Write proxy for field `TXTE`"]
pub struct TXTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[TXT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[TXT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTE_A::_1)
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
#[doc = "Guard Timer Violated Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTVE_A {
    #[doc = "0: The assertion of IS7816\\[GTV\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[GTV\\] results in the generation of an interrupt."]
    _1,
}
impl From<GTVE_A> for bool {
    #[inline(always)]
    fn from(variant: GTVE_A) -> Self {
        match variant {
            GTVE_A::_0 => false,
            GTVE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GTVE`"]
pub type GTVE_R = crate::R<bool, GTVE_A>;
impl GTVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTVE_A {
        match self.bits {
            false => GTVE_A::_0,
            true => GTVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTVE_A::_1
    }
}
#[doc = "Write proxy for field `GTVE`"]
pub struct GTVE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[GTV\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTVE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[GTV\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTVE_A::_1)
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
#[doc = "ATR Duration Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTE_A {
    #[doc = "0: The assertion of IS7816\\[ADT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[ADT\\] results in the generation of an interrupt."]
    _1,
}
impl From<ADTE_A> for bool {
    #[inline(always)]
    fn from(variant: ADTE_A) -> Self {
        match variant {
            ADTE_A::_0 => false,
            ADTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADTE`"]
pub type ADTE_R = crate::R<bool, ADTE_A>;
impl ADTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTE_A {
        match self.bits {
            false => ADTE_A::_0,
            true => ADTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTE_A::_1
    }
}
#[doc = "Write proxy for field `ADTE`"]
pub struct ADTE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[ADT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[ADT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTE_A::_1)
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
#[doc = "Initial Character Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDE_A {
    #[doc = "0: The assertion of IS7816\\[INITD\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[INITD\\] results in the generation of an interrupt."]
    _1,
}
impl From<INITDE_A> for bool {
    #[inline(always)]
    fn from(variant: INITDE_A) -> Self {
        match variant {
            INITDE_A::_0 => false,
            INITDE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INITDE`"]
pub type INITDE_R = crate::R<bool, INITDE_A>;
impl INITDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITDE_A {
        match self.bits {
            false => INITDE_A::_0,
            true => INITDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITDE_A::_1
    }
}
#[doc = "Write proxy for field `INITDE`"]
pub struct INITDE_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[INITD\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITDE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[INITD\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITDE_A::_1)
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
#[doc = "Block Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWTE_A {
    #[doc = "0: The assertion of IS7816\\[BWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[BWT\\] results in the generation of an interrupt."]
    _1,
}
impl From<BWTE_A> for bool {
    #[inline(always)]
    fn from(variant: BWTE_A) -> Self {
        match variant {
            BWTE_A::_0 => false,
            BWTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BWTE`"]
pub type BWTE_R = crate::R<bool, BWTE_A>;
impl BWTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWTE_A {
        match self.bits {
            false => BWTE_A::_0,
            true => BWTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWTE_A::_1
    }
}
#[doc = "Write proxy for field `BWTE`"]
pub struct BWTE_W<'a> {
    w: &'a mut W,
}
impl<'a> BWTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[BWT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[BWT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWTE_A::_1)
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
#[doc = "Character Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWTE_A {
    #[doc = "0: The assertion of IS7816\\[CWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[CWT\\] results in the generation of an interrupt."]
    _1,
}
impl From<CWTE_A> for bool {
    #[inline(always)]
    fn from(variant: CWTE_A) -> Self {
        match variant {
            CWTE_A::_0 => false,
            CWTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CWTE`"]
pub type CWTE_R = crate::R<bool, CWTE_A>;
impl CWTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWTE_A {
        match self.bits {
            false => CWTE_A::_0,
            true => CWTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWTE_A::_1
    }
}
#[doc = "Write proxy for field `CWTE`"]
pub struct CWTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[CWT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[CWT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWTE_A::_1)
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
#[doc = "Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTE_A {
    #[doc = "0: The assertion of IS7816\\[WT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "1: The assertion of IS7816\\[WT\\] results in the generation of an interrupt."]
    _1,
}
impl From<WTE_A> for bool {
    #[inline(always)]
    fn from(variant: WTE_A) -> Self {
        match variant {
            WTE_A::_0 => false,
            WTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WTE`"]
pub type WTE_R = crate::R<bool, WTE_A>;
impl WTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTE_A {
        match self.bits {
            false => WTE_A::_0,
            true => WTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTE_A::_1
    }
}
#[doc = "Write proxy for field `WTE`"]
pub struct WTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The assertion of IS7816\\[WT\\] does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[WT\\] results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTE_A::_1)
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
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn rxte(&self) -> RXTE_R {
        RXTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn txte(&self) -> TXTE_R {
        TXTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    pub fn gtve(&self) -> GTVE_R {
        GTVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline(always)]
    pub fn adte(&self) -> ADTE_R {
        ADTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    pub fn initde(&self) -> INITDE_R {
        INITDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn bwte(&self) -> BWTE_R {
        BWTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn cwte(&self) -> CWTE_R {
        CWTE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn rxte(&mut self) -> RXTE_W {
        RXTE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn txte(&mut self) -> TXTE_W {
        TXTE_W { w: self }
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    pub fn gtve(&mut self) -> GTVE_W {
        GTVE_W { w: self }
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline(always)]
    pub fn adte(&mut self) -> ADTE_W {
        ADTE_W { w: self }
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    pub fn initde(&mut self) -> INITDE_W {
        INITDE_W { w: self }
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn bwte(&mut self) -> BWTE_W {
        BWTE_W { w: self }
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn cwte(&mut self) -> CWTE_W {
        CWTE_W { w: self }
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W {
        WTE_W { w: self }
    }
}
