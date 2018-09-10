#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C1 {
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
#[doc = "Possible values of the field `IREFSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTENR {
    #[doc = "Internal reference clock is disabled in Stop mode."]
    _0,
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    _1,
}
impl IREFSTENR {
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
            IREFSTENR::_0 => false,
            IREFSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IREFSTENR {
        match value {
            false => IREFSTENR::_0,
            true => IREFSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IREFSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IREFSTENR::_1
    }
}
#[doc = "Possible values of the field `IRCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCLKENR {
    #[doc = "MCGIRCLK inactive."]
    _0,
    #[doc = "MCGIRCLK active."]
    _1,
}
impl IRCLKENR {
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
            IRCLKENR::_0 => false,
            IRCLKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCLKENR {
        match value {
            false => IRCLKENR::_0,
            true => IRCLKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRCLKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRCLKENR::_1
    }
}
#[doc = "Possible values of the field `IREFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSR {
    #[doc = "External reference clock is selected."]
    _0,
    #[doc = "The slow internal reference clock is selected."]
    _1,
}
impl IREFSR {
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
            IREFSR::_0 => false,
            IREFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IREFSR {
        match value {
            false => IREFSR::_0,
            true => IREFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IREFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IREFSR::_1
    }
}
#[doc = "Possible values of the field `FRDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDIVR {
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    _000,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    _001,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    _010,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    _011,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    _100,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    _101,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    _110,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    _111,
}
impl FRDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRDIVR::_000 => 0,
            FRDIVR::_001 => 1,
            FRDIVR::_010 => 2,
            FRDIVR::_011 => 3,
            FRDIVR::_100 => 4,
            FRDIVR::_101 => 5,
            FRDIVR::_110 => 6,
            FRDIVR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRDIVR {
        match value {
            0 => FRDIVR::_000,
            1 => FRDIVR::_001,
            2 => FRDIVR::_010,
            3 => FRDIVR::_011,
            4 => FRDIVR::_100,
            5 => FRDIVR::_101,
            6 => FRDIVR::_110,
            7 => FRDIVR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FRDIVR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FRDIVR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FRDIVR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FRDIVR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FRDIVR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FRDIVR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FRDIVR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FRDIVR::_111
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "Encoding 0 - Output of FLL or PLLCS is selected (depends on PLLS control bit)."]
    _00,
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = "Encoding 3 - Reserved."]
    _11,
}
impl CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSR::_00 => 0,
            CLKSR::_01 => 1,
            CLKSR::_10 => 2,
            CLKSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSR {
        match value {
            0 => CLKSR::_00,
            1 => CLKSR::_01,
            2 => CLKSR::_10,
            3 => CLKSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CLKSR::_11
    }
}
#[doc = "Values that can be written to the field `IREFSTEN`"]
pub enum IREFSTENW {
    #[doc = "Internal reference clock is disabled in Stop mode."]
    _0,
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    _1,
}
impl IREFSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IREFSTENW::_0 => false,
            IREFSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IREFSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _IREFSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IREFSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFSTENW::_0)
    }
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFSTENW::_1)
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
#[doc = "Values that can be written to the field `IRCLKEN`"]
pub enum IRCLKENW {
    #[doc = "MCGIRCLK inactive."]
    _0,
    #[doc = "MCGIRCLK active."]
    _1,
}
impl IRCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRCLKENW::_0 => false,
            IRCLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGIRCLK inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCLKENW::_0)
    }
    #[doc = "MCGIRCLK active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCLKENW::_1)
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
#[doc = "Values that can be written to the field `IREFS`"]
pub enum IREFSW {
    #[doc = "External reference clock is selected."]
    _0,
    #[doc = "The slow internal reference clock is selected."]
    _1,
}
impl IREFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IREFSW::_0 => false,
            IREFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IREFSW<'a> {
    w: &'a mut W,
}
impl<'a> _IREFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IREFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFSW::_0)
    }
    #[doc = "The slow internal reference clock is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFSW::_1)
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
#[doc = "Values that can be written to the field `FRDIV`"]
pub enum FRDIVW {
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    _000,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    _001,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    _010,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    _011,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    _100,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    _101,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    _110,
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    _111,
}
impl FRDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRDIVW::_000 => 0,
            FRDIVW::_001 => 1,
            FRDIVW::_010 => 2,
            FRDIVW::_011 => 3,
            FRDIVW::_100 => 4,
            FRDIVW::_101 => 5,
            FRDIVW::_110 => 6,
            FRDIVW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FRDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FRDIVW::_000)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FRDIVW::_001)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FRDIVW::_010)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FRDIVW::_011)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FRDIVW::_100)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FRDIVW::_101)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FRDIVW::_110)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FRDIVW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKS`"]
pub enum CLKSW {
    #[doc = "Encoding 0 - Output of FLL or PLLCS is selected (depends on PLLS control bit)."]
    _00,
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = "Encoding 3 - Reserved."]
    _11,
}
impl CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSW::_00 => 0,
            CLKSW::_01 => 1,
            CLKSW::_10 => 2,
            CLKSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Encoding 0 - Output of FLL or PLLCS is selected (depends on PLLS control bit)."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKSW::_00)
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKSW::_01)
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKSW::_10)
    }
    #[doc = "Encoding 3 - Reserved."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline]
    pub fn irefsten(&self) -> IREFSTENR {
        IREFSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline]
    pub fn irclken(&self) -> IRCLKENR {
        IRCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline]
    pub fn irefs(&self) -> IREFSR {
        IREFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline]
    pub fn frdiv(&self) -> FRDIVR {
        FRDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline]
    pub fn irefsten(&mut self) -> _IREFSTENW {
        _IREFSTENW { w: self }
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline]
    pub fn irclken(&mut self) -> _IRCLKENW {
        _IRCLKENW { w: self }
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline]
    pub fn irefs(&mut self) -> _IREFSW {
        _IREFSW { w: self }
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline]
    pub fn frdiv(&mut self) -> _FRDIVW {
        _FRDIVW { w: self }
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline]
    pub fn clks(&mut self) -> _CLKSW {
        _CLKSW { w: self }
    }
}
