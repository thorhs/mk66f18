#[doc = "Reader of register TFWR"]
pub type R = crate::R<u32, super::TFWR>;
#[doc = "Writer for register TFWR"]
pub type W = crate::W<u32, super::TFWR>;
#[doc = "Register TFWR `reset()`'s with value 0"]
impl crate::ResetValue for super::TFWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit FIFO Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFWR_A {
    #[doc = "0: 64 bytes written."]
    _000000,
    #[doc = "1: 64 bytes written."]
    _000001,
    #[doc = "2: 128 bytes written."]
    _000010,
    #[doc = "3: 192 bytes written."]
    _000011,
    #[doc = "31: 1984 bytes written."]
    _011111,
}
impl From<TFWR_A> for u8 {
    #[inline(always)]
    fn from(variant: TFWR_A) -> Self {
        match variant {
            TFWR_A::_000000 => 0,
            TFWR_A::_000001 => 1,
            TFWR_A::_000010 => 2,
            TFWR_A::_000011 => 3,
            TFWR_A::_011111 => 31,
        }
    }
}
#[doc = "Reader of field `TFWR`"]
pub type TFWR_R = crate::R<u8, TFWR_A>;
impl TFWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TFWR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TFWR_A::_000000),
            1 => Val(TFWR_A::_000001),
            2 => Val(TFWR_A::_000010),
            3 => Val(TFWR_A::_000011),
            31 => Val(TFWR_A::_011111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == TFWR_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == TFWR_A::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == TFWR_A::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        *self == TFWR_A::_000011
    }
    #[doc = "Checks if the value of the field is `_011111`"]
    #[inline(always)]
    pub fn is_011111(&self) -> bool {
        *self == TFWR_A::_011111
    }
}
#[doc = "Write proxy for field `TFWR`"]
pub struct TFWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFWR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TFWR_A::_000000)
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(TFWR_A::_000001)
    }
    #[doc = "128 bytes written."]
    #[inline(always)]
    pub fn _000010(self) -> &'a mut W {
        self.variant(TFWR_A::_000010)
    }
    #[doc = "192 bytes written."]
    #[inline(always)]
    pub fn _000011(self) -> &'a mut W {
        self.variant(TFWR_A::_000011)
    }
    #[doc = "1984 bytes written."]
    #[inline(always)]
    pub fn _011111(self) -> &'a mut W {
        self.variant(TFWR_A::_011111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Store And Forward Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRFWD_A {
    #[doc = "0: Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    _0,
    #[doc = "1: Enabled."]
    _1,
}
impl From<STRFWD_A> for bool {
    #[inline(always)]
    fn from(variant: STRFWD_A) -> Self {
        match variant {
            STRFWD_A::_0 => false,
            STRFWD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STRFWD`"]
pub type STRFWD_R = crate::R<bool, STRFWD_A>;
impl STRFWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRFWD_A {
        match self.bits {
            false => STRFWD_A::_0,
            true => STRFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STRFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STRFWD_A::_1
    }
}
#[doc = "Write proxy for field `STRFWD`"]
pub struct STRFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> STRFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRFWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STRFWD_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STRFWD_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&self) -> TFWR_R {
        TFWR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&self) -> STRFWD_R {
        STRFWD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&mut self) -> TFWR_W {
        TFWR_W { w: self }
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&mut self) -> STRFWD_W {
        STRFWD_W { w: self }
    }
}
