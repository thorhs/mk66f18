#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CERQ {
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
}
#[doc = r" Proxy"]
pub struct _CERQW<'a> {
    w: &'a mut W,
}
impl<'a> _CERQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAER`"]
pub enum CAERW {
    #[doc = "Clear only the ERQ bit specified in the CERQ field"]
    _0,
    #[doc = "Clear all bits in ERQ"]
    _1,
}
impl CAERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAERW::_0 => false,
            CAERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAERW<'a> {
    w: &'a mut W,
}
impl<'a> _CAERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear only the ERQ bit specified in the CERQ field"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAERW::_0)
    }
    #[doc = "Clear all bits in ERQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAERW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOP`"]
pub enum NOPW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "No operation, ignore the other bits in this register"]
    _1,
}
impl NOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOPW::_0 => false,
            NOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOPW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOPW::_1)
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
    #[doc = "Bits 0:4 - Clear Enable Request"]
    #[inline]
    pub fn cerq(&mut self) -> _CERQW {
        _CERQW { w: self }
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline]
    pub fn caer(&mut self) -> _CAERW {
        _CAERW { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline]
    pub fn nop(&mut self) -> _NOPW {
        _NOPW { w: self }
    }
}
