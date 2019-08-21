#[doc = "Reader of register CFIFO"]
pub type R = crate::R<u8, super::CFIFO>;
#[doc = "Writer for register CFIFO"]
pub type W = crate::W<u8, super::CFIFO>;
#[doc = "Register CFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::CFIFO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFE_A {
    #[doc = "0: RXUF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "1: RXUF flag generates an interrupt to the host."]
    _1,
}
impl From<RXUFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXUFE_A) -> Self {
        match variant {
            RXUFE_A::_0 => false,
            RXUFE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXUFE`"]
pub type RXUFE_R = crate::R<bool, RXUFE_A>;
impl RXUFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUFE_A {
        match self.bits {
            false => RXUFE_A::_0,
            true => RXUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXUFE_A::_1
    }
}
#[doc = "Write proxy for field `RXUFE`"]
pub struct RXUFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFE_A::_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFE_A {
    #[doc = "0: TXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "1: TXOF flag generates an interrupt to the host."]
    _1,
}
impl From<TXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFE_A) -> Self {
        match variant {
            TXOFE_A::_0 => false,
            TXOFE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXOFE`"]
pub type TXOFE_R = crate::R<bool, TXOFE_A>;
impl TXOFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFE_A {
        match self.bits {
            false => TXOFE_A::_0,
            true => TXOFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXOFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXOFE_A::_1
    }
}
#[doc = "Write proxy for field `TXOFE`"]
pub struct TXOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFE_A::_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Receive FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFE_A {
    #[doc = "0: RXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "1: RXOF flag generates an interrupt to the host."]
    _1,
}
impl From<RXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOFE_A) -> Self {
        match variant {
            RXOFE_A::_0 => false,
            RXOFE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXOFE`"]
pub type RXOFE_R = crate::R<bool, RXOFE_A>;
impl RXOFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOFE_A {
        match self.bits {
            false => RXOFE_A::_0,
            true => RXOFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXOFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXOFE_A::_1
    }
}
#[doc = "Write proxy for field `RXOFE`"]
pub struct RXOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOFE_A::_0)
    }
    #[doc = "RXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Receive FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0,
    #[doc = "1: All data in the receive FIFO/buffer is cleared out."]
    _1,
}
impl From<RXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_AW) -> Self {
        match variant {
            RXFLUSH_AW::_0 => false,
            RXFLUSH_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RXFLUSH`"]
pub struct RXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFLUSH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmit FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0,
    #[doc = "1: All data in the transmit FIFO/Buffer is cleared out."]
    _1,
}
impl From<TXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_AW) -> Self {
        match variant {
            TXFLUSH_AW::_0 => false,
            TXFLUSH_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `TXFLUSH`"]
pub struct TXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFLUSH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RXUFE_R {
        RXUFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TXOFE_R {
        TXOFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxofe(&self) -> RXOFE_R {
        RXOFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&mut self) -> RXUFE_W {
        RXUFE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&mut self) -> TXOFE_W {
        TXOFE_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxofe(&mut self) -> RXOFE_W {
        RXOFE_W { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RXFLUSH_W {
        RXFLUSH_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn txflush(&mut self) -> TXFLUSH_W {
        TXFLUSH_W { w: self }
    }
}
