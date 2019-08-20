#[doc = "Reader of register ET7816"]
pub type R = crate::R<u8, super::ET7816>;
#[doc = "Writer for register ET7816"]
pub type W = crate::W<u8, super::ET7816>;
#[doc = "Register ET7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::ET7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXTHRESHOLD`"]
pub type RXTHRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTHRESHOLD`"]
pub struct RXTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Transmit NACK Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTHRESHOLD_A {
    #[doc = "0: TXT asserts on the first NACK that is received."]
    _0,
    #[doc = "1: TXT asserts on the second NACK that is received."]
    _1,
}
impl From<TXTHRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: TXTHRESHOLD_A) -> Self {
        match variant {
            TXTHRESHOLD_A::_0 => 0,
            TXTHRESHOLD_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `TXTHRESHOLD`"]
pub type TXTHRESHOLD_R = crate::R<u8, TXTHRESHOLD_A>;
impl TXTHRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXTHRESHOLD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXTHRESHOLD_A::_0),
            1 => Val(TXTHRESHOLD_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXTHRESHOLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXTHRESHOLD_A::_1
    }
}
#[doc = "Write proxy for field `TXTHRESHOLD`"]
pub struct TXTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTHRESHOLD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TXT asserts on the first NACK that is received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTHRESHOLD_A::_0)
    }
    #[doc = "TXT asserts on the second NACK that is received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTHRESHOLD_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    pub fn rxthreshold(&self) -> RXTHRESHOLD_R {
        RXTHRESHOLD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    pub fn txthreshold(&self) -> TXTHRESHOLD_R {
        TXTHRESHOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    pub fn rxthreshold(&mut self) -> RXTHRESHOLD_W {
        RXTHRESHOLD_W { w: self }
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    pub fn txthreshold(&mut self) -> TXTHRESHOLD_W {
        TXTHRESHOLD_W { w: self }
    }
}
