#[doc = "Reader of register SCGC5"]
pub type R = crate::R<u32, super::SCGC5>;
#[doc = "Writer for register SCGC5"]
pub type W = crate::W<u32, super::SCGC5>;
#[doc = "Register SCGC5 `reset()`'s with value 0x0004_0182"]
impl crate::ResetValue for super::SCGC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0182
    }
}
#[doc = "Low Power Timer Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR_A {
    #[doc = "0: Access disabled"]
    _0,
    #[doc = "1: Access enabled"]
    _1,
}
impl From<LPTMR_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR_A) -> Self {
        match variant {
            LPTMR_A::_0 => false,
            LPTMR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LPTMR`"]
pub type LPTMR_R = crate::R<bool, LPTMR_A>;
impl LPTMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR_A {
        match self.bits {
            false => LPTMR_A::_0,
            true => LPTMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPTMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPTMR_A::_1
    }
}
#[doc = "Write proxy for field `LPTMR`"]
pub struct LPTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR_A::_0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR_A::_1)
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
#[doc = "TSI Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSI_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<TSI_A> for bool {
    #[inline(always)]
    fn from(variant: TSI_A) -> Self {
        match variant {
            TSI_A::_0 => false,
            TSI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TSI`"]
pub type TSI_R = crate::R<bool, TSI_A>;
impl TSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSI_A {
        match self.bits {
            false => TSI_A::_0,
            true => TSI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSI_A::_1
    }
}
#[doc = "Write proxy for field `TSI`"]
pub struct TSI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSI_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSI_A::_1)
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
#[doc = "Port A Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTA_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PORTA_A> for bool {
    #[inline(always)]
    fn from(variant: PORTA_A) -> Self {
        match variant {
            PORTA_A::_0 => false,
            PORTA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORTA`"]
pub type PORTA_R = crate::R<bool, PORTA_A>;
impl PORTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTA_A {
        match self.bits {
            false => PORTA_A::_0,
            true => PORTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTA_A::_1
    }
}
#[doc = "Write proxy for field `PORTA`"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTA_A::_1)
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
#[doc = "Port B Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTB_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PORTB_A> for bool {
    #[inline(always)]
    fn from(variant: PORTB_A) -> Self {
        match variant {
            PORTB_A::_0 => false,
            PORTB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORTB`"]
pub type PORTB_R = crate::R<bool, PORTB_A>;
impl PORTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTB_A {
        match self.bits {
            false => PORTB_A::_0,
            true => PORTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTB_A::_1
    }
}
#[doc = "Write proxy for field `PORTB`"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTB_A::_1)
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
#[doc = "Port C Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTC_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PORTC_A> for bool {
    #[inline(always)]
    fn from(variant: PORTC_A) -> Self {
        match variant {
            PORTC_A::_0 => false,
            PORTC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORTC`"]
pub type PORTC_R = crate::R<bool, PORTC_A>;
impl PORTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTC_A {
        match self.bits {
            false => PORTC_A::_0,
            true => PORTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTC_A::_1
    }
}
#[doc = "Write proxy for field `PORTC`"]
pub struct PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTC_A::_1)
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
#[doc = "Port D Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTD_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PORTD_A> for bool {
    #[inline(always)]
    fn from(variant: PORTD_A) -> Self {
        match variant {
            PORTD_A::_0 => false,
            PORTD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORTD`"]
pub type PORTD_R = crate::R<bool, PORTD_A>;
impl PORTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTD_A {
        match self.bits {
            false => PORTD_A::_0,
            true => PORTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTD_A::_1
    }
}
#[doc = "Write proxy for field `PORTD`"]
pub struct PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTD_A::_1)
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
#[doc = "Port E Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTE_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<PORTE_A> for bool {
    #[inline(always)]
    fn from(variant: PORTE_A) -> Self {
        match variant {
            PORTE_A::_0 => false,
            PORTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORTE`"]
pub type PORTE_R = crate::R<bool, PORTE_A>;
impl PORTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTE_A {
        match self.bits {
            false => PORTE_A::_0,
            true => PORTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTE_A::_1
    }
}
#[doc = "Write proxy for field `PORTE`"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTE_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&self) -> LPTMR_R {
        LPTMR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSI Clock Gate Control"]
    #[inline(always)]
    pub fn tsi(&self) -> TSI_R {
        TSI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&mut self) -> LPTMR_W {
        LPTMR_W { w: self }
    }
    #[doc = "Bit 5 - TSI Clock Gate Control"]
    #[inline(always)]
    pub fn tsi(&mut self) -> TSI_W {
        TSI_W { w: self }
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
}
