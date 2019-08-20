#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u8, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u8, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPULLUPNONOTG_A {
    #[doc = "0: DP Pullup in non-OTG device mode is not enabled."]
    _0,
    #[doc = "1: DP Pullup in non-OTG device mode is enabled."]
    _1,
}
impl From<DPPULLUPNONOTG_A> for bool {
    #[inline(always)]
    fn from(variant: DPPULLUPNONOTG_A) -> Self {
        match variant {
            DPPULLUPNONOTG_A::_0 => false,
            DPPULLUPNONOTG_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPPULLUPNONOTG`"]
pub type DPPULLUPNONOTG_R = crate::R<bool, DPPULLUPNONOTG_A>;
impl DPPULLUPNONOTG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPULLUPNONOTG_A {
        match self.bits {
            false => DPPULLUPNONOTG_A::_0,
            true => DPPULLUPNONOTG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPPULLUPNONOTG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPPULLUPNONOTG_A::_1
    }
}
#[doc = "Write proxy for field `DPPULLUPNONOTG`"]
pub struct DPPULLUPNONOTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULLUPNONOTG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPPULLUPNONOTG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DP Pullup in non-OTG device mode is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPPULLUPNONOTG_A::_0)
    }
    #[doc = "DP Pullup in non-OTG device mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPPULLUPNONOTG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    pub fn dppullupnonotg(&self) -> DPPULLUPNONOTG_R {
        DPPULLUPNONOTG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    pub fn dppullupnonotg(&mut self) -> DPPULLUPNONOTG_W {
        DPPULLUPNONOTG_W { w: self }
    }
}
