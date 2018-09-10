#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `GO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl GOR {
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
            GOR::_0 => false,
            GOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GOR {
        match value {
            false => GOR::_0,
            true => GOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GOR::_1
    }
}
#[doc = "Possible values of the field `HA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl HAR {
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
            HAR::_0 => false,
            HAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HAR {
        match value {
            false => HAR::_0,
            true => HAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HAR::_1
    }
}
#[doc = "Possible values of the field `INTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMR {
    #[doc = "Not masked"]
    _0,
    #[doc = "Masked"]
    _1,
}
impl INTMR {
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
            INTMR::_0 => false,
            INTMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTMR {
        match value {
            false => INTMR::_0,
            true => INTMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTMR::_1
    }
}
#[doc = "Possible values of the field `SLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLPR {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Sleep (low-power) mode"]
    _1,
}
impl SLPR {
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
            SLPR::_0 => false,
            SLPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLPR {
        match value {
            false => SLPR::_0,
            true => SLPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLPR::_1
    }
}
#[doc = "Values that can be written to the field `GO`"]
pub enum GOW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl GOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GOW::_0 => false,
            GOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GOW<'a> {
    w: &'a mut W,
}
impl<'a> _GOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GOW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GOW::_1)
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
#[doc = "Values that can be written to the field `HA`"]
pub enum HAW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl HAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAW::_0 => false,
            HAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAW<'a> {
    w: &'a mut W,
}
impl<'a> _HAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HAW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HAW::_1)
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
#[doc = "Values that can be written to the field `INTM`"]
pub enum INTMW {
    #[doc = "Not masked"]
    _0,
    #[doc = "Masked"]
    _1,
}
impl INTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTMW::_0 => false,
            INTMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTMW<'a> {
    w: &'a mut W,
}
impl<'a> _INTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTMW::_0)
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTMW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLRI`"]
pub enum CLRIW {
    #[doc = "Do not clear the interrupt."]
    _0,
    #[doc = "Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    _1,
}
impl CLRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRIW::_0 => false,
            CLRIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear the interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLRIW::_0)
    }
    #[doc = "Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLRIW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLP`"]
pub enum SLPW {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Sleep (low-power) mode"]
    _1,
}
impl SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLPW::_0 => false,
            SLPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLPW::_0)
    }
    #[doc = "Sleep (low-power) mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLPW::_1)
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
    #[doc = "Bit 0 - Go"]
    #[inline]
    pub fn go(&self) -> GOR {
        GOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline]
    pub fn ha(&self) -> HAR {
        HAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline]
    pub fn intm(&self) -> INTMR {
        INTMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline]
    pub fn slp(&self) -> SLPR {
        SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Go"]
    #[inline]
    pub fn go(&mut self) -> _GOW {
        _GOW { w: self }
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline]
    pub fn ha(&mut self) -> _HAW {
        _HAW { w: self }
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline]
    pub fn intm(&mut self) -> _INTMW {
        _INTMW { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt"]
    #[inline]
    pub fn clri(&mut self) -> _CLRIW {
        _CLRIW { w: self }
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline]
    pub fn slp(&mut self) -> _SLPW {
        _SLPW { w: self }
    }
}
