#[doc = "Reader of register DEVICEADDR"]
pub type R = crate::R<u32, super::DEVICEADDR>;
#[doc = "Writer for register DEVICEADDR"]
pub type W = crate::W<u32, super::DEVICEADDR>;
#[doc = "Register DEVICEADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVICEADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Device Address Advance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBADRA_A {
    #[doc = "0: Writes to USBADR are instantaneous."]
    _0,
    #[doc = "1: When this bit is written to a 1 at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR is loaded from the holding register."]
    _1,
}
impl From<USBADRA_A> for bool {
    #[inline(always)]
    fn from(variant: USBADRA_A) -> Self {
        match variant {
            USBADRA_A::_0 => false,
            USBADRA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBADRA`"]
pub type USBADRA_R = crate::R<bool, USBADRA_A>;
impl USBADRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBADRA_A {
        match self.bits {
            false => USBADRA_A::_0,
            true => USBADRA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBADRA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBADRA_A::_1
    }
}
#[doc = "Write proxy for field `USBADRA`"]
pub struct USBADRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USBADRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBADRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to USBADR are instantaneous."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBADRA_A::_0)
    }
    #[doc = "When this bit is written to a 1 at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR is loaded from the holding register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBADRA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USBADR`"]
pub type USBADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBADR`"]
pub struct USBADR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    pub fn usbadra(&self) -> USBADRA_R {
        USBADRA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:31 - Device Address"]
    #[inline(always)]
    pub fn usbadr(&self) -> USBADR_R {
        USBADR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    pub fn usbadra(&mut self) -> USBADRA_W {
        USBADRA_W { w: self }
    }
    #[doc = "Bits 25:31 - Device Address"]
    #[inline(always)]
    pub fn usbadr(&mut self) -> USBADR_W {
        USBADR_W { w: self }
    }
}
