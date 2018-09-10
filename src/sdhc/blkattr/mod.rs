#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLKATTR {
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
#[doc = "Possible values of the field `BLKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSIZER {
    #[doc = "No data transfer."]
    _0,
    #[doc = "1 Byte"]
    _1,
    #[doc = "2 Bytes"]
    _10,
    #[doc = "3 Bytes"]
    _11,
    #[doc = "4 Bytes"]
    _100,
    #[doc = "511 Bytes"]
    _111111111,
    #[doc = "512 Bytes"]
    _1000000000,
    #[doc = "2048 Bytes"]
    _100000000000,
    #[doc = "4096 Bytes"]
    _1000000000000,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKSIZER::_0 => 0,
            BLKSIZER::_1 => 1,
            BLKSIZER::_10 => 2,
            BLKSIZER::_11 => 3,
            BLKSIZER::_100 => 4,
            BLKSIZER::_111111111 => 511,
            BLKSIZER::_1000000000 => 512,
            BLKSIZER::_100000000000 => 2048,
            BLKSIZER::_1000000000000 => 4096,
            BLKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKSIZER {
        match value {
            0 => BLKSIZER::_0,
            1 => BLKSIZER::_1,
            2 => BLKSIZER::_10,
            3 => BLKSIZER::_11,
            4 => BLKSIZER::_100,
            511 => BLKSIZER::_111111111,
            512 => BLKSIZER::_1000000000,
            2048 => BLKSIZER::_100000000000,
            4096 => BLKSIZER::_1000000000000,
            i => BLKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BLKSIZER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BLKSIZER::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BLKSIZER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == BLKSIZER::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == BLKSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_111111111`"]
    #[inline]
    pub fn is_111111111(&self) -> bool {
        *self == BLKSIZER::_111111111
    }
    #[doc = "Checks if the value of the field is `_1000000000`"]
    #[inline]
    pub fn is_1000000000(&self) -> bool {
        *self == BLKSIZER::_1000000000
    }
    #[doc = "Checks if the value of the field is `_100000000000`"]
    #[inline]
    pub fn is_100000000000(&self) -> bool {
        *self == BLKSIZER::_100000000000
    }
    #[doc = "Checks if the value of the field is `_1000000000000`"]
    #[inline]
    pub fn is_1000000000000(&self) -> bool {
        *self == BLKSIZER::_1000000000000
    }
}
#[doc = "Possible values of the field `BLKCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKCNTR {
    #[doc = "Stop count."]
    _0,
    #[doc = "1 block"]
    _1,
    #[doc = "2 blocks"]
    _10,
    #[doc = "65535 blocks"]
    _1111111111111111,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKCNTR::_0 => 0,
            BLKCNTR::_1 => 1,
            BLKCNTR::_10 => 2,
            BLKCNTR::_1111111111111111 => 65535,
            BLKCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKCNTR {
        match value {
            0 => BLKCNTR::_0,
            1 => BLKCNTR::_1,
            2 => BLKCNTR::_10,
            65535 => BLKCNTR::_1111111111111111,
            i => BLKCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BLKCNTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BLKCNTR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BLKCNTR::_10
    }
    #[doc = "Checks if the value of the field is `_1111111111111111`"]
    #[inline]
    pub fn is_1111111111111111(&self) -> bool {
        *self == BLKCNTR::_1111111111111111
    }
}
#[doc = "Values that can be written to the field `BLKSIZE`"]
pub enum BLKSIZEW {
    #[doc = "No data transfer."]
    _0,
    #[doc = "1 Byte"]
    _1,
    #[doc = "2 Bytes"]
    _10,
    #[doc = "3 Bytes"]
    _11,
    #[doc = "4 Bytes"]
    _100,
    #[doc = "511 Bytes"]
    _111111111,
    #[doc = "512 Bytes"]
    _1000000000,
    #[doc = "2048 Bytes"]
    _100000000000,
    #[doc = "4096 Bytes"]
    _1000000000000,
}
impl BLKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKSIZEW::_0 => 0,
            BLKSIZEW::_1 => 1,
            BLKSIZEW::_10 => 2,
            BLKSIZEW::_11 => 3,
            BLKSIZEW::_100 => 4,
            BLKSIZEW::_111111111 => 511,
            BLKSIZEW::_1000000000 => 512,
            BLKSIZEW::_100000000000 => 2048,
            BLKSIZEW::_1000000000000 => 4096,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKSIZEW::_0)
    }
    #[doc = "1 Byte"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKSIZEW::_1)
    }
    #[doc = "2 Bytes"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKSIZEW::_10)
    }
    #[doc = "3 Bytes"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(BLKSIZEW::_11)
    }
    #[doc = "4 Bytes"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(BLKSIZEW::_100)
    }
    #[doc = "511 Bytes"]
    #[inline]
    pub fn _111111111(self) -> &'a mut W {
        self.variant(BLKSIZEW::_111111111)
    }
    #[doc = "512 Bytes"]
    #[inline]
    pub fn _1000000000(self) -> &'a mut W {
        self.variant(BLKSIZEW::_1000000000)
    }
    #[doc = "2048 Bytes"]
    #[inline]
    pub fn _100000000000(self) -> &'a mut W {
        self.variant(BLKSIZEW::_100000000000)
    }
    #[doc = "4096 Bytes"]
    #[inline]
    pub fn _1000000000000(self) -> &'a mut W {
        self.variant(BLKSIZEW::_1000000000000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLKCNT`"]
pub enum BLKCNTW {
    #[doc = "Stop count."]
    _0,
    #[doc = "1 block"]
    _1,
    #[doc = "2 blocks"]
    _10,
    #[doc = "65535 blocks"]
    _1111111111111111,
}
impl BLKCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKCNTW::_0 => 0,
            BLKCNTW::_1 => 1,
            BLKCNTW::_10 => 2,
            BLKCNTW::_1111111111111111 => 65535,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stop count."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKCNTW::_0)
    }
    #[doc = "1 block"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKCNTW::_1)
    }
    #[doc = "2 blocks"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKCNTW::_10)
    }
    #[doc = "65535 blocks"]
    #[inline]
    pub fn _1111111111111111(self) -> &'a mut W {
        self.variant(BLKCNTW::_1111111111111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline]
    pub fn blksize(&self) -> BLKSIZER {
        BLKSIZER::_from({
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline]
    pub fn blkcnt(&self) -> BLKCNTR {
        BLKCNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline]
    pub fn blksize(&mut self) -> _BLKSIZEW {
        _BLKSIZEW { w: self }
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline]
    pub fn blkcnt(&mut self) -> _BLKCNTW {
        _BLKCNTW { w: self }
    }
}
