#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VENDOR {
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
#[doc = "Possible values of the field `EXBLKNU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXBLKNUR {
    #[doc = "None exact block read."]
    _0,
    #[doc = "Exact block read for SDIO CMD53."]
    _1,
}
impl EXBLKNUR {
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
            EXBLKNUR::_0 => false,
            EXBLKNUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXBLKNUR {
        match value {
            false => EXBLKNUR::_0,
            true => EXBLKNUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXBLKNUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXBLKNUR::_1
    }
}
#[doc = r" Value of the field"]
pub struct INTSTVALR {
    bits: u8,
}
impl INTSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `EXBLKNU`"]
pub enum EXBLKNUW {
    #[doc = "None exact block read."]
    _0,
    #[doc = "Exact block read for SDIO CMD53."]
    _1,
}
impl EXBLKNUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXBLKNUW::_0 => false,
            EXBLKNUW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXBLKNUW<'a> {
    w: &'a mut W,
}
impl<'a> _EXBLKNUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXBLKNUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "None exact block read."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXBLKNUW::_0)
    }
    #[doc = "Exact block read for SDIO CMD53."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXBLKNUW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline]
    pub fn exblknu(&self) -> EXBLKNUR {
        EXBLKNUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Internal State Value"]
    #[inline]
    pub fn intstval(&self) -> INTSTVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTSTVALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline]
    pub fn exblknu(&mut self) -> _EXBLKNUW {
        _EXBLKNUW { w: self }
    }
}
