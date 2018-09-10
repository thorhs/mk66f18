#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PROCTL {
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
#[doc = "Possible values of the field `LCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTLR {
    #[doc = "LED off."]
    _0,
    #[doc = "LED on."]
    _1,
}
impl LCTLR {
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
            LCTLR::_0 => false,
            LCTLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCTLR {
        match value {
            false => LCTLR::_0,
            true => LCTLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LCTLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LCTLR::_1
    }
}
#[doc = "Possible values of the field `DTW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTWR {
    #[doc = "1-bit mode"]
    _00,
    #[doc = "4-bit mode"]
    _01,
    #[doc = "8-bit mode"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTWR::_00 => 0,
            DTWR::_01 => 1,
            DTWR::_10 => 2,
            DTWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTWR {
        match value {
            0 => DTWR::_00,
            1 => DTWR::_01,
            2 => DTWR::_10,
            i => DTWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DTWR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DTWR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DTWR::_10
    }
}
#[doc = "Possible values of the field `D3CD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3CDR {
    #[doc = "DAT3 does not monitor card Insertion."]
    _0,
    #[doc = "DAT3 as card detection pin."]
    _1,
}
impl D3CDR {
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
            D3CDR::_0 => false,
            D3CDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D3CDR {
        match value {
            false => D3CDR::_0,
            true => D3CDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == D3CDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == D3CDR::_1
    }
}
#[doc = "Possible values of the field `EMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMODER {
    #[doc = "Big endian mode"]
    _00,
    #[doc = "Half word big endian mode"]
    _01,
    #[doc = "Little endian mode"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMODER::_00 => 0,
            EMODER::_01 => 1,
            EMODER::_10 => 2,
            EMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMODER {
        match value {
            0 => EMODER::_00,
            1 => EMODER::_01,
            2 => EMODER::_10,
            i => EMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == EMODER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == EMODER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == EMODER::_10
    }
}
#[doc = "Possible values of the field `CDTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTLR {
    #[doc = "Card detect test level is 0, no card inserted."]
    _0,
    #[doc = "Card detect test level is 1, card inserted."]
    _1,
}
impl CDTLR {
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
            CDTLR::_0 => false,
            CDTLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDTLR {
        match value {
            false => CDTLR::_0,
            true => CDTLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CDTLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CDTLR::_1
    }
}
#[doc = "Possible values of the field `CDSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSSR {
    #[doc = "Card detection level is selected for normal purpose."]
    _0,
    #[doc = "Card detection test level is selected for test purpose."]
    _1,
}
impl CDSSR {
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
            CDSSR::_0 => false,
            CDSSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDSSR {
        match value {
            false => CDSSR::_0,
            true => CDSSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CDSSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CDSSR::_1
    }
}
#[doc = "Possible values of the field `DMAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASR {
    #[doc = "No DMA or simple DMA is selected."]
    _00,
    #[doc = "ADMA1 is selected."]
    _01,
    #[doc = "ADMA2 is selected."]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMASR::_00 => 0,
            DMASR::_01 => 1,
            DMASR::_10 => 2,
            DMASR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMASR {
        match value {
            0 => DMASR::_00,
            1 => DMASR::_01,
            2 => DMASR::_10,
            i => DMASR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DMASR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DMASR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DMASR::_10
    }
}
#[doc = "Possible values of the field `SABGREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABGREQR {
    #[doc = "Transfer"]
    _0,
    #[doc = "Stop"]
    _1,
}
impl SABGREQR {
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
            SABGREQR::_0 => false,
            SABGREQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SABGREQR {
        match value {
            false => SABGREQR::_0,
            true => SABGREQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SABGREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SABGREQR::_1
    }
}
#[doc = "Possible values of the field `CREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREQR {
    #[doc = "No effect."]
    _0,
    #[doc = "Restart"]
    _1,
}
impl CREQR {
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
            CREQR::_0 => false,
            CREQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CREQR {
        match value {
            false => CREQR::_0,
            true => CREQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CREQR::_1
    }
}
#[doc = "Possible values of the field `RWCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTLR {
    #[doc = "Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    _0,
    #[doc = "Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    _1,
}
impl RWCTLR {
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
            RWCTLR::_0 => false,
            RWCTLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWCTLR {
        match value {
            false => RWCTLR::_0,
            true => RWCTLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWCTLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWCTLR::_1
    }
}
#[doc = "Possible values of the field `IABG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IABGR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl IABGR {
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
            IABGR::_0 => false,
            IABGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IABGR {
        match value {
            false => IABGR::_0,
            true => IABGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IABGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IABGR::_1
    }
}
#[doc = "Possible values of the field `WECINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINTR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECINTR {
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
            WECINTR::_0 => false,
            WECINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECINTR {
        match value {
            false => WECINTR::_0,
            true => WECINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WECINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WECINTR::_1
    }
}
#[doc = "Possible values of the field `WECINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINSR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECINSR {
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
            WECINSR::_0 => false,
            WECINSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECINSR {
        match value {
            false => WECINSR::_0,
            true => WECINSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WECINSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WECINSR::_1
    }
}
#[doc = "Possible values of the field `WECRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECRMR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECRMR {
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
            WECRMR::_0 => false,
            WECRMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WECRMR {
        match value {
            false => WECRMR::_0,
            true => WECRMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WECRMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WECRMR::_1
    }
}
#[doc = "Values that can be written to the field `LCTL`"]
pub enum LCTLW {
    #[doc = "LED off."]
    _0,
    #[doc = "LED on."]
    _1,
}
impl LCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCTLW::_0 => false,
            LCTLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _LCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LED off."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCTLW::_0)
    }
    #[doc = "LED on."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCTLW::_1)
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
#[doc = "Values that can be written to the field `DTW`"]
pub enum DTWW {
    #[doc = "1-bit mode"]
    _00,
    #[doc = "4-bit mode"]
    _01,
    #[doc = "8-bit mode"]
    _10,
}
impl DTWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTWW::_00 => 0,
            DTWW::_01 => 1,
            DTWW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTWW<'a> {
    w: &'a mut W,
}
impl<'a> _DTWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1-bit mode"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DTWW::_00)
    }
    #[doc = "4-bit mode"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DTWW::_01)
    }
    #[doc = "8-bit mode"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTWW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `D3CD`"]
