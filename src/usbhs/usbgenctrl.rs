#[doc = "Reader of register USBGENCTRL"]
pub type R = crate::R<u32, super::USBGENCTRL>;
#[doc = "Writer for register USBGENCTRL"]
pub type W = crate::W<u32, super::USBGENCTRL>;
#[doc = "Register USBGENCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBGENCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU_IE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<WU_IE_A> for bool {
    #[inline(always)]
    fn from(variant: WU_IE_A) -> Self {
        match variant {
            WU_IE_A::_0 => false,
            WU_IE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WU_IE`"]
pub type WU_IE_R = crate::R<bool, WU_IE_A>;
impl WU_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WU_IE_A {
        match self.bits {
            false => WU_IE_A::_0,
            true => WU_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WU_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WU_IE_A::_1
    }
}
#[doc = "Write proxy for field `WU_IE`"]
pub struct WU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WU_IE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WU_IE_A::_1)
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
#[doc = "Wakeup Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU_INT_CLR_A {
    #[doc = "0: Default, no action."]
    _0,
    #[doc = "1: Clear the wake-up interrupt."]
    _1,
}
impl From<WU_INT_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: WU_INT_CLR_A) -> Self {
        match variant {
            WU_INT_CLR_A::_0 => false,
            WU_INT_CLR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WU_INT_CLR`"]
pub type WU_INT_CLR_R = crate::R<bool, WU_INT_CLR_A>;
impl WU_INT_CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WU_INT_CLR_A {
        match self.bits {
            false => WU_INT_CLR_A::_0,
            true => WU_INT_CLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WU_INT_CLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WU_INT_CLR_A::_1
    }
}
#[doc = "Write proxy for field `WU_INT_CLR`"]
pub struct WU_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_INT_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WU_INT_CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Default, no action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WU_INT_CLR_A::_0)
    }
    #[doc = "Clear the wake-up interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WU_INT_CLR_A::_1)
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
impl R {
    #[doc = "Bit 0 - Wakeup Interrupt Enable"]
    #[inline(always)]
    pub fn wu_ie(&self) -> WU_IE_R {
        WU_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Interrupt Clear"]
    #[inline(always)]
    pub fn wu_int_clr(&self) -> WU_INT_CLR_R {
        WU_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Interrupt Enable"]
    #[inline(always)]
    pub fn wu_ie(&mut self) -> WU_IE_W {
        WU_IE_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Interrupt Clear"]
    #[inline(always)]
    pub fn wu_int_clr(&mut self) -> WU_INT_CLR_W {
        WU_INT_CLR_W { w: self }
    }
}
