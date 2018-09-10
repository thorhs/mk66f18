#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PFB23CR {
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
#[doc = "Possible values of the field `B1IPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1IPER {
    #[doc = "Do not prefetch in response to instruction fetches."]
    _0,
    #[doc = "Enable prefetches in response to instruction fetches."]
    _1,
}
impl B1IPER {
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
            B1IPER::_0 => false,
            B1IPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B1IPER {
        match value {
            false => B1IPER::_0,
            true => B1IPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B1IPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B1IPER::_1
    }
}
#[doc = "Possible values of the field `B1DPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1DPER {
    #[doc = "Do not prefetch in response to data references."]
    _0,
    #[doc = "Enable prefetches in response to data references."]
    _1,
}
impl B1DPER {
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
            B1DPER::_0 => false,
            B1DPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B1DPER {
        match value {
            false => B1DPER::_0,
            true => B1DPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B1DPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B1DPER::_1
    }
}
#[doc = "Possible values of the field `B1ICE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1ICER {
    #[doc = "Do not cache instruction fetches."]
    _0,
    #[doc = "Cache instruction fetches."]
    _1,
}
impl B1ICER {
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
            B1ICER::_0 => false,
            B1ICER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B1ICER {
        match value {
            false => B1ICER::_0,
            true => B1ICER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B1ICER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B1ICER::_1
    }
}
#[doc = "Possible values of the field `B1DCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1DCER {
    #[doc = "Do not cache data references."]
    _0,
    #[doc = "Cache data references."]
    _1,
}
impl B1DCER {
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
            B1DCER::_0 => false,
            B1DCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B1DCER {
        match value {
            false => B1DCER::_0,
            true => B1DCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == B1DCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == B1DCER::_1
    }
}
#[doc = "Possible values of the field `B1MW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1MWR {
    #[doc = "32 bits"]
    _00,
    #[doc = "64 bits"]
    _01,
    #[doc = "128 bits"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl B1MWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            B1MWR::_00 => 0,
            B1MWR::_01 => 1,
            B1MWR::_10 => 2,
            B1MWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> B1MWR {
        match value {
            0 => B1MWR::_00,
            1 => B1MWR::_01,
            2 => B1MWR::_10,
            i => B1MWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == B1MWR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == B1MWR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == B1MWR::_10
    }
}
#[doc = r" Value of the field"]
pub struct B1RWSCR {
    bits: u8,
}
impl B1RWSCR {
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
#[doc = "Values that can be written to the field `B1IPE`"]
pub enum B1IPEW {
    #[doc = "Do not prefetch in response to instruction fetches."]
    _0,
    #[doc = "Enable prefetches in response to instruction fetches."]
    _1,
}
impl B1IPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B1IPEW::_0 => false,
            B1IPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B1IPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B1IPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B1IPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1IPEW::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1IPEW::_1)
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
#[doc = "Values that can be written to the field `B1DPE`"]
pub enum B1DPEW {
    #[doc = "Do not prefetch in response to data references."]
    _0,
    #[doc = "Enable prefetches in response to data references."]
    _1,
}
impl B1DPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B1DPEW::_0 => false,
            B1DPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B1DPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B1DPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B1DPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DPEW::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DPEW::_1)
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
#[doc = "Values that can be written to the field `B1ICE`"]
pub enum B1ICEW {
    #[doc = "Do not cache instruction fetches."]
    _0,
    #[doc = "Cache instruction fetches."]
    _1,
}
impl B1ICEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B1ICEW::_0 => false,
            B1ICEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B1ICEW<'a> {
    w: &'a mut W,
}
impl<'a> _B1ICEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B1ICEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1ICEW::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1ICEW::_1)
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
#[doc = "Values that can be written to the field `B1DCE`"]
pub enum B1DCEW {
    #[doc = "Do not cache data references."]
    _0,
    #[doc = "Cache data references."]
    _1,
}
impl B1DCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B1DCEW::_0 => false,
            B1DCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B1DCEW<'a> {
    w: &'a mut W,
}
impl<'a> _B1DCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B1DCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not cache data references."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DCEW::_0)
    }
    #[doc = "Cache data references."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DCEW::_1)
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
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline]
    pub fn b1ipe(&self) -> B1IPER {
        B1IPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline]
    pub fn b1dpe(&self) -> B1DPER {
        B1DPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline]
    pub fn b1ice(&self) -> B1ICER {
        B1ICER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline]
    pub fn b1dce(&self) -> B1DCER {
        B1DCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - Bank 1 Memory Width"]
    #[inline]
    pub fn b1mw(&self) -> B1MWR {
        B1MWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Bank 1 Read Wait State Control"]
    #[inline]
    pub fn b1rwsc(&self) -> B1RWSCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B1RWSCR { bits }
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
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline]
    pub fn b1ipe(&mut self) -> _B1IPEW {
        _B1IPEW { w: self }
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline]
    pub fn b1dpe(&mut self) -> _B1DPEW {
        _B1DPEW { w: self }
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline]
    pub fn b1ice(&mut self) -> _B1ICEW {
        _B1ICEW { w: self }
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline]
    pub fn b1dce(&mut self) -> _B1DCEW {
        _B1DCEW { w: self }
    }
}
