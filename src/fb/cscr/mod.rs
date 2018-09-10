#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSCR {
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
#[doc = "Possible values of the field `BSTW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSTWR {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
    _0,
    #[doc = "Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    _1,
}
impl BSTWR {
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
            BSTWR::_0 => false,
            BSTWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSTWR {
        match value {
            false => BSTWR::_0,
            true => BSTWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSTWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSTWR::_1
    }
}
#[doc = "Possible values of the field `BSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSTRR {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
    _0,
    #[doc = "Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
    _1,
}
impl BSTRR {
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
            BSTRR::_0 => false,
            BSTRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSTRR {
        match value {
            false => BSTRR::_0,
            true => BSTRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSTRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSTRR::_1
    }
}
#[doc = "Possible values of the field `BEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEMR {
    #[doc = "FB_BE is asserted for data write only."]
    _0,
    #[doc = "FB_BE is asserted for data read and write accesses."]
    _1,
}
impl BEMR {
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
            BEMR::_0 => false,
            BEMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEMR {
        match value {
            false => BEMR::_0,
            true => BEMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEMR::_1
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
    _00,
    #[doc = "8-bit port size. Valid data is sampled and driven on FB_D\\[31:24\\] when BLS is 0b, or FB_D\\[7:0\\] when BLS is 1b."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_00 => 0,
            PSR::_01 => 1,
            PSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_00,
            1 => PSR::_01,
            i => PSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PSR::_01
    }
}
#[doc = "Possible values of the field `AA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAR {
    #[doc = "Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
    _0,
    #[doc = "Enabled. Internal transfer acknowledge is asserted as specified by WS."]
    _1,
}
impl AAR {
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
            AAR::_0 => false,
            AAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAR {
        match value {
            false => AAR::_0,
            true => AAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AAR::_1
    }
}
#[doc = "Possible values of the field `BLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLSR {
    #[doc = "Not shifted. Data is left-aligned on FB_AD."]
    _0,
    #[doc = "Shifted. Data is right-aligned on FB_AD."]
    _1,
}
impl BLSR {
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
            BLSR::_0 => false,
            BLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLSR {
        match value {
            false => BLSR::_0,
            true => BLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BLSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WSR {
    bits: u8,
}
impl WSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WRAH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRAHR {
    #[doc = "1 cycle (default for all but FB_CS0 )"]
    _00,
    #[doc = "2 cycles"]
    _01,
    #[doc = "3 cycles"]
    _10,
    #[doc = "4 cycles (default for FB_CS0 )"]
    _11,
}
impl WRAHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRAHR::_00 => 0,
            WRAHR::_01 => 1,
            WRAHR::_10 => 2,
            WRAHR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRAHR {
        match value {
            0 => WRAHR::_00,
            1 => WRAHR::_01,
            2 => WRAHR::_10,
            3 => WRAHR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WRAHR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WRAHR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WRAHR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WRAHR::_11
    }
}
#[doc = "Possible values of the field `RDAH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAHR {
    #[doc = "When AA is 1b, 1 cycle. When AA is 0b, 0 cycles."]
    _00,
    #[doc = "When AA is 1b, 2 cycles. When AA is 0b, 1 cycle."]
    _01,
    #[doc = "When AA is 1b, 3 cycles. When AA is 0b, 2 cycles."]
    _10,
    #[doc = "When AA is 1b, 4 cycles. When AA is 0b, 3 cycles."]
    _11,
}
impl RDAHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDAHR::_00 => 0,
            RDAHR::_01 => 1,
            RDAHR::_10 => 2,
            RDAHR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDAHR {
        match value {
            0 => RDAHR::_00,
            1 => RDAHR::_01,
            2 => RDAHR::_10,
            3 => RDAHR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RDAHR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RDAHR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RDAHR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RDAHR::_11
    }
}
#[doc = "Possible values of the field `ASET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASETR {
    #[doc = "Assert FB_CSn on the first rising clock edge after the address is asserted (default for all but FB_CS0 )."]
    _00,
    #[doc = "Assert FB_CSn on the second rising clock edge after the address is asserted."]
    _01,
    #[doc = "Assert FB_CSn on the third rising clock edge after the address is asserted."]
    _10,
    #[doc = "Assert FB_CSn on the fourth rising clock edge after the address is asserted (default for FB_CS0 )."]
    _11,
}
impl ASETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASETR::_00 => 0,
            ASETR::_01 => 1,
            ASETR::_10 => 2,
            ASETR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASETR {
        match value {
            0 => ASETR::_00,
            1 => ASETR::_01,
            2 => ASETR::_10,
            3 => ASETR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ASETR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ASETR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ASETR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ASETR::_11
    }
}
#[doc = "Possible values of the field `EXTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSR {
    #[doc = "Disabled. FB_TS /FB_ALE asserts for one bus clock cycle."]
    _0,
    #[doc = "Enabled. FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts."]
    _1,
}
impl EXTSR {
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
            EXTSR::_0 => false,
            EXTSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTSR {
        match value {
            false => EXTSR::_0,
            true => EXTSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXTSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXTSR::_1
    }
}
#[doc = "Possible values of the field `SWSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSENR {
    #[doc = "Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
    _0,
    #[doc = "Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
    _1,
}
impl SWSENR {
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
            SWSENR::_0 => false,
            SWSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWSENR {
        match value {
            false => SWSENR::_0,
            true => SWSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWSENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SWSR {
    bits: u8,
}
impl SWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BSTW`"]
pub enum BSTWW {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
    _0,
    #[doc = "Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    _1,
}
impl BSTWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSTWW::_0 => false,
            BSTWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSTWW<'a> {
    w: &'a mut W,
}
impl<'a> _BSTWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSTWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSTWW::_0)
    }
    #[doc = "Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSTWW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BSTR`"]
pub enum BSTRW {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
    _0,
    #[doc = "Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
    _1,
}
impl BSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSTRW::_0 => false,
            BSTRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _BSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSTRW::_0)
    }
    #[doc = "Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSTRW::_1)
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
#[doc = "Values that can be written to the field `BEM`"]
pub enum BEMW {
    #[doc = "FB_BE is asserted for data write only."]
    _0,
    #[doc = "FB_BE is asserted for data read and write accesses."]
    _1,
}
impl BEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEMW::_0 => false,
            BEMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEMW<'a> {
    w: &'a mut W,
}
impl<'a> _BEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FB_BE is asserted for data write only."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEMW::_0)
    }
    #[doc = "FB_BE is asserted for data read and write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEMW::_1)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
    _00,
    #[doc = "8-bit port size. Valid data is sampled and driven on FB_D\\[31:24\\] when BLS is 0b, or FB_D\\[7:0\\] when BLS is 1b."]
    _01,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_00 => 0,
            PSW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSW::_00)
    }
    #[doc = "8-bit port size. Valid data is sampled and driven on FB_D\\[31:24\\] when BLS is 0b, or FB_D\\[7:0\\] when BLS is 1b."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AA`"]
