#[doc = "Reader of register ESR1"]
pub type R = crate::R<u32, super::ESR1>;
#[doc = "Writer for register ESR1"]
pub type W = crate::W<u32, super::ESR1>;
#[doc = "Register ESR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wake-Up Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKINT_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: Indicates a recessive to dominant transition was received on the CAN bus."]
    _1,
}
impl From<WAKINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKINT_A) -> Self {
        match variant {
            WAKINT_A::_0 => false,
            WAKINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAKINT`"]
pub type WAKINT_R = crate::R<bool, WAKINT_A>;
impl WAKINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKINT_A {
        match self.bits {
            false => WAKINT_A::_0,
            true => WAKINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKINT_A::_1
    }
}
#[doc = "Write proxy for field `WAKINT`"]
pub struct WAKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKINT_A::_0)
    }
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKINT_A::_1)
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
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: Indicates setting of any Error Bit in the Error and Status Register."]
    _1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        match variant {
            ERRINT_A::_0 => false,
            ERRINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERRINT`"]
pub type ERRINT_R = crate::R<bool, ERRINT_A>;
impl ERRINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::_0,
            true => ERRINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRINT_A::_1
    }
}
#[doc = "Write proxy for field `ERRINT`"]
pub struct ERRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRINT_A::_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRINT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Bus Off Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFINT_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: FlexCAN module entered Bus Off state."]
    _1,
}
impl From<BOFFINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFINT_A) -> Self {
        match variant {
            BOFFINT_A::_0 => false,
            BOFFINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BOFFINT`"]
pub type BOFFINT_R = crate::R<bool, BOFFINT_A>;
impl BOFFINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFINT_A {
        match self.bits {
            false => BOFFINT_A::_0,
            true => BOFFINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOFFINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOFFINT_A::_1
    }
}
#[doc = "Write proxy for field `BOFFINT`"]
pub struct BOFFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFINT_A::_0)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFINT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "FlexCAN In Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: FlexCAN is not receiving a message."]
    _0,
    #[doc = "1: FlexCAN is receiving a message."]
    _1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        match variant {
            RX_A::_0 => false,
            RX_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::_0,
            true => RX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_A::_1
    }
}
#[doc = "Fault Confinement State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTCONF_A {
    #[doc = "0: Error Active"]
    _00,
    #[doc = "1: Error Passive"]
    _01,
}
impl From<FLTCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTCONF_A) -> Self {
        match variant {
            FLTCONF_A::_00 => 0,
            FLTCONF_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `FLTCONF`"]
pub type FLTCONF_R = crate::R<u8, FLTCONF_A>;
impl FLTCONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLTCONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLTCONF_A::_00),
            1 => Val(FLTCONF_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLTCONF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLTCONF_A::_01
    }
}
#[doc = "FlexCAN In Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: FlexCAN is not transmitting a message."]
    _0,
    #[doc = "1: FlexCAN is transmitting a message."]
    _1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        match variant {
            TX_A::_0 => false,
            TX_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::_0,
            true => TX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_A::_1
    }
}
#[doc = "This bit indicates when CAN bus is in IDLE state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: CAN bus is now IDLE."]
    _1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        match variant {
            IDLE_A::_0 => false,
            IDLE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::_0,
            true => IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLE_A::_1
    }
}
#[doc = "Rx Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWRN_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: RXERRCNT is greater than or equal to 96."]
    _1,
}
impl From<RXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: RXWRN_A) -> Self {
        match variant {
            RXWRN_A::_0 => false,
            RXWRN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXWRN`"]
pub type RXWRN_R = crate::R<bool, RXWRN_A>;
impl RXWRN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWRN_A {
        match self.bits {
            false => RXWRN_A::_0,
            true => RXWRN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXWRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXWRN_A::_1
    }
}
#[doc = "TX Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXWRN_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: TXERRCNT is greater than or equal to 96."]
    _1,
}
impl From<TXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: TXWRN_A) -> Self {
        match variant {
            TXWRN_A::_0 => false,
            TXWRN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXWRN`"]
pub type TXWRN_R = crate::R<bool, TXWRN_A>;
impl TXWRN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWRN_A {
        match self.bits {
            false => TXWRN_A::_0,
            true => TXWRN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXWRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXWRN_A::_1
    }
}
#[doc = "Stuffing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: A Stuffing Error occurred since last read of this register."]
    _1,
}
impl From<STFERR_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_A) -> Self {
        match variant {
            STFERR_A::_0 => false,
            STFERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STFERR`"]
pub type STFERR_R = crate::R<bool, STFERR_A>;
impl STFERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_A {
        match self.bits {
            false => STFERR_A::_0,
            true => STFERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STFERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STFERR_A::_1
    }
}
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    _1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        match variant {
            FRMERR_A::_0 => false,
            FRMERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRMERR`"]
pub type FRMERR_R = crate::R<bool, FRMERR_A>;
impl FRMERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::_0,
            true => FRMERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRMERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRMERR_A::_1
    }
}
#[doc = "Cyclic Redundancy Check Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    _1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        match variant {
            CRCERR_A::_0 => false,
            CRCERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, CRCERR_A>;
impl CRCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::_0,
            true => CRCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCERR_A::_1
    }
}
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: An ACK error occurred since last read of this register."]
    _1,
}
impl From<ACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKERR_A) -> Self {
        match variant {
            ACKERR_A::_0 => false,
            ACKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACKERR`"]
pub type ACKERR_R = crate::R<bool, ACKERR_A>;
impl ACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKERR_A {
        match self.bits {
            false => ACKERR_A::_0,
            true => ACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKERR_A::_1
    }
}
#[doc = "Bit0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    _1,
}
impl From<BIT0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_A) -> Self {
        match variant {
            BIT0ERR_A::_0 => false,
            BIT0ERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BIT0ERR`"]
pub type BIT0ERR_R = crate::R<bool, BIT0ERR_A>;
impl BIT0ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_A {
        match self.bits {
            false => BIT0ERR_A::_0,
            true => BIT0ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIT0ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIT0ERR_A::_1
    }
}
#[doc = "Bit1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERR_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    _1,
}
impl From<BIT1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_A) -> Self {
        match variant {
            BIT1ERR_A::_0 => false,
            BIT1ERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BIT1ERR`"]
pub type BIT1ERR_R = crate::R<bool, BIT1ERR_A>;
impl BIT1ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_A {
        match self.bits {
            false => BIT1ERR_A::_0,
            true => BIT1ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIT1ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIT1ERR_A::_1
    }
}
#[doc = "Rx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNINT_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl From<RWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNINT_A) -> Self {
        match variant {
            RWRNINT_A::_0 => false,
            RWRNINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWRNINT`"]
pub type RWRNINT_R = crate::R<bool, RWRNINT_A>;
impl RWRNINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNINT_A {
        match self.bits {
            false => RWRNINT_A::_0,
            true => RWRNINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWRNINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWRNINT_A::_1
    }
}
#[doc = "Write proxy for field `RWRNINT`"]
pub struct RWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWRNINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWRNINT_A::_0)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWRNINT_A::_1)
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
#[doc = "Tx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNINT_A {
    #[doc = "0: No such occurrence."]
    _0,
    #[doc = "1: The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    _1,
}
impl From<TWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNINT_A) -> Self {
        match variant {
            TWRNINT_A::_0 => false,
            TWRNINT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TWRNINT`"]
pub type TWRNINT_R = crate::R<bool, TWRNINT_A>;
impl TWRNINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNINT_A {
        match self.bits {
            false => TWRNINT_A::_0,
            true => TWRNINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWRNINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWRNINT_A::_1
    }
}
#[doc = "Write proxy for field `TWRNINT`"]
pub struct TWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWRNINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRNINT_A::_0)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRNINT_A::_1)
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
#[doc = "CAN Synchronization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCH_A {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus."]
    _0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus."]
    _1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        match variant {
            SYNCH_A::_0 => false,
            SYNCH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SYNCH`"]
pub type SYNCH_R = crate::R<bool, SYNCH_A>;
impl SYNCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::_0,
            true => SYNCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCH_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakint(&self) -> WAKINT_R {
        WAKINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&self) -> BOFFINT_R {
        BOFFINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline(always)]
    pub fn fltconf(&self) -> FLTCONF_R {
        FLTCONF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit indicates when CAN bus is in IDLE state"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline(always)]
    pub fn rxwrn(&self) -> RXWRN_R {
        RXWRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline(always)]
    pub fn txwrn(&self) -> TXWRN_R {
        TXWRN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline(always)]
    pub fn stferr(&self) -> STFERR_R {
        STFERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline(always)]
    pub fn ackerr(&self) -> ACKERR_R {
        ACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline(always)]
    pub fn bit0err(&self) -> BIT0ERR_R {
        BIT0ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline(always)]
    pub fn bit1err(&self) -> BIT1ERR_R {
        BIT1ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RWRNINT_R {
        RWRNINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&self) -> TWRNINT_R {
        TWRNINT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakint(&mut self) -> WAKINT_W {
        WAKINT_W { w: self }
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ERRINT_W {
        ERRINT_W { w: self }
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&mut self) -> BOFFINT_W {
        BOFFINT_W { w: self }
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&mut self) -> RWRNINT_W {
        RWRNINT_W { w: self }
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&mut self) -> TWRNINT_W {
        TWRNINT_W { w: self }
    }
}
