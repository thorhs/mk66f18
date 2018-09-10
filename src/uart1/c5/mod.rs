#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C5 {
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
#[doc = "Possible values of the field `RDMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMASR {
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl RDMASR {
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
            RDMASR::_0 => false,
            RDMASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMASR {
        match value {
            false => RDMASR::_0,
            true => RDMASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDMASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDMASR::_1
    }
}
#[doc = "Possible values of the field `TDMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMASR {
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    _0,
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl TDMASR {
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
            TDMASR::_0 => false,
            TDMASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDMASR {
        match value {
            false => TDMASR::_0,
            true => TDMASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDMASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDMASR::_1
    }
}
#[doc = "Values that can be written to the field `RDMAS`"]
pub enum RDMASW {
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl RDMASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMASW::_0 => false,
            RDMASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMASW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMASW::_0)
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMASW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TDMAS`"]
pub enum TDMASW {
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    _0,
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl TDMASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDMASW::_0 => false,
            TDMASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDMASW<'a> {
    w: &'a mut W,
}
impl<'a> _TDMASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDMASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMASW::_0)
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMASW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline]
    pub fn rdmas(&self) -> RDMASR {
        RDMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline]
    pub fn tdmas(&self) -> TDMASR {
        TDMASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline]
    pub fn rdmas(&mut self) -> _RDMASW {
        _RDMASW { w: self }
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline]
    pub fn tdmas(&mut self) -> _TDMASW {
        _TDMASW { w: self }
    }
}
