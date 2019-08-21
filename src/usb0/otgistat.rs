#[doc = "Reader of register OTGISTAT"]
pub type R = crate::R<u8, super::OTGISTAT>;
#[doc = "Writer for register OTGISTAT"]
pub type W = crate::W<u8, super::OTGISTAT>;
#[doc = "Register OTGISTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGISTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AVBUSCHG`"]
pub type AVBUSCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVBUSCHG`"]
pub struct AVBUSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBUSCHG_W<'a> {
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
#[doc = "Reader of field `B_SESS_CHG`"]
pub type B_SESS_CHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B_SESS_CHG`"]
pub struct B_SESS_CHG_W<'a> {
    w: &'a mut W,
}
impl<'a> B_SESS_CHG_W<'a> {
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
#[doc = "Reader of field `SESSVLDCHG`"]
pub type SESSVLDCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSVLDCHG`"]
pub struct SESSVLDCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSVLDCHG_W<'a> {
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
#[doc = "Reader of field `LINE_STATE_CHG`"]
pub type LINE_STATE_CHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINE_STATE_CHG`"]
pub struct LINE_STATE_CHG_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_STATE_CHG_W<'a> {
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
#[doc = "Reader of field `ONEMSEC`"]
pub type ONEMSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONEMSEC`"]
pub struct ONEMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSEC_W<'a> {
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
#[doc = "Reader of field `IDCHG`"]
pub type IDCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDCHG`"]
pub struct IDCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCHG_W<'a> {
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
    #[doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device."]
    #[inline(always)]
    pub fn avbuschg(&self) -> AVBUSCHG_R {
        AVBUSCHG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device."]
    #[inline(always)]
    pub fn b_sess_chg(&self) -> B_SESS_CHG_R {
        B_SESS_CHG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
    #[inline(always)]
    pub fn sessvldchg(&self) -> SESSVLDCHG_R {
        SESSVLDCHG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\] and CTL\\[JSTATE\\] bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    pub fn line_state_chg(&self) -> LINE_STATE_CHG_R {
        LINE_STATE_CHG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    pub fn onemsec(&self) -> ONEMSEC_R {
        ONEMSEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed."]
    #[inline(always)]
    pub fn idchg(&self) -> IDCHG_R {
        IDCHG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device."]
    #[inline(always)]
    pub fn avbuschg(&mut self) -> AVBUSCHG_W {
        AVBUSCHG_W { w: self }
    }
    #[doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device."]
    #[inline(always)]
    pub fn b_sess_chg(&mut self) -> B_SESS_CHG_W {
        B_SESS_CHG_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
    #[inline(always)]
    pub fn sessvldchg(&mut self) -> SESSVLDCHG_W {
        SESSVLDCHG_W { w: self }
    }
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\] and CTL\\[JSTATE\\] bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    pub fn line_state_chg(&mut self) -> LINE_STATE_CHG_W {
        LINE_STATE_CHG_W { w: self }
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    pub fn onemsec(&mut self) -> ONEMSEC_W {
        ONEMSEC_W { w: self }
    }
    #[doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed."]
    #[inline(always)]
    pub fn idchg(&mut self) -> IDCHG_W {
        IDCHG_W { w: self }
    }
}
