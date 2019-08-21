#[doc = "Reader of register SIGNAL_OVERRIDE"]
pub type R = crate::R<u32, super::SIGNAL_OVERRIDE>;
#[doc = "Writer for register SIGNAL_OVERRIDE"]
pub type W = crate::W<u32, super::SIGNAL_OVERRIDE>;
#[doc = "Register SIGNAL_OVERRIDE `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGNAL_OVERRIDE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Phase Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    _00,
    #[doc = "1: Reserved, not for customer use."]
    _01,
    #[doc = "2: Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    _10,
    #[doc = "3: Reserved, not for customer use."]
    _11,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::_00 => 0,
            PS_A::_01 => 1,
            PS_A::_10 => 2,
            PS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_00,
            1 => PS_A::_01,
            2 => PS_A::_10,
            3 => PS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PS_A::_11
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PS_A::_00)
    }
    #[doc = "Reserved, not for customer use."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PS_A::_01)
    }
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PS_A::_10)
    }
    #[doc = "Reserved, not for customer use."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
}
