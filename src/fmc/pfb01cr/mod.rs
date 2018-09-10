#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PFB01CR {
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
pub struct RFUR {
    bits: bool,
}
impl RFUR {
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
#[doc = "Possible values of the field `B0IPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0IPER {
    #[doc = "Do not prefetch in response to instruction fetches."]
    _0,
    #[doc = "Enable prefetches in response to instruction fetches."]
    _1,
}
impl B0IPER {
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
            B0IPER::_0 => false,
            B0IPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B0IPER {
        match value {
            false => B0IPER::_0,
            true => B0IPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B0IPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B0IPER::_1
    }
}
#[doc = "Possible values of the field `B0DPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0DPER {
    #[doc = "Do not prefetch in response to data references."]
    _0,
    #[doc = "Enable prefetches in response to data references."]
    _1,
}
impl B0DPER {
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
            B0DPER::_0 => false,
            B0DPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B0DPER {
        match value {
            false => B0DPER::_0,
            true => B0DPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B0DPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B0DPER::_1
    }
}
#[doc = "Possible values of the field `B0ICE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0ICER {
    #[doc = "Do not cache instruction fetches."]
    _0,
    #[doc = "Cache instruction fetches."]
    _1,
}
impl B0ICER {
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
            B0ICER::_0 => false,
            B0ICER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B0ICER {
        match value {
            false => B0ICER::_0,
            true => B0ICER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B0ICER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B0ICER::_1
    }
}
#[doc = "Possible values of the field `B0DCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0DCER {
    #[doc = "Do not cache data references."]
    _0,
    #[doc = "Cache data references."]
    _1,
}
impl B0DCER {
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
            B0DCER::_0 => false,
            B0DCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B0DCER {
        match value {
            false => B0DCER::_0,
            true => B0DCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B0DCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B0DCER::_1
    }
}
#[doc = "Possible values of the field `CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCR {
    #[doc = "LRU replacement algorithm per set across all four ways"]
    _000,
    #[doc = "Independent LRU with ways \\[0-1\\] for ifetches, \\[2-3\\] for data"]
    _010,
    #[doc = "Independent LRU with ways \\[0-2\\] for ifetches, \\[3\\] for data"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRCR::_000 => 0,
            CRCR::_010 => 2,
            CRCR::_011 => 3,
            CRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRCR {
        match value {
            0 => CRCR::_000,
            2 => CRCR::_010,
            3 => CRCR::_011,
            i => CRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CRCR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CRCR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CRCR::_011
    }
}
#[doc = "Possible values of the field `B0MW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B0MWR {
    #[doc = "32 bits"]
    _00,
    #[doc = "64 bits"]
    _01,
    #[doc = "128 bits"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl B0MWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            B0MWR::_00 => 0,
            B0MWR::_01 => 1,
            B0MWR::_10 => 2,
            B0MWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> B0MWR {
        match value {
            0 => B0MWR::_00,
            1 => B0MWR::_01,
            2 => B0MWR::_10,
            i => B0MWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == B0MWR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == B0MWR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == B0MWR::_10
    }
}
#[doc = "Possible values of the field `CLCK_WAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLCK_WAYR {
    #[doc = "Cache way is unlocked and may be displaced"]
    _0,
    #[doc = "Cache way is locked and its contents are not displaced"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLCK_WAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLCK_WAYR::_0 => 0,
            CLCK_WAYR::_1 => 1,
            CLCK_WAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLCK_WAYR {
        match value {
            0 => CLCK_WAYR::_0,
            1 => CLCK_WAYR::_1,
            i => CLCK_WAYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLCK_WAYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLCK_WAYR::_1
    }
}
#[doc = r" Value of the field"]
pub struct B0RWSCR {
    bits: u8,
}
impl B0RWSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RFUW<'a> {
    w: &'a mut W,
}
impl<'a> _RFUW<'a> {
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
#[doc = "Values that can be written to the field `B0IPE`"]
pub enum B0IPEW {
    #[doc = "Do not prefetch in response to instruction fetches."]
    _0,
    #[doc = "Enable prefetches in response to instruction fetches."]
    _1,
}
impl B0IPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B0IPEW::_0 => false,
            B0IPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B0IPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B0IPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B0IPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0IPEW::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0IPEW::_1)
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
#[doc = "Values that can be written to the field `B0DPE`"]
pub enum B0DPEW {
    #[doc = "Do not prefetch in response to data references."]
    _0,
    #[doc = "Enable prefetches in response to data references."]
    _1,
}
impl B0DPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B0DPEW::_0 => false,
            B0DPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B0DPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B0DPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B0DPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DPEW::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DPEW::_1)
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
#[doc = "Values that can be written to the field `B0ICE`"]
pub enum B0ICEW {
    #[doc = "Do not cache instruction fetches."]
    _0,
    #[doc = "Cache instruction fetches."]
    _1,
}
impl B0ICEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B0ICEW::_0 => false,
            B0ICEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B0ICEW<'a> {
    w: &'a mut W,
}
impl<'a> _B0ICEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B0ICEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0ICEW::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0ICEW::_1)
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
#[doc = "Values that can be written to the field `B0DCE`"]
pub enum B0DCEW {
    #[doc = "Do not cache data references."]
    _0,
    #[doc = "Cache data references."]
    _1,
}
impl B0DCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B0DCEW::_0 => false,
            B0DCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B0DCEW<'a> {
    w: &'a mut W,
}
impl<'a> _B0DCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B0DCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not cache data references."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DCEW::_0)
    }
    #[doc = "Cache data references."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DCEW::_1)
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
#[doc = "Values that can be written to the field `CRC`"]
pub enum CRCW {
    #[doc = "LRU replacement algorithm per set across all four ways"]
    _000,
    #[doc = "Independent LRU with ways \\[0-1\\] for ifetches, \\[2-3\\] for data"]
    _010,
    #[doc = "Independent LRU with ways \\[0-2\\] for ifetches, \\[3\\] for data"]
    _011,
}
impl CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRCW::_000 => 0,
            CRCW::_010 => 2,
            CRCW::_011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LRU replacement algorithm per set across all four ways"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CRCW::_000)
    }
    #[doc = "Independent LRU with ways \\[0-1\\] for ifetches, \\[2-3\\] for data"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CRCW::_010)
    }
    #[doc = "Independent LRU with ways \\[0-2\\] for ifetches, \\[3\\] for data"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CRCW::_011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S_B_INV`"]
