#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSCTL {
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
#[doc = "Possible values of the field `IPGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGENR {
    #[doc = "Bus clock will be internally gated off."]
    _0,
    #[doc = "Bus clock will not be automatically gated off."]
    _1,
}
impl IPGENR {
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
            IPGENR::_0 => false,
            IPGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPGENR {
        match value {
            false => IPGENR::_0,
            true => IPGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPGENR::_1
    }
}
#[doc = "Possible values of the field `HCKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKENR {
    #[doc = "System clock will be internally gated off."]
    _0,
    #[doc = "System clock will not be automatically gated off."]
    _1,
}
impl HCKENR {
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
            HCKENR::_0 => false,
            HCKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCKENR {
        match value {
            false => HCKENR::_0,
            true => HCKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HCKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HCKENR::_1
    }
}
#[doc = "Possible values of the field `PEREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERENR {
    #[doc = "SDHC clock will be internally gated off."]
    _0,
    #[doc = "SDHC clock will not be automatically gated off."]
    _1,
}
impl PERENR {
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
            PERENR::_0 => false,
            PERENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERENR {
        match value {
            false => PERENR::_0,
            true => PERENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PERENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PERENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SDCLKENR {
    bits: bool,
}
impl SDCLKENR {
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
#[doc = "Possible values of the field `DVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVSR {
    #[doc = "Divisor by 1."]
    _0,
    #[doc = "Divisor by 2."]
    _1,
    #[doc = "Divisor by 15."]
    _1110,
    #[doc = "Divisor by 16."]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DVSR::_0 => 0,
            DVSR::_1 => 1,
            DVSR::_1110 => 14,
            DVSR::_1111 => 15,
            DVSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DVSR {
        match value {
            0 => DVSR::_0,
            1 => DVSR::_1,
            14 => DVSR::_1110,
            15 => DVSR::_1111,
            i => DVSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DVSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DVSR::_1
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DVSR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DVSR::_1111
    }
}
#[doc = "Possible values of the field `SDCLKFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKFSR {
    #[doc = "Base clock divided by 2."]
    _1,
    #[doc = "Base clock divided by 4."]
    _10,
    #[doc = "Base clock divided by 8."]
    _100,
    #[doc = "Base clock divided by 16."]
    _1000,
    #[doc = "Base clock divided by 32."]
    _10000,
    #[doc = "Base clock divided by 64."]
    _100000,
    #[doc = "Base clock divided by 128."]
    _1000000,
    #[doc = "Base clock divided by 256."]
    _10000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDCLKFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDCLKFSR::_1 => 1,
            SDCLKFSR::_10 => 2,
            SDCLKFSR::_100 => 4,
            SDCLKFSR::_1000 => 8,
            SDCLKFSR::_10000 => 16,
            SDCLKFSR::_100000 => 32,
            SDCLKFSR::_1000000 => 64,
            SDCLKFSR::_10000000 => 128,
            SDCLKFSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDCLKFSR {
        match value {
            1 => SDCLKFSR::_1,
            2 => SDCLKFSR::_10,
            4 => SDCLKFSR::_100,
            8 => SDCLKFSR::_1000,
            16 => SDCLKFSR::_10000,
            32 => SDCLKFSR::_100000,
            64 => SDCLKFSR::_1000000,
            128 => SDCLKFSR::_10000000,
            i => SDCLKFSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDCLKFSR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SDCLKFSR::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SDCLKFSR::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == SDCLKFSR::_1000
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == SDCLKFSR::_10000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline]
    pub fn is_100000(&self) -> bool {
        *self == SDCLKFSR::_100000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline]
    pub fn is_1000000(&self) -> bool {
        *self == SDCLKFSR::_1000000
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline]
    pub fn is_10000000(&self) -> bool {
        *self == SDCLKFSR::_10000000
    }
}
#[doc = "Possible values of the field `DTOCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCVR {
    #[doc = "SDCLK x 2 13"]
    _0000,
    #[doc = "SDCLK x 2 14"]
    _0001,
    #[doc = "SDCLK x 2 27"]
    _1110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTOCVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOCVR::_0000 => 0,
            DTOCVR::_0001 => 1,
            DTOCVR::_1110 => 14,
            DTOCVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOCVR {
        match value {
            0 => DTOCVR::_0000,
            1 => DTOCVR::_0001,
            14 => DTOCVR::_1110,
            i => DTOCVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DTOCVR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DTOCVR::_0001
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DTOCVR::_1110
    }
}
#[doc = r" Value of the field"]
pub struct INITAR {
    bits: bool,
}
impl INITAR {
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
#[doc = "Values that can be written to the field `IPGEN`"]
pub enum IPGENW {
    #[doc = "Bus clock will be internally gated off."]
    _0,
    #[doc = "Bus clock will not be automatically gated off."]
    _1,
}
impl IPGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPGENW::_0 => false,
            IPGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPGENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus clock will be internally gated off."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPGENW::_0)
    }
    #[doc = "Bus clock will not be automatically gated off."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPGENW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HCKEN`"]
pub enum HCKENW {
    #[doc = "System clock will be internally gated off."]
    _0,
    #[doc = "System clock will not be automatically gated off."]
    _1,
}
impl HCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCKENW::_0 => false,
            HCKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _HCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System clock will be internally gated off."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCKENW::_0)
    }
    #[doc = "System clock will not be automatically gated off."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCKENW::_1)
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
#[doc = "Values that can be written to the field `PEREN`"]
pub enum PERENW {
    #[doc = "SDHC clock will be internally gated off."]
    _0,
    #[doc = "SDHC clock will not be automatically gated off."]
    _1,
}
impl PERENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERENW::_0 => false,
            PERENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SDHC clock will be internally gated off."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PERENW::_0)
    }
    #[doc = "SDHC clock will not be automatically gated off."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PERENW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKENW<'a> {
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
#[doc = "Values that can be written to the field `DVS`"]
pub enum DVSW {
    #[doc = "Divisor by 1."]
    _0,
    #[doc = "Divisor by 2."]
    _1,
    #[doc = "Divisor by 15."]
    _1110,
    #[doc = "Divisor by 16."]
    _1111,
}
impl DVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DVSW::_0 => 0,
            DVSW::_1 => 1,
            DVSW::_1110 => 14,
            DVSW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DVSW<'a> {
    w: &'a mut W,
}
impl<'a> _DVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DVSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divisor by 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVSW::_0)
    }
    #[doc = "Divisor by 2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVSW::_1)
    }
    #[doc = "Divisor by 15."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DVSW::_1110)
    }
    #[doc = "Divisor by 16."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DVSW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDCLKFS`"]
