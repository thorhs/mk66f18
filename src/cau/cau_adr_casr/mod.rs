#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAU_ADR_CASR {
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
#[doc = "Values that can be written to the field `IC`"]
pub enum ICW {
    #[doc = "No illegal commands issued"]
    _0,
    #[doc = "Illegal command issued"]
    _1,
}
impl ICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICW::_0 => false,
            ICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICW<'a> {
    w: &'a mut W,
}
impl<'a> _ICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No illegal commands issued"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICW::_0)
    }
    #[doc = "Illegal command issued"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICW::_1)
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
#[doc = "Values that can be written to the field `DPE`"]
pub enum DPEW {
    #[doc = "No error detected"]
    _0,
    #[doc = "DES key parity error detected"]
    _1,
}
impl DPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPEW::_0 => false,
            DPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPEW::_0)
    }
    #[doc = "DES key parity error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPEW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VER`"]
pub enum VERW {
    #[doc = "Initial CAU version"]
    _0001,
    #[doc = "Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010,
}
impl VERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VERW::_0001 => 1,
            VERW::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VERW<'a> {
    w: &'a mut W,
}
impl<'a> _VERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Initial CAU version"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(VERW::_0001)
    }
    #[doc = "Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(VERW::_0010)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536870912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn ic(&mut self) -> _ICW {
        _ICW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn dpe(&mut self) -> _DPEW {
        _DPEW { w: self }
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline]
    pub fn ver(&mut self) -> _VERW {
        _VERW { w: self }
    }
}
