#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMER2_BC12 {
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
pub struct TVDMSRC_ONR {
    bits: u16,
}
impl TVDMSRC_ONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TWAIT_AFTER_PRDR {
    bits: u16,
}
impl TWAIT_AFTER_PRDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TVDMSRC_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _TVDMSRC_ONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TWAIT_AFTER_PRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TWAIT_AFTER_PRDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline]
    pub fn tvdmsrc_on(&self) -> TVDMSRC_ONR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TVDMSRC_ONR { bits }
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline]
    pub fn twait_after_prd(&self) -> TWAIT_AFTER_PRDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TWAIT_AFTER_PRDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline]
    pub fn tvdmsrc_on(&mut self) -> _TVDMSRC_ONW {
        _TVDMSRC_ONW { w: self }
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline]
    pub fn twait_after_prd(&mut self) -> _TWAIT_AFTER_PRDW {
        _TWAIT_AFTER_PRDW { w: self }
    }
}
