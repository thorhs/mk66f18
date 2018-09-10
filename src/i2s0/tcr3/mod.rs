#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR3 {
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
#[doc = r" Value of the field"]
pub struct WDFLR {
    bits: u8,
}
impl WDFLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCER {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCER::_0 => 0,
            TCER::_1 => 1,
            TCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCER {
        match value {
            0 => TCER::_0,
            1 => TCER::_1,
            i => TCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCER::_1
    }
}
#[doc = r" Proxy"]
pub struct _WDFLW<'a> {
    w: &'a mut W,
}
impl<'a> _WDFLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCE`"]
pub enum TCEW {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
}
impl TCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCEW::_0 => 0,
            TCEW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCEW::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCEW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFR`"]
pub enum CFRW {
    #[doc = "No effect."]
    _0,
    #[doc = "Transmit data channel N FIFO is reset."]
    _1,
}
impl CFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFRW::_0 => 0,
            CFRW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFRW<'a> {
    w: &'a mut W,
}
impl<'a> _CFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRW::_0)
    }
    #[doc = "Transmit data channel N FIFO is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline]
    pub fn wdfl(&self) -> WDFLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WDFLR { bits }
    }
    #[doc = "Bits 16:17 - Transmit Channel Enable"]
    #[inline]
    pub fn tce(&self) -> TCER {
        TCER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline]
    pub fn wdfl(&mut self) -> _WDFLW {
        _WDFLW { w: self }
    }
    #[doc = "Bits 16:17 - Transmit Channel Enable"]
    #[inline]
    pub fn tce(&mut self) -> _TCEW {
        _TCEW { w: self }
    }
    #[doc = "Bits 24:25 - Channel FIFO Reset"]
    #[inline]
    pub fn cfr(&mut self) -> _CFRW {
        _CFRW { w: self }
    }
}
