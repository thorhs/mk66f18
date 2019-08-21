#[doc = "Reader of register MODEM"]
pub type R = crate::R<u8, super::MODEM>;
#[doc = "Writer for register MODEM"]
pub type W = crate::W<u8, super::MODEM>;
#[doc = "Register MODEM `reset()`'s with value 0"]
impl crate::ResetValue for super::MODEM {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmitter clear-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSE_A {
    #[doc = "0: CTS has no effect on the transmitter."]
    _0,
    #[doc = "1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1,
}
impl From<TXCTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSE_A) -> Self {
        match variant {
            TXCTSE_A::_0 => false,
            TXCTSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXCTSE`"]
pub type TXCTSE_R = crate::R<bool, TXCTSE_A>;
impl TXCTSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSE_A {
        match self.bits {
            false => TXCTSE_A::_0,
            true => TXCTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXCTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXCTSE_A::_1
    }
}
#[doc = "Write proxy for field `TXCTSE`"]
pub struct TXCTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCTSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSE_A::_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSE_A::_1)
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
#[doc = "Transmitter request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSE_A {
    #[doc = "0: The transmitter has no effect on RTS."]
    _0,
    #[doc = "1: When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    _1,
}
impl From<TXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSE_A) -> Self {
        match variant {
            TXRTSE_A::_0 => false,
            TXRTSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXRTSE`"]
pub type TXRTSE_R = crate::R<bool, TXRTSE_A>;
impl TXRTSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSE_A {
        match self.bits {
            false => TXRTSE_A::_0,
            true => TXRTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRTSE_A::_1
    }
}
#[doc = "Write proxy for field `TXRTSE`"]
pub struct TXRTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRTSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSE_A::_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit. (FIFO) (FIFO)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSE_A::_1)
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
#[doc = "Transmitter request-to-send polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSPOL_A {
    #[doc = "0: Transmitter RTS is active low."]
    _0,
    #[doc = "1: Transmitter RTS is active high."]
    _1,
}
impl From<TXRTSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSPOL_A) -> Self {
        match variant {
            TXRTSPOL_A::_0 => false,
            TXRTSPOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXRTSPOL`"]
pub type TXRTSPOL_R = crate::R<bool, TXRTSPOL_A>;
impl TXRTSPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSPOL_A {
        match self.bits {
            false => TXRTSPOL_A::_0,
            true => TXRTSPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRTSPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRTSPOL_A::_1
    }
}
#[doc = "Write proxy for field `TXRTSPOL`"]
pub struct TXRTSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRTSPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_1)
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
#[doc = "Receiver request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRTSE_A {
    #[doc = "0: The receiver has no effect on RTS."]
    _0,
    #[doc = "1: RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]. See Hardware flow control"]
    _1,
}
impl From<RXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RXRTSE_A) -> Self {
        match variant {
            RXRTSE_A::_0 => false,
            RXRTSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXRTSE`"]
pub type RXRTSE_R = crate::R<bool, RXRTSE_A>;
impl RXRTSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRTSE_A {
        match self.bits {
            false => RXRTSE_A::_0,
            true => RXRTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXRTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXRTSE_A::_1
    }
}
#[doc = "Write proxy for field `RXRTSE`"]
pub struct RXRTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRTSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRTSE_A::_0)
    }
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]. See Hardware flow control"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXRTSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&self) -> TXCTSE_R {
        TXCTSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&self) -> TXRTSE_R {
        TXRTSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&self) -> TXRTSPOL_R {
        TXRTSPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&self) -> RXRTSE_R {
        RXRTSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&mut self) -> TXCTSE_W {
        TXCTSE_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&mut self) -> TXRTSE_W {
        TXRTSE_W { w: self }
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&mut self) -> TXRTSPOL_W {
        TXRTSPOL_W { w: self }
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&mut self) -> RXRTSE_W {
        RXRTSE_W { w: self }
    }
}
