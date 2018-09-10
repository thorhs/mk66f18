#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PTOR {
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
#[doc = "Values that can be written to the field `PTTO`"]
pub enum PTTOW {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PTTOW::_0 => 0,
            PTTOW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTOW<'a> {
    w: &'a mut W,
}
impl<'a> _PTTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTOW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTOW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline]
    pub fn ptto(&mut self) -> _PTTOW {
        _PTTOW { w: self }
    }
}
