#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWD_CLR {
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
#[doc = "Possible values of the field `TXPWDFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDFSR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the drivers into high-impedance output"]
    _1,
}
impl TXPWDFSR {
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
            TXPWDFSR::_0 => false,
            TXPWDFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDFSR {
        match value {
            false => TXPWDFSR::_0,
            true => TXPWDFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXPWDFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXPWDFSR::_1
    }
}
#[doc = "Possible values of the field `TXPWDIBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDIBIASR {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the USB is in suspend mode. This effectively powers down the entire USB transmit path"]
    _1,
}
impl TXPWDIBIASR {
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
            TXPWDIBIASR::_0 => false,
            TXPWDIBIASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDIBIASR {
        match value {
            false => TXPWDIBIASR::_0,
            true => TXPWDIBIASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXPWDIBIASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXPWDIBIASR::_1
    }
}
#[doc = "Possible values of the field `TXPWDV2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDV2IR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    _1,
}
impl TXPWDV2IR {
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
            TXPWDV2IR::_0 => false,
            TXPWDV2IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDV2IR {
        match value {
            false => TXPWDV2IR::_0,
            true => TXPWDV2IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXPWDV2IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXPWDV2IR::_1
    }
}
#[doc = "Possible values of the field `RXPWDENV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDENVR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    _1,
}
impl RXPWDENVR {
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
            RXPWDENVR::_0 => false,
            RXPWDENVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDENVR {
        match value {
            false => RXPWDENVR::_0,
            true => RXPWDENVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXPWDENVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXPWDENVR::_1
    }
}
#[doc = "Possible values of the field `RXPWD1PT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWD1PT1R {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    _1,
}
impl RXPWD1PT1R {
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
            RXPWD1PT1R::_0 => false,
            RXPWD1PT1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWD1PT1R {
        match value {
            false => RXPWD1PT1R::_0,
            true => RXPWD1PT1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXPWD1PT1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXPWD1PT1R::_1
    }
}
#[doc = "Possible values of the field `RXPWDDIFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDDIFFR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB high-speed differential receiver"]
    _1,
}
impl RXPWDDIFFR {
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
            RXPWDDIFFR::_0 => false,
            RXPWDDIFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDDIFFR {
        match value {
            false => RXPWDDIFFR::_0,
            true => RXPWDDIFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXPWDDIFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXPWDDIFFR::_1
    }
}
#[doc = "Possible values of the field `RXPWDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDRXR {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    _1,
}
impl RXPWDRXR {
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
            RXPWDRXR::_0 => false,
            RXPWDRXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDRXR {
        match value {
            false => RXPWDRXR::_0,
            true => RXPWDRXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXPWDRXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXPWDRXR::_1
    }
}
#[doc = "Values that can be written to the field `TXPWDFS`"]
pub enum TXPWDFSW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the drivers into high-impedance output"]
    _1,
}
impl TXPWDFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDFSW::_0 => false,
            TXPWDFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPWDFSW::_0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the drivers into high-impedance output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPWDFSW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPWDIBIAS`"]
pub enum TXPWDIBIASW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the USB is in suspend mode. This effectively powers down the entire USB transmit path"]
    _1,
}
impl TXPWDIBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDIBIASW::_0 => false,
            TXPWDIBIASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDIBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDIBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDIBIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPWDIBIASW::_0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the USB is in suspend mode. This effectively powers down the entire USB transmit path"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPWDIBIASW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPWDV2I`"]
pub enum TXPWDV2IW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    _1,
}
impl TXPWDV2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDV2IW::_0 => false,
            TXPWDV2IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDV2IW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDV2IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDV2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXPWDV2IW::_0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXPWDV2IW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWDENV`"]
pub enum RXPWDENVW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    _1,
}
impl RXPWDENVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDENVW::_0 => false,
            RXPWDENVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDENVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDENVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDENVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPWDENVW::_0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPWDENVW::_1)
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
#[doc = "Values that can be written to the field `RXPWD1PT1`"]
pub enum RXPWD1PT1W {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    _1,
}
impl RXPWD1PT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWD1PT1W::_0 => false,
            RXPWD1PT1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWD1PT1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWD1PT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWD1PT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPWD1PT1W::_0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPWD1PT1W::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWDDIFF`"]
pub enum RXPWDDIFFW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Power-down the USB high-speed differential receiver"]
    _1,
}
impl RXPWDDIFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDDIFFW::_0 => false,
            RXPWDDIFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDDIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDDIFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDDIFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPWDDIFFW::_0)
    }
    #[doc = "Power-down the USB high-speed differential receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPWDDIFFW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWDRX`"]
pub enum RXPWDRXW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    _1,
}
impl RXPWDRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDRXW::_0 => false,
            RXPWDRXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXPWDRXW::_0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXPWDRXW::_1)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdfs(&self) -> TXPWDFSR {
        TXPWDFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdibias(&self) -> TXPWDIBIASR {
        TXPWDIBIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdv2i(&self) -> TXPWDV2IR {
        TXPWDV2IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwdenv(&self) -> RXPWDENVR {
        RXPWDENVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1R {
        RXPWD1PT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwddiff(&self) -> RXPWDDIFFR {
        RXPWDDIFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwdrx(&self) -> RXPWDRXR {
        RXPWDRXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1973248 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdfs(&mut self) -> _TXPWDFSW {
        _TXPWDFSW { w: self }
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdibias(&mut self) -> _TXPWDIBIASW {
        _TXPWDIBIASW { w: self }
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn txpwdv2i(&mut self) -> _TXPWDV2IW {
        _TXPWDV2IW { w: self }
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwdenv(&mut self) -> _RXPWDENVW {
        _RXPWDENVW { w: self }
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwd1pt1(&mut self) -> _RXPWD1PT1W {
        _RXPWD1PT1W { w: self }
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwddiff(&mut self) -> _RXPWDDIFFW {
        _RXPWDDIFFW { w: self }
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of USBPHY_CTRL is enabled"]
    #[inline]
    pub fn rxpwdrx(&mut self) -> _RXPWDRXW {
        _RXPWDRXW { w: self }
    }
}
