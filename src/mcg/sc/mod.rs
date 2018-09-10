#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SC {
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
#[doc = "Possible values of the field `LOCS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCS0R {
    #[doc = "Loss of OSC0 has not occurred."]
    _0,
    #[doc = "Loss of OSC0 has occurred."]
    _1,
}
impl LOCS0R {
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
            LOCS0R::_0 => false,
            LOCS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCS0R {
        match value {
            false => LOCS0R::_0,
            true => LOCS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCS0R::_1
    }
}
#[doc = "Possible values of the field `FCRDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRDIVR {
    #[doc = "Divide Factor is 1"]
    _000,
    #[doc = "Divide Factor is 2."]
    _001,
    #[doc = "Divide Factor is 4."]
    _010,
    #[doc = "Divide Factor is 8."]
    _011,
    #[doc = "Divide Factor is 16"]
    _100,
    #[doc = "Divide Factor is 32"]
    _101,
    #[doc = "Divide Factor is 64"]
    _110,
    #[doc = "Divide Factor is 128."]
    _111,
}
impl FCRDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCRDIVR::_000 => 0,
            FCRDIVR::_001 => 1,
            FCRDIVR::_010 => 2,
            FCRDIVR::_011 => 3,
            FCRDIVR::_100 => 4,
            FCRDIVR::_101 => 5,
            FCRDIVR::_110 => 6,
            FCRDIVR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCRDIVR {
        match value {
            0 => FCRDIVR::_000,
            1 => FCRDIVR::_001,
            2 => FCRDIVR::_010,
            3 => FCRDIVR::_011,
            4 => FCRDIVR::_100,
            5 => FCRDIVR::_101,
            6 => FCRDIVR::_110,
            7 => FCRDIVR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FCRDIVR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FCRDIVR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FCRDIVR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FCRDIVR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FCRDIVR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FCRDIVR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FCRDIVR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FCRDIVR::_111
    }
}
#[doc = "Possible values of the field `FLTPRSRV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTPRSRVR {
    #[doc = "FLL filter and FLL frequency will reset on changes to currect clock mode."]
    _0,
    #[doc = "Fll filter and FLL frequency retain their previous values during new clock mode change."]
    _1,
}
impl FLTPRSRVR {
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
            FLTPRSRVR::_0 => false,
            FLTPRSRVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLTPRSRVR {
        match value {
            false => FLTPRSRVR::_0,
            true => FLTPRSRVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLTPRSRVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLTPRSRVR::_1
    }
}
#[doc = "Possible values of the field `ATMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATMFR {
    #[doc = "Automatic Trim Machine completed normally."]
    _0,
    #[doc = "Automatic Trim Machine failed."]
    _1,
}
impl ATMFR {
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
            ATMFR::_0 => false,
            ATMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATMFR {
        match value {
            false => ATMFR::_0,
            true => ATMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ATMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ATMFR::_1
    }
}
#[doc = "Possible values of the field `ATMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATMSR {
    #[doc = "32 kHz Internal Reference Clock selected."]
    _0,
    #[doc = "4 MHz Internal Reference Clock selected."]
    _1,
}
impl ATMSR {
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
            ATMSR::_0 => false,
            ATMSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATMSR {
        match value {
            false => ATMSR::_0,
            true => ATMSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ATMSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ATMSR::_1
    }
}
#[doc = "Possible values of the field `ATME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATMER {
    #[doc = "Auto Trim Machine disabled."]
    _0,
    #[doc = "Auto Trim Machine enabled."]
    _1,
}
impl ATMER {
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
            ATMER::_0 => false,
            ATMER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATMER {
        match value {
            false => ATMER::_0,
            true => ATMER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ATMER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ATMER::_1
    }
}
#[doc = "Values that can be written to the field `LOCS0`"]
pub enum LOCS0W {
    #[doc = "Loss of OSC0 has not occurred."]
    _0,
    #[doc = "Loss of OSC0 has occurred."]
    _1,
}
impl LOCS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCS0W::_0 => false,
            LOCS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCS0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loss of OSC0 has not occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS0W::_0)
    }
    #[doc = "Loss of OSC0 has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS0W::_1)
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
#[doc = "Values that can be written to the field `FCRDIV`"]
pub enum FCRDIVW {
    #[doc = "Divide Factor is 1"]
    _000,
    #[doc = "Divide Factor is 2."]
    _001,
    #[doc = "Divide Factor is 4."]
    _010,
    #[doc = "Divide Factor is 8."]
    _011,
    #[doc = "Divide Factor is 16"]
    _100,
    #[doc = "Divide Factor is 32"]
    _101,
    #[doc = "Divide Factor is 64"]
    _110,
    #[doc = "Divide Factor is 128."]
    _111,
}
impl FCRDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCRDIVW::_000 => 0,
            FCRDIVW::_001 => 1,
            FCRDIVW::_010 => 2,
            FCRDIVW::_011 => 3,
            FCRDIVW::_100 => 4,
            FCRDIVW::_101 => 5,
            FCRDIVW::_110 => 6,
            FCRDIVW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCRDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FCRDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCRDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide Factor is 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FCRDIVW::_000)
    }
    #[doc = "Divide Factor is 2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FCRDIVW::_001)
    }
    #[doc = "Divide Factor is 4."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FCRDIVW::_010)
    }
    #[doc = "Divide Factor is 8."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FCRDIVW::_011)
    }
    #[doc = "Divide Factor is 16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FCRDIVW::_100)
    }
    #[doc = "Divide Factor is 32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FCRDIVW::_101)
    }
    #[doc = "Divide Factor is 64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FCRDIVW::_110)
    }
    #[doc = "Divide Factor is 128."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FCRDIVW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLTPRSRV`"]