pub enum AAW {
    #[doc = "Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
    _0,
    #[doc = "Enabled. Internal transfer acknowledge is asserted as specified by WS."]
    _1,
}
impl AAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAW::_0 => false,
            AAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAW<'a> {
    w: &'a mut W,
}
impl<'a> _AAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAW::_0)
    }
    #[doc = "Enabled. Internal transfer acknowledge is asserted as specified by WS."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAW::_1)
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
#[doc = "Values that can be written to the field `BLS`"]
pub enum BLSW {
    #[doc = "Not shifted. Data is left-aligned on FB_AD."]
    _0,
    #[doc = "Shifted. Data is right-aligned on FB_AD."]
    _1,
}
impl BLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLSW::_0 => false,
            BLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLSW<'a> {
    w: &'a mut W,
}
impl<'a> _BLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not shifted. Data is left-aligned on FB_AD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLSW::_0)
    }
    #[doc = "Shifted. Data is right-aligned on FB_AD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLSW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WSW<'a> {
    w: &'a mut W,
}
impl<'a> _WSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRAH`"]
pub enum WRAHW {
    #[doc = "1 cycle (default for all but FB_CS0 )"]
    _00,
    #[doc = "2 cycles"]
    _01,
    #[doc = "3 cycles"]
    _10,
    #[doc = "4 cycles (default for FB_CS0 )"]
    _11,
}
impl WRAHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRAHW::_00 => 0,
            WRAHW::_01 => 1,
            WRAHW::_10 => 2,
            WRAHW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRAHW<'a> {
    w: &'a mut W,
}
impl<'a> _WRAHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRAHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 cycle (default for all but FB_CS0 )"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WRAHW::_00)
    }
    #[doc = "2 cycles"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WRAHW::_01)
    }
    #[doc = "3 cycles"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WRAHW::_10)
    }
    #[doc = "4 cycles (default for FB_CS0 )"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WRAHW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDAH`"]
pub enum RDAHW {
    #[doc = "When AA is 1b, 1 cycle. When AA is 0b, 0 cycles."]
    _00,
    #[doc = "When AA is 1b, 2 cycles. When AA is 0b, 1 cycle."]
    _01,
    #[doc = "When AA is 1b, 3 cycles. When AA is 0b, 2 cycles."]
    _10,
    #[doc = "When AA is 1b, 4 cycles. When AA is 0b, 3 cycles."]
    _11,
}
impl RDAHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDAHW::_00 => 0,
            RDAHW::_01 => 1,
            RDAHW::_10 => 2,
            RDAHW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDAHW<'a> {
    w: &'a mut W,
}
impl<'a> _RDAHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDAHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When AA is 1b, 1 cycle. When AA is 0b, 0 cycles."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RDAHW::_00)
    }
    #[doc = "When AA is 1b, 2 cycles. When AA is 0b, 1 cycle."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RDAHW::_01)
    }
    #[doc = "When AA is 1b, 3 cycles. When AA is 0b, 2 cycles."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RDAHW::_10)
    }
    #[doc = "When AA is 1b, 4 cycles. When AA is 0b, 3 cycles."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RDAHW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASET`"]
