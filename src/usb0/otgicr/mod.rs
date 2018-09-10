#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::OTGICR {
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
#[doc = "Possible values of the field `AVBUSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVBUSENR {
    #[doc = "Disables the AVBUSCHG interrupt."]
    _0,
    #[doc = "Enables the AVBUSCHG interrupt."]
    _1,
}
impl AVBUSENR {
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
            AVBUSENR::_0 => false,
            AVBUSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVBUSENR {
        match value {
            false => AVBUSENR::_0,
            true => AVBUSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVBUSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVBUSENR::_1
    }
}
#[doc = "Possible values of the field `BSESSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSESSENR {
    #[doc = "Disables the B_SESS_CHG interrupt."]
    _0,
    #[doc = "Enables the B_SESS_CHG interrupt."]
    _1,
}
impl BSESSENR {
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
            BSESSENR::_0 => false,
            BSESSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSESSENR {
        match value {
            false => BSESSENR::_0,
            true => BSESSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSESSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSESSENR::_1
    }
}
#[doc = "Possible values of the field `SESSVLDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESSVLDENR {
    #[doc = "Disables the SESSVLDCHG interrupt."]
    _0,
    #[doc = "Enables the SESSVLDCHG interrupt."]
    _1,
}
impl SESSVLDENR {
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
            SESSVLDENR::_0 => false,
            SESSVLDENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SESSVLDENR {
        match value {
            false => SESSVLDENR::_0,
            true => SESSVLDENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SESSVLDENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SESSVLDENR::_1
    }
}
#[doc = "Possible values of the field `LINESTATEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATEENR {
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    _0,
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    _1,
}
impl LINESTATEENR {
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
            LINESTATEENR::_0 => false,
            LINESTATEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINESTATEENR {
        match value {
            false => LINESTATEENR::_0,
            true => LINESTATEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LINESTATEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LINESTATEENR::_1
    }
}
#[doc = "Possible values of the field `ONEMSECEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEMSECENR {
    #[doc = "Diables the 1ms timer interrupt."]
    _0,
    #[doc = "Enables the 1ms timer interrupt."]
    _1,
}
impl ONEMSECENR {
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
            ONEMSECENR::_0 => false,
            ONEMSECENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONEMSECENR {
        match value {
            false => ONEMSECENR::_0,
            true => ONEMSECENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ONEMSECENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ONEMSECENR::_1
    }
}
#[doc = "Possible values of the field `IDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDENR {
    #[doc = "The ID interrupt is disabled"]
    _0,
    #[doc = "The ID interrupt is enabled"]
    _1,
}
impl IDENR {
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
            IDENR::_0 => false,
            IDENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDENR {
        match value {
            false => IDENR::_0,
            true => IDENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDENR::_1
    }
}
#[doc = "Values that can be written to the field `AVBUSEN`"]
pub enum AVBUSENW {
    #[doc = "Disables the AVBUSCHG interrupt."]
    _0,
    #[doc = "Enables the AVBUSCHG interrupt."]
    _1,
}
impl AVBUSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVBUSENW::_0 => false,
            AVBUSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVBUSENW<'a> {
    w: &'a mut W,
}
impl<'a> _AVBUSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVBUSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the AVBUSCHG interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVBUSENW::_0)
    }
    #[doc = "Enables the AVBUSCHG interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVBUSENW::_1)
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
#[doc = "Values that can be written to the field `BSESSEN`"]
pub enum BSESSENW {
    #[doc = "Disables the B_SESS_CHG interrupt."]
    _0,
    #[doc = "Enables the B_SESS_CHG interrupt."]
    _1,
}
impl BSESSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSESSENW::_0 => false,
            BSESSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSESSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BSESSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSESSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the B_SESS_CHG interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSESSENW::_0)
    }
    #[doc = "Enables the B_SESS_CHG interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSESSENW::_1)
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
#[doc = "Values that can be written to the field `SESSVLDEN`"]
pub enum SESSVLDENW {
    #[doc = "Disables the SESSVLDCHG interrupt."]
    _0,
    #[doc = "Enables the SESSVLDCHG interrupt."]
    _1,
}
impl SESSVLDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SESSVLDENW::_0 => false,
            SESSVLDENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SESSVLDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SESSVLDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SESSVLDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the SESSVLDCHG interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SESSVLDENW::_0)
    }
    #[doc = "Enables the SESSVLDCHG interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SESSVLDENW::_1)
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
#[doc = "Values that can be written to the field `LINESTATEEN`"]
pub enum LINESTATEENW {
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    _0,
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    _1,
}
impl LINESTATEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINESTATEENW::_0 => false,
            LINESTATEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINESTATEENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINESTATEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINESTATEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEENW::_0)
    }
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEENW::_1)
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
#[doc = "Values that can be written to the field `ONEMSECEN`"]
pub enum ONEMSECENW {
    #[doc = "Diables the 1ms timer interrupt."]
    _0,
    #[doc = "Enables the 1ms timer interrupt."]
    _1,
}
impl ONEMSECENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONEMSECENW::_0 => false,
            ONEMSECENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONEMSECENW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEMSECENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONEMSECENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diables the 1ms timer interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECENW::_0)
    }
    #[doc = "Enables the 1ms timer interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECENW::_1)
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
#[doc = "Values that can be written to the field `IDEN`"]
pub enum IDENW {
    #[doc = "The ID interrupt is disabled"]
    _0,
    #[doc = "The ID interrupt is enabled"]
    _1,
}
impl IDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDENW::_0 => false,
            IDENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The ID interrupt is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDENW::_0)
    }
    #[doc = "The ID interrupt is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDENW::_1)
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
    #[doc = "Bit 0 - A VBUS Valid Interrupt Enable"]
    #[inline]
    pub fn avbusen(&self) -> AVBUSENR {
        AVBUSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - B Session END Interrupt Enable"]
    #[inline]
    pub fn bsessen(&self) -> BSESSENR {
        BSESSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Session Valid Interrupt Enable"]
    #[inline]
    pub fn sessvlden(&self) -> SESSVLDENR {
        SESSVLDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline]
    pub fn linestateen(&self) -> LINESTATEENR {
        LINESTATEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline]
    pub fn onemsecen(&self) -> ONEMSECENR {
        ONEMSECENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - ID Interrupt Enable"]
    #[inline]
    pub fn iden(&self) -> IDENR {
        IDENR::_from({
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
    #[doc = "Bit 0 - A VBUS Valid Interrupt Enable"]
    #[inline]
    pub fn avbusen(&mut self) -> _AVBUSENW {
        _AVBUSENW { w: self }
    }
    #[doc = "Bit 2 - B Session END Interrupt Enable"]
    #[inline]
    pub fn bsessen(&mut self) -> _BSESSENW {
        _BSESSENW { w: self }
    }
    #[doc = "Bit 3 - Session Valid Interrupt Enable"]
    #[inline]
    pub fn sessvlden(&mut self) -> _SESSVLDENW {
        _SESSVLDENW { w: self }
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline]
    pub fn linestateen(&mut self) -> _LINESTATEENW {
        _LINESTATEENW { w: self }
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline]
    pub fn onemsecen(&mut self) -> _ONEMSECENW {
        _ONEMSECENW { w: self }
    }
    #[doc = "Bit 7 - ID Interrupt Enable"]
    #[inline]
    pub fn iden(&mut self) -> _IDENW {
        _IDENW { w: self }
    }
}
