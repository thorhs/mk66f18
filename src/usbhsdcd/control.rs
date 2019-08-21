#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Interrupt Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACK_AW {
    #[doc = "0: Do not clear the interrupt."]
    _0,
    #[doc = "1: Clear the IF bit (interrupt flag)."]
    _1,
}
impl From<IACK_AW> for bool {
    #[inline(always)]
    fn from(variant: IACK_AW) -> Self {
        match variant {
            IACK_AW::_0 => false,
            IACK_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `IACK`"]
pub struct IACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IACK_AW::_0)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IACK_AW::_1)
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
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF_A {
    #[doc = "0: No interrupt is pending."]
    _0,
    #[doc = "1: An interrupt is pending."]
    _1,
}
impl From<IF_A> for bool {
    #[inline(always)]
    fn from(variant: IF_A) -> Self {
        match variant {
            IF_A::_0 => false,
            IF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IF`"]
pub type IF_R = crate::R<bool, IF_A>;
impl IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF_A {
        match self.bits {
            false => IF_A::_0,
            true => IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF_A::_1
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Disable interrupts to the system."]
    _0,
    #[doc = "1: Enable interrupts to the system."]
    _1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        match variant {
            IE_A::_0 => false,
            IE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, IE_A>;
impl IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::_0,
            true => IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_A::_1
    }
}
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_A::_0)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_A::_1)
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
#[doc = "BC1.2 compatibility. This bit cannot be changed after start detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BC12_A {
    #[doc = "0: Compatible with BC1.1 (default)"]
    _0,
    #[doc = "1: Compatible with BC1.2"]
    _1,
}
impl From<BC12_A> for bool {
    #[inline(always)]
    fn from(variant: BC12_A) -> Self {
        match variant {
            BC12_A::_0 => false,
            BC12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BC12`"]
pub type BC12_R = crate::R<bool, BC12_A>;
impl BC12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC12_A {
        match self.bits {
            false => BC12_A::_0,
            true => BC12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BC12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BC12_A::_1
    }
}
#[doc = "Write proxy for field `BC12`"]
pub struct BC12_W<'a> {
    w: &'a mut W,
}
impl<'a> BC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BC12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BC12_A::_0)
    }
    #[doc = "Compatible with BC1.2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BC12_A::_1)
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
#[doc = "Start Change Detection Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "0: Do not start the sequence. Writes of this value have no effect."]
    _0,
    #[doc = "1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    _1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        match variant {
            START_AW::_0 => false,
            START_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_AW::_0)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_AW::_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_AW {
    #[doc = "0: Do not perform a software reset."]
    _0,
    #[doc = "1: Perform a software reset."]
    _1,
}
impl From<SR_AW> for bool {
    #[inline(always)]
    fn from(variant: SR_AW) -> Self {
        match variant {
            SR_AW::_0 => false,
            SR_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SR_AW::_0)
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SR_AW::_1)
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
impl R {
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline(always)]
    pub fn bc12(&self) -> BC12_R {
        BC12_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    pub fn iack(&mut self) -> IACK_W {
        IACK_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline(always)]
    pub fn bc12(&mut self) -> BC12_W {
        BC12_W { w: self }
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
}