pub enum SDCLKFSW {
    #[doc = "Base clock divided by 2."]
    _1,
    #[doc = "Base clock divided by 4."]
    _10,
    #[doc = "Base clock divided by 8."]
    _100,
    #[doc = "Base clock divided by 16."]
    _1000,
    #[doc = "Base clock divided by 32."]
    _10000,
    #[doc = "Base clock divided by 64."]
    _100000,
    #[doc = "Base clock divided by 128."]
    _1000000,
    #[doc = "Base clock divided by 256."]
    _10000000,
}
impl SDCLKFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCLKFSW::_1 => 1,
            SDCLKFSW::_10 => 2,
            SDCLKFSW::_100 => 4,
            SDCLKFSW::_1000 => 8,
            SDCLKFSW::_10000 => 16,
            SDCLKFSW::_100000 => 32,
            SDCLKFSW::_1000000 => 64,
            SDCLKFSW::_10000000 => 128,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKFSW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCLKFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Base clock divided by 2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCLKFSW::_1)
    }
    #[doc = "Base clock divided by 4."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDCLKFSW::_10)
    }
    #[doc = "Base clock divided by 8."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SDCLKFSW::_100)
    }
    #[doc = "Base clock divided by 16."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SDCLKFSW::_1000)
    }
    #[doc = "Base clock divided by 32."]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(SDCLKFSW::_10000)
    }
    #[doc = "Base clock divided by 64."]
    #[inline]
    pub fn _100000(self) -> &'a mut W {
        self.variant(SDCLKFSW::_100000)
    }
    #[doc = "Base clock divided by 128."]
    #[inline]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(SDCLKFSW::_1000000)
    }
    #[doc = "Base clock divided by 256."]
    #[inline]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(SDCLKFSW::_10000000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTOCV`"]
