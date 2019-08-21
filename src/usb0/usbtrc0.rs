#[doc = "Reader of register USBTRC0"]
pub type R = crate::R<u8, super::USBTRC0>;
#[doc = "Writer for register USBTRC0"]
pub type W = crate::W<u8, super::USBTRC0>;
#[doc = "Register USBTRC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBTRC0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB Asynchronous Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RESUME_INT_A {
    #[doc = "0: No interrupt was generated."]
    _0,
    #[doc = "1: Interrupt was generated because of the USB asynchronous interrupt."]
    _1,
}
impl From<USB_RESUME_INT_A> for bool {
    #[inline(always)]
    fn from(variant: USB_RESUME_INT_A) -> Self {
        match variant {
            USB_RESUME_INT_A::_0 => false,
            USB_RESUME_INT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USB_RESUME_INT`"]
pub type USB_RESUME_INT_R = crate::R<bool, USB_RESUME_INT_A>;
impl USB_RESUME_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_RESUME_INT_A {
        match self.bits {
            false => USB_RESUME_INT_A::_0,
            true => USB_RESUME_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USB_RESUME_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USB_RESUME_INT_A::_1
    }
}
#[doc = "Synchronous USB Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_DET_A {
    #[doc = "0: Synchronous interrupt has not been detected."]
    _0,
    #[doc = "1: Synchronous interrupt has been detected."]
    _1,
}
impl From<SYNC_DET_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_DET_A) -> Self {
        match variant {
            SYNC_DET_A::_0 => false,
            SYNC_DET_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SYNC_DET`"]
pub type SYNC_DET_R = crate::R<bool, SYNC_DET_A>;
impl SYNC_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_DET_A {
        match self.bits {
            false => SYNC_DET_A::_0,
            true => SYNC_DET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_DET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_DET_A::_1
    }
}
#[doc = "Reader of field `USB_CLK_RECOVERY_INT`"]
pub type USB_CLK_RECOVERY_INT_R = crate::R<bool, bool>;
#[doc = "Asynchronous Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESMEN_A {
    #[doc = "0: USB asynchronous wakeup from suspend mode disabled."]
    _0,
    #[doc = "1: USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    _1,
}
impl From<USBRESMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBRESMEN_A) -> Self {
        match variant {
            USBRESMEN_A::_0 => false,
            USBRESMEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBRESMEN`"]
pub type USBRESMEN_R = crate::R<bool, USBRESMEN_A>;
impl USBRESMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRESMEN_A {
        match self.bits {
            false => USBRESMEN_A::_0,
            true => USBRESMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBRESMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBRESMEN_A::_1
    }
}
#[doc = "Write proxy for field `USBRESMEN`"]
pub struct USBRESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESMEN_A::_0)
    }
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESMEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "USB Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_AW {
    #[doc = "0: Normal USB module operation."]
    _0,
    #[doc = "1: Returns the USB module to its reset state."]
    _1,
}
impl From<USBRESET_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_AW) -> Self {
        match variant {
            USBRESET_AW::_0 => false,
            USBRESET_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USBRESET`"]
pub struct USBRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal USB module operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESET_AW::_0)
    }
    #[doc = "Returns the USB module to its reset state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESET_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB Asynchronous Interrupt"]
    #[inline(always)]
    pub fn usb_resume_int(&self) -> USB_RESUME_INT_R {
        USB_RESUME_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronous USB Interrupt Detect"]
    #[inline(always)]
    pub fn sync_det(&self) -> SYNC_DET_R {
        SYNC_DET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Combined USB Clock Recovery interrupt status"]
    #[inline(always)]
    pub fn usb_clk_recovery_int(&self) -> USB_CLK_RECOVERY_INT_R {
        USB_CLK_RECOVERY_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn usbresmen(&self) -> USBRESMEN_R {
        USBRESMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn usbresmen(&mut self) -> USBRESMEN_W {
        USBRESMEN_W { w: self }
    }
    #[doc = "Bit 7 - USB Reset"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> USBRESET_W {
        USBRESET_W { w: self }
    }
}
