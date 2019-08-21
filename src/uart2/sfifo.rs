#[doc = "Reader of register SFIFO"]
pub type R = crate::R<u8, super::SFIFO>;
#[doc = "Writer for register SFIFO"]
pub type W = crate::W<u8, super::SFIFO>;
#[doc = "Register SFIFO `reset()`'s with value 0xc0"]
impl crate::ResetValue for super::SFIFO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc0
    }
}
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUF_A {
    #[doc = "0: No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "1: At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1,
}
impl From<RXUF_A> for bool {
    #[inline(always)]
    fn from(variant: RXUF_A) -> Self {
        match variant {
            RXUF_A::_0 => false,
            RXUF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, RXUF_A>;
impl RXUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUF_A {
        match self.bits {
            false => RXUF_A::_0,
            true => RXUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXUF_A::_1
    }
}
#[doc = "Write proxy for field `RXUF`"]
pub struct RXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUF_A::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUF_A::_1)
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
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOF_A {
    #[doc = "0: No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "1: At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl From<TXOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOF_A) -> Self {
        match variant {
            TXOF_A::_0 => false,
            TXOF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, TXOF_A>;
impl TXOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOF_A {
        match self.bits {
            false => TXOF_A::_0,
            true => TXOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXOF_A::_1
    }
}
#[doc = "Write proxy for field `TXOF`"]
pub struct TXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOF_A::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOF_A::_1)
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
#[doc = "Receiver Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOF_A {
    #[doc = "0: No receive buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "1: At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl From<RXOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOF_A) -> Self {
        match variant {
            RXOF_A::_0 => false,
            RXOF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXOF`"]
pub type RXOF_R = crate::R<bool, RXOF_A>;
impl RXOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOF_A {
        match self.bits {
            false => RXOF_A::_0,
            true => RXOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXOF_A::_1
    }
}
#[doc = "Write proxy for field `RXOF`"]
pub struct RXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXOF_A::_0)
    }
    #[doc = "At least one receive buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXOF_A::_1)
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
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer is not empty."]
    _0,
    #[doc = "1: Receive buffer is empty."]
    _1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        match variant {
            RXEMPT_A::_0 => false,
            RXEMPT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXEMPT`"]
pub type RXEMPT_R = crate::R<bool, RXEMPT_A>;
impl RXEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::_0,
            true => RXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEMPT_A::_1
    }
}
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPT_A {
    #[doc = "0: Transmit buffer is not empty."]
    _0,
    #[doc = "1: Transmit buffer is empty."]
    _1,
}
impl From<TXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPT_A) -> Self {
        match variant {
            TXEMPT_A::_0 => false,
            TXEMPT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXEMPT`"]
pub type TXEMPT_R = crate::R<bool, TXEMPT_A>;
impl TXEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPT_A {
        match self.bits {
            false => TXEMPT_A::_0,
            true => TXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEMPT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W { w: self }
    }
}
