#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSOR {
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
#[doc = "Values that can be written to the field `PTSO`"]
pub enum PTSOW {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PTSOW::_0 => 0,
            PTSOW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSOW<'a> {
    w: &'a mut W,
}
impl<'a> _PTSOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSOW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSOW::_1)
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
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline]
    pub fn ptso(&mut self) -> _PTSOW {
        _PTSOW { w: self }
    }
}
