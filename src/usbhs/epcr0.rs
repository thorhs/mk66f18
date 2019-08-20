#[doc = "Reader of register EPCR0"]
pub type R = crate::R<u32, super::EPCR0>;
#[doc = "Writer for register EPCR0"]
pub type W = crate::W<u32, super::EPCR0>;
#[doc = "Register EPCR0 `reset()`'s with value 0x0080_0080"]
impl crate::ResetValue for super::EPCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0080
    }
}
#[doc = "RX endpoint Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXS_A {
    #[doc = "0: Endpoint OK"]
    _0,
    #[doc = "1: Endpoint stalled"]
    _1,
}
impl From<RXS_A> for bool {
    #[inline(always)]
    fn from(variant: RXS_A) -> Self {
        match variant {
            RXS_A::_0 => false,
            RXS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXS`"]
pub type RXS_R = crate::R<bool, RXS_A>;
impl RXS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXS_A {
        match self.bits {
            false => RXS_A::_0,
            true => RXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXS_A::_1
    }
}
#[doc = "Write proxy for field `RXS`"]
pub struct RXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXS_A::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXS_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "RX endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXT_A {
    #[doc = "0: Control"]
    _00,
}
impl From<RXT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXT_A) -> Self {
        match variant {
            RXT_A::_00 => 0,
        }
    }
}
#[doc = "Reader of field `RXT`"]
pub type RXT_R = crate::R<u8, RXT_A>;
impl RXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXT_A::_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RXT_A::_00
    }
}
#[doc = "RX endpoint Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXE_A {
    #[doc = "1: Enabled"]
    _1,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        match variant {
            RXE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, RXE_A>;
impl RXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RXE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RXE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXE_A::_1
    }
}
#[doc = "TX Endpoint Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXS_A {
    #[doc = "0: Endpoint OK"]
    _0,
    #[doc = "1: Endpoint stalled"]
    _1,
}
impl From<TXS_A> for bool {
    #[inline(always)]
    fn from(variant: TXS_A) -> Self {
        match variant {
            TXS_A::_0 => false,
            TXS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXS`"]
pub type TXS_R = crate::R<bool, TXS_A>;
impl TXS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXS_A {
        match self.bits {
            false => TXS_A::_0,
            true => TXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXS_A::_1
    }
}
#[doc = "Write proxy for field `TXS`"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXS_A::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "TX Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXT_A {
    #[doc = "0: Control"]
    _00,
}
impl From<TXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXT_A) -> Self {
        match variant {
            TXT_A::_00 => 0,
        }
    }
}
#[doc = "Reader of field `TXT`"]
pub type TXT_R = crate::R<u8, TXT_A>;
impl TXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXT_A::_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TXT_A::_00
    }
}
#[doc = "TX Endpoint Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "1: Enable"]
    _1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        match variant {
            TXE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TXE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TXE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXE_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - RX endpoint Type"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - RX endpoint Enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TX Endpoint Stall"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 23 - TX Endpoint Enable"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W { w: self }
    }
    #[doc = "Bit 16 - TX Endpoint Stall"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
}