pub enum S_B_INVW {
    #[doc = "Speculation buffer is not affected"]
    _0,
    #[doc = "Invalidate (clear) speculation buffer"]
    _1,
}
impl S_B_INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S_B_INVW::_0 => false,
            S_B_INVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S_B_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _S_B_INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S_B_INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Speculation buffer is not affected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(S_B_INVW::_0)
    }
    #[doc = "Invalidate (clear) speculation buffer"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(S_B_INVW::_1)
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
#[doc = "Values that can be written to the field `CINV_WAY`"]
pub enum CINV_WAYW {
    #[doc = "No cache way invalidation for the corresponding cache"]
    _0,
    #[doc = "Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    _1,
}
impl CINV_WAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CINV_WAYW::_0 => 0,
            CINV_WAYW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINV_WAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CINV_WAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINV_WAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No cache way invalidation for the corresponding cache"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINV_WAYW::_0)
    }
    #[doc = "Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINV_WAYW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLCK_WAY`"]
pub enum CLCK_WAYW {
    #[doc = "Cache way is unlocked and may be displaced"]
    _0,
    #[doc = "Cache way is locked and its contents are not displaced"]
    _1,
}
impl CLCK_WAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLCK_WAYW::_0 => 0,
            CLCK_WAYW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLCK_WAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLCK_WAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLCK_WAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Cache way is unlocked and may be displaced"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLCK_WAYW::_0)
    }
    #[doc = "Cache way is locked and its contents are not displaced"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLCK_WAYW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Reserved for future use"]
    #[inline]
    pub fn rfu(&self) -> RFUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFUR { bits }
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline]
    pub fn b0ipe(&self) -> B0IPER {
        B0IPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline]
    pub fn b0dpe(&self) -> B0DPER {
        B0DPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline]
    pub fn b0ice(&self) -> B0ICER {
        B0ICER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline]
    pub fn b0dce(&self) -> B0DCER {
        B0DCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline]
    pub fn crc(&self) -> CRCR {
        CRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - Bank 0 Memory Width"]
    #[inline]
    pub fn b0mw(&self) -> B0MWR {
        B0MWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline]
    pub fn clck_way(&self) -> CLCK_WAYR {
        CLCK_WAYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Bank 0 Read Wait State Control"]
    #[inline]
    pub fn b0rwsc(&self) -> B0RWSCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B0RWSCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 805568543 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Reserved for future use"]
    #[inline]
    pub fn rfu(&mut self) -> _RFUW {
        _RFUW { w: self }
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline]
    pub fn b0ipe(&mut self) -> _B0IPEW {
        _B0IPEW { w: self }
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline]
    pub fn b0dpe(&mut self) -> _B0DPEW {
        _B0DPEW { w: self }
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline]
    pub fn b0ice(&mut self) -> _B0ICEW {
        _B0ICEW { w: self }
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline]
    pub fn b0dce(&mut self) -> _B0DCEW {
        _B0DCEW { w: self }
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bit 19 - Invalidate Prefetch Speculation Buffer"]
    #[inline]
    pub fn s_b_inv(&mut self) -> _S_B_INVW {
        _S_B_INVW { w: self }
    }
    #[doc = "Bits 20:23 - Cache Invalidate Way x"]
    #[inline]
    pub fn cinv_way(&mut self) -> _CINV_WAYW {
        _CINV_WAYW { w: self }
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline]
    pub fn clck_way(&mut self) -> _CLCK_WAYW {
        _CLCK_WAYW { w: self }
    }
}
