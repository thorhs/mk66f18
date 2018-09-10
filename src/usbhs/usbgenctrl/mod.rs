#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBGENCTRL {
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
#[doc = "Possible values of the field `WU_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU_IER {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WU_IER {
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
            WU_IER::_0 => false,
            WU_IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WU_IER {
        match value {
            false => WU_IER::_0,
            true => WU_IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WU_IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WU_IER::_1
    }
}
#[doc = "Possible values of the field `WU_INT_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU_INT_CLRR {
    #[doc = "Default, no action."]
    _0,
    #[doc = "Clear the wake-up interrupt."]
    _1,
}
impl WU_INT_CLRR {
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
            WU_INT_CLRR::_0 => false,
            WU_INT_CLRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WU_INT_CLRR {
        match value {
            false => WU_INT_CLRR::_0,
            true => WU_INT_CLRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WU_INT_CLRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WU_INT_CLRR::_1
    }
}
#[doc = "Values that can be written to the field `WU_IE`"]
pub enum WU_IEW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WU_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WU_IEW::_0 => false,
            WU_IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _WU_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU_IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WU_IEW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WU_IEW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU_INT_CLR`"]
pub enum WU_INT_CLRW {
    #[doc = "Default, no action."]
    _0,
    #[doc = "Clear the wake-up interrupt."]
    _1,
}
impl WU_INT_CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WU_INT_CLRW::_0 => false,
            WU_INT_CLRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU_INT_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _WU_INT_CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU_INT_CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default, no action."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WU_INT_CLRW::_0)
    }
    #[doc = "Clear the wake-up interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WU_INT_CLRW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wakeup Interrupt Enable"]
    #[inline]
    pub fn wu_ie(&self) -> WU_IER {
        WU_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Interrupt Clear"]
    #[inline]
    pub fn wu_int_clr(&self) -> WU_INT_CLRR {
        WU_INT_CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wakeup Interrupt Enable"]
    #[inline]
    pub fn wu_ie(&mut self) -> _WU_IEW {
        _WU_IEW { w: self }
    }
    #[doc = "Bit 5 - Wakeup Interrupt Clear"]
    #[inline]
    pub fn wu_int_clr(&mut self) -> _WU_INT_CLRW {
        _WU_INT_CLRW { w: self }
    }
}
