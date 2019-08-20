#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x0200_0000"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0000
    }
}
#[doc = "Reader of field `POPNXTPTR`"]
pub type POPNXTPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXCTR`"]
pub type RXCTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXNXTPTR`"]
pub type TXNXTPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXCTR`"]
pub type TXCTR_R = crate::R<u8, u8>;
#[doc = "Receive FIFO Drain Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_A {
    #[doc = "0: RX FIFO is empty."]
    _0,
    #[doc = "1: RX FIFO is not empty."]
    _1,
}
impl From<RFDF_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_A) -> Self {
        match variant {
            RFDF_A::_0 => false,
            RFDF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RFDF`"]
pub type RFDF_R = crate::R<bool, RFDF_A>;
impl RFDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_A {
        match self.bits {
            false => RFDF_A::_0,
            true => RFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDF_A::_1
    }
}
#[doc = "Write proxy for field `RFDF`"]
pub struct RFDF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX FIFO is empty."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_A::_0)
    }
    #[doc = "RX FIFO is not empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOF_A {
    #[doc = "0: No Rx FIFO overflow."]
    _0,
    #[doc = "1: Rx FIFO overflow has occurred."]
    _1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        match variant {
            RFOF_A::_0 => false,
            RFOF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RFOF`"]
pub type RFOF_R = crate::R<bool, RFOF_A>;
impl RFOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_A::_1
    }
}
#[doc = "Write proxy for field `RFOF`"]
pub struct RFOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Rx FIFO overflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_A::_0)
    }
    #[doc = "Rx FIFO overflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Transmit FIFO Fill Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_A {
    #[doc = "0: TX FIFO is full."]
    _0,
    #[doc = "1: TX FIFO is not full."]
    _1,
}
impl From<TFFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_A) -> Self {
        match variant {
            TFFF_A::_0 => false,
            TFFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TFFF`"]
pub type TFFF_R = crate::R<bool, TFFF_A>;
impl TFFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_A {
        match self.bits {
            false => TFFF_A::_0,
            true => TFFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFF_A::_1
    }
}
#[doc = "Write proxy for field `TFFF`"]
pub struct TFFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TX FIFO is full."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_A::_0)
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFUF_A {
    #[doc = "0: No TX FIFO underflow."]
    _0,
    #[doc = "1: TX FIFO underflow has occurred."]
    _1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        match variant {
            TFUF_A::_0 => false,
            TFUF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TFUF`"]
pub type TFUF_R = crate::R<bool, TFUF_A>;
impl TFUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_A::_1
    }
}
#[doc = "Write proxy for field `TFUF`"]
pub struct TFUF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No TX FIFO underflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_A::_0)
    }
    #[doc = "TX FIFO underflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "End of Queue Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQF_A {
    #[doc = "0: EOQ is not set in the executing command."]
    _0,
    #[doc = "1: EOQ is set in the executing SPI command."]
    _1,
}
impl From<EOQF_A> for bool {
    #[inline(always)]
    fn from(variant: EOQF_A) -> Self {
        match variant {
            EOQF_A::_0 => false,
            EOQF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOQF`"]
pub type EOQF_R = crate::R<bool, EOQF_A>;
impl EOQF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQF_A {
        match self.bits {
            false => EOQF_A::_0,
            true => EOQF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOQF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOQF_A::_1
    }
}
#[doc = "Write proxy for field `EOQF`"]
pub struct EOQF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOQF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOQF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EOQ is not set in the executing command."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_A::_0)
    }
    #[doc = "EOQ is set in the executing SPI command."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "TX and RX Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXS_A {
    #[doc = "0: Transmit and receive operations are disabled (The module is in Stopped state)."]
    _0,
    #[doc = "1: Transmit and receive operations are enabled (The module is in Running state)."]
    _1,
}
impl From<TXRXS_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXS_A) -> Self {
        match variant {
            TXRXS_A::_0 => false,
            TXRXS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXRXS`"]
pub type TXRXS_R = crate::R<bool, TXRXS_A>;
impl TXRXS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXS_A {
        match self.bits {
            false => TXRXS_A::_0,
            true => TXRXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRXS_A::_1
    }
}
#[doc = "Write proxy for field `TXRXS`"]
pub struct TXRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRXS_A::_0)
    }
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRXS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: Transfer not complete."]
    _0,
    #[doc = "1: Transfer complete."]
    _1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        match variant {
            TCF_A::_0 => false,
            TCF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, TCF_A>;
impl TCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pop Next Pointer"]
    #[inline(always)]
    pub fn popnxtptr(&self) -> POPNXTPTR_R {
        POPNXTPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX FIFO Counter"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnxtptr(&self) -> TXNXTPTR_R {
        TXNXTPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TX FIFO Counter"]
    #[inline(always)]
    pub fn txctr(&self) -> TXCTR_R {
        TXCTR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&self) -> TFFF_R {
        TFFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&self) -> EOQF_R {
        EOQF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&self) -> TXRXS_R {
        TXRXS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&mut self) -> RFDF_W {
        RFDF_W { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&mut self) -> RFOF_W {
        RFOF_W { w: self }
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&mut self) -> TFFF_W {
        TFFF_W { w: self }
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&mut self) -> TFUF_W {
        TFUF_W { w: self }
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&mut self) -> EOQF_W {
        EOQF_W { w: self }
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&mut self) -> TXRXS_W {
        TXRXS_W { w: self }
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
}
