#[doc = "Reader of register RX"]
pub type R = crate::R<u32, super::RX>;
#[doc = "Writer for register RX"]
pub type W = crate::W<u32, super::RX>;
#[doc = "Register RX `reset()`'s with value 0"]
impl crate::ResetValue for super::RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The ENVADJ field adjusts the trip point for the envelope detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.1000 V"]
    _000,
    #[doc = "1: Trip-Level Voltage is 0.1125 V"]
    _001,
    #[doc = "2: Trip-Level Voltage is 0.1250 V"]
    _010,
    #[doc = "3: Trip-Level Voltage is 0.0875 V"]
    _011,
}
impl From<ENVADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: ENVADJ_A) -> Self {
        match variant {
            ENVADJ_A::_000 => 0,
            ENVADJ_A::_001 => 1,
            ENVADJ_A::_010 => 2,
            ENVADJ_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `ENVADJ`"]
pub type ENVADJ_R = crate::R<u8, ENVADJ_A>;
impl ENVADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENVADJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENVADJ_A::_000),
            1 => Val(ENVADJ_A::_001),
            2 => Val(ENVADJ_A::_010),
            3 => Val(ENVADJ_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ENVADJ_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ENVADJ_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ENVADJ_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ENVADJ_A::_011
    }
}
#[doc = "Write proxy for field `ENVADJ`"]
pub struct ENVADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENVADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ENVADJ_A::_000)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ENVADJ_A::_001)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ENVADJ_A::_010)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ENVADJ_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCONADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.56875 V"]
    _000,
    #[doc = "1: Trip-Level Voltage is 0.55000 V"]
    _001,
    #[doc = "2: Trip-Level Voltage is 0.58125 V"]
    _010,
    #[doc = "3: Trip-Level Voltage is 0.60000 V"]
    _011,
}
impl From<DISCONADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: DISCONADJ_A) -> Self {
        match variant {
            DISCONADJ_A::_000 => 0,
            DISCONADJ_A::_001 => 1,
            DISCONADJ_A::_010 => 2,
            DISCONADJ_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `DISCONADJ`"]
pub type DISCONADJ_R = crate::R<u8, DISCONADJ_A>;
impl DISCONADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISCONADJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DISCONADJ_A::_000),
            1 => Val(DISCONADJ_A::_001),
            2 => Val(DISCONADJ_A::_010),
            3 => Val(DISCONADJ_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DISCONADJ_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DISCONADJ_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DISCONADJ_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DISCONADJ_A::_011
    }
}
#[doc = "Write proxy for field `DISCONADJ`"]
pub struct DISCONADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCONADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DISCONADJ_A::_000)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DISCONADJ_A::_001)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DISCONADJ_A::_010)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DISCONADJ_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBYPASS_A {
    #[doc = "0: Normal operation."]
    _0,
    #[doc = "1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    _1,
}
impl From<RXDBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDBYPASS_A) -> Self {
        match variant {
            RXDBYPASS_A::_0 => false,
            RXDBYPASS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXDBYPASS`"]
pub type RXDBYPASS_R = crate::R<bool, RXDBYPASS_A>;
impl RXDBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDBYPASS_A {
        match self.bits {
            false => RXDBYPASS_A::_0,
            true => RXDBYPASS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDBYPASS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDBYPASS_A::_1
    }
}
#[doc = "Write proxy for field `RXDBYPASS`"]
pub struct RXDBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::_0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> ENVADJ_W {
        ENVADJ_W { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DISCONADJ_W {
        DISCONADJ_W { w: self }
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W {
        RXDBYPASS_W { w: self }
    }
}
