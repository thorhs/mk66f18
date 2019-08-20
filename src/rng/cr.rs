#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Go\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GO_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<GO_A> for bool {
    #[inline(always)]
    fn from(variant: GO_A) -> Self {
        match variant {
            GO_A::_0 => false,
            GO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `GO`"]
pub type GO_R = crate::R<bool, GO_A>;
impl GO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GO_A {
        match self.bits {
            false => GO_A::_0,
            true => GO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GO_A::_1
    }
}
#[doc = "Write proxy for field `GO`"]
pub struct GO_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GO_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GO_A::_1)
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
#[doc = "High Assurance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HA_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<HA_A> for bool {
    #[inline(always)]
    fn from(variant: HA_A) -> Self {
        match variant {
            HA_A::_0 => false,
            HA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HA`"]
pub type HA_R = crate::R<bool, HA_A>;
impl HA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HA_A {
        match self.bits {
            false => HA_A::_0,
            true => HA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HA_A::_1
    }
}
#[doc = "Write proxy for field `HA`"]
pub struct HA_W<'a> {
    w: &'a mut W,
}
impl<'a> HA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HA_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HA_A::_1)
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
#[doc = "Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTM_A {
    #[doc = "0: Not masked"]
    _0,
    #[doc = "1: Masked"]
    _1,
}
impl From<INTM_A> for bool {
    #[inline(always)]
    fn from(variant: INTM_A) -> Self {
        match variant {
            INTM_A::_0 => false,
            INTM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `INTM`"]
pub type INTM_R = crate::R<bool, INTM_A>;
impl INTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTM_A {
        match self.bits {
            false => INTM_A::_0,
            true => INTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTM_A::_1
    }
}
#[doc = "Write proxy for field `INTM`"]
pub struct INTM_W<'a> {
    w: &'a mut W,
}
impl<'a> INTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTM_A::_0)
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTM_A::_1)
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
#[doc = "Clear Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRI_AW {
    #[doc = "0: Do not clear the interrupt."]
    _0,
    #[doc = "1: Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    _1,
}
impl From<CLRI_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRI_AW) -> Self {
        match variant {
            CLRI_AW::_0 => false,
            CLRI_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CLRI`"]
pub struct CLRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLRI_AW::_0)
    }
    #[doc = "Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLRI_AW::_1)
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
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLP_A {
    #[doc = "0: Normal mode"]
    _0,
    #[doc = "1: Sleep (low-power) mode"]
    _1,
}
impl From<SLP_A> for bool {
    #[inline(always)]
    fn from(variant: SLP_A) -> Self {
        match variant {
            SLP_A::_0 => false,
            SLP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLP`"]
pub type SLP_R = crate::R<bool, SLP_A>;
impl SLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_A {
        match self.bits {
            false => SLP_A::_0,
            true => SLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLP_A::_1
    }
}
#[doc = "Write proxy for field `SLP`"]
pub struct SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLP_A::_0)
    }
    #[doc = "Sleep (low-power) mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLP_A::_1)
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
impl R {
    #[doc = "Bit 0 - Go"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    pub fn slp(&self) -> SLP_R {
        SLP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Go"]
    #[inline(always)]
    pub fn go(&mut self) -> GO_W {
        GO_W { w: self }
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W {
        HA_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W {
        INTM_W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt"]
    #[inline(always)]
    pub fn clri(&mut self) -> CLRI_W {
        CLRI_W { w: self }
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    pub fn slp(&mut self) -> SLP_W {
        SLP_W { w: self }
    }
}
