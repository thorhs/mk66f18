#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::BDH {
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
pub struct SBRR {
    bits: u8,
}
impl SBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SBNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNSR {
    #[doc = "Data frame consists of a single stop bit."]
    _0,
    #[doc = "Data frame consists of two stop bits."]
    _1,
}
impl SBNSR {
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
            SBNSR::_0 => false,
            SBNSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBNSR {
        match value {
            false => SBNSR::_0,
            true => SBNSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBNSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBNSR::_1
    }
}
#[doc = "Possible values of the field `RXEDGIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIER {
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    _0,
    #[doc = "RXEDGIF interrupt request enabled."]
    _1,
}
impl RXEDGIER {
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
            RXEDGIER::_0 => false,
            RXEDGIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIER {
        match value {
            false => RXEDGIER::_0,
            true => RXEDGIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIER::_1
    }
}
#[doc = "Possible values of the field `LBKDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIER {
    #[doc = "LBKDIF interrupt requests disabled."]
    _0,
    #[doc = "LBKDIF interrupt requests enabled."]
    _1,
}
impl LBKDIER {
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
            LBKDIER::_0 => false,
            LBKDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIER {
        match value {
            false => LBKDIER::_0,
            true => LBKDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDIER::_1
    }
}
#[doc = r" Proxy"]
pub struct _SBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBRW<'a> {
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
#[doc = "Values that can be written to the field `SBNS`"]
pub enum SBNSW {
    #[doc = "Data frame consists of a single stop bit."]
    _0,
    #[doc = "Data frame consists of two stop bits."]
    _1,
}
impl SBNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBNSW::_0 => false,
            SBNSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBNSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBNSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data frame consists of a single stop bit."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNSW::_0)
    }
    #[doc = "Data frame consists of two stop bits."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNSW::_1)
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
#[doc = "Values that can be written to the field `RXEDGIE`"]
pub enum RXEDGIEW {
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    _0,
    #[doc = "RXEDGIF interrupt request enabled."]
    _1,
}
impl RXEDGIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIEW::_0 => false,
            RXEDGIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIEW::_0)
    }
    #[doc = "RXEDGIF interrupt request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIEW::_1)
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
#[doc = "Values that can be written to the field `LBKDIE`"]
pub enum LBKDIEW {
    #[doc = "LBKDIF interrupt requests disabled."]
    _0,
    #[doc = "LBKDIF interrupt requests enabled."]
    _1,
}
impl LBKDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIEW::_0 => false,
            LBKDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LBKDIF interrupt requests disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIEW::_0)
    }
    #[doc = "LBKDIF interrupt requests enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIEW::_1)
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
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline]
    pub fn sbr(&self) -> SBRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        SBRR { bits }
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&self) -> SBNSR {
        SBNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&self) -> RXEDGIER {
        RXEDGIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&self) -> LBKDIER {
        LBKDIER::_from({
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
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline]
    pub fn sbr(&mut self) -> _SBRW {
        _SBRW { w: self }
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&mut self) -> _SBNSW {
        _SBNSW { w: self }
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&mut self) -> _RXEDGIEW {
        _RXEDGIEW { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&mut self) -> _LBKDIEW {
        _LBKDIEW { w: self }
    }
}
