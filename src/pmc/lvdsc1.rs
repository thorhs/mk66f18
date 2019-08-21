#[doc = "Reader of register LVDSC1"]
pub type R = crate::R<u8, super::LVDSC1>;
#[doc = "Writer for register LVDSC1"]
pub type W = crate::W<u8, super::LVDSC1>;
#[doc = "Register LVDSC1 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::LVDSC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Low-Voltage Detect Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDV_A {
    #[doc = "0: Low trip point selected (V LVD = V LVDL )"]
    _00,
    #[doc = "1: High trip point selected (V LVD = V LVDH )"]
    _01,
}
impl From<LVDV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDV_A) -> Self {
        match variant {
            LVDV_A::_00 => 0,
            LVDV_A::_01 => 1,
        }
    }
}
#[doc = "Reader of field `LVDV`"]
pub type LVDV_R = crate::R<u8, LVDV_A>;
impl LVDV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LVDV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LVDV_A::_00),
            1 => Val(LVDV_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LVDV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LVDV_A::_01
    }
}
#[doc = "Write proxy for field `LVDV`"]
pub struct LVDV_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low trip point selected (V LVD = V LVDL )"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVDV_A::_00)
    }
    #[doc = "High trip point selected (V LVD = V LVDH )"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVDV_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRE_A {
    #[doc = "0: LVDF does not generate hardware resets"]
    _0,
    #[doc = "1: Force an MCU reset when LVDF = 1"]
    _1,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        match variant {
            LVDRE_A::_0 => false,
            LVDRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVDRE`"]
pub type LVDRE_R = crate::R<bool, LVDRE_A>;
impl LVDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRE_A {
        match self.bits {
            false => LVDRE_A::_0,
            true => LVDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDRE_A::_1
    }
}
#[doc = "Write proxy for field `LVDRE`"]
pub struct LVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LVDF does not generate hardware resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
    }
    #[doc = "Force an MCU reset when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDRE_A::_1)
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
#[doc = "Low-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "1: Request a hardware interrupt when LVDF = 1"]
    _1,
}
impl From<LVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDIE_A) -> Self {
        match variant {
            LVDIE_A::_0 => false,
            LVDIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVDIE`"]
pub type LVDIE_R = crate::R<bool, LVDIE_A>;
impl LVDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDIE_A {
        match self.bits {
            false => LVDIE_A::_0,
            true => LVDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDIE_A::_1
    }
}
#[doc = "Write proxy for field `LVDIE`"]
pub struct LVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDIE_A::_1)
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
#[doc = "Write proxy for field `LVDACK`"]
pub struct LVDACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDACK_W<'a> {
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
#[doc = "Low-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDF_A {
    #[doc = "0: Low-voltage event not detected"]
    _0,
    #[doc = "1: Low-voltage event detected"]
    _1,
}
impl From<LVDF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDF_A) -> Self {
        match variant {
            LVDF_A::_0 => false,
            LVDF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVDF`"]
pub type LVDF_R = crate::R<bool, LVDF_A>;
impl LVDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDF_A {
        match self.bits {
            false => LVDF_A::_0,
            true => LVDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&self) -> LVDV_R {
        LVDV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&self) -> LVDIE_R {
        LVDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Detect Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&mut self) -> LVDV_W {
        LVDV_W { w: self }
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&mut self) -> LVDRE_W {
        LVDRE_W { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&mut self) -> LVDIE_W {
        LVDIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Detect Acknowledge"]
    #[inline(always)]
    pub fn lvdack(&mut self) -> LVDACK_W {
        LVDACK_W { w: self }
    }
}
