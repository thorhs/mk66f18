#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MSC {
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
#[doc = "Possible values of the field `MCGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCGENR {
    #[doc = "Modulator and carrier generator disabled"]
    _0,
    #[doc = "Modulator and carrier generator enabled"]
    _1,
}
impl MCGENR {
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
            MCGENR::_0 => false,
            MCGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCGENR {
        match value {
            false => MCGENR::_0,
            true => MCGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MCGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MCGENR::_1
    }
}
#[doc = "Possible values of the field `EOCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIER {
    #[doc = "CPU interrupt is disabled."]
    _0,
    #[doc = "CPU interrupt is enabled."]
    _1,
}
impl EOCIER {
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
            EOCIER::_0 => false,
            EOCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOCIER {
        match value {
            false => EOCIER::_0,
            true => EOCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOCIER::_1
    }
}
#[doc = "Possible values of the field `FSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSKR {
    #[doc = "The CMT operates in Time or Baseband mode."]
    _0,
    #[doc = "The CMT operates in FSK mode."]
    _1,
}
impl FSKR {
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
            FSKR::_0 => false,
            FSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSKR {
        match value {
            false => FSKR::_0,
            true => FSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSKR::_1
    }
}
#[doc = "Possible values of the field `BASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASER {
    #[doc = "Baseband mode is disabled."]
    _0,
    #[doc = "Baseband mode is enabled."]
    _1,
}
impl BASER {
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
            BASER::_0 => false,
            BASER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BASER {
        match value {
            false => BASER::_0,
            true => BASER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BASER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BASER::_1
    }
}
#[doc = "Possible values of the field `EXSPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXSPCR {
    #[doc = "Extended space is disabled."]
    _0,
    #[doc = "Extended space is enabled."]
    _1,
}
impl EXSPCR {
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
            EXSPCR::_0 => false,
            EXSPCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXSPCR {
        match value {
            false => EXSPCR::_0,
            true => EXSPCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXSPCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXSPCR::_1
    }
}
#[doc = "Possible values of the field `CMTDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTDIVR {
    #[doc = "IF * 1"]
    _00,
    #[doc = "IF * 2"]
    _01,
    #[doc = "IF * 4"]
    _10,
    #[doc = "IF * 8"]
    _11,
}
impl CMTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMTDIVR::_00 => 0,
            CMTDIVR::_01 => 1,
            CMTDIVR::_10 => 2,
            CMTDIVR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMTDIVR {
        match value {
            0 => CMTDIVR::_00,
            1 => CMTDIVR::_01,
            2 => CMTDIVR::_10,
            3 => CMTDIVR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CMTDIVR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CMTDIVR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CMTDIVR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CMTDIVR::_11
    }
}
#[doc = "Possible values of the field `EOCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCFR {
    #[doc = "End of modulation cycle has not occured since the flag last cleared."]
    _0,
    #[doc = "End of modulator cycle has occurred."]
    _1,
}
impl EOCFR {
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
            EOCFR::_0 => false,
            EOCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOCFR {
        match value {
            false => EOCFR::_0,
            true => EOCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOCFR::_1
    }
}
#[doc = "Values that can be written to the field `MCGEN`"]
pub enum MCGENW {
    #[doc = "Modulator and carrier generator disabled"]
    _0,
    #[doc = "Modulator and carrier generator enabled"]
    _1,
}
impl MCGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCGENW::_0 => false,
            MCGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Modulator and carrier generator disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCGENW::_0)
    }
    #[doc = "Modulator and carrier generator enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCGENW::_1)
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
#[doc = "Values that can be written to the field `EOCIE`"]
pub enum EOCIEW {
    #[doc = "CPU interrupt is disabled."]
    _0,
    #[doc = "CPU interrupt is enabled."]
    _1,
}
impl EOCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOCIEW::_0 => false,
            EOCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EOCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CPU interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCIEW::_0)
    }
    #[doc = "CPU interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCIEW::_1)
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
#[doc = "Values that can be written to the field `FSK`"]
pub enum FSKW {
    #[doc = "The CMT operates in Time or Baseband mode."]
    _0,
    #[doc = "The CMT operates in FSK mode."]
    _1,
}
impl FSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSKW::_0 => false,
            FSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSKW<'a> {
    w: &'a mut W,
}
impl<'a> _FSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CMT operates in Time or Baseband mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSKW::_0)
    }
    #[doc = "The CMT operates in FSK mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSKW::_1)
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
#[doc = "Values that can be written to the field `BASE`"]
pub enum BASEW {
    #[doc = "Baseband mode is disabled."]
    _0,
    #[doc = "Baseband mode is enabled."]
    _1,
}
impl BASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BASEW::_0 => false,
            BASEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BASEW<'a> {
    w: &'a mut W,
}
impl<'a> _BASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Baseband mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BASEW::_0)
    }
    #[doc = "Baseband mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BASEW::_1)
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
#[doc = "Values that can be written to the field `EXSPC`"]
pub enum EXSPCW {
    #[doc = "Extended space is disabled."]
    _0,
    #[doc = "Extended space is enabled."]
    _1,
}
impl EXSPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXSPCW::_0 => false,
            EXSPCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXSPCW<'a> {
    w: &'a mut W,
}
impl<'a> _EXSPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXSPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Extended space is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXSPCW::_0)
    }
    #[doc = "Extended space is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXSPCW::_1)
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
#[doc = "Values that can be written to the field `CMTDIV`"]
pub enum CMTDIVW {
    #[doc = "IF * 1"]
    _00,
    #[doc = "IF * 2"]
    _01,
    #[doc = "IF * 4"]
    _10,
    #[doc = "IF * 8"]
    _11,
}
impl CMTDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMTDIVW::_00 => 0,
            CMTDIVW::_01 => 1,
            CMTDIVW::_10 => 2,
            CMTDIVW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CMTDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMTDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IF * 1"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMTDIVW::_00)
    }
    #[doc = "IF * 2"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMTDIVW::_01)
    }
    #[doc = "IF * 4"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMTDIVW::_10)
    }
    #[doc = "IF * 8"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMTDIVW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline]
    pub fn mcgen(&self) -> MCGENR {
        MCGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline]
    pub fn eocie(&self) -> EOCIER {
        EOCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline]
    pub fn fsk(&self) -> FSKR {
        FSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline]
    pub fn base(&self) -> BASER {
        BASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline]
    pub fn exspc(&self) -> EXSPCR {
        EXSPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline]
    pub fn cmtdiv(&self) -> CMTDIVR {
        CMTDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - End Of Cycle Status Flag"]
    #[inline]
    pub fn eocf(&self) -> EOCFR {
        EOCFR::_from({
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
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline]
    pub fn mcgen(&mut self) -> _MCGENW {
        _MCGENW { w: self }
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline]
    pub fn eocie(&mut self) -> _EOCIEW {
        _EOCIEW { w: self }
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline]
    pub fn fsk(&mut self) -> _FSKW {
        _FSKW { w: self }
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline]
    pub fn base(&mut self) -> _BASEW {
        _BASEW { w: self }
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline]
    pub fn exspc(&mut self) -> _EXSPCW {
        _EXSPCW { w: self }
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline]
    pub fn cmtdiv(&mut self) -> _CMTDIVW {
        _CMTDIVW { w: self }
    }
}
