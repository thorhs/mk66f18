#[doc = "Reader of register ISFR"]
pub type R = crate::R<u32, super::ISFR>;
#[doc = "Writer for register ISFR"]
pub type W = crate::W<u32, super::ISFR>;
#[doc = "Register ISFR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF0_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF0_A> for bool {
    #[inline(always)]
    fn from(variant: ISF0_A) -> Self {
        match variant {
            ISF0_A::_0 => false,
            ISF0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF0`"]
pub type ISF0_R = crate::R<bool, ISF0_A>;
impl ISF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF0_A {
        match self.bits {
            false => ISF0_A::_0,
            true => ISF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF0_A::_1
    }
}
#[doc = "Write proxy for field `ISF0`"]
pub struct ISF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF0_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF0_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF1_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF1_A> for bool {
    #[inline(always)]
    fn from(variant: ISF1_A) -> Self {
        match variant {
            ISF1_A::_0 => false,
            ISF1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF1`"]
pub type ISF1_R = crate::R<bool, ISF1_A>;
impl ISF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF1_A {
        match self.bits {
            false => ISF1_A::_0,
            true => ISF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF1_A::_1
    }
}
#[doc = "Write proxy for field `ISF1`"]
pub struct ISF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF1_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF1_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF2_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF2_A> for bool {
    #[inline(always)]
    fn from(variant: ISF2_A) -> Self {
        match variant {
            ISF2_A::_0 => false,
            ISF2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF2`"]
pub type ISF2_R = crate::R<bool, ISF2_A>;
impl ISF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF2_A {
        match self.bits {
            false => ISF2_A::_0,
            true => ISF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF2_A::_1
    }
}
#[doc = "Write proxy for field `ISF2`"]
pub struct ISF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF2_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF2_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF3_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF3_A> for bool {
    #[inline(always)]
    fn from(variant: ISF3_A) -> Self {
        match variant {
            ISF3_A::_0 => false,
            ISF3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF3`"]
pub type ISF3_R = crate::R<bool, ISF3_A>;
impl ISF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF3_A {
        match self.bits {
            false => ISF3_A::_0,
            true => ISF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF3_A::_1
    }
}
#[doc = "Write proxy for field `ISF3`"]
pub struct ISF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF3_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF3_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF4_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF4_A> for bool {
    #[inline(always)]
    fn from(variant: ISF4_A) -> Self {
        match variant {
            ISF4_A::_0 => false,
            ISF4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF4`"]
pub type ISF4_R = crate::R<bool, ISF4_A>;
impl ISF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF4_A {
        match self.bits {
            false => ISF4_A::_0,
            true => ISF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF4_A::_1
    }
}
#[doc = "Write proxy for field `ISF4`"]
pub struct ISF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF4_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF4_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF5_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF5_A> for bool {
    #[inline(always)]
    fn from(variant: ISF5_A) -> Self {
        match variant {
            ISF5_A::_0 => false,
            ISF5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF5`"]
pub type ISF5_R = crate::R<bool, ISF5_A>;
impl ISF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF5_A {
        match self.bits {
            false => ISF5_A::_0,
            true => ISF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF5_A::_1
    }
}
#[doc = "Write proxy for field `ISF5`"]
pub struct ISF5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF5_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF5_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF6_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF6_A> for bool {
    #[inline(always)]
    fn from(variant: ISF6_A) -> Self {
        match variant {
            ISF6_A::_0 => false,
            ISF6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF6`"]
pub type ISF6_R = crate::R<bool, ISF6_A>;
impl ISF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF6_A {
        match self.bits {
            false => ISF6_A::_0,
            true => ISF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF6_A::_1
    }
}
#[doc = "Write proxy for field `ISF6`"]
pub struct ISF6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF6_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF6_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF7_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF7_A> for bool {
    #[inline(always)]
    fn from(variant: ISF7_A) -> Self {
        match variant {
            ISF7_A::_0 => false,
            ISF7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF7`"]
pub type ISF7_R = crate::R<bool, ISF7_A>;
impl ISF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF7_A {
        match self.bits {
            false => ISF7_A::_0,
            true => ISF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF7_A::_1
    }
}
#[doc = "Write proxy for field `ISF7`"]
pub struct ISF7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF7_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF7_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF8_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF8_A> for bool {
    #[inline(always)]
    fn from(variant: ISF8_A) -> Self {
        match variant {
            ISF8_A::_0 => false,
            ISF8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF8`"]
pub type ISF8_R = crate::R<bool, ISF8_A>;
impl ISF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF8_A {
        match self.bits {
            false => ISF8_A::_0,
            true => ISF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF8_A::_1
    }
}
#[doc = "Write proxy for field `ISF8`"]
pub struct ISF8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF8_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF8_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF9_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF9_A> for bool {
    #[inline(always)]
    fn from(variant: ISF9_A) -> Self {
        match variant {
            ISF9_A::_0 => false,
            ISF9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF9`"]
pub type ISF9_R = crate::R<bool, ISF9_A>;
impl ISF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF9_A {
        match self.bits {
            false => ISF9_A::_0,
            true => ISF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF9_A::_1
    }
}
#[doc = "Write proxy for field `ISF9`"]
pub struct ISF9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF9_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF9_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF10_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF10_A> for bool {
    #[inline(always)]
    fn from(variant: ISF10_A) -> Self {
        match variant {
            ISF10_A::_0 => false,
            ISF10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF10`"]
pub type ISF10_R = crate::R<bool, ISF10_A>;
impl ISF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF10_A {
        match self.bits {
            false => ISF10_A::_0,
            true => ISF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF10_A::_1
    }
}
#[doc = "Write proxy for field `ISF10`"]
pub struct ISF10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF10_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF10_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF11_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF11_A> for bool {
    #[inline(always)]
    fn from(variant: ISF11_A) -> Self {
        match variant {
            ISF11_A::_0 => false,
            ISF11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF11`"]
pub type ISF11_R = crate::R<bool, ISF11_A>;
impl ISF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF11_A {
        match self.bits {
            false => ISF11_A::_0,
            true => ISF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF11_A::_1
    }
}
#[doc = "Write proxy for field `ISF11`"]
pub struct ISF11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF11_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF11_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF12_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF12_A> for bool {
    #[inline(always)]
    fn from(variant: ISF12_A) -> Self {
        match variant {
            ISF12_A::_0 => false,
            ISF12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF12`"]
pub type ISF12_R = crate::R<bool, ISF12_A>;
impl ISF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF12_A {
        match self.bits {
            false => ISF12_A::_0,
            true => ISF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF12_A::_1
    }
}
#[doc = "Write proxy for field `ISF12`"]
pub struct ISF12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF12_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF12_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF13_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF13_A> for bool {
    #[inline(always)]
    fn from(variant: ISF13_A) -> Self {
        match variant {
            ISF13_A::_0 => false,
            ISF13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF13`"]
pub type ISF13_R = crate::R<bool, ISF13_A>;
impl ISF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF13_A {
        match self.bits {
            false => ISF13_A::_0,
            true => ISF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF13_A::_1
    }
}
#[doc = "Write proxy for field `ISF13`"]
pub struct ISF13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF13_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF13_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF14_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF14_A> for bool {
    #[inline(always)]
    fn from(variant: ISF14_A) -> Self {
        match variant {
            ISF14_A::_0 => false,
            ISF14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF14`"]
pub type ISF14_R = crate::R<bool, ISF14_A>;
impl ISF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF14_A {
        match self.bits {
            false => ISF14_A::_0,
            true => ISF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF14_A::_1
    }
}
#[doc = "Write proxy for field `ISF14`"]
pub struct ISF14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF14_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF14_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF15_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF15_A> for bool {
    #[inline(always)]
    fn from(variant: ISF15_A) -> Self {
        match variant {
            ISF15_A::_0 => false,
            ISF15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF15`"]
pub type ISF15_R = crate::R<bool, ISF15_A>;
impl ISF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF15_A {
        match self.bits {
            false => ISF15_A::_0,
            true => ISF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF15_A::_1
    }
}
#[doc = "Write proxy for field `ISF15`"]
pub struct ISF15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF15_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF15_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF16_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF16_A> for bool {
    #[inline(always)]
    fn from(variant: ISF16_A) -> Self {
        match variant {
            ISF16_A::_0 => false,
            ISF16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF16`"]
pub type ISF16_R = crate::R<bool, ISF16_A>;
impl ISF16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF16_A {
        match self.bits {
            false => ISF16_A::_0,
            true => ISF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF16_A::_1
    }
}
#[doc = "Write proxy for field `ISF16`"]
pub struct ISF16_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF16_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF16_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF17_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF17_A> for bool {
    #[inline(always)]
    fn from(variant: ISF17_A) -> Self {
        match variant {
            ISF17_A::_0 => false,
            ISF17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF17`"]
pub type ISF17_R = crate::R<bool, ISF17_A>;
impl ISF17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF17_A {
        match self.bits {
            false => ISF17_A::_0,
            true => ISF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF17_A::_1
    }
}
#[doc = "Write proxy for field `ISF17`"]
pub struct ISF17_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF17_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF17_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF18_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF18_A> for bool {
    #[inline(always)]
    fn from(variant: ISF18_A) -> Self {
        match variant {
            ISF18_A::_0 => false,
            ISF18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF18`"]
pub type ISF18_R = crate::R<bool, ISF18_A>;
impl ISF18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF18_A {
        match self.bits {
            false => ISF18_A::_0,
            true => ISF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF18_A::_1
    }
}
#[doc = "Write proxy for field `ISF18`"]
pub struct ISF18_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF18_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF18_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF19_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF19_A> for bool {
    #[inline(always)]
    fn from(variant: ISF19_A) -> Self {
        match variant {
            ISF19_A::_0 => false,
            ISF19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF19`"]
pub type ISF19_R = crate::R<bool, ISF19_A>;
impl ISF19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF19_A {
        match self.bits {
            false => ISF19_A::_0,
            true => ISF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF19_A::_1
    }
}
#[doc = "Write proxy for field `ISF19`"]
pub struct ISF19_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF19_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF19_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF20_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF20_A> for bool {
    #[inline(always)]
    fn from(variant: ISF20_A) -> Self {
        match variant {
            ISF20_A::_0 => false,
            ISF20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF20`"]
pub type ISF20_R = crate::R<bool, ISF20_A>;
impl ISF20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF20_A {
        match self.bits {
            false => ISF20_A::_0,
            true => ISF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF20_A::_1
    }
}
#[doc = "Write proxy for field `ISF20`"]
pub struct ISF20_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF20_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF20_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF21_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF21_A> for bool {
    #[inline(always)]
    fn from(variant: ISF21_A) -> Self {
        match variant {
            ISF21_A::_0 => false,
            ISF21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF21`"]
pub type ISF21_R = crate::R<bool, ISF21_A>;
impl ISF21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF21_A {
        match self.bits {
            false => ISF21_A::_0,
            true => ISF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF21_A::_1
    }
}
#[doc = "Write proxy for field `ISF21`"]
pub struct ISF21_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF21_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF21_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF22_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF22_A> for bool {
    #[inline(always)]
    fn from(variant: ISF22_A) -> Self {
        match variant {
            ISF22_A::_0 => false,
            ISF22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF22`"]
pub type ISF22_R = crate::R<bool, ISF22_A>;
impl ISF22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF22_A {
        match self.bits {
            false => ISF22_A::_0,
            true => ISF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF22_A::_1
    }
}
#[doc = "Write proxy for field `ISF22`"]
pub struct ISF22_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF22_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF22_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF23_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF23_A> for bool {
    #[inline(always)]
    fn from(variant: ISF23_A) -> Self {
        match variant {
            ISF23_A::_0 => false,
            ISF23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF23`"]
pub type ISF23_R = crate::R<bool, ISF23_A>;
impl ISF23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF23_A {
        match self.bits {
            false => ISF23_A::_0,
            true => ISF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF23_A::_1
    }
}
#[doc = "Write proxy for field `ISF23`"]
pub struct ISF23_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF23_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF23_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF24_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF24_A> for bool {
    #[inline(always)]
    fn from(variant: ISF24_A) -> Self {
        match variant {
            ISF24_A::_0 => false,
            ISF24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF24`"]
pub type ISF24_R = crate::R<bool, ISF24_A>;
impl ISF24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF24_A {
        match self.bits {
            false => ISF24_A::_0,
            true => ISF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF24_A::_1
    }
}
#[doc = "Write proxy for field `ISF24`"]
pub struct ISF24_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF24_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF24_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF25_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF25_A> for bool {
    #[inline(always)]
    fn from(variant: ISF25_A) -> Self {
        match variant {
            ISF25_A::_0 => false,
            ISF25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF25`"]
pub type ISF25_R = crate::R<bool, ISF25_A>;
impl ISF25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF25_A {
        match self.bits {
            false => ISF25_A::_0,
            true => ISF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF25_A::_1
    }
}
#[doc = "Write proxy for field `ISF25`"]
pub struct ISF25_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF25_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF25_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF26_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF26_A> for bool {
    #[inline(always)]
    fn from(variant: ISF26_A) -> Self {
        match variant {
            ISF26_A::_0 => false,
            ISF26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF26`"]
pub type ISF26_R = crate::R<bool, ISF26_A>;
impl ISF26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF26_A {
        match self.bits {
            false => ISF26_A::_0,
            true => ISF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF26_A::_1
    }
}
#[doc = "Write proxy for field `ISF26`"]
pub struct ISF26_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF26_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF26_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF27_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF27_A> for bool {
    #[inline(always)]
    fn from(variant: ISF27_A) -> Self {
        match variant {
            ISF27_A::_0 => false,
            ISF27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF27`"]
pub type ISF27_R = crate::R<bool, ISF27_A>;
impl ISF27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF27_A {
        match self.bits {
            false => ISF27_A::_0,
            true => ISF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF27_A::_1
    }
}
#[doc = "Write proxy for field `ISF27`"]
pub struct ISF27_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF27_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF27_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF28_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF28_A> for bool {
    #[inline(always)]
    fn from(variant: ISF28_A) -> Self {
        match variant {
            ISF28_A::_0 => false,
            ISF28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF28`"]
pub type ISF28_R = crate::R<bool, ISF28_A>;
impl ISF28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF28_A {
        match self.bits {
            false => ISF28_A::_0,
            true => ISF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF28_A::_1
    }
}
#[doc = "Write proxy for field `ISF28`"]
pub struct ISF28_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF28_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF28_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF29_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF29_A> for bool {
    #[inline(always)]
    fn from(variant: ISF29_A) -> Self {
        match variant {
            ISF29_A::_0 => false,
            ISF29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF29`"]
pub type ISF29_R = crate::R<bool, ISF29_A>;
impl ISF29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF29_A {
        match self.bits {
            false => ISF29_A::_0,
            true => ISF29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF29_A::_1
    }
}
#[doc = "Write proxy for field `ISF29`"]
pub struct ISF29_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF29_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF29_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF30_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF30_A> for bool {
    #[inline(always)]
    fn from(variant: ISF30_A) -> Self {
        match variant {
            ISF30_A::_0 => false,
            ISF30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF30`"]
pub type ISF30_R = crate::R<bool, ISF30_A>;
impl ISF30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF30_A {
        match self.bits {
            false => ISF30_A::_0,
            true => ISF30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF30_A::_1
    }
}
#[doc = "Write proxy for field `ISF30`"]
pub struct ISF30_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF30_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF30_A::_1)
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
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF31_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl From<ISF31_A> for bool {
    #[inline(always)]
    fn from(variant: ISF31_A) -> Self {
        match variant {
            ISF31_A::_0 => false,
            ISF31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ISF31`"]
pub type ISF31_R = crate::R<bool, ISF31_A>;
impl ISF31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF31_A {
        match self.bits {
            false => ISF31_A::_0,
            true => ISF31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF31_A::_1
    }
}
#[doc = "Write proxy for field `ISF31`"]
pub struct ISF31_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF31_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF31_A::_1)
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
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&self) -> ISF0_R {
        ISF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&self) -> ISF1_R {
        ISF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&self) -> ISF2_R {
        ISF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&self) -> ISF3_R {
        ISF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&self) -> ISF4_R {
        ISF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&self) -> ISF5_R {
        ISF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&self) -> ISF6_R {
        ISF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&self) -> ISF7_R {
        ISF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&self) -> ISF8_R {
        ISF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&self) -> ISF9_R {
        ISF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&self) -> ISF10_R {
        ISF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&self) -> ISF11_R {
        ISF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&self) -> ISF12_R {
        ISF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&self) -> ISF13_R {
        ISF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&self) -> ISF14_R {
        ISF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&self) -> ISF15_R {
        ISF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&self) -> ISF16_R {
        ISF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&self) -> ISF17_R {
        ISF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&self) -> ISF18_R {
        ISF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&self) -> ISF19_R {
        ISF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&self) -> ISF20_R {
        ISF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&self) -> ISF21_R {
        ISF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&self) -> ISF22_R {
        ISF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&self) -> ISF23_R {
        ISF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&self) -> ISF24_R {
        ISF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&self) -> ISF25_R {
        ISF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&self) -> ISF26_R {
        ISF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&self) -> ISF27_R {
        ISF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&self) -> ISF28_R {
        ISF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&self) -> ISF29_R {
        ISF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&self) -> ISF30_R {
        ISF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&self) -> ISF31_R {
        ISF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&mut self) -> ISF0_W {
        ISF0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&mut self) -> ISF1_W {
        ISF1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&mut self) -> ISF2_W {
        ISF2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&mut self) -> ISF3_W {
        ISF3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&mut self) -> ISF4_W {
        ISF4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&mut self) -> ISF5_W {
        ISF5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&mut self) -> ISF6_W {
        ISF6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&mut self) -> ISF7_W {
        ISF7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&mut self) -> ISF8_W {
        ISF8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&mut self) -> ISF9_W {
        ISF9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&mut self) -> ISF10_W {
        ISF10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&mut self) -> ISF11_W {
        ISF11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&mut self) -> ISF12_W {
        ISF12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&mut self) -> ISF13_W {
        ISF13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&mut self) -> ISF14_W {
        ISF14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&mut self) -> ISF15_W {
        ISF15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&mut self) -> ISF16_W {
        ISF16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&mut self) -> ISF17_W {
        ISF17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&mut self) -> ISF18_W {
        ISF18_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&mut self) -> ISF19_W {
        ISF19_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&mut self) -> ISF20_W {
        ISF20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&mut self) -> ISF21_W {
        ISF21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&mut self) -> ISF22_W {
        ISF22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&mut self) -> ISF23_W {
        ISF23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&mut self) -> ISF24_W {
        ISF24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&mut self) -> ISF25_W {
        ISF25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&mut self) -> ISF26_W {
        ISF26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&mut self) -> ISF27_W {
        ISF27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&mut self) -> ISF28_W {
        ISF28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&mut self) -> ISF29_W {
        ISF29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&mut self) -> ISF30_W {
        ISF30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&mut self) -> ISF31_W {
        ISF31_W { w: self }
    }
}