pub enum DTOCVW {
    #[doc = "SDCLK x 2 13"]
    _0000,
    #[doc = "SDCLK x 2 14"]
    _0001,
    #[doc = "SDCLK x 2 27"]
    _1110,
}
impl DTOCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOCVW::_0000 => 0,
            DTOCVW::_0001 => 1,
            DTOCVW::_1110 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOCVW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOCVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDCLK x 2 13"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCVW::_0000)
    }
    #[doc = "SDCLK x 2 14"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCVW::_0001)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCVW::_1110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTA`"]
pub enum RSTAW {
    #[doc = "No reset."]
    _0,
    #[doc = "Reset."]
    _1,
}
impl RSTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTAW::_0 => false,
            RSTAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTAW::_0)
    }
    #[doc = "Reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTAW::_1)
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
#[doc = "Values that can be written to the field `RSTC`"]
pub enum RSTCW {
    #[doc = "No reset."]
    _0,
    #[doc = "Reset."]
    _1,
}
impl RSTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTCW::_0 => false,
            RSTCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTCW::_0)
    }
    #[doc = "Reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTCW::_1)
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
#[doc = "Values that can be written to the field `RSTD`"]
pub enum RSTDW {
    #[doc = "No reset."]
    _0,
    #[doc = "Reset."]
    _1,
}
impl RSTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTDW::_0 => false,
            RSTDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTDW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTDW::_0)
    }
    #[doc = "Reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTDW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INITAW<'a> {
    w: &'a mut W,
}
impl<'a> _INITAW<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline]
    pub fn ipgen(&self) -> IPGENR {
        IPGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline]
    pub fn hcken(&self) -> HCKENR {
        HCKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline]
    pub fn peren(&self) -> PERENR {
        PERENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline]
    pub fn sdclken(&self) -> SDCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDCLKENR { bits }
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline]
    pub fn dvs(&self) -> DVSR {
        DVSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfs(&self) -> SDCLKFSR {
        SDCLKFSR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dtocv(&self) -> DTOCVR {
        DTOCVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline]
    pub fn inita(&self) -> INITAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INITAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32776 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline]
    pub fn ipgen(&mut self) -> _IPGENW {
        _IPGENW { w: self }
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline]
    pub fn hcken(&mut self) -> _HCKENW {
        _HCKENW { w: self }
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline]
    pub fn peren(&mut self) -> _PERENW {
        _PERENW { w: self }
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline]
    pub fn sdclken(&mut self) -> _SDCLKENW {
        _SDCLKENW { w: self }
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline]
    pub fn dvs(&mut self) -> _DVSW {
        _DVSW { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfs(&mut self) -> _SDCLKFSW {
        _SDCLKFSW { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dtocv(&mut self) -> _DTOCVW {
        _DTOCVW { w: self }
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline]
    pub fn rsta(&mut self) -> _RSTAW {
        _RSTAW { w: self }
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline]
    pub fn rstc(&mut self) -> _RSTCW {
        _RSTCW { w: self }
    }
    #[doc = "Bit 26 - Software Reset For DAT Line"]
    #[inline]
    pub fn rstd(&mut self) -> _RSTDW {
        _RSTDW { w: self }
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline]
    pub fn inita(&mut self) -> _INITAW {
        _INITAW { w: self }
    }
}
