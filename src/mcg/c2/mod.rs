#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C2 {
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
#[doc = "Possible values of the field `IRCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCSR {
    #[doc = "Slow internal reference clock selected."]
    _0,
    #[doc = "Fast internal reference clock selected."]
    _1,
}
impl IRCSR {
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
            IRCSR::_0 => false,
            IRCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCSR {
        match value {
            false => IRCSR::_0,
            true => IRCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRCSR::_1
    }
}
#[doc = "Possible values of the field `LP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPR {
    #[doc = "FLL or PLL is not disabled in bypass modes."]
    _0,
    #[doc = "FLL or PLL is disabled in bypass modes (lower power)"]
    _1,
}
impl LPR {
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
            LPR::_0 => false,
            LPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPR {
        match value {
            false => LPR::_0,
            true => LPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPR::_1
    }
}
#[doc = "Possible values of the field `EREFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFSR {
    #[doc = "External reference clock requested."]
    _0,
    #[doc = "Oscillator requested."]
    _1,
}
impl EREFSR {
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
            EREFSR::_0 => false,
            EREFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EREFSR {
        match value {
            false => EREFSR::_0,
            true => EREFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EREFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EREFSR::_1
    }
}
#[doc = "Possible values of the field `HGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGOR {
    #[doc = "Configure crystal oscillator for low-power operation."]
    _0,
    #[doc = "Configure crystal oscillator for high-gain operation."]
    _1,
}
impl HGOR {
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
            HGOR::_0 => false,
            HGOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HGOR {
        match value {
            false => HGOR::_0,
            true => HGOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HGOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HGOR::_1
    }
}
#[doc = "Possible values of the field `RANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGER {
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00,
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANGER::_00 => 0,
            RANGER::_01 => 1,
            RANGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANGER {
        match value {
            0 => RANGER::_00,
            1 => RANGER::_01,
            i => RANGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RANGER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RANGER::_01
    }
}
#[doc = r" Value of the field"]
pub struct FCFTRIMR {
    bits: bool,
}
impl FCFTRIMR {
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
#[doc = "Possible values of the field `LOCRE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE0R {
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    _1,
}
impl LOCRE0R {
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
            LOCRE0R::_0 => false,
            LOCRE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCRE0R {
        match value {
            false => LOCRE0R::_0,
            true => LOCRE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCRE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCRE0R::_1
    }
}
#[doc = "Values that can be written to the field `IRCS`"]
pub enum IRCSW {
    #[doc = "Slow internal reference clock selected."]
    _0,
    #[doc = "Fast internal reference clock selected."]
    _1,
}
impl IRCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRCSW::_0 => false,
            IRCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IRCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slow internal reference clock selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCSW::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCSW::_1)
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
#[doc = "Values that can be written to the field `LP`"]
pub enum LPW {
    #[doc = "FLL or PLL is not disabled in bypass modes."]
    _0,
    #[doc = "FLL or PLL is disabled in bypass modes (lower power)"]
    _1,
}
impl LPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPW::_0 => false,
            LPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLL or PLL is not disabled in bypass modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPW::_0)
    }
    #[doc = "FLL or PLL is disabled in bypass modes (lower power)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPW::_1)
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
#[doc = "Values that can be written to the field `EREFS`"]
pub enum EREFSW {
    #[doc = "External reference clock requested."]
    _0,
    #[doc = "Oscillator requested."]
    _1,
}
impl EREFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EREFSW::_0 => false,
            EREFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EREFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EREFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EREFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock requested."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFSW::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFSW::_1)
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
#[doc = "Values that can be written to the field `HGO`"]
pub enum HGOW {
    #[doc = "Configure crystal oscillator for low-power operation."]
    _0,
    #[doc = "Configure crystal oscillator for high-gain operation."]
    _1,
}
impl HGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HGOW::_0 => false,
            HGOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HGOW<'a> {
    w: &'a mut W,
}
impl<'a> _HGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HGOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGOW::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGOW::_1)
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
#[doc = "Values that can be written to the field `RANGE`"]
pub enum RANGEW {
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00,
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01,
}
impl RANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RANGEW::_00 => 0,
            RANGEW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _RANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANGEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGEW::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGEW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCFTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FCFTRIMW<'a> {
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
#[doc = "Values that can be written to the field `LOCRE0`"]
pub enum LOCRE0W {
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    _1,
}
impl LOCRE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCRE0W::_0 => false,
            LOCRE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCRE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCRE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCRE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE0W::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE0W::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline]
    pub fn ircs(&self) -> IRCSR {
        IRCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline]
    pub fn lp(&self) -> LPR {
        LPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs(&self) -> EREFSR {
        EREFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo(&self) -> HGOR {
        HGOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline]
    pub fn range(&self) -> RANGER {
        RANGER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline]
    pub fn fcftrim(&self) -> FCFTRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        FCFTRIMR { bits }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre0(&self) -> LOCRE0R {
        LOCRE0R::_from({
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
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline]
    pub fn ircs(&mut self) -> _IRCSW {
        _IRCSW { w: self }
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline]
    pub fn lp(&mut self) -> _LPW {
        _LPW { w: self }
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs(&mut self) -> _EREFSW {
        _EREFSW { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo(&mut self) -> _HGOW {
        _HGOW { w: self }
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline]
    pub fn range(&mut self) -> _RANGEW {
        _RANGEW { w: self }
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline]
    pub fn fcftrim(&mut self) -> _FCFTRIMW {
        _FCFTRIMW { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre0(&mut self) -> _LOCRE0W {
        _LOCRE0W { w: self }
    }
}
