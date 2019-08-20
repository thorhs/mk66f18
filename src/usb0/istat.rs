#[doc = "Reader of register ISTAT"]
pub type R = crate::R<u8, super::ISTAT>;
#[doc = "Writer for register ISTAT"]
pub type W = crate::W<u8, super::ISTAT>;
#[doc = "Register ISTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ISTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
#[doc = "Reader of field `SOFTOK`"]
pub type SOFTOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTOK`"]
pub struct SOFTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTOK_W<'a> {
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
#[doc = "Reader of field `TOKDNE`"]
pub type TOKDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOKDNE`"]
pub struct TOKDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKDNE_W<'a> {
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
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Attach Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTACH_A {
    #[doc = "0: No Attach is detected since the last time the ATTACH bit was cleared."]
    _0,
    #[doc = "1: A peripheral is now present and must be configured (a stable non-SE0 state is detected for more than 2.5 us)."]
    _1,
}
impl From<ATTACH_A> for bool {
    #[inline(always)]
    fn from(variant: ATTACH_A) -> Self {
        match variant {
            ATTACH_A::_0 => false,
            ATTACH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ATTACH`"]
pub type ATTACH_R = crate::R<bool, ATTACH_A>;
impl ATTACH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTACH_A {
        match self.bits {
            false => ATTACH_A::_0,
            true => ATTACH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTACH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTACH_A::_1
    }
}
#[doc = "Write proxy for field `ATTACH`"]
pub struct ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATTACH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Attach is detected since the last time the ATTACH bit was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACH_A::_0)
    }
    #[doc = "A peripheral is now present and must be configured (a stable non-SE0 state is detected for more than 2.5 us)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACH_A::_1)
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
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    pub fn softok(&self) -> SOFTOK_R {
        SOFTOK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&self) -> TOKDNE_R {
        TOKDNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    pub fn softok(&mut self) -> SOFTOK_W {
        SOFTOK_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&mut self) -> TOKDNE_W {
        TOKDNE_W { w: self }
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    pub fn attach(&mut self) -> ATTACH_W {
        ATTACH_W { w: self }
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
}
