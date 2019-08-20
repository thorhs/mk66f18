#[doc = "Reader of register OC"]
pub type R = crate::R<u8, super::OC>;
#[doc = "Writer for register OC"]
pub type W = crate::W<u8, super::OC>;
#[doc = "Register OC `reset()`'s with value 0"]
impl crate::ResetValue for super::OC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IRO Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IROPEN_A {
    #[doc = "0: The IRO signal is disabled."]
    _0,
    #[doc = "1: The IRO signal is enabled as output."]
    _1,
}
impl From<IROPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IROPEN_A) -> Self {
        match variant {
            IROPEN_A::_0 => false,
            IROPEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IROPEN`"]
pub type IROPEN_R = crate::R<bool, IROPEN_A>;
impl IROPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IROPEN_A {
        match self.bits {
            false => IROPEN_A::_0,
            true => IROPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IROPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IROPEN_A::_1
    }
}
#[doc = "Write proxy for field `IROPEN`"]
pub struct IROPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IROPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IROPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The IRO signal is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IROPEN_A::_0)
    }
    #[doc = "The IRO signal is enabled as output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IROPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "CMT Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTPOL_A {
    #[doc = "0: The IRO signal is active-low."]
    _0,
    #[doc = "1: The IRO signal is active-high."]
    _1,
}
impl From<CMTPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CMTPOL_A) -> Self {
        match variant {
            CMTPOL_A::_0 => false,
            CMTPOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CMTPOL`"]
pub type CMTPOL_R = crate::R<bool, CMTPOL_A>;
impl CMTPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTPOL_A {
        match self.bits {
            false => CMTPOL_A::_0,
            true => CMTPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMTPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMTPOL_A::_1
    }
}
#[doc = "Write proxy for field `CMTPOL`"]
pub struct CMTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMTPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMTPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The IRO signal is active-low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMTPOL_A::_0)
    }
    #[doc = "The IRO signal is active-high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMTPOL_A::_1)
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
#[doc = "Reader of field `IROL`"]
pub type IROL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IROL`"]
pub struct IROL_W<'a> {
    w: &'a mut W,
}
impl<'a> IROL_W<'a> {
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
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    pub fn iropen(&self) -> IROPEN_R {
        IROPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    pub fn cmtpol(&self) -> CMTPOL_R {
        CMTPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    pub fn irol(&self) -> IROL_R {
        IROL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    pub fn iropen(&mut self) -> IROPEN_W {
        IROPEN_W { w: self }
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    pub fn cmtpol(&mut self) -> CMTPOL_W {
        CMTPOL_W { w: self }
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    pub fn irol(&mut self) -> IROL_W {
        IROL_W { w: self }
    }
}