pub enum FLTPRSRVW {
    #[doc = "FLL filter and FLL frequency will reset on changes to currect clock mode."]
    _0,
    #[doc = "Fll filter and FLL frequency retain their previous values during new clock mode change."]
    _1,
}
impl FLTPRSRVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLTPRSRVW::_0 => false,
            FLTPRSRVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLTPRSRVW<'a> {
    w: &'a mut W,
}
impl<'a> _FLTPRSRVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLTPRSRVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLL filter and FLL frequency will reset on changes to currect clock mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLTPRSRVW::_0)
    }
    #[doc = "Fll filter and FLL frequency retain their previous values during new clock mode change."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLTPRSRVW::_1)
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
#[doc = "Values that can be written to the field `ATMF`"]
pub enum ATMFW {
    #[doc = "Automatic Trim Machine completed normally."]
    _0,
    #[doc = "Automatic Trim Machine failed."]
    _1,
}
impl ATMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATMFW::_0 => false,
            ATMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATMFW<'a> {
    w: &'a mut W,
}
impl<'a> _ATMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic Trim Machine completed normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMFW::_0)
    }
    #[doc = "Automatic Trim Machine failed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMFW::_1)
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
#[doc = "Values that can be written to the field `ATMS`"]
pub enum ATMSW {
    #[doc = "32 kHz Internal Reference Clock selected."]
    _0,
    #[doc = "4 MHz Internal Reference Clock selected."]
    _1,
}
impl ATMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATMSW::_0 => false,
            ATMSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATMSW<'a> {
    w: &'a mut W,
}
impl<'a> _ATMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "32 kHz Internal Reference Clock selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMSW::_0)
    }
    #[doc = "4 MHz Internal Reference Clock selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMSW::_1)
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
#[doc = "Values that can be written to the field `ATME`"]
pub enum ATMEW {
    #[doc = "Auto Trim Machine disabled."]
    _0,
    #[doc = "Auto Trim Machine enabled."]
    _1,
}
impl ATMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATMEW::_0 => false,
            ATMEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATMEW<'a> {
    w: &'a mut W,
}
impl<'a> _ATMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto Trim Machine disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMEW::_0)
    }
    #[doc = "Auto Trim Machine enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMEW::_1)
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
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline]
    pub fn locs0(&self) -> LOCS0R {
        LOCS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline]
    pub fn fcrdiv(&self) -> FCRDIVR {
        FCRDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline]
    pub fn fltprsrv(&self) -> FLTPRSRVR {
        FLTPRSRVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline]
    pub fn atmf(&self) -> ATMFR {
        ATMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline]
    pub fn atms(&self) -> ATMSR {
        ATMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline]
    pub fn atme(&self) -> ATMER {
        ATMER::_from({
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
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline]
    pub fn locs0(&mut self) -> _LOCS0W {
        _LOCS0W { w: self }
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline]
    pub fn fcrdiv(&mut self) -> _FCRDIVW {
        _FCRDIVW { w: self }
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline]
    pub fn fltprsrv(&mut self) -> _FLTPRSRVW {
        _FLTPRSRVW { w: self }
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline]
    pub fn atmf(&mut self) -> _ATMFW {
        _ATMFW { w: self }
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline]
    pub fn atms(&mut self) -> _ATMSW {
        _ATMSW { w: self }
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline]
    pub fn atme(&mut self) -> _ATMEW {
        _ATMEW { w: self }
    }
}