pub enum D3CDW {
    #[doc = "DAT3 does not monitor card Insertion."]
    _0,
    #[doc = "DAT3 as card detection pin."]
    _1,
}
impl D3CDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D3CDW::_0 => false,
            D3CDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D3CDW<'a> {
    w: &'a mut W,
}
impl<'a> _D3CDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D3CDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAT3 does not monitor card Insertion."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(D3CDW::_0)
    }
    #[doc = "DAT3 as card detection pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(D3CDW::_1)
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
#[doc = "Values that can be written to the field `EMODE`"]
pub enum EMODEW {
    #[doc = "Big endian mode"]
    _00,
    #[doc = "Half word big endian mode"]
    _01,
    #[doc = "Little endian mode"]
    _10,
}
impl EMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMODEW::_00 => 0,
            EMODEW::_01 => 1,
            EMODEW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Big endian mode"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMODEW::_00)
    }
    #[doc = "Half word big endian mode"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMODEW::_01)
    }
    #[doc = "Little endian mode"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMODEW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDTL`"]
pub enum CDTLW {
    #[doc = "Card detect test level is 0, no card inserted."]
    _0,
    #[doc = "Card detect test level is 1, card inserted."]
    _1,
}
impl CDTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDTLW::_0 => false,
            CDTLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDTLW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card detect test level is 0, no card inserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDTLW::_0)
    }
    #[doc = "Card detect test level is 1, card inserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDTLW::_1)
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
#[doc = "Values that can be written to the field `CDSS`"]
pub enum CDSSW {
    #[doc = "Card detection level is selected for normal purpose."]
    _0,
    #[doc = "Card detection test level is selected for test purpose."]
    _1,
}
impl CDSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDSSW::_0 => false,
            CDSSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDSSW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card detection level is selected for normal purpose."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDSSW::_0)
    }
    #[doc = "Card detection test level is selected for test purpose."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDSSW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAS`"]
pub enum DMASW {
    #[doc = "No DMA or simple DMA is selected."]
    _00,
    #[doc = "ADMA1 is selected."]
    _01,
    #[doc = "ADMA2 is selected."]
    _10,
}
impl DMASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMASW::_00 => 0,
            DMASW::_01 => 1,
            DMASW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMASW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No DMA or simple DMA is selected."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DMASW::_00)
    }
    #[doc = "ADMA1 is selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DMASW::_01)
    }
    #[doc = "ADMA2 is selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DMASW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SABGREQ`"]
