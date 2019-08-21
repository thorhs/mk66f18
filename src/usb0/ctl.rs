#[doc = "Reader of register CTL"]
pub type R = crate::R<u8, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u8, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBENSOFEN_A {
    #[doc = "0: Disables the USB Module."]
    _0,
    #[doc = "1: Enables the USB Module."]
    _1,
}
impl From<USBENSOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBENSOFEN_A) -> Self {
        match variant {
            USBENSOFEN_A::_0 => false,
            USBENSOFEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBENSOFEN`"]
pub type USBENSOFEN_R = crate::R<bool, USBENSOFEN_A>;
impl USBENSOFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBENSOFEN_A {
        match self.bits {
            false => USBENSOFEN_A::_0,
            true => USBENSOFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBENSOFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBENSOFEN_A::_1
    }
}
#[doc = "Write proxy for field `USBENSOFEN`"]
pub struct USBENSOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBENSOFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBENSOFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the USB Module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_0)
    }
    #[doc = "Enables the USB Module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_1)
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
#[doc = "Reader of field `ODDRST`"]
pub type ODDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODDRST`"]
pub struct ODDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ODDRST_W<'a> {
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
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Reader of field `HOSTMODEEN`"]
pub type HOSTMODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTMODEEN`"]
pub struct HOSTMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTMODEEN_W<'a> {
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
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXSUSPENDTOKENBUSY`"]
pub type TXSUSPENDTOKENBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSUSPENDTOKENBUSY`"]
pub struct TXSUSPENDTOKENBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSUSPENDTOKENBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SE0`"]
pub type SE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE0`"]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
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
#[doc = "Reader of field `JSTATE`"]
pub type JSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSTATE`"]
pub struct JSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTATE_W<'a> {
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
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&self) -> USBENSOFEN_R {
        USBENSOFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&self) -> ODDRST_R {
        ODDRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&self) -> HOSTMODEEN_R {
        HOSTMODEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&self) -> TXSUSPENDTOKENBUSY_R {
        TXSUSPENDTOKENBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&mut self) -> USBENSOFEN_W {
        USBENSOFEN_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&mut self) -> ODDRST_W {
        ODDRST_W { w: self }
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&mut self) -> HOSTMODEEN_W {
        HOSTMODEEN_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&mut self) -> TXSUSPENDTOKENBUSY_W {
        TXSUSPENDTOKENBUSY_W { w: self }
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&mut self) -> JSTATE_W {
        JSTATE_W { w: self }
    }
}
