#[doc = "Reader of register IRQSTAT"]
pub type R = crate::R<u32, super::IRQSTAT>;
#[doc = "Writer for register IRQSTAT"]
pub type W = crate::W<u32, super::IRQSTAT>;
#[doc = "Register IRQSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC_A {
    #[doc = "0: Command not complete."]
    _0,
    #[doc = "1: Command complete."]
    _1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        match variant {
            CC_A::_0 => false,
            CC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<bool, CC_A>;
impl CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::_0,
            true => CC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CC_A::_1
    }
}
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CC_A::_0)
    }
    #[doc = "Command complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CC_A::_1)
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
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: Transfer not complete."]
    _0,
    #[doc = "1: Transfer complete."]
    _1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        match variant {
            TC_A::_0 => false,
            TC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, TC_A>;
impl TC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC_A::_0)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC_A::_1)
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
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGE_A {
    #[doc = "0: No block gap event."]
    _0,
    #[doc = "1: Transaction stopped at block gap."]
    _1,
}
impl From<BGE_A> for bool {
    #[inline(always)]
    fn from(variant: BGE_A) -> Self {
        match variant {
            BGE_A::_0 => false,
            BGE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BGE`"]
pub type BGE_R = crate::R<bool, BGE_A>;
impl BGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGE_A {
        match self.bits {
            false => BGE_A::_0,
            true => BGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGE_A::_1
    }
}
#[doc = "Write proxy for field `BGE`"]
pub struct BGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No block gap event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGE_A::_0)
    }
    #[doc = "Transaction stopped at block gap."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGE_A::_1)
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
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINT_A {
    #[doc = "0: No DMA Interrupt."]
    _0,
    #[doc = "1: DMA Interrupt is generated."]
    _1,
}
impl From<DINT_A> for bool {
    #[inline(always)]
    fn from(variant: DINT_A) -> Self {
        match variant {
            DINT_A::_0 => false,
            DINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DINT`"]
pub type DINT_R = crate::R<bool, DINT_A>;
impl DINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINT_A {
        match self.bits {
            false => DINT_A::_0,
            true => DINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINT_A::_1
    }
}
#[doc = "Write proxy for field `DINT`"]
pub struct DINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINT_A::_0)
    }
    #[doc = "DMA Interrupt is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINT_A::_1)
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
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWR_A {
    #[doc = "0: Not ready to write buffer."]
    _0,
    #[doc = "1: Ready to write buffer."]
    _1,
}
impl From<BWR_A> for bool {
    #[inline(always)]
    fn from(variant: BWR_A) -> Self {
        match variant {
            BWR_A::_0 => false,
            BWR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BWR`"]
pub type BWR_R = crate::R<bool, BWR_A>;
impl BWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWR_A {
        match self.bits {
            false => BWR_A::_0,
            true => BWR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWR_A::_1
    }
}
#[doc = "Write proxy for field `BWR`"]
pub struct BWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not ready to write buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWR_A::_0)
    }
    #[doc = "Ready to write buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWR_A::_1)
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
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRR_A {
    #[doc = "0: Not ready to read buffer."]
    _0,
    #[doc = "1: Ready to read buffer."]
    _1,
}
impl From<BRR_A> for bool {
    #[inline(always)]
    fn from(variant: BRR_A) -> Self {
        match variant {
            BRR_A::_0 => false,
            BRR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BRR`"]
pub type BRR_R = crate::R<bool, BRR_A>;
impl BRR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRR_A {
        match self.bits {
            false => BRR_A::_0,
            true => BRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRR_A::_1
    }
}
#[doc = "Write proxy for field `BRR`"]
pub struct BRR_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not ready to read buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRR_A::_0)
    }
    #[doc = "Ready to read buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRR_A::_1)
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
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINS_A {
    #[doc = "0: Card state unstable or removed."]
    _0,
    #[doc = "1: Card inserted."]
    _1,
}
impl From<CINS_A> for bool {
    #[inline(always)]
    fn from(variant: CINS_A) -> Self {
        match variant {
            CINS_A::_0 => false,
            CINS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CINS`"]
pub type CINS_R = crate::R<bool, CINS_A>;
impl CINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINS_A {
        match self.bits {
            false => CINS_A::_0,
            true => CINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINS_A::_1
    }
}
#[doc = "Write proxy for field `CINS`"]
pub struct CINS_W<'a> {
    w: &'a mut W,
}
impl<'a> CINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card state unstable or removed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINS_A::_0)
    }
    #[doc = "Card inserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINS_A::_1)
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
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRM_A {
    #[doc = "0: Card state unstable or inserted."]
    _0,
    #[doc = "1: Card removed."]
    _1,
}
impl From<CRM_A> for bool {
    #[inline(always)]
    fn from(variant: CRM_A) -> Self {
        match variant {
            CRM_A::_0 => false,
            CRM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRM`"]
pub type CRM_R = crate::R<bool, CRM_A>;
impl CRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRM_A {
        match self.bits {
            false => CRM_A::_0,
            true => CRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRM_A::_1
    }
}
#[doc = "Write proxy for field `CRM`"]
pub struct CRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card state unstable or inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRM_A::_0)
    }
    #[doc = "Card removed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRM_A::_1)
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
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINT_A {
    #[doc = "0: No Card Interrupt."]
    _0,
    #[doc = "1: Generate Card Interrupt."]
    _1,
}
impl From<CINT_A> for bool {
    #[inline(always)]
    fn from(variant: CINT_A) -> Self {
        match variant {
            CINT_A::_0 => false,
            CINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CINT`"]
pub type CINT_R = crate::R<bool, CINT_A>;
impl CINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINT_A {
        match self.bits {
            false => CINT_A::_0,
            true => CINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINT_A::_1
    }
}
#[doc = "Write proxy for field `CINT`"]
pub struct CINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Card Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINT_A::_0)
    }
    #[doc = "Generate Card Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINT_A::_1)
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
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Time out."]
    _1,
}
impl From<CTOE_A> for bool {
    #[inline(always)]
    fn from(variant: CTOE_A) -> Self {
        match variant {
            CTOE_A::_0 => false,
            CTOE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CTOE`"]
pub type CTOE_R = crate::R<bool, CTOE_A>;
impl CTOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOE_A {
        match self.bits {
            false => CTOE_A::_0,
            true => CTOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTOE_A::_1
    }
}
#[doc = "Write proxy for field `CTOE`"]
pub struct CTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOE_A::_0)
    }
    #[doc = "Time out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOE_A::_1)
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
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: CRC Error generated."]
    _1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        match variant {
            CCE_A::_0 => false,
            CCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, CCE_A>;
impl CCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::_0,
            true => CCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCE_A::_1
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCE_A::_0)
    }
    #[doc = "CRC Error generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCE_A::_1)
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
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: End Bit Error generated."]
    _1,
}
impl From<CEBE_A> for bool {
    #[inline(always)]
    fn from(variant: CEBE_A) -> Self {
        match variant {
            CEBE_A::_0 => false,
            CEBE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CEBE`"]
pub type CEBE_R = crate::R<bool, CEBE_A>;
impl CEBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBE_A {
        match self.bits {
            false => CEBE_A::_0,
            true => CEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEBE_A::_1
    }
}
#[doc = "Write proxy for field `CEBE`"]
pub struct CEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBE_A::_0)
    }
    #[doc = "End Bit Error generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBE_A::_1)
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
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error."]
    _1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        match variant {
            CIE_A::_0 => false,
            CIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CIE`"]
pub type CIE_R = crate::R<bool, CIE_A>;
impl CIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::_0,
            true => CIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIE_A::_1
    }
}
#[doc = "Write proxy for field `CIE`"]
pub struct CIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIE_A::_1)
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
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Time out."]
    _1,
}
impl From<DTOE_A> for bool {
    #[inline(always)]
    fn from(variant: DTOE_A) -> Self {
        match variant {
            DTOE_A::_0 => false,
            DTOE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DTOE`"]
pub type DTOE_R = crate::R<bool, DTOE_A>;
impl DTOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOE_A {
        match self.bits {
            false => DTOE_A::_0,
            true => DTOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOE_A::_1
    }
}
#[doc = "Write proxy for field `DTOE`"]
pub struct DTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOE_A::_0)
    }
    #[doc = "Time out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOE_A::_1)
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
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error."]
    _1,
}
impl From<DCE_A> for bool {
    #[inline(always)]
    fn from(variant: DCE_A) -> Self {
        match variant {
            DCE_A::_0 => false,
            DCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DCE`"]
pub type DCE_R = crate::R<bool, DCE_A>;
impl DCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCE_A {
        match self.bits {
            false => DCE_A::_0,
            true => DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCE_A::_1
    }
}
#[doc = "Write proxy for field `DCE`"]
pub struct DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCE_A::_1)
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
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error."]
    _1,
}
impl From<DEBE_A> for bool {
    #[inline(always)]
    fn from(variant: DEBE_A) -> Self {
        match variant {
            DEBE_A::_0 => false,
            DEBE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DEBE`"]
pub type DEBE_R = crate::R<bool, DEBE_A>;
impl DEBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBE_A {
        match self.bits {
            false => DEBE_A::_0,
            true => DEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEBE_A::_1
    }
}
#[doc = "Write proxy for field `DEBE`"]
pub struct DEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12E_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error."]
    _1,
}
impl From<AC12E_A> for bool {
    #[inline(always)]
    fn from(variant: AC12E_A) -> Self {
        match variant {
            AC12E_A::_0 => false,
            AC12E_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12E`"]
pub type AC12E_R = crate::R<bool, AC12E_A>;
impl AC12E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12E_A {
        match self.bits {
            false => AC12E_A::_0,
            true => AC12E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12E_A::_1
    }
}
#[doc = "Write proxy for field `AC12E`"]
pub struct AC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12E_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12E_A::_1)
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
#[doc = "DMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error."]
    _1,
}
impl From<DMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAE_A) -> Self {
        match variant {
            DMAE_A::_0 => false,
            DMAE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAE`"]
pub type DMAE_R = crate::R<bool, DMAE_A>;
impl DMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAE_A {
        match self.bits {
            false => DMAE_A::_0,
            true => DMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAE_A::_1
    }
}
#[doc = "Write proxy for field `DMAE`"]
pub struct DMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn bge(&self) -> BGE_R {
        BGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dint(&self) -> DINT_R {
        DINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn ctoe(&self) -> CTOE_R {
        CTOE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&self) -> CEBE_R {
        CEBE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&self) -> DEBE_R {
        DEBE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> AC12E_R {
        AC12E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn bge(&mut self) -> BGE_W {
        BGE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dint(&mut self) -> DINT_W {
        DINT_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwr(&mut self) -> BWR_W {
        BWR_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W {
        BRR_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&mut self) -> CINS_W {
        CINS_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crm(&mut self) -> CRM_W {
        CRM_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn ctoe(&mut self) -> CTOE_W {
        CTOE_W { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&mut self) -> CEBE_W {
        CEBE_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W {
        CIE_W { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&mut self) -> DTOE_W {
        DTOE_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W {
        DCE_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&mut self) -> DEBE_W {
        DEBE_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> AC12E_W {
        AC12E_W { w: self }
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W {
        DMAE_W { w: self }
    }
}