pub enum ASETW {
    #[doc = "Assert FB_CSn on the first rising clock edge after the address is asserted (default for all but FB_CS0 )."]
    _00,
    #[doc = "Assert FB_CSn on the second rising clock edge after the address is asserted."]
    _01,
    #[doc = "Assert FB_CSn on the third rising clock edge after the address is asserted."]
    _10,
    #[doc = "Assert FB_CSn on the fourth rising clock edge after the address is asserted (default for FB_CS0 )."]
    _11,
}
impl ASETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASETW::_00 => 0,
            ASETW::_01 => 1,
            ASETW::_10 => 2,
            ASETW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASETW<'a> {
    w: &'a mut W,
}
impl<'a> _ASETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASETW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Assert FB_CSn on the first rising clock edge after the address is asserted (default for all but FB_CS0 )."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ASETW::_00)
    }
    #[doc = "Assert FB_CSn on the second rising clock edge after the address is asserted."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ASETW::_01)
    }
    #[doc = "Assert FB_CSn on the third rising clock edge after the address is asserted."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ASETW::_10)
    }
    #[doc = "Assert FB_CSn on the fourth rising clock edge after the address is asserted (default for FB_CS0 )."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ASETW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTS`"]
pub enum EXTSW {
    #[doc = "Disabled. FB_TS /FB_ALE asserts for one bus clock cycle."]
    _0,
    #[doc = "Enabled. FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts."]
    _1,
}
impl EXTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTSW::_0 => false,
            EXTSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. FB_TS /FB_ALE asserts for one bus clock cycle."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTSW::_0)
    }
    #[doc = "Enabled. FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTSW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWSEN`"]
pub enum SWSENW {
    #[doc = "Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
    _0,
    #[doc = "Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
    _1,
}
impl SWSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWSENW::_0 => false,
            SWSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSENW::_0)
    }
    #[doc = "Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSENW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWSW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 3 - Burst-Write Enable"]
    #[inline]
    pub fn bstw(&self) -> BSTWR {
        BSTWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Burst-Read Enable"]
    #[inline]
    pub fn bstr(&self) -> BSTRR {
        BSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Byte-Enable Mode"]
    #[inline]
    pub fn bem(&self) -> BEMR {
        BEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Port Size"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Auto-Acknowledge Enable"]
    #[inline]
    pub fn aa(&self) -> AAR {
        AAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Byte-Lane Shift"]
    #[inline]
    pub fn bls(&self) -> BLSR {
        BLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:15 - Wait States"]
    #[inline]
    pub fn ws(&self) -> WSR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WSR { bits }
    }
    #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
    #[inline]
    pub fn wrah(&self) -> WRAHR {
        WRAHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
    #[inline]
    pub fn rdah(&self) -> RDAHR {
        RDAHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Address Setup"]
    #[inline]
    pub fn aset(&self) -> ASETR {
        ASETR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
    #[inline]
    pub fn exts(&self) -> EXTSR {
        EXTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Secondary Wait State Enable"]
    #[inline]
    pub fn swsen(&self) -> SWSENR {
        SWSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:31 - Secondary Wait States"]
    #[inline]
    pub fn sws(&self) -> SWSR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SWSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4193280 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Burst-Write Enable"]
    #[inline]
    pub fn bstw(&mut self) -> _BSTWW {
        _BSTWW { w: self }
    }
    #[doc = "Bit 4 - Burst-Read Enable"]
    #[inline]
    pub fn bstr(&mut self) -> _BSTRW {
        _BSTRW { w: self }
    }
    #[doc = "Bit 5 - Byte-Enable Mode"]
    #[inline]
    pub fn bem(&mut self) -> _BEMW {
        _BEMW { w: self }
    }
    #[doc = "Bits 6:7 - Port Size"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 8 - Auto-Acknowledge Enable"]
    #[inline]
    pub fn aa(&mut self) -> _AAW {
        _AAW { w: self }
    }
    #[doc = "Bit 9 - Byte-Lane Shift"]
    #[inline]
    pub fn bls(&mut self) -> _BLSW {
        _BLSW { w: self }
    }
    #[doc = "Bits 10:15 - Wait States"]
    #[inline]
    pub fn ws(&mut self) -> _WSW {
        _WSW { w: self }
    }
    #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
    #[inline]
    pub fn wrah(&mut self) -> _WRAHW {
        _WRAHW { w: self }
    }
    #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
    #[inline]
    pub fn rdah(&mut self) -> _RDAHW {
        _RDAHW { w: self }
    }
    #[doc = "Bits 20:21 - Address Setup"]
    #[inline]
    pub fn aset(&mut self) -> _ASETW {
        _ASETW { w: self }
    }
    #[doc = "Bit 22 - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
    #[inline]
    pub fn exts(&mut self) -> _EXTSW {
        _EXTSW { w: self }
    }
    #[doc = "Bit 23 - Secondary Wait State Enable"]
    #[inline]
    pub fn swsen(&mut self) -> _SWSENW {
        _SWSENW { w: self }
    }
    #[doc = "Bits 26:31 - Secondary Wait States"]
    #[inline]
    pub fn sws(&mut self) -> _SWSW {
        _SWSW { w: self }
    }
}
