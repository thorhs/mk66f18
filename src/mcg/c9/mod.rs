#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C9 {
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
#[doc = "Possible values of the field `EXT_PLL_LOCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_PLL_LOCSR {
    #[doc = "Loss of MCG EXT_PLL has not occurred."]
    _0,
    #[doc = "Loss of MCG EXT_PLL has occurred."]
    _1,
}
impl EXT_PLL_LOCSR {
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
            EXT_PLL_LOCSR::_0 => false,
            EXT_PLL_LOCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_PLL_LOCSR {
        match value {
            false => EXT_PLL_LOCSR::_0,
            true => EXT_PLL_LOCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXT_PLL_LOCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXT_PLL_LOCSR::_1
    }
}
#[doc = "Possible values of the field `PLL_LOCRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCRER {
    #[doc = "Interrupt request is generated on a invalid or loss of the MCG external PLL clock."]
    _0,
    #[doc = "Generates a system reset request on a invalid or loss of the MCG external PLL clock."]
    _1,
}
impl PLL_LOCRER {
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
            PLL_LOCRER::_0 => false,
            PLL_LOCRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_LOCRER {
        match value {
            false => PLL_LOCRER::_0,
            true => PLL_LOCRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_LOCRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_LOCRER::_1
    }
}
#[doc = "Possible values of the field `PLL_CME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CMER {
    #[doc = "External clock monitor is disabled for EXT_PLL clock."]
    _0,
    #[doc = "External clock monitor is enabled for EXT_PLL clock."]
    _1,
}
impl PLL_CMER {
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
            PLL_CMER::_0 => false,
            PLL_CMER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_CMER {
        match value {
            false => PLL_CMER::_0,
            true => PLL_CMER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_CMER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_CMER::_1
    }
}
#[doc = "Values that can be written to the field `EXT_PLL_LOCS`"]
pub enum EXT_PLL_LOCSW {
    #[doc = "Loss of MCG EXT_PLL has not occurred."]
    _0,
    #[doc = "Loss of MCG EXT_PLL has occurred."]
    _1,
}
impl EXT_PLL_LOCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXT_PLL_LOCSW::_0 => false,
            EXT_PLL_LOCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXT_PLL_LOCSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_PLL_LOCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_PLL_LOCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loss of MCG EXT_PLL has not occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXT_PLL_LOCSW::_0)
    }
    #[doc = "Loss of MCG EXT_PLL has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXT_PLL_LOCSW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_LOCRE`"]
pub enum PLL_LOCREW {
    #[doc = "Interrupt request is generated on a invalid or loss of the MCG external PLL clock."]
    _0,
    #[doc = "Generates a system reset request on a invalid or loss of the MCG external PLL clock."]
    _1,
}
impl PLL_LOCREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_LOCREW::_0 => false,
            PLL_LOCREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_LOCREW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LOCREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_LOCREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is generated on a invalid or loss of the MCG external PLL clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_LOCREW::_0)
    }
    #[doc = "Generates a system reset request on a invalid or loss of the MCG external PLL clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_LOCREW::_1)
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
#[doc = "Values that can be written to the field `PLL_CME`"]
pub enum PLL_CMEW {
    #[doc = "External clock monitor is disabled for EXT_PLL clock."]
    _0,
    #[doc = "External clock monitor is enabled for EXT_PLL clock."]
    _1,
}
impl PLL_CMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_CMEW::_0 => false,
            PLL_CMEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_CMEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_CMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_CMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External clock monitor is disabled for EXT_PLL clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_CMEW::_0)
    }
    #[doc = "External clock monitor is enabled for EXT_PLL clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_CMEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - External PLL Loss of Clock Status"]
    #[inline]
    pub fn ext_pll_locs(&self) -> EXT_PLL_LOCSR {
        EXT_PLL_LOCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - MCG External PLL Loss of Clock Reset Enable"]
    #[inline]
    pub fn pll_locre(&self) -> PLL_LOCRER {
        PLL_LOCRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - MCG External PLL Clock Monitor Enable"]
    #[inline]
    pub fn pll_cme(&self) -> PLL_CMER {
        PLL_CMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - External PLL Loss of Clock Status"]
    #[inline]
    pub fn ext_pll_locs(&mut self) -> _EXT_PLL_LOCSW {
        _EXT_PLL_LOCSW { w: self }
    }
    #[doc = "Bit 4 - MCG External PLL Loss of Clock Reset Enable"]
    #[inline]
    pub fn pll_locre(&mut self) -> _PLL_LOCREW {
        _PLL_LOCREW { w: self }
    }
    #[doc = "Bit 5 - MCG External PLL Clock Monitor Enable"]
    #[inline]
    pub fn pll_cme(&mut self) -> _PLL_CMEW {
        _PLL_CMEW { w: self }
    }
}
