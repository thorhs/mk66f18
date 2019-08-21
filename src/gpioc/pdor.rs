#[doc = "Reader of register PDOR"]
pub type R = crate::R<u32, super::PDOR>;
#[doc = "Writer for register PDOR"]
pub type W = crate::W<u32, super::PDOR>;
#[doc = "Register PDOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO0_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO0_A> for bool {
    #[inline(always)]
    fn from(variant: PDO0_A) -> Self {
        match variant {
            PDO0_A::_0 => false,
            PDO0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO0`"]
pub type PDO0_R = crate::R<bool, PDO0_A>;
impl PDO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO0_A {
        match self.bits {
            false => PDO0_A::_0,
            true => PDO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO0_A::_1
    }
}
#[doc = "Write proxy for field `PDO0`"]
pub struct PDO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO0_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO0_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO1_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO1_A> for bool {
    #[inline(always)]
    fn from(variant: PDO1_A) -> Self {
        match variant {
            PDO1_A::_0 => false,
            PDO1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO1`"]
pub type PDO1_R = crate::R<bool, PDO1_A>;
impl PDO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO1_A {
        match self.bits {
            false => PDO1_A::_0,
            true => PDO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO1_A::_1
    }
}
#[doc = "Write proxy for field `PDO1`"]
pub struct PDO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO1_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO1_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO2_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO2_A> for bool {
    #[inline(always)]
    fn from(variant: PDO2_A) -> Self {
        match variant {
            PDO2_A::_0 => false,
            PDO2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO2`"]
pub type PDO2_R = crate::R<bool, PDO2_A>;
impl PDO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO2_A {
        match self.bits {
            false => PDO2_A::_0,
            true => PDO2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO2_A::_1
    }
}
#[doc = "Write proxy for field `PDO2`"]
pub struct PDO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO2_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO2_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO3_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO3_A> for bool {
    #[inline(always)]
    fn from(variant: PDO3_A) -> Self {
        match variant {
            PDO3_A::_0 => false,
            PDO3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO3`"]
pub type PDO3_R = crate::R<bool, PDO3_A>;
impl PDO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO3_A {
        match self.bits {
            false => PDO3_A::_0,
            true => PDO3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO3_A::_1
    }
}
#[doc = "Write proxy for field `PDO3`"]
pub struct PDO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO3_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO3_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO4_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO4_A> for bool {
    #[inline(always)]
    fn from(variant: PDO4_A) -> Self {
        match variant {
            PDO4_A::_0 => false,
            PDO4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO4`"]
pub type PDO4_R = crate::R<bool, PDO4_A>;
impl PDO4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO4_A {
        match self.bits {
            false => PDO4_A::_0,
            true => PDO4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO4_A::_1
    }
}
#[doc = "Write proxy for field `PDO4`"]
pub struct PDO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO4_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO4_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO5_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO5_A> for bool {
    #[inline(always)]
    fn from(variant: PDO5_A) -> Self {
        match variant {
            PDO5_A::_0 => false,
            PDO5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO5`"]
pub type PDO5_R = crate::R<bool, PDO5_A>;
impl PDO5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO5_A {
        match self.bits {
            false => PDO5_A::_0,
            true => PDO5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO5_A::_1
    }
}
#[doc = "Write proxy for field `PDO5`"]
pub struct PDO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO5_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO5_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO6_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO6_A> for bool {
    #[inline(always)]
    fn from(variant: PDO6_A) -> Self {
        match variant {
            PDO6_A::_0 => false,
            PDO6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO6`"]
pub type PDO6_R = crate::R<bool, PDO6_A>;
impl PDO6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO6_A {
        match self.bits {
            false => PDO6_A::_0,
            true => PDO6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO6_A::_1
    }
}
#[doc = "Write proxy for field `PDO6`"]
pub struct PDO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO6_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO6_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO7_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO7_A> for bool {
    #[inline(always)]
    fn from(variant: PDO7_A) -> Self {
        match variant {
            PDO7_A::_0 => false,
            PDO7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO7`"]
pub type PDO7_R = crate::R<bool, PDO7_A>;
impl PDO7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO7_A {
        match self.bits {
            false => PDO7_A::_0,
            true => PDO7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO7_A::_1
    }
}
#[doc = "Write proxy for field `PDO7`"]
pub struct PDO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO7_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO7_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO8_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO8_A> for bool {
    #[inline(always)]
    fn from(variant: PDO8_A) -> Self {
        match variant {
            PDO8_A::_0 => false,
            PDO8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO8`"]
pub type PDO8_R = crate::R<bool, PDO8_A>;
impl PDO8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO8_A {
        match self.bits {
            false => PDO8_A::_0,
            true => PDO8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO8_A::_1
    }
}
#[doc = "Write proxy for field `PDO8`"]
pub struct PDO8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO8_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO8_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO9_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO9_A> for bool {
    #[inline(always)]
    fn from(variant: PDO9_A) -> Self {
        match variant {
            PDO9_A::_0 => false,
            PDO9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO9`"]
pub type PDO9_R = crate::R<bool, PDO9_A>;
impl PDO9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO9_A {
        match self.bits {
            false => PDO9_A::_0,
            true => PDO9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO9_A::_1
    }
}
#[doc = "Write proxy for field `PDO9`"]
pub struct PDO9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO9_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO9_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO10_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO10_A> for bool {
    #[inline(always)]
    fn from(variant: PDO10_A) -> Self {
        match variant {
            PDO10_A::_0 => false,
            PDO10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO10`"]
pub type PDO10_R = crate::R<bool, PDO10_A>;
impl PDO10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO10_A {
        match self.bits {
            false => PDO10_A::_0,
            true => PDO10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO10_A::_1
    }
}
#[doc = "Write proxy for field `PDO10`"]
pub struct PDO10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO10_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO10_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO11_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO11_A> for bool {
    #[inline(always)]
    fn from(variant: PDO11_A) -> Self {
        match variant {
            PDO11_A::_0 => false,
            PDO11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO11`"]
pub type PDO11_R = crate::R<bool, PDO11_A>;
impl PDO11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO11_A {
        match self.bits {
            false => PDO11_A::_0,
            true => PDO11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO11_A::_1
    }
}
#[doc = "Write proxy for field `PDO11`"]
pub struct PDO11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO11_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO11_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO12_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO12_A> for bool {
    #[inline(always)]
    fn from(variant: PDO12_A) -> Self {
        match variant {
            PDO12_A::_0 => false,
            PDO12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO12`"]
pub type PDO12_R = crate::R<bool, PDO12_A>;
impl PDO12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO12_A {
        match self.bits {
            false => PDO12_A::_0,
            true => PDO12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO12_A::_1
    }
}
#[doc = "Write proxy for field `PDO12`"]
pub struct PDO12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO12_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO12_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO13_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO13_A> for bool {
    #[inline(always)]
    fn from(variant: PDO13_A) -> Self {
        match variant {
            PDO13_A::_0 => false,
            PDO13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO13`"]
pub type PDO13_R = crate::R<bool, PDO13_A>;
impl PDO13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO13_A {
        match self.bits {
            false => PDO13_A::_0,
            true => PDO13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO13_A::_1
    }
}
#[doc = "Write proxy for field `PDO13`"]
pub struct PDO13_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO13_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO13_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO14_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO14_A> for bool {
    #[inline(always)]
    fn from(variant: PDO14_A) -> Self {
        match variant {
            PDO14_A::_0 => false,
            PDO14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO14`"]
pub type PDO14_R = crate::R<bool, PDO14_A>;
impl PDO14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO14_A {
        match self.bits {
            false => PDO14_A::_0,
            true => PDO14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO14_A::_1
    }
}
#[doc = "Write proxy for field `PDO14`"]
pub struct PDO14_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO14_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO15_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO15_A> for bool {
    #[inline(always)]
    fn from(variant: PDO15_A) -> Self {
        match variant {
            PDO15_A::_0 => false,
            PDO15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO15`"]
pub type PDO15_R = crate::R<bool, PDO15_A>;
impl PDO15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO15_A {
        match self.bits {
            false => PDO15_A::_0,
            true => PDO15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO15_A::_1
    }
}
#[doc = "Write proxy for field `PDO15`"]
pub struct PDO15_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO15_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO16_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO16_A> for bool {
    #[inline(always)]
    fn from(variant: PDO16_A) -> Self {
        match variant {
            PDO16_A::_0 => false,
            PDO16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO16`"]
pub type PDO16_R = crate::R<bool, PDO16_A>;
impl PDO16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO16_A {
        match self.bits {
            false => PDO16_A::_0,
            true => PDO16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO16_A::_1
    }
}
#[doc = "Write proxy for field `PDO16`"]
pub struct PDO16_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO16_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO16_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO17_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO17_A> for bool {
    #[inline(always)]
    fn from(variant: PDO17_A) -> Self {
        match variant {
            PDO17_A::_0 => false,
            PDO17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO17`"]
pub type PDO17_R = crate::R<bool, PDO17_A>;
impl PDO17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO17_A {
        match self.bits {
            false => PDO17_A::_0,
            true => PDO17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO17_A::_1
    }
}
#[doc = "Write proxy for field `PDO17`"]
pub struct PDO17_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO17_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO17_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO18_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO18_A> for bool {
    #[inline(always)]
    fn from(variant: PDO18_A) -> Self {
        match variant {
            PDO18_A::_0 => false,
            PDO18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO18`"]
pub type PDO18_R = crate::R<bool, PDO18_A>;
impl PDO18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO18_A {
        match self.bits {
            false => PDO18_A::_0,
            true => PDO18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO18_A::_1
    }
}
#[doc = "Write proxy for field `PDO18`"]
pub struct PDO18_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO18_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO18_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO19_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO19_A> for bool {
    #[inline(always)]
    fn from(variant: PDO19_A) -> Self {
        match variant {
            PDO19_A::_0 => false,
            PDO19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO19`"]
pub type PDO19_R = crate::R<bool, PDO19_A>;
impl PDO19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO19_A {
        match self.bits {
            false => PDO19_A::_0,
            true => PDO19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO19_A::_1
    }
}
#[doc = "Write proxy for field `PDO19`"]
pub struct PDO19_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO19_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO19_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO20_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO20_A> for bool {
    #[inline(always)]
    fn from(variant: PDO20_A) -> Self {
        match variant {
            PDO20_A::_0 => false,
            PDO20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO20`"]
pub type PDO20_R = crate::R<bool, PDO20_A>;
impl PDO20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO20_A {
        match self.bits {
            false => PDO20_A::_0,
            true => PDO20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO20_A::_1
    }
}
#[doc = "Write proxy for field `PDO20`"]
pub struct PDO20_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO20_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO20_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO21_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO21_A> for bool {
    #[inline(always)]
    fn from(variant: PDO21_A) -> Self {
        match variant {
            PDO21_A::_0 => false,
            PDO21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO21`"]
pub type PDO21_R = crate::R<bool, PDO21_A>;
impl PDO21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO21_A {
        match self.bits {
            false => PDO21_A::_0,
            true => PDO21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO21_A::_1
    }
}
#[doc = "Write proxy for field `PDO21`"]
pub struct PDO21_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO21_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO21_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO22_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO22_A> for bool {
    #[inline(always)]
    fn from(variant: PDO22_A) -> Self {
        match variant {
            PDO22_A::_0 => false,
            PDO22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO22`"]
pub type PDO22_R = crate::R<bool, PDO22_A>;
impl PDO22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO22_A {
        match self.bits {
            false => PDO22_A::_0,
            true => PDO22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO22_A::_1
    }
}
#[doc = "Write proxy for field `PDO22`"]
pub struct PDO22_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO22_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO22_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO23_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO23_A> for bool {
    #[inline(always)]
    fn from(variant: PDO23_A) -> Self {
        match variant {
            PDO23_A::_0 => false,
            PDO23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO23`"]
pub type PDO23_R = crate::R<bool, PDO23_A>;
impl PDO23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO23_A {
        match self.bits {
            false => PDO23_A::_0,
            true => PDO23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO23_A::_1
    }
}
#[doc = "Write proxy for field `PDO23`"]
pub struct PDO23_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO23_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO23_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO24_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO24_A> for bool {
    #[inline(always)]
    fn from(variant: PDO24_A) -> Self {
        match variant {
            PDO24_A::_0 => false,
            PDO24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO24`"]
pub type PDO24_R = crate::R<bool, PDO24_A>;
impl PDO24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO24_A {
        match self.bits {
            false => PDO24_A::_0,
            true => PDO24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO24_A::_1
    }
}
#[doc = "Write proxy for field `PDO24`"]
pub struct PDO24_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO24_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO24_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO25_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO25_A> for bool {
    #[inline(always)]
    fn from(variant: PDO25_A) -> Self {
        match variant {
            PDO25_A::_0 => false,
            PDO25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO25`"]
pub type PDO25_R = crate::R<bool, PDO25_A>;
impl PDO25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO25_A {
        match self.bits {
            false => PDO25_A::_0,
            true => PDO25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO25_A::_1
    }
}
#[doc = "Write proxy for field `PDO25`"]
pub struct PDO25_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO25_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO25_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO26_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO26_A> for bool {
    #[inline(always)]
    fn from(variant: PDO26_A) -> Self {
        match variant {
            PDO26_A::_0 => false,
            PDO26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO26`"]
pub type PDO26_R = crate::R<bool, PDO26_A>;
impl PDO26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO26_A {
        match self.bits {
            false => PDO26_A::_0,
            true => PDO26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO26_A::_1
    }
}
#[doc = "Write proxy for field `PDO26`"]
pub struct PDO26_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO26_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO26_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO27_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO27_A> for bool {
    #[inline(always)]
    fn from(variant: PDO27_A) -> Self {
        match variant {
            PDO27_A::_0 => false,
            PDO27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO27`"]
pub type PDO27_R = crate::R<bool, PDO27_A>;
impl PDO27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO27_A {
        match self.bits {
            false => PDO27_A::_0,
            true => PDO27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO27_A::_1
    }
}
#[doc = "Write proxy for field `PDO27`"]
pub struct PDO27_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO27_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO27_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO28_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO28_A> for bool {
    #[inline(always)]
    fn from(variant: PDO28_A) -> Self {
        match variant {
            PDO28_A::_0 => false,
            PDO28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO28`"]
pub type PDO28_R = crate::R<bool, PDO28_A>;
impl PDO28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO28_A {
        match self.bits {
            false => PDO28_A::_0,
            true => PDO28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO28_A::_1
    }
}
#[doc = "Write proxy for field `PDO28`"]
pub struct PDO28_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO28_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO28_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO29_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO29_A> for bool {
    #[inline(always)]
    fn from(variant: PDO29_A) -> Self {
        match variant {
            PDO29_A::_0 => false,
            PDO29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO29`"]
pub type PDO29_R = crate::R<bool, PDO29_A>;
impl PDO29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO29_A {
        match self.bits {
            false => PDO29_A::_0,
            true => PDO29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO29_A::_1
    }
}
#[doc = "Write proxy for field `PDO29`"]
pub struct PDO29_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO29_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO29_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO30_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO30_A> for bool {
    #[inline(always)]
    fn from(variant: PDO30_A) -> Self {
        match variant {
            PDO30_A::_0 => false,
            PDO30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO30`"]
pub type PDO30_R = crate::R<bool, PDO30_A>;
impl PDO30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO30_A {
        match self.bits {
            false => PDO30_A::_0,
            true => PDO30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO30_A::_1
    }
}
#[doc = "Write proxy for field `PDO30`"]
pub struct PDO30_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO30_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO30_A::_1)
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
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO31_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl From<PDO31_A> for bool {
    #[inline(always)]
    fn from(variant: PDO31_A) -> Self {
        match variant {
            PDO31_A::_0 => false,
            PDO31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDO31`"]
pub type PDO31_R = crate::R<bool, PDO31_A>;
impl PDO31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO31_A {
        match self.bits {
            false => PDO31_A::_0,
            true => PDO31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO31_A::_1
    }
}
#[doc = "Write proxy for field `PDO31`"]
pub struct PDO31_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDO31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO31_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO31_A::_1)
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
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    pub fn pdo0(&self) -> PDO0_R {
        PDO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    pub fn pdo1(&self) -> PDO1_R {
        PDO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    pub fn pdo2(&self) -> PDO2_R {
        PDO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    pub fn pdo3(&self) -> PDO3_R {
        PDO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    pub fn pdo4(&self) -> PDO4_R {
        PDO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    pub fn pdo5(&self) -> PDO5_R {
        PDO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    pub fn pdo6(&self) -> PDO6_R {
        PDO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    pub fn pdo7(&self) -> PDO7_R {
        PDO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    pub fn pdo8(&self) -> PDO8_R {
        PDO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    pub fn pdo9(&self) -> PDO9_R {
        PDO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    pub fn pdo10(&self) -> PDO10_R {
        PDO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    pub fn pdo11(&self) -> PDO11_R {
        PDO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    pub fn pdo12(&self) -> PDO12_R {
        PDO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    pub fn pdo13(&self) -> PDO13_R {
        PDO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    pub fn pdo14(&self) -> PDO14_R {
        PDO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    pub fn pdo15(&self) -> PDO15_R {
        PDO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    pub fn pdo16(&self) -> PDO16_R {
        PDO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    pub fn pdo17(&self) -> PDO17_R {
        PDO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    pub fn pdo18(&self) -> PDO18_R {
        PDO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    pub fn pdo19(&self) -> PDO19_R {
        PDO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    pub fn pdo20(&self) -> PDO20_R {
        PDO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    pub fn pdo21(&self) -> PDO21_R {
        PDO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    pub fn pdo22(&self) -> PDO22_R {
        PDO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    pub fn pdo23(&self) -> PDO23_R {
        PDO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    pub fn pdo24(&self) -> PDO24_R {
        PDO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    pub fn pdo25(&self) -> PDO25_R {
        PDO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    pub fn pdo26(&self) -> PDO26_R {
        PDO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    pub fn pdo27(&self) -> PDO27_R {
        PDO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    pub fn pdo28(&self) -> PDO28_R {
        PDO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    pub fn pdo29(&self) -> PDO29_R {
        PDO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    pub fn pdo30(&self) -> PDO30_R {
        PDO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo31(&self) -> PDO31_R {
        PDO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    pub fn pdo0(&mut self) -> PDO0_W {
        PDO0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    pub fn pdo1(&mut self) -> PDO1_W {
        PDO1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    pub fn pdo2(&mut self) -> PDO2_W {
        PDO2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    pub fn pdo3(&mut self) -> PDO3_W {
        PDO3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    pub fn pdo4(&mut self) -> PDO4_W {
        PDO4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    pub fn pdo5(&mut self) -> PDO5_W {
        PDO5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    pub fn pdo6(&mut self) -> PDO6_W {
        PDO6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    pub fn pdo7(&mut self) -> PDO7_W {
        PDO7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    pub fn pdo8(&mut self) -> PDO8_W {
        PDO8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    pub fn pdo9(&mut self) -> PDO9_W {
        PDO9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    pub fn pdo10(&mut self) -> PDO10_W {
        PDO10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    pub fn pdo11(&mut self) -> PDO11_W {
        PDO11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    pub fn pdo12(&mut self) -> PDO12_W {
        PDO12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    pub fn pdo13(&mut self) -> PDO13_W {
        PDO13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    pub fn pdo14(&mut self) -> PDO14_W {
        PDO14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    pub fn pdo15(&mut self) -> PDO15_W {
        PDO15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    pub fn pdo16(&mut self) -> PDO16_W {
        PDO16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    pub fn pdo17(&mut self) -> PDO17_W {
        PDO17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    pub fn pdo18(&mut self) -> PDO18_W {
        PDO18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    pub fn pdo19(&mut self) -> PDO19_W {
        PDO19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    pub fn pdo20(&mut self) -> PDO20_W {
        PDO20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    pub fn pdo21(&mut self) -> PDO21_W {
        PDO21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    pub fn pdo22(&mut self) -> PDO22_W {
        PDO22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    pub fn pdo23(&mut self) -> PDO23_W {
        PDO23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    pub fn pdo24(&mut self) -> PDO24_W {
        PDO24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    pub fn pdo25(&mut self) -> PDO25_W {
        PDO25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    pub fn pdo26(&mut self) -> PDO26_W {
        PDO26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    pub fn pdo27(&mut self) -> PDO27_W {
        PDO27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    pub fn pdo28(&mut self) -> PDO28_W {
        PDO28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    pub fn pdo29(&mut self) -> PDO29_W {
        PDO29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    pub fn pdo30(&mut self) -> PDO30_W {
        PDO30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo31(&mut self) -> PDO31_W {
        PDO31_W { w: self }
    }
}
