#[doc = "Reader of register TOKEN"]
pub type R = crate::R<u8, super::TOKEN>;
#[doc = "Writer for register TOKEN"]
pub type W = crate::W<u8, super::TOKEN>;
#[doc = "Register TOKEN `reset()`'s with value 0"]
impl crate::ResetValue for super::TOKEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOKENENDPT`"]
pub type TOKENENDPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOKENENDPT`"]
pub struct TOKENENDPT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKENENDPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Contains the token type executed by the USB module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOKENPID_A {
    #[doc = "1: OUT Token. USB Module performs an OUT (TX) transaction."]
    _0001,
    #[doc = "9: IN Token. USB Module performs an In (RX) transaction."]
    _1001,
    #[doc = "13: SETUP Token. USB Module performs a SETUP (TX) transaction"]
    _1101,
}
impl From<TOKENPID_A> for u8 {
    #[inline(always)]
    fn from(variant: TOKENPID_A) -> Self {
        match variant {
            TOKENPID_A::_0001 => 1,
            TOKENPID_A::_1001 => 9,
            TOKENPID_A::_1101 => 13,
        }
    }
}
#[doc = "Reader of field `TOKENPID`"]
pub type TOKENPID_R = crate::R<u8, TOKENPID_A>;
impl TOKENPID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TOKENPID_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TOKENPID_A::_0001),
            9 => Val(TOKENPID_A::_1001),
            13 => Val(TOKENPID_A::_1101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TOKENPID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TOKENPID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TOKENPID_A::_1101
    }
}
#[doc = "Write proxy for field `TOKENPID`"]
pub struct TOKENPID_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKENPID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOKENPID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "OUT Token. USB Module performs an OUT (TX) transaction."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TOKENPID_A::_0001)
    }
    #[doc = "IN Token. USB Module performs an In (RX) transaction."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TOKENPID_A::_1001)
    }
    #[doc = "SETUP Token. USB Module performs a SETUP (TX) transaction"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TOKENPID_A::_1101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline(always)]
    pub fn tokenendpt(&self) -> TOKENENDPT_R {
        TOKENENDPT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline(always)]
    pub fn tokenpid(&self) -> TOKENPID_R {
        TOKENPID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline(always)]
    pub fn tokenendpt(&mut self) -> TOKENENDPT_W {
        TOKENENDPT_W { w: self }
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline(always)]
    pub fn tokenpid(&mut self) -> TOKENPID_W {
        TOKENPID_W { w: self }
    }
}
