#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::INTEN {
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
#[doc = "Possible values of the field `USBRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSTENR {
    #[doc = "Disables the USBRST interrupt."]
    _0,
    #[doc = "Enables the USBRST interrupt."]
    _1,
}
impl USBRSTENR {
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
            USBRSTENR::_0 => false,
            USBRSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRSTENR {
        match value {
            false => USBRSTENR::_0,
            true => USBRSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBRSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBRSTENR::_1
    }
}
#[doc = "Possible values of the field `ERROREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORENR {
    #[doc = "Disables the ERROR interrupt."]
    _0,
    #[doc = "Enables the ERROR interrupt."]
    _1,
}
impl ERRORENR {
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
            ERRORENR::_0 => false,
            ERRORENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORENR {
        match value {
            false => ERRORENR::_0,
            true => ERRORENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRORENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRORENR::_1
    }
}
#[doc = "Possible values of the field `SOFTOKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTOKENR {
    #[doc = "Disbles the SOFTOK interrupt."]
    _0,
    #[doc = "Enables the SOFTOK interrupt."]
    _1,
}
impl SOFTOKENR {
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
            SOFTOKENR::_0 => false,
            SOFTOKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFTOKENR {
        match value {
            false => SOFTOKENR::_0,
            true => SOFTOKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOFTOKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOFTOKENR::_1
    }
}
#[doc = "Possible values of the field `TOKDNEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOKDNEENR {
    #[doc = "Disables the TOKDNE interrupt."]
    _0,
    #[doc = "Enables the TOKDNE interrupt."]
    _1,
}
impl TOKDNEENR {
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
            TOKDNEENR::_0 => false,
            TOKDNEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOKDNEENR {
        match value {
            false => TOKDNEENR::_0,
            true => TOKDNEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOKDNEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOKDNEENR::_1
    }
}
#[doc = "Possible values of the field `SLEEPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENR {
    #[doc = "Disables the SLEEP interrupt."]
    _0,
    #[doc = "Enables the SLEEP interrupt."]
    _1,
}
impl SLEEPENR {
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
            SLEEPENR::_0 => false,
            SLEEPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPENR {
        match value {
            false => SLEEPENR::_0,
            true => SLEEPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLEEPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLEEPENR::_1
    }
}
#[doc = "Possible values of the field `RESUMEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMEENR {
    #[doc = "Disables the RESUME interrupt."]
    _0,
    #[doc = "Enables the RESUME interrupt."]
    _1,
}
impl RESUMEENR {
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
            RESUMEENR::_0 => false,
            RESUMEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESUMEENR {
        match value {
            false => RESUMEENR::_0,
            true => RESUMEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESUMEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESUMEENR::_1
    }
}
#[doc = "Possible values of the field `ATTACHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTACHENR {
    #[doc = "Disables the ATTACH interrupt."]
    _0,
    #[doc = "Enables the ATTACH interrupt."]
    _1,
}
impl ATTACHENR {
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
            ATTACHENR::_0 => false,
            ATTACHENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATTACHENR {
        match value {
            false => ATTACHENR::_0,
            true => ATTACHENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ATTACHENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ATTACHENR::_1
    }
}
#[doc = "Possible values of the field `STALLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLENR {
    #[doc = "Diasbles the STALL interrupt."]
    _0,
    #[doc = "Enables the STALL interrupt."]
    _1,
}
impl STALLENR {
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
            STALLENR::_0 => false,
            STALLENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALLENR {
        match value {
            false => STALLENR::_0,
            true => STALLENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALLENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALLENR::_1
    }
}
#[doc = "Values that can be written to the field `USBRSTEN`"]
pub enum USBRSTENW {
    #[doc = "Disables the USBRST interrupt."]
    _0,
    #[doc = "Enables the USBRST interrupt."]
    _1,
}
impl USBRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRSTENW::_0 => false,
            USBRSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the USBRST interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRSTENW::_0)
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRSTENW::_1)
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
#[doc = "Values that can be written to the field `ERROREN`"]
pub enum ERRORENW {
    #[doc = "Disables the ERROR interrupt."]
    _0,
    #[doc = "Enables the ERROR interrupt."]
    _1,
}
impl ERRORENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORENW::_0 => false,
            ERRORENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRORENW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRORENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the ERROR interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRORENW::_0)
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRORENW::_1)
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
#[doc = "Values that can be written to the field `SOFTOKEN`"]
pub enum SOFTOKENW {
    #[doc = "Disbles the SOFTOK interrupt."]
    _0,
    #[doc = "Enables the SOFTOK interrupt."]
    _1,
}
impl SOFTOKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFTOKENW::_0 => false,
            SOFTOKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFTOKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTOKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFTOKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTOKENW::_0)
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTOKENW::_1)
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
#[doc = "Values that can be written to the field `TOKDNEEN`"]
pub enum TOKDNEENW {
    #[doc = "Disables the TOKDNE interrupt."]
    _0,
    #[doc = "Enables the TOKDNE interrupt."]
    _1,
}
impl TOKDNEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOKDNEENW::_0 => false,
            TOKDNEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOKDNEENW<'a> {
    w: &'a mut W,
}
impl<'a> _TOKDNEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOKDNEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOKDNEENW::_0)
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOKDNEENW::_1)
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
#[doc = "Values that can be written to the field `SLEEPEN`"]
pub enum SLEEPENW {
    #[doc = "Disables the SLEEP interrupt."]
    _0,
    #[doc = "Enables the SLEEP interrupt."]
    _1,
}
impl SLEEPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPENW::_0 => false,
            SLEEPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the SLEEP interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPENW::_0)
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPENW::_1)
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
#[doc = "Values that can be written to the field `RESUMEEN`"]
pub enum RESUMEENW {
    #[doc = "Disables the RESUME interrupt."]
    _0,
    #[doc = "Enables the RESUME interrupt."]
    _1,
}
impl RESUMEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESUMEENW::_0 => false,
            RESUMEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESUMEENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESUMEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the RESUME interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESUMEENW::_0)
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESUMEENW::_1)
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
#[doc = "Values that can be written to the field `ATTACHEN`"]
pub enum ATTACHENW {
    #[doc = "Disables the ATTACH interrupt."]
    _0,
    #[doc = "Enables the ATTACH interrupt."]
    _1,
}
impl ATTACHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATTACHENW::_0 => false,
            ATTACHENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATTACHENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATTACHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATTACHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the ATTACH interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACHENW::_0)
    }
    #[doc = "Enables the ATTACH interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACHENW::_1)
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
#[doc = "Values that can be written to the field `STALLEN`"]
pub enum STALLENW {
    #[doc = "Diasbles the STALL interrupt."]
    _0,
    #[doc = "Enables the STALL interrupt."]
    _1,
}
impl STALLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALLENW::_0 => false,
            STALLENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALLENW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diasbles the STALL interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALLENW::_0)
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALLENW::_1)
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
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline]
    pub fn usbrsten(&self) -> USBRSTENR {
        USBRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline]
    pub fn erroren(&self) -> ERRORENR {
        ERRORENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline]
    pub fn softoken(&self) -> SOFTOKENR {
        SOFTOKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline]
    pub fn tokdneen(&self) -> TOKDNEENR {
        TOKDNEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline]
    pub fn sleepen(&self) -> SLEEPENR {
        SLEEPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline]
    pub fn resumeen(&self) -> RESUMEENR {
        RESUMEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline]
    pub fn attachen(&self) -> ATTACHENR {
        ATTACHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline]
    pub fn stallen(&self) -> STALLENR {
        STALLENR::_from({
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
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline]
    pub fn usbrsten(&mut self) -> _USBRSTENW {
        _USBRSTENW { w: self }
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline]
    pub fn erroren(&mut self) -> _ERRORENW {
        _ERRORENW { w: self }
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline]
    pub fn softoken(&mut self) -> _SOFTOKENW {
        _SOFTOKENW { w: self }
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline]
    pub fn tokdneen(&mut self) -> _TOKDNEENW {
        _TOKDNEENW { w: self }
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline]
    pub fn sleepen(&mut self) -> _SLEEPENW {
        _SLEEPENW { w: self }
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline]
    pub fn resumeen(&mut self) -> _RESUMEENW {
        _RESUMEENW { w: self }
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline]
    pub fn attachen(&mut self) -> _ATTACHENW {
        _ATTACHENW { w: self }
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline]
    pub fn stallen(&mut self) -> _STALLENW {
        _STALLENW { w: self }
    }
}
