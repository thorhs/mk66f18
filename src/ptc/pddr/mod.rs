#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDDR {
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
#[doc = "Possible values of the field `PDD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDR {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PDDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PDDR::_0 => 0,
            PDDR::_1 => 1,
            PDDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PDDR {
        match value {
            0 => PDDR::_0,
            1 => PDDR::_1,
            i => PDDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDDR::_1
    }
}
#[doc = "Values that can be written to the field `PDD`"]
pub enum PDDW {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PDDW::_0 => 0,
            PDDW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDDW::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDDW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline]
    pub fn pdd(&self) -> PDDR {
        PDDR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline]
    pub fn pdd(&mut self) -> _PDDW {
        _PDDW { w: self }
    }
}
