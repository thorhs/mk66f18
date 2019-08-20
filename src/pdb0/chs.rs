#[doc = "Reader of register CH%sS"]
pub type R = crate::R<u32, super::CHS>;
#[doc = "Writer for register CH%sS"]
pub type W = crate::W<u32, super::CHS>;
#[doc = "Register CH%sS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR0_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        match variant {
            ERR0_A::_0 => false,
            ERR0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR0`"]
pub type ERR0_R = crate::R<bool, ERR0_A>;
impl ERR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::_0,
            true => ERR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR0_A::_1
    }
}
#[doc = "Write proxy for field `ERR0`"]
pub struct ERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR1_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        match variant {
            ERR1_A::_0 => false,
            ERR1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR1`"]
pub type ERR1_R = crate::R<bool, ERR1_A>;
impl ERR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::_0,
            true => ERR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR1_A::_1
    }
}
#[doc = "Write proxy for field `ERR1`"]
pub struct ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR2_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        match variant {
            ERR2_A::_0 => false,
            ERR2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR2`"]
pub type ERR2_R = crate::R<bool, ERR2_A>;
impl ERR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::_0,
            true => ERR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR2_A::_1
    }
}
#[doc = "Write proxy for field `ERR2`"]
pub struct ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR3_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        match variant {
            ERR3_A::_0 => false,
            ERR3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR3`"]
pub type ERR3_R = crate::R<bool, ERR3_A>;
impl ERR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::_0,
            true => ERR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR3_A::_1
    }
}
#[doc = "Write proxy for field `ERR3`"]
pub struct ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR4_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        match variant {
            ERR4_A::_0 => false,
            ERR4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR4`"]
pub type ERR4_R = crate::R<bool, ERR4_A>;
impl ERR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::_0,
            true => ERR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR4_A::_1
    }
}
#[doc = "Write proxy for field `ERR4`"]
pub struct ERR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR5_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        match variant {
            ERR5_A::_0 => false,
            ERR5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR5`"]
pub type ERR5_R = crate::R<bool, ERR5_A>;
impl ERR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::_0,
            true => ERR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR5_A::_1
    }
}
#[doc = "Write proxy for field `ERR5`"]
pub struct ERR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR6_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        match variant {
            ERR6_A::_0 => false,
            ERR6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR6`"]
pub type ERR6_R = crate::R<bool, ERR6_A>;
impl ERR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::_0,
            true => ERR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR6_A::_1
    }
}
#[doc = "Write proxy for field `ERR6`"]
pub struct ERR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6_A::_1)
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
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR7_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    _1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        match variant {
            ERR7_A::_0 => false,
            ERR7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR7`"]
pub type ERR7_R = crate::R<bool, ERR7_A>;
impl ERR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::_0,
            true => ERR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR7_A::_1
    }
}
#[doc = "Write proxy for field `ERR7`"]
pub struct ERR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\] is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7_A::_1)
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
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CF`"]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err0(&mut self) -> ERR0_W {
        ERR0_W { w: self }
    }
    #[doc = "Bit 1 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err1(&mut self) -> ERR1_W {
        ERR1_W { w: self }
    }
    #[doc = "Bit 2 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err2(&mut self) -> ERR2_W {
        ERR2_W { w: self }
    }
    #[doc = "Bit 3 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err3(&mut self) -> ERR3_W {
        ERR3_W { w: self }
    }
    #[doc = "Bit 4 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err4(&mut self) -> ERR4_W {
        ERR4_W { w: self }
    }
    #[doc = "Bit 5 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err5(&mut self) -> ERR5_W {
        ERR5_W { w: self }
    }
    #[doc = "Bit 6 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err6(&mut self) -> ERR6_W {
        ERR6_W { w: self }
    }
    #[doc = "Bit 7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err7(&mut self) -> ERR7_W {
        ERR7_W { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
}
