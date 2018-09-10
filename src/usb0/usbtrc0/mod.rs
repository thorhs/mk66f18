#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::USBTRC0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `USB_RESUME_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RESUME_INTR {
    #[doc = "No interrupt was generated."]
    _0,
    #[doc = "Interrupt was generated because of the USB asynchronous interrupt."]
    _1,
}
impl USB_RESUME_INTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USB_RESUME_INTR::_0 => false,
            USB_RESUME_INTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_RESUME_INTR {
        match value {
            false => USB_RESUME_INTR::_0,
            true => USB_RESUME_INTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USB_RESUME_INTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USB_RESUME_INTR::_1
    }
}
#[doc = "Possible values of the field `SYNC_DET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_DETR {
    #[doc = "Synchronous interrupt has not been detected."]
    _0,
    #[doc = "Synchronous interrupt has been detected."]
    _1,
}
impl SYNC_DETR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYNC_DETR::_0 => false,
            SYNC_DETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNC_DETR {
        match value {
            false => SYNC_DETR::_0,
            true => SYNC_DETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNC_DETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNC_DETR::_1
    }
}
#[doc = r" Value of the field"]
pub struct USB_CLK_RECOVERY_INTR {
    bits: bool,
}
impl USB_CLK_RECOVERY_INTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `USBRESMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESMENR {
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    _0,
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    _1,
}
impl USBRESMENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USBRESMENR::_0 => false,
            USBRESMENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRESMENR {
        match value {
            false => USBRESMENR::_0,
            true => USBRESMENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBRESMENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBRESMENR::_1
    }
}
#[doc = "Values that can be written to the field `USBRESMEN`"]
pub enum USBRESMENW {
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    _0,
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    _1,
}
impl USBRESMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRESMENW::_0 => false,
            USBRESMENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRESMENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRESMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRESMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESMENW::_0)
    }
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESMENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBRESET`"]
pub enum USBRESETW {
    #[doc = "Normal USB module operation."]
    _0,
    #[doc = "Returns the USB module to its reset state."]
    _1,
}
impl USBRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRESETW::_0 => false,
            USBRESETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal USB module operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESETW::_0)
    }
    #[doc = "Returns the USB module to its reset state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESETW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - USB Asynchronous Interrupt"]
    #[inline]
    pub fn usb_resume_int(&self) -> USB_RESUME_INTR {
        USB_RESUME_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Synchronous USB Interrupt Detect"]
    #[inline]
    pub fn sync_det(&self) -> SYNC_DETR {
        SYNC_DETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Combined USB Clock Recovery interrupt status"]
    #[inline]
    pub fn usb_clk_recovery_int(&self) -> USB_CLK_RECOVERY_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        USB_CLK_RECOVERY_INTR { bits }
    }
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline]
    pub fn usbresmen(&self) -> USBRESMENR {
        USBRESMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline]
    pub fn usbresmen(&mut self) -> _USBRESMENW {
        _USBRESMENW { w: self }
    }
    #[doc = "Bit 7 - USB Reset"]
    #[inline]
    pub fn usbreset(&mut self) -> _USBRESETW {
        _USBRESETW { w: self }
    }
}