pub enum SABGREQW {
    #[doc = "Transfer"]
    _0,
    #[doc = "Stop"]
    _1,
}
impl SABGREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SABGREQW::_0 => false,
            SABGREQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SABGREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SABGREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SABGREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SABGREQW::_0)
    }
    #[doc = "Stop"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SABGREQW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CREQ`"]
pub enum CREQW {
    #[doc = "No effect."]
    _0,
    #[doc = "Restart"]
    _1,
}
impl CREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CREQW::_0 => false,
            CREQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CREQW::_0)
    }
    #[doc = "Restart"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CREQW::_1)
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
#[doc = "Values that can be written to the field `RWCTL`"]
pub enum RWCTLW {
    #[doc = "Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    _0,
    #[doc = "Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    _1,
}
impl RWCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWCTLW::_0 => false,
            RWCTLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWCTLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable read wait control, and stop SD clock at block gap when SABGREQ is set."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWCTLW::_0)
    }
    #[doc = "Enable read wait control, and assert read wait without stopping SD clock at block gap when SABGREQ bit is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWCTLW::_1)
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
#[doc = "Values that can be written to the field `IABG`"]
pub enum IABGW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl IABGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IABGW::_0 => false,
            IABGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IABGW<'a> {
    w: &'a mut W,
}
impl<'a> _IABGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IABGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IABGW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IABGW::_1)
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
#[doc = "Values that can be written to the field `WECINT`"]
pub enum WECINTW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECINTW::_0 => false,
            WECINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WECINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINTW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINTW::_1)
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
#[doc = "Values that can be written to the field `WECINS`"]
pub enum WECINSW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECINSW::_0 => false,
            WECINSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECINSW<'a> {
    w: &'a mut W,
}
impl<'a> _WECINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECINSW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECINSW::_1)
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
#[doc = "Values that can be written to the field `WECRM`"]
pub enum WECRMW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl WECRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WECRMW::_0 => false,
            WECRMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WECRMW<'a> {
    w: &'a mut W,
}
impl<'a> _WECRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WECRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WECRMW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WECRMW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn lctl(&self) -> LCTLR {
        LCTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline]
    pub fn dtw(&self) -> DTWR {
        DTWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline]
    pub fn d3cd(&self) -> D3CDR {
        D3CDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline]
    pub fn emode(&self) -> EMODER {
        EMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtl(&self) -> CDTLR {
        CDTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline]
    pub fn cdss(&self) -> CDSSR {
        CDSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline]
    pub fn dmas(&self) -> DMASR {
        DMASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline]
    pub fn sabgreq(&self) -> SABGREQR {
        SABGREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn creq(&self) -> CREQR {
        CREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rwctl(&self) -> RWCTLR {
        RWCTLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline]
    pub fn iabg(&self) -> IABGR {
        IABGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wecint(&self) -> WECINTR {
        WECINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wecins(&self) -> WECINSR {
        WECINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wecrm(&self) -> WECRMR {
        WECRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn lctl(&mut self) -> _LCTLW {
        _LCTLW { w: self }
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline]
    pub fn dtw(&mut self) -> _DTWW {
        _DTWW { w: self }
    }
    #[doc = "Bit 3 - DAT3 As Card Detection Pin"]
    #[inline]
    pub fn d3cd(&mut self) -> _D3CDW {
        _D3CDW { w: self }
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline]
    pub fn emode(&mut self) -> _EMODEW {
        _EMODEW { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtl(&mut self) -> _CDTLW {
        _CDTLW { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline]
    pub fn cdss(&mut self) -> _CDSSW {
        _CDSSW { w: self }
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline]
    pub fn dmas(&mut self) -> _DMASW {
        _DMASW { w: self }
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline]
    pub fn sabgreq(&mut self) -> _SABGREQW {
        _SABGREQW { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn creq(&mut self) -> _CREQW {
        _CREQW { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rwctl(&mut self) -> _RWCTLW {
        _RWCTLW { w: self }
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline]
    pub fn iabg(&mut self) -> _IABGW {
        _IABGW { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline]
    pub fn wecint(&mut self) -> _WECINTW {
        _WECINTW { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline]
    pub fn wecins(&mut self) -> _WECINSW {
        _WECINSW { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline]
    pub fn wecrm(&mut self) -> _WECRMW {
        _WECRMW { w: self }
    }
}
