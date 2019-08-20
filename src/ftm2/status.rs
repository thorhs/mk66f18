#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH0F_A> for bool {
    #[inline(always)]
    fn from(variant: CH0F_A) -> Self {
        match variant {
            CH0F_A::_0 => false,
            CH0F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH0F`"]
pub type CH0F_R = crate::R<bool, CH0F_A>;
impl CH0F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0F_A {
        match self.bits {
            false => CH0F_A::_0,
            true => CH0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0F_A::_1
    }
}
#[doc = "Write proxy for field `CH0F`"]
pub struct CH0F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0F_A::_1)
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
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        match variant {
            CH1F_A::_0 => false,
            CH1F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, CH1F_A>;
impl CH1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::_0,
            true => CH1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1F_A::_1
    }
}
#[doc = "Write proxy for field `CH1F`"]
pub struct CH1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1F_A::_1)
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
#[doc = "Channel 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH2F_A> for bool {
    #[inline(always)]
    fn from(variant: CH2F_A) -> Self {
        match variant {
            CH2F_A::_0 => false,
            CH2F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH2F`"]
pub type CH2F_R = crate::R<bool, CH2F_A>;
impl CH2F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2F_A {
        match self.bits {
            false => CH2F_A::_0,
            true => CH2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2F_A::_1
    }
}
#[doc = "Write proxy for field `CH2F`"]
pub struct CH2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2F_A::_1)
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
#[doc = "Channel 3 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH3F_A> for bool {
    #[inline(always)]
    fn from(variant: CH3F_A) -> Self {
        match variant {
            CH3F_A::_0 => false,
            CH3F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH3F`"]
pub type CH3F_R = crate::R<bool, CH3F_A>;
impl CH3F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3F_A {
        match self.bits {
            false => CH3F_A::_0,
            true => CH3F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3F_A::_1
    }
}
#[doc = "Write proxy for field `CH3F`"]
pub struct CH3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3F_A::_1)
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
#[doc = "Channel 4 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH4F_A> for bool {
    #[inline(always)]
    fn from(variant: CH4F_A) -> Self {
        match variant {
            CH4F_A::_0 => false,
            CH4F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH4F`"]
pub type CH4F_R = crate::R<bool, CH4F_A>;
impl CH4F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4F_A {
        match self.bits {
            false => CH4F_A::_0,
            true => CH4F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4F_A::_1
    }
}
#[doc = "Write proxy for field `CH4F`"]
pub struct CH4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4F_A::_1)
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
#[doc = "Channel 5 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH5F_A> for bool {
    #[inline(always)]
    fn from(variant: CH5F_A) -> Self {
        match variant {
            CH5F_A::_0 => false,
            CH5F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH5F`"]
pub type CH5F_R = crate::R<bool, CH5F_A>;
impl CH5F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5F_A {
        match self.bits {
            false => CH5F_A::_0,
            true => CH5F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5F_A::_1
    }
}
#[doc = "Write proxy for field `CH5F`"]
pub struct CH5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5F_A::_1)
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
#[doc = "Channel 6 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH6F_A> for bool {
    #[inline(always)]
    fn from(variant: CH6F_A) -> Self {
        match variant {
            CH6F_A::_0 => false,
            CH6F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH6F`"]
pub type CH6F_R = crate::R<bool, CH6F_A>;
impl CH6F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6F_A {
        match self.bits {
            false => CH6F_A::_0,
            true => CH6F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6F_A::_1
    }
}
#[doc = "Write proxy for field `CH6F`"]
pub struct CH6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6F_A::_1)
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
#[doc = "Channel 7 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7F_A {
    #[doc = "0: No channel event has occurred."]
    _0,
    #[doc = "1: A channel event has occurred."]
    _1,
}
impl From<CH7F_A> for bool {
    #[inline(always)]
    fn from(variant: CH7F_A) -> Self {
        match variant {
            CH7F_A::_0 => false,
            CH7F_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CH7F`"]
pub type CH7F_R = crate::R<bool, CH7F_A>;
impl CH7F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7F_A {
        match self.bits {
            false => CH7F_A::_0,
            true => CH7F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7F_A::_1
    }
}
#[doc = "Write proxy for field `CH7F`"]
pub struct CH7F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7F_A::_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline(always)]
    pub fn ch7f(&self) -> CH7F_R {
        CH7F_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&mut self) -> CH0F_W {
        CH0F_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&mut self) -> CH1F_W {
        CH1F_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&mut self) -> CH2F_W {
        CH2F_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&mut self) -> CH3F_W {
        CH3F_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&mut self) -> CH4F_W {
        CH4F_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&mut self) -> CH5F_W {
        CH5F_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline(always)]
    pub fn ch6f(&mut self) -> CH6F_W {
        CH6F_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline(always)]
    pub fn ch7f(&mut self) -> CH7F_W {
        CH7F_W { w: self }
    }
}
