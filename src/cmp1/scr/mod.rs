#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SCR {
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
pub struct COUTR {
    bits: bool,
}
impl COUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `CFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFFR {
    #[doc = "Falling-edge on COUT has not been detected."]
    _0,
    #[doc = "Falling-edge on COUT has occurred."]
    _1,
}
impl CFFR {
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
            CFFR::_0 => false,
            CFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFFR {
        match value {
            false => CFFR::_0,
            true => CFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CFFR::_1
    }
}
#[doc = "Possible values of the field `CFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFRR {
    #[doc = "Rising-edge on COUT has not been detected."]
    _0,
    #[doc = "Rising-edge on COUT has occurred."]
    _1,
}
impl CFRR {
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
            CFRR::_0 => false,
            CFRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFRR {
        match value {
            false => CFRR::_0,
            true => CFRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CFRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CFRR::_1
    }
}
#[doc = "Possible values of the field `IEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEFR {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IEFR {
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
            IEFR::_0 => false,
            IEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEFR {
        match value {
            false => IEFR::_0,
            true => IEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IEFR::_1
    }
}
#[doc = "Possible values of the field `IER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERR {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IERR {
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
            IERR::_0 => false,
            IERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IERR {
        match value {
            false => IERR::_0,
            true => IERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IERR::_1
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled."]
    _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = "Values that can be written to the field `CFF`"]
pub enum CFFW {
    #[doc = "Falling-edge on COUT has not been detected."]
    _0,
    #[doc = "Falling-edge on COUT has occurred."]
    _1,
}
impl CFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFFW::_0 => false,
            CFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFFW::_0)
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFFW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFR`"]
pub enum CFRW {
    #[doc = "Rising-edge on COUT has not been detected."]
    _0,
    #[doc = "Rising-edge on COUT has occurred."]
    _1,
}
impl CFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFRW::_0 => false,
            CFRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFRW<'a> {
    w: &'a mut W,
}
impl<'a> _CFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRW::_0)
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IEF`"]
pub enum IEFW {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEFW::_0 => false,
            IEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEFW<'a> {
    w: &'a mut W,
}
impl<'a> _IEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEFW::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEFW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IER`"]
pub enum IERW {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IERW::_0 => false,
            IERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IERW<'a> {
    w: &'a mut W,
}
impl<'a> _IERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERW::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERW::_1)
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
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled."]
    _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline]
    pub fn cout(&self) -> COUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COUTR { bits }
    }
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&self) -> CFFR {
        CFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&self) -> CFRR {
        CFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&self) -> IEFR {
        IEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&self) -> IERR {
        IERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&mut self) -> _CFFW {
        _CFFW { w: self }
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&mut self) -> _CFRW {
        _CFRW { w: self }
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&mut self) -> _IEFW {
        _IEFW { w: self }
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&mut self) -> _IERW {
        _IERW { w: self }
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
