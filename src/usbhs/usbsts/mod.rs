#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBSTS {
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
#[doc = r" Value of the field"]
pub struct UIR {
    bits: bool,
}
impl UIR {
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
#[doc = "Possible values of the field `UEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEIR {
    #[doc = "No error"]
    _0,
    #[doc = "Error detected"]
    _1,
}
impl UEIR {
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
            UEIR::_0 => false,
            UEIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UEIR {
        match value {
            false => UEIR::_0,
            true => UEIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UEIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UEIR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PCIR {
    bits: bool,
}
impl PCIR {
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
#[doc = r" Value of the field"]
pub struct FRIR {
    bits: bool,
}
impl FRIR {
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
#[doc = "Possible values of the field `SEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIR {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Error"]
    _1,
}
impl SEIR {
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
            SEIR::_0 => false,
            SEIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEIR {
        match value {
            false => SEIR::_0,
            true => SEIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEIR::_1
    }
}
#[doc = "Possible values of the field `AAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAIR {
    #[doc = "No async advance interrupt"]
    _0,
    #[doc = "Async advance interrupt"]
    _1,
}
impl AAIR {
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
            AAIR::_0 => false,
            AAIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAIR {
        match value {
            false => AAIR::_0,
            true => AAIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AAIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AAIR::_1
    }
}
#[doc = "Possible values of the field `URI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URIR {
    #[doc = "No reset received"]
    _0,
    #[doc = "Reset received"]
    _1,
}
impl URIR {
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
            URIR::_0 => false,
            URIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> URIR {
        match value {
            false => URIR::_0,
            true => URIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == URIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == URIR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SRIR {
    bits: bool,
}
impl SRIR {
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
#[doc = "Possible values of the field `SLI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLIR {
    #[doc = "Active"]
    _0,
    #[doc = "Suspended"]
    _1,
}
impl SLIR {
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
            SLIR::_0 => false,
            SLIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLIR {
        match value {
            false => SLIR::_0,
            true => SLIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLIR::_1
    }
}
#[doc = "Possible values of the field `HCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCHR {
    #[doc = "Running"]
    _0,
    #[doc = "Halted"]
    _1,
}
impl HCHR {
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
            HCHR::_0 => false,
            HCHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCHR {
        match value {
            false => HCHR::_0,
            true => HCHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HCHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HCHR::_1
    }
}
#[doc = "Possible values of the field `RCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCLR {
    #[doc = "Non-empty asynchronous schedule"]
    _0,
    #[doc = "Empty asynchronous schedule"]
    _1,
}
impl RCLR {
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
            RCLR::_0 => false,
            RCLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCLR {
        match value {
            false => RCLR::_0,
            true => RCLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCLR::_1
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl PSR {
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
            PSR::_0 => false,
            PSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::_0,
            true => PSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSR::_1
    }
}
#[doc = "Possible values of the field `AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl ASR {
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
            ASR::_0 => false,
            ASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASR {
        match value {
            false => ASR::_0,
            true => ASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASR::_1
    }
}
#[doc = r" Value of the field"]
pub struct NAKIR {
    bits: bool,
}
impl NAKIR {
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
#[doc = r" Value of the field"]
pub struct UAIR {
    bits: bool,
}
impl UAIR {
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
#[doc = r" Value of the field"]
pub struct UPIR {
    bits: bool,
}
impl UPIR {
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
#[doc = "Possible values of the field `TI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI0R {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl TI0R {
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
            TI0R::_0 => false,
            TI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI0R {
        match value {
            false => TI0R::_0,
            true => TI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TI0R::_1
    }
}
#[doc = "Possible values of the field `TI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1R {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl TI1R {
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
            TI1R::_0 => false,
            TI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI1R {
        match value {
            false => TI1R::_0,
            true => TI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TI1R::_1
    }
}
#[doc = r" Proxy"]
pub struct _UIW<'a> {
    w: &'a mut W,
}
impl<'a> _UIW<'a> {
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
#[doc = "Values that can be written to the field `UEI`"]
pub enum UEIW {
    #[doc = "No error"]
    _0,
    #[doc = "Error detected"]
    _1,
}
impl UEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UEIW::_0 => false,
            UEIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UEIW<'a> {
    w: &'a mut W,
}
impl<'a> _UEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UEIW::_0)
    }
    #[doc = "Error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UEIW::_1)
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
#[doc = r" Proxy"]
pub struct _PCIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIW<'a> {
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
#[doc = r" Proxy"]
pub struct _FRIW<'a> {
    w: &'a mut W,
}
impl<'a> _FRIW<'a> {
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
#[doc = "Values that can be written to the field `SEI`"]
pub enum SEIW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Error"]
    _1,
}
impl SEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEIW::_0 => false,
            SEIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEIW<'a> {
    w: &'a mut W,
}
impl<'a> _SEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIW::_0)
    }
    #[doc = "Error"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIW::_1)
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
#[doc = "Values that can be written to the field `AAI`"]
pub enum AAIW {
    #[doc = "No async advance interrupt"]
    _0,
    #[doc = "Async advance interrupt"]
    _1,
}
impl AAIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAIW::_0 => false,
            AAIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAIW<'a> {
    w: &'a mut W,
}
impl<'a> _AAIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No async advance interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAIW::_0)
    }
    #[doc = "Async advance interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAIW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `URI`"]
pub enum URIW {
    #[doc = "No reset received"]
    _0,
    #[doc = "Reset received"]
    _1,
}
impl URIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            URIW::_0 => false,
            URIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URIW<'a> {
    w: &'a mut W,
}
impl<'a> _URIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: URIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset received"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(URIW::_0)
    }
    #[doc = "Reset received"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(URIW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRIW<'a> {
    w: &'a mut W,
}
impl<'a> _SRIW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLI`"]
pub enum SLIW {
    #[doc = "Active"]
    _0,
    #[doc = "Suspended"]
    _1,
}
impl SLIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLIW::_0 => false,
            SLIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLIW<'a> {
    w: &'a mut W,
}
impl<'a> _SLIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLIW::_0)
    }
    #[doc = "Suspended"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLIW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UAIW<'a> {
    w: &'a mut W,
}
impl<'a> _UAIW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPIW<'a> {
    w: &'a mut W,
}
impl<'a> _UPIW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TI0`"]
pub enum TI0W {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl TI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TI0W::_0 => false,
            TI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TI0W<'a> {
    w: &'a mut W,
}
impl<'a> _TI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TI0W::_0)
    }
    #[doc = "Interrupt occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TI0W::_1)
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
#[doc = "Values that can be written to the field `TI1`"]
pub enum TI1W {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl TI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TI1W::_0 => false,
            TI1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TI1W<'a> {
    w: &'a mut W,
}
impl<'a> _TI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TI1W::_0)
    }
    #[doc = "Interrupt occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TI1W::_1)
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
    #[doc = "Bit 0 - USB Interrupt (USBINT)"]
    #[inline]
    pub fn ui(&self) -> UIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UIR { bits }
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline]
    pub fn uei(&self) -> UEIR {
        UEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port Change detect"]
    #[inline]
    pub fn pci(&self) -> PCIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIR { bits }
    }
    #[doc = "Bit 3 - Frame-list Rollover"]
    #[inline]
    pub fn fri(&self) -> FRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRIR { bits }
    }
    #[doc = "Bit 4 - System Error"]
    #[inline]
    pub fn sei(&self) -> SEIR {
        SEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline]
    pub fn aai(&self) -> AAIR {
        AAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - USB Reset received"]
    #[inline]
    pub fn uri(&self) -> URIR {
        URIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SOF Received"]
    #[inline]
    pub fn sri(&self) -> SRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRIR { bits }
    }
    #[doc = "Bit 8 - Device-controller suspend"]
    #[inline]
    pub fn sli(&self) -> SLIR {
        SLIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Host Controller Halted"]
    #[inline]
    pub fn hch(&self) -> HCHR {
        HCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline]
    pub fn rcl(&self) -> RCLR {
        RCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Periodic schedule Status"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Asynchronous schedule Status"]
    #[inline]
    pub fn as_(&self) -> ASR {
        ASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - NAK Interrupt"]
    #[inline]
    pub fn naki(&self) -> NAKIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKIR { bits }
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt"]
    #[inline]
    pub fn uai(&self) -> UAIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UAIR { bits }
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt"]
    #[inline]
    pub fn upi(&self) -> UPIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPIR { bits }
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt"]
    #[inline]
    pub fn ti0(&self) -> TI0R {
        TI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt"]
    #[inline]
    pub fn ti1(&self) -> TI1R {
        TI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - USB Interrupt (USBINT)"]
    #[inline]
    pub fn ui(&mut self) -> _UIW {
        _UIW { w: self }
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline]
    pub fn uei(&mut self) -> _UEIW {
        _UEIW { w: self }
    }
    #[doc = "Bit 2 - Port Change detect"]
    #[inline]
    pub fn pci(&mut self) -> _PCIW {
        _PCIW { w: self }
    }
    #[doc = "Bit 3 - Frame-list Rollover"]
    #[inline]
    pub fn fri(&mut self) -> _FRIW {
        _FRIW { w: self }
    }
    #[doc = "Bit 4 - System Error"]
    #[inline]
    pub fn sei(&mut self) -> _SEIW {
        _SEIW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline]
    pub fn aai(&mut self) -> _AAIW {
        _AAIW { w: self }
    }
    #[doc = "Bit 6 - USB Reset received"]
    #[inline]
    pub fn uri(&mut self) -> _URIW {
        _URIW { w: self }
    }
    #[doc = "Bit 7 - SOF Received"]
    #[inline]
    pub fn sri(&mut self) -> _SRIW {
        _SRIW { w: self }
    }
    #[doc = "Bit 8 - Device-controller suspend"]
    #[inline]
    pub fn sli(&mut self) -> _SLIW {
        _SLIW { w: self }
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt"]
    #[inline]
    pub fn uai(&mut self) -> _UAIW {
        _UAIW { w: self }
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt"]
    #[inline]
    pub fn upi(&mut self) -> _UPIW {
        _UPIW { w: self }
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt"]
    #[inline]
    pub fn ti0(&mut self) -> _TI0W {
        _TI0W { w: self }
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt"]
    #[inline]
    pub fn ti1(&mut self) -> _TI1W {
        _TI1W { w: self }
    }
}
