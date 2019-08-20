#[doc = "Reader of register BLKATTR"]
pub type R = crate::R<u32, super::BLKATTR>;
#[doc = "Writer for register BLKATTR"]
pub type W = crate::W<u32, super::BLKATTR>;
#[doc = "Register BLKATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BLKATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transfer Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSIZE_A {
    #[doc = "0: No data transfer."]
    _0,
    #[doc = "1: 1 Byte"]
    _1,
    #[doc = "2: 2 Bytes"]
    _10,
    #[doc = "3: 3 Bytes"]
    _11,
    #[doc = "4: 4 Bytes"]
    _100,
    #[doc = "511: 511 Bytes"]
    _111111111,
    #[doc = "512: 512 Bytes"]
    _1000000000,
    #[doc = "2048: 2048 Bytes"]
    _100000000000,
    #[doc = "4096: 4096 Bytes"]
    _1000000000000,
}
impl From<BLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSIZE_A) -> Self {
        match variant {
            BLKSIZE_A::_0 => 0,
            BLKSIZE_A::_1 => 1,
            BLKSIZE_A::_10 => 2,
            BLKSIZE_A::_11 => 3,
            BLKSIZE_A::_100 => 4,
            BLKSIZE_A::_111111111 => 511,
            BLKSIZE_A::_1000000000 => 512,
            BLKSIZE_A::_100000000000 => 2048,
            BLKSIZE_A::_1000000000000 => 4096,
        }
    }
}
#[doc = "Reader of field `BLKSIZE`"]
pub type BLKSIZE_R = crate::R<u16, BLKSIZE_A>;
impl BLKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BLKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLKSIZE_A::_0),
            1 => Val(BLKSIZE_A::_1),
            2 => Val(BLKSIZE_A::_10),
            3 => Val(BLKSIZE_A::_11),
            4 => Val(BLKSIZE_A::_100),
            511 => Val(BLKSIZE_A::_111111111),
            512 => Val(BLKSIZE_A::_1000000000),
            2048 => Val(BLKSIZE_A::_100000000000),
            4096 => Val(BLKSIZE_A::_1000000000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLKSIZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLKSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BLKSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BLKSIZE_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BLKSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_111111111`"]
    #[inline(always)]
    pub fn is_111111111(&self) -> bool {
        *self == BLKSIZE_A::_111111111
    }
    #[doc = "Checks if the value of the field is `_1000000000`"]
    #[inline(always)]
    pub fn is_1000000000(&self) -> bool {
        *self == BLKSIZE_A::_1000000000
    }
    #[doc = "Checks if the value of the field is `_100000000000`"]
    #[inline(always)]
    pub fn is_100000000000(&self) -> bool {
        *self == BLKSIZE_A::_100000000000
    }
    #[doc = "Checks if the value of the field is `_1000000000000`"]
    #[inline(always)]
    pub fn is_1000000000000(&self) -> bool {
        *self == BLKSIZE_A::_1000000000000
    }
}
#[doc = "Write proxy for field `BLKSIZE`"]
pub struct BLKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No data transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_0)
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1)
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_10)
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_11)
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_100)
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn _111111111(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_111111111)
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn _1000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1000000000)
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn _100000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_100000000000)
    }
    #[doc = "4096 Bytes"]
    #[inline(always)]
    pub fn _1000000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1000000000000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Blocks Count For Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKCNT_A {
    #[doc = "0: Stop count."]
    _0,
    #[doc = "1: 1 block"]
    _1,
    #[doc = "2: 2 blocks"]
    _10,
    #[doc = "65535: 65535 blocks"]
    _1111111111111111,
}
impl From<BLKCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKCNT_A) -> Self {
        match variant {
            BLKCNT_A::_0 => 0,
            BLKCNT_A::_1 => 1,
            BLKCNT_A::_10 => 2,
            BLKCNT_A::_1111111111111111 => 65535,
        }
    }
}
#[doc = "Reader of field `BLKCNT`"]
pub type BLKCNT_R = crate::R<u16, BLKCNT_A>;
impl BLKCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BLKCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLKCNT_A::_0),
            1 => Val(BLKCNT_A::_1),
            2 => Val(BLKCNT_A::_10),
            65535 => Val(BLKCNT_A::_1111111111111111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLKCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLKCNT_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BLKCNT_A::_10
    }
    #[doc = "Checks if the value of the field is `_1111111111111111`"]
    #[inline(always)]
    pub fn is_1111111111111111(&self) -> bool {
        *self == BLKCNT_A::_1111111111111111
    }
}
#[doc = "Write proxy for field `BLKCNT`"]
pub struct BLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stop count."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKCNT_A::_0)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKCNT_A::_1)
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKCNT_A::_10)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn _1111111111111111(self) -> &'a mut W {
        self.variant(BLKCNT_A::_1111111111111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BLKSIZE_W {
        BLKSIZE_W { w: self }
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline(always)]
    pub fn blkcnt(&mut self) -> BLKCNT_W {
        BLKCNT_W { w: self }
    }
}
