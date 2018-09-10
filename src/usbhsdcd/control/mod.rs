#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONTROL {
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
#[doc = "Possible values of the field `IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFR {
    #[doc = "No interrupt is pending."]
    _0,
    #[doc = "An interrupt is pending."]
    _1,
}
impl IFR {
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
            IFR::_0 => false,
            IFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFR {
        match value {
            false => IFR::_0,
            true => IFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IFR::_1
    }
}
#[doc = "Possible values of the field `IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IER {
    #[doc = "Disable interrupts to the system."]
    _0,
    #[doc = "Enable interrupts to the system."]
    _1,
}
impl IER {
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
            IER::_0 => false,
            IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IER {
        match value {
            false => IER::_0,
            true => IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IER::_1
    }
}
#[doc = "Possible values of the field `BC12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BC12R {
    #[doc = "Compatible with BC1.1 (default)"]
    _0,
    #[doc = "Compatible with BC1.2"]
    _1,
}
impl BC12R {
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
            BC12R::_0 => false,
            BC12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BC12R {
        match value {
            false => BC12R::_0,
            true => BC12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BC12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BC12R::_1
    }
}
#[doc = "Values that can be written to the field `IACK`"]
pub enum IACKW {
    #[doc = "Do not clear the interrupt."]
    _0,
    #[doc = "Clear the IF bit (interrupt flag)."]
    _1,
}
impl IACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IACKW::_0 => false,
            IACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IACKW<'a> {
    w: &'a mut W,
}
impl<'a> _IACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear the interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IACKW::_0)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IACKW::_1)
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
#[doc = "Values that can be written to the field `IE`"]
pub enum IEW {
    #[doc = "Disable interrupts to the system."]
    _0,
    #[doc = "Enable interrupts to the system."]
    _1,
}
impl IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEW::_0 => false,
            IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupts to the system."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEW::_0)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BC12`"]
pub enum BC12W {
    #[doc = "Compatible with BC1.1 (default)"]
    _0,
    #[doc = "Compatible with BC1.2"]
    _1,
}
impl BC12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BC12W::_0 => false,
            BC12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BC12W<'a> {
    w: &'a mut W,
}
impl<'a> _BC12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BC12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BC12W::_0)
    }
    #[doc = "Compatible with BC1.2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BC12W::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    _0,
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    _1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::_0 => false,
            STARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTW::_0)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "Do not perform a software reset."]
    _0,
    #[doc = "Perform a software reset."]
    _1,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::_0 => false,
            SRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not perform a software reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRW::_0)
    }
    #[doc = "Perform a software reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRW::_1)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline]
    pub fn if_(&self) -> IFR {
        IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline]
    pub fn ie(&self) -> IER {
        IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline]
    pub fn bc12(&self) -> BC12R {
        BC12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65536 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline]
    pub fn iack(&mut self) -> _IACKW {
        _IACKW { w: self }
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline]
    pub fn ie(&mut self) -> _IEW {
        _IEW { w: self }
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline]
    pub fn bc12(&mut self) -> _BC12W {
        _BC12W { w: self }
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
}
