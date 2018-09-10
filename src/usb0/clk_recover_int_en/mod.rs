#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CLK_RECOVER_INT_EN {
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
#[doc = "Possible values of the field `OVF_ERROR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_ERROR_ENR {
    #[doc = "The interrupt will be masked"]
    _0,
    #[doc = "The interrupt will be enabled (default)"]
    _1,
}
impl OVF_ERROR_ENR {
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
            OVF_ERROR_ENR::_0 => false,
            OVF_ERROR_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVF_ERROR_ENR {
        match value {
            false => OVF_ERROR_ENR::_0,
            true => OVF_ERROR_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OVF_ERROR_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OVF_ERROR_ENR::_1
    }
}
#[doc = "Values that can be written to the field `OVF_ERROR_EN`"]
pub enum OVF_ERROR_ENW {
    #[doc = "The interrupt will be masked"]
    _0,
    #[doc = "The interrupt will be enabled (default)"]
    _1,
}
impl OVF_ERROR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVF_ERROR_ENW::_0 => false,
            OVF_ERROR_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVF_ERROR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVF_ERROR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVF_ERROR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt will be masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_ERROR_ENW::_0)
    }
    #[doc = "The interrupt will be enabled (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_ERROR_ENW::_1)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline]
    pub fn ovf_error_en(&self) -> OVF_ERROR_ENR {
        OVF_ERROR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline]
    pub fn ovf_error_en(&mut self) -> _OVF_ERROR_ENW {
        _OVF_ERROR_ENW { w: self }
    }
}
