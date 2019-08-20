#[doc = "Reader of register IRQSIGEN"]
pub type R = crate::R<u32, super::IRQSIGEN>;
#[doc = "Writer for register IRQSIGEN"]
pub type W = crate::W<u32, super::IRQSIGEN>;
#[doc = "Register IRQSIGEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQSIGEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCIEN_A) -> Self {
        match variant {
            CCIEN_A::_0 => false,
            CCIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCIEN`"]
pub type CCIEN_R = crate::R<bool, CCIEN_A>;
impl CCIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIEN_A {
        match self.bits {
            false => CCIEN_A::_0,
            true => CCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIEN_A::_1
    }
}
#[doc = "Write proxy for field `CCIEN`"]
pub struct CCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIEN_A::_1)
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
#[doc = "Transfer Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<TCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCIEN_A) -> Self {
        match variant {
            TCIEN_A::_0 => false,
            TCIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCIEN`"]
pub type TCIEN_R = crate::R<bool, TCIEN_A>;
impl TCIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIEN_A {
        match self.bits {
            false => TCIEN_A::_0,
            true => TCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIEN_A::_1
    }
}
#[doc = "Write proxy for field `TCIEN`"]
pub struct TCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIEN_A::_1)
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
#[doc = "Block Gap Event Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<BGEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGEIEN_A) -> Self {
        match variant {
            BGEIEN_A::_0 => false,
            BGEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BGEIEN`"]
pub type BGEIEN_R = crate::R<bool, BGEIEN_A>;
impl BGEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGEIEN_A {
        match self.bits {
            false => BGEIEN_A::_0,
            true => BGEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGEIEN_A::_1
    }
}
#[doc = "Write proxy for field `BGEIEN`"]
pub struct BGEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEIEN_A::_1)
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
#[doc = "DMA Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<DINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTIEN_A) -> Self {
        match variant {
            DINTIEN_A::_0 => false,
            DINTIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DINTIEN`"]
pub type DINTIEN_R = crate::R<bool, DINTIEN_A>;
impl DINTIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTIEN_A {
        match self.bits {
            false => DINTIEN_A::_0,
            true => DINTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINTIEN_A::_1
    }
}
#[doc = "Write proxy for field `DINTIEN`"]
pub struct DINTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DINTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINTIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTIEN_A::_1)
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
#[doc = "Buffer Write Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<BWRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRIEN_A) -> Self {
        match variant {
            BWRIEN_A::_0 => false,
            BWRIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BWRIEN`"]
pub type BWRIEN_R = crate::R<bool, BWRIEN_A>;
impl BWRIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRIEN_A {
        match self.bits {
            false => BWRIEN_A::_0,
            true => BWRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWRIEN_A::_1
    }
}
#[doc = "Write proxy for field `BWRIEN`"]
pub struct BWRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BWRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRIEN_A::_1)
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
#[doc = "Buffer Read Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<BRRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRIEN_A) -> Self {
        match variant {
            BRRIEN_A::_0 => false,
            BRRIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BRRIEN`"]
pub type BRRIEN_R = crate::R<bool, BRRIEN_A>;
impl BRRIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRIEN_A {
        match self.bits {
            false => BRRIEN_A::_0,
            true => BRRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRRIEN_A::_1
    }
}
#[doc = "Write proxy for field `BRRIEN`"]
pub struct BRRIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRRIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRRIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRIEN_A::_1)
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
#[doc = "Card Insertion Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CINSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSIEN_A) -> Self {
        match variant {
            CINSIEN_A::_0 => false,
            CINSIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CINSIEN`"]
pub type CINSIEN_R = crate::R<bool, CINSIEN_A>;
impl CINSIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSIEN_A {
        match self.bits {
            false => CINSIEN_A::_0,
            true => CINSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINSIEN_A::_1
    }
}
#[doc = "Write proxy for field `CINSIEN`"]
pub struct CINSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CINSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINSIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSIEN_A::_1)
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
#[doc = "Card Removal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CRMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMIEN_A) -> Self {
        match variant {
            CRMIEN_A::_0 => false,
            CRMIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRMIEN`"]
pub type CRMIEN_R = crate::R<bool, CRMIEN_A>;
impl CRMIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMIEN_A {
        match self.bits {
            false => CRMIEN_A::_0,
            true => CRMIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRMIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRMIEN_A::_1
    }
}
#[doc = "Write proxy for field `CRMIEN`"]
pub struct CRMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRMIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMIEN_A::_1)
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
#[doc = "Card Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTIEN_A) -> Self {
        match variant {
            CINTIEN_A::_0 => false,
            CINTIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CINTIEN`"]
pub type CINTIEN_R = crate::R<bool, CINTIEN_A>;
impl CINTIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTIEN_A {
        match self.bits {
            false => CINTIEN_A::_0,
            true => CINTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINTIEN_A::_1
    }
}
#[doc = "Write proxy for field `CINTIEN`"]
pub struct CINTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CINTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINTIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTIEN_A::_1)
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
#[doc = "Command Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOEIEN_A) -> Self {
        match variant {
            CTOEIEN_A::_0 => false,
            CTOEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CTOEIEN`"]
pub type CTOEIEN_R = crate::R<bool, CTOEIEN_A>;
impl CTOEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOEIEN_A {
        match self.bits {
            false => CTOEIEN_A::_0,
            true => CTOEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTOEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTOEIEN_A::_1
    }
}
#[doc = "Write proxy for field `CTOEIEN`"]
pub struct CTOEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTOEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOEIEN_A::_1)
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
#[doc = "Command CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIEN_A) -> Self {
        match variant {
            CCEIEN_A::_0 => false,
            CCEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCEIEN`"]
pub type CCEIEN_R = crate::R<bool, CCEIEN_A>;
impl CCEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCEIEN_A {
        match self.bits {
            false => CCEIEN_A::_0,
            true => CCEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCEIEN_A::_1
    }
}
#[doc = "Write proxy for field `CCEIEN`"]
pub struct CCEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCEIEN_A::_1)
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
#[doc = "Command End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBEIEN_A) -> Self {
        match variant {
            CEBEIEN_A::_0 => false,
            CEBEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CEBEIEN`"]
pub type CEBEIEN_R = crate::R<bool, CEBEIEN_A>;
impl CEBEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBEIEN_A {
        match self.bits {
            false => CEBEIEN_A::_0,
            true => CEBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEBEIEN_A::_1
    }
}
#[doc = "Write proxy for field `CEBEIEN`"]
pub struct CEBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEBEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBEIEN_A::_1)
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
#[doc = "Command Index Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<CIEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIEIEN_A) -> Self {
        match variant {
            CIEIEN_A::_0 => false,
            CIEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CIEIEN`"]
pub type CIEIEN_R = crate::R<bool, CIEIEN_A>;
impl CIEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIEIEN_A {
        match self.bits {
            false => CIEIEN_A::_0,
            true => CIEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIEIEN_A::_1
    }
}
#[doc = "Write proxy for field `CIEIEN`"]
pub struct CIEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CIEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIEIEN_A::_1)
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
#[doc = "Data Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<DTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOEIEN_A) -> Self {
        match variant {
            DTOEIEN_A::_0 => false,
            DTOEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DTOEIEN`"]
pub type DTOEIEN_R = crate::R<bool, DTOEIEN_A>;
impl DTOEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOEIEN_A {
        match self.bits {
            false => DTOEIEN_A::_0,
            true => DTOEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOEIEN_A::_1
    }
}
#[doc = "Write proxy for field `DTOEIEN`"]
pub struct DTOEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOEIEN_A::_1)
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
#[doc = "Data CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<DCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEIEN_A) -> Self {
        match variant {
            DCEIEN_A::_0 => false,
            DCEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DCEIEN`"]
pub type DCEIEN_R = crate::R<bool, DCEIEN_A>;
impl DCEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEIEN_A {
        match self.bits {
            false => DCEIEN_A::_0,
            true => DCEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCEIEN_A::_1
    }
}
#[doc = "Write proxy for field `DCEIEN`"]
pub struct DCEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCEIEN_A::_1)
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
#[doc = "Data End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<DEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBEIEN_A) -> Self {
        match variant {
            DEBEIEN_A::_0 => false,
            DEBEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DEBEIEN`"]
pub type DEBEIEN_R = crate::R<bool, DEBEIEN_A>;
impl DEBEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBEIEN_A {
        match self.bits {
            false => DEBEIEN_A::_0,
            true => DEBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEBEIEN_A::_1
    }
}
#[doc = "Write proxy for field `DEBEIEN`"]
pub struct DEBEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBEIEN_A::_1)
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
#[doc = "Auto CMD12 Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<AC12EIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EIEN_A) -> Self {
        match variant {
            AC12EIEN_A::_0 => false,
            AC12EIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12EIEN`"]
pub type AC12EIEN_R = crate::R<bool, AC12EIEN_A>;
impl AC12EIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EIEN_A {
        match self.bits {
            false => AC12EIEN_A::_0,
            true => AC12EIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EIEN_A::_1
    }
}
#[doc = "Write proxy for field `AC12EIEN`"]
pub struct AC12EIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12EIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12EIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EIEN_A::_1)
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
#[doc = "DMA Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEIEN_A {
    #[doc = "0: Masked"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<DMAEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEIEN_A) -> Self {
        match variant {
            DMAEIEN_A::_0 => false,
            DMAEIEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAEIEN`"]
pub type DMAEIEN_R = crate::R<bool, DMAEIEN_A>;
impl DMAEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEIEN_A {
        match self.bits {
            false => DMAEIEN_A::_0,
            true => DMAEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEIEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEIEN`"]
pub struct DMAEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEIEN_A::_1)
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
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccien(&self) -> CCIEN_R {
        CCIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn bgeien(&self) -> BGEIEN_R {
        BGEIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dintien(&self) -> DINTIEN_R {
        DINTIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bwrien(&self) -> BWRIEN_R {
        BWRIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brrien(&self) -> BRRIEN_R {
        BRRIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn cinsien(&self) -> CINSIEN_R {
        CINSIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn crmien(&self) -> CRMIEN_R {
        CRMIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline(always)]
    pub fn cintien(&self) -> CINTIEN_R {
        CINTIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctoeien(&self) -> CTOEIEN_R {
        CTOEIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CCEIEN_R {
        CCEIEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn cebeien(&self) -> CEBEIEN_R {
        CEBEIEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn cieien(&self) -> CIEIEN_R {
        CIEIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoeien(&self) -> DTOEIEN_R {
        DTOEIEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dceien(&self) -> DCEIEN_R {
        DCEIEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn debeien(&self) -> DEBEIEN_R {
        DEBEIEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn ac12eien(&self) -> AC12EIEN_R {
        AC12EIEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeien(&self) -> DMAEIEN_R {
        DMAEIEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccien(&mut self) -> CCIEN_W {
        CCIEN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcien(&mut self) -> TCIEN_W {
        TCIEN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn bgeien(&mut self) -> BGEIEN_W {
        BGEIEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dintien(&mut self) -> DINTIEN_W {
        DINTIEN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bwrien(&mut self) -> BWRIEN_W {
        BWRIEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brrien(&mut self) -> BRRIEN_W {
        BRRIEN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn cinsien(&mut self) -> CINSIEN_W {
        CINSIEN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn crmien(&mut self) -> CRMIEN_W {
        CRMIEN_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline(always)]
    pub fn cintien(&mut self) -> CINTIEN_W {
        CINTIEN_W { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctoeien(&mut self) -> CTOEIEN_W {
        CTOEIEN_W { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn cceien(&mut self) -> CCEIEN_W {
        CCEIEN_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn cebeien(&mut self) -> CEBEIEN_W {
        CEBEIEN_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn cieien(&mut self) -> CIEIEN_W {
        CIEIEN_W { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoeien(&mut self) -> DTOEIEN_W {
        DTOEIEN_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dceien(&mut self) -> DCEIEN_W {
        DCEIEN_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn debeien(&mut self) -> DEBEIEN_W {
        DEBEIEN_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn ac12eien(&mut self) -> AC12EIEN_W {
        AC12EIEN_W { w: self }
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeien(&mut self) -> DMAEIEN_W {
        DMAEIEN_W { w: self }
    }
}
