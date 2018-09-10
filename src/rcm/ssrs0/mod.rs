#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SSRS0 {
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
#[doc = "Possible values of the field `SWAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAKEUPR {
    #[doc = "Reset not caused by LLWU module wakeup source"]
    _0,
    #[doc = "Reset caused by LLWU module wakeup source"]
    _1,
}
impl SWAKEUPR {
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
            SWAKEUPR::_0 => false,
            SWAKEUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAKEUPR {
        match value {
            false => SWAKEUPR::_0,
            true => SWAKEUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWAKEUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWAKEUPR::_1
    }
}
#[doc = "Possible values of the field `SLVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDR {
    #[doc = "Reset not caused by LVD trip or POR"]
    _0,
    #[doc = "Reset caused by LVD trip or POR"]
    _1,
}
impl SLVDR {
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
            SLVDR::_0 => false,
            SLVDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDR {
        match value {
            false => SLVDR::_0,
            true => SLVDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLVDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLVDR::_1
    }
}
#[doc = "Possible values of the field `SLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCR {
    #[doc = "Reset not caused by a loss of external clock."]
    _0,
    #[doc = "Reset caused by a loss of external clock."]
    _1,
}
impl SLOCR {
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
            SLOCR::_0 => false,
            SLOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOCR {
        match value {
            false => SLOCR::_0,
            true => SLOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOCR::_1
    }
}
#[doc = "Possible values of the field `SLOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOLR {
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    _0,
    #[doc = "Reset caused by a loss of lock in the PLL"]
    _1,
}
impl SLOLR {
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
            SLOLR::_0 => false,
            SLOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOLR {
        match value {
            false => SLOLR::_0,
            true => SLOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOLR::_1
    }
}
#[doc = "Possible values of the field `SWDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOGR {
    #[doc = "Reset not caused by watchdog timeout"]
    _0,
    #[doc = "Reset caused by watchdog timeout"]
    _1,
}
impl SWDOGR {
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
            SWDOGR::_0 => false,
            SWDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWDOGR {
        match value {
            false => SWDOGR::_0,
            true => SWDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWDOGR::_1
    }
}
#[doc = "Possible values of the field `SPIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPINR {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl SPINR {
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
            SPINR::_0 => false,
            SPINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPINR {
        match value {
            false => SPINR::_0,
            true => SPINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPINR::_1
    }
}
#[doc = "Possible values of the field `SPOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPORR {
    #[doc = "Reset not caused by POR"]
    _0,
    #[doc = "Reset caused by POR"]
    _1,
}
impl SPORR {
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
            SPORR::_0 => false,
            SPORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPORR {
        match value {
            false => SPORR::_0,
            true => SPORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPORR::_1
    }
}
#[doc = "Values that can be written to the field `SWAKEUP`"]
pub enum SWAKEUPW {
    #[doc = "Reset not caused by LLWU module wakeup source"]
    _0,
    #[doc = "Reset caused by LLWU module wakeup source"]
    _1,
}
impl SWAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWAKEUPW::_0 => false,
            SWAKEUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWAKEUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAKEUPW::_0)
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAKEUPW::_1)
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
#[doc = "Values that can be written to the field `SLVD`"]
pub enum SLVDW {
    #[doc = "Reset not caused by LVD trip or POR"]
    _0,
    #[doc = "Reset caused by LVD trip or POR"]
    _1,
}
impl SLVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDW::_0 => false,
            SLVDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVDW::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVDW::_1)
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
#[doc = "Values that can be written to the field `SLOC`"]
pub enum SLOCW {
    #[doc = "Reset not caused by a loss of external clock."]
    _0,
    #[doc = "Reset caused by a loss of external clock."]
    _1,
}
impl SLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOCW::_0 => false,
            SLOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCW::_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCW::_1)
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
#[doc = "Values that can be written to the field `SLOL`"]
pub enum SLOLW {
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    _0,
    #[doc = "Reset caused by a loss of lock in the PLL"]
    _1,
}
impl SLOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOLW::_0 => false,
            SLOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOLW::_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOLW::_1)
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
#[doc = "Values that can be written to the field `SWDOG`"]
pub enum SWDOGW {
    #[doc = "Reset not caused by watchdog timeout"]
    _0,
    #[doc = "Reset caused by watchdog timeout"]
    _1,
}
impl SWDOGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWDOGW::_0 => false,
            SWDOGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWDOGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWDOGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWDOGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOGW::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOGW::_1)
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
#[doc = "Values that can be written to the field `SPIN`"]
pub enum SPINW {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl SPINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPINW::_0 => false,
            SPINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPINW<'a> {
    w: &'a mut W,
}
impl<'a> _SPINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPINW::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPINW::_1)
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
#[doc = "Values that can be written to the field `SPOR`"]
pub enum SPORW {
    #[doc = "Reset not caused by POR"]
    _0,
    #[doc = "Reset caused by POR"]
    _1,
}
impl SPORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPORW::_0 => false,
            SPORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPORW<'a> {
    w: &'a mut W,
}
impl<'a> _SPORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPORW::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPORW::_1)
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
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline]
    pub fn swakeup(&self) -> SWAKEUPR {
        SWAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline]
    pub fn slvd(&self) -> SLVDR {
        SLVDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline]
    pub fn sloc(&self) -> SLOCR {
        SLOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline]
    pub fn slol(&self) -> SLOLR {
        SLOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline]
    pub fn swdog(&self) -> SWDOGR {
        SWDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline]
    pub fn spin(&self) -> SPINR {
        SPINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline]
    pub fn spor(&self) -> SPORR {
        SPORR::_from({
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
        W { bits: 130 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline]
    pub fn swakeup(&mut self) -> _SWAKEUPW {
        _SWAKEUPW { w: self }
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline]
    pub fn slvd(&mut self) -> _SLVDW {
        _SLVDW { w: self }
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline]
    pub fn sloc(&mut self) -> _SLOCW {
        _SLOCW { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline]
    pub fn slol(&mut self) -> _SLOLW {
        _SLOLW { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline]
    pub fn swdog(&mut self) -> _SWDOGW {
        _SWDOGW { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline]
    pub fn spin(&mut self) -> _SPINW {
        _SPINW { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline]
    pub fn spor(&mut self) -> _SPORW {
        _SPORW { w: self }
    }
}
