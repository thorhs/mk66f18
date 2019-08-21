#[doc = "Reader of register SOPT1CFG"]
pub type R = crate::R<u32, super::SOPT1CFG>;
#[doc = "Writer for register SOPT1CFG"]
pub type W = crate::W<u32, super::SOPT1CFG>;
#[doc = "Register SOPT1CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT1CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB voltage regulator enable write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URWE_A {
    #[doc = "0: SOPT1 USBREGEN cannot be written."]
    _0,
    #[doc = "1: SOPT1 USBREGEN can be written."]
    _1,
}
impl From<URWE_A> for bool {
    #[inline(always)]
    fn from(variant: URWE_A) -> Self {
        match variant {
            URWE_A::_0 => false,
            URWE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `URWE`"]
pub type URWE_R = crate::R<bool, URWE_A>;
impl URWE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URWE_A {
        match self.bits {
            false => URWE_A::_0,
            true => URWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == URWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == URWE_A::_1
    }
}
#[doc = "Write proxy for field `URWE`"]
pub struct URWE_W<'a> {
    w: &'a mut W,
}
impl<'a> URWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: URWE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOPT1 USBREGEN cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(URWE_A::_0)
    }
    #[doc = "SOPT1 USBREGEN can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(URWE_A::_1)
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
#[doc = "USB voltage regulator VLP standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVSWE_A {
    #[doc = "0: SOPT1 USBVSTBY cannot be written."]
    _0,
    #[doc = "1: SOPT1 USBVSTBY can be written."]
    _1,
}
impl From<UVSWE_A> for bool {
    #[inline(always)]
    fn from(variant: UVSWE_A) -> Self {
        match variant {
            UVSWE_A::_0 => false,
            UVSWE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UVSWE`"]
pub type UVSWE_R = crate::R<bool, UVSWE_A>;
impl UVSWE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVSWE_A {
        match self.bits {
            false => UVSWE_A::_0,
            true => UVSWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UVSWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UVSWE_A::_1
    }
}
#[doc = "Write proxy for field `UVSWE`"]
pub struct UVSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> UVSWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UVSWE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOPT1 USBVSTBY cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UVSWE_A::_0)
    }
    #[doc = "SOPT1 USBVSTBY can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UVSWE_A::_1)
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
#[doc = "USB voltage regulator stop standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USSWE_A {
    #[doc = "0: SOPT1 USBSSTBY cannot be written."]
    _0,
    #[doc = "1: SOPT1 USBSSTBY can be written."]
    _1,
}
impl From<USSWE_A> for bool {
    #[inline(always)]
    fn from(variant: USSWE_A) -> Self {
        match variant {
            USSWE_A::_0 => false,
            USSWE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USSWE`"]
pub type USSWE_R = crate::R<bool, USSWE_A>;
impl USSWE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USSWE_A {
        match self.bits {
            false => USSWE_A::_0,
            true => USSWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USSWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USSWE_A::_1
    }
}
#[doc = "Write proxy for field `USSWE`"]
pub struct USSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> USSWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USSWE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOPT1 USBSSTBY cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USSWE_A::_0)
    }
    #[doc = "SOPT1 USBSSTBY can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USSWE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    pub fn urwe(&self) -> URWE_R {
        URWE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    pub fn uvswe(&self) -> UVSWE_R {
        UVSWE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    pub fn usswe(&self) -> USSWE_R {
        USSWE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    pub fn urwe(&mut self) -> URWE_W {
        URWE_W { w: self }
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    pub fn uvswe(&mut self) -> UVSWE_W {
        UVSWE_W { w: self }
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    pub fn usswe(&mut self) -> USSWE_W {
        USSWE_W { w: self }
    }
}
