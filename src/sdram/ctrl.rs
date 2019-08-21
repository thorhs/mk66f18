#[doc = "Reader of register CTRL"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RC`"]
pub type RC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RC`"]
pub struct RC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u16) & 0x01ff);
        self.w
    }
}
#[doc = "Refresh timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIM_A {
    #[doc = "0: 3 clocks"]
    _00,
    #[doc = "1: 6 clocks"]
    _01,
    #[doc = "2: 9 clocks"]
    _10,
    #[doc = "3: 9 clocks"]
    _11,
}
impl From<RTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RTIM_A) -> Self {
        match variant {
            RTIM_A::_00 => 0,
            RTIM_A::_01 => 1,
            RTIM_A::_10 => 2,
            RTIM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `RTIM`"]
pub type RTIM_R = crate::R<u8, RTIM_A>;
impl RTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTIM_A {
        match self.bits {
            0 => RTIM_A::_00,
            1 => RTIM_A::_01,
            2 => RTIM_A::_10,
            3 => RTIM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTIM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTIM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTIM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTIM_A::_11
    }
}
#[doc = "Write proxy for field `RTIM`"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTIM_A::_00)
    }
    #[doc = "6 clocks"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTIM_A::_01)
    }
    #[doc = "9 clocks"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTIM_A::_10)
    }
    #[doc = "9 clocks"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTIM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
#[doc = "Initiate self-refresh command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_A {
    #[doc = "0: Take no action or issue a selfx command to exit self refresh."]
    _0,
    #[doc = "1: SDRAM controller sends a self command to both SDRAM blocks to put them in low-power, self-refresh state where they remain until IS is cleared. When IS is cleared, the controller sends a selfx command for the SDRAMs to exit self-refresh. The refresh counter is suspended while the SDRAMs are in self-refresh; the SDRAM controls the refresh period."]
    _1,
}
impl From<IS_A> for bool {
    #[inline(always)]
    fn from(variant: IS_A) -> Self {
        match variant {
            IS_A::_0 => false,
            IS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IS`"]
pub type IS_R = crate::R<bool, IS_A>;
impl IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_A {
        match self.bits {
            false => IS_A::_0,
            true => IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_A::_1
    }
}
#[doc = "Write proxy for field `IS`"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Take no action or issue a selfx command to exit self refresh."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_A::_0)
    }
    #[doc = "SDRAM controller sends a self command to both SDRAM blocks to put them in low-power, self-refresh state where they remain until IS is cleared. When IS is cleared, the controller sends a selfx command for the SDRAMs to exit self-refresh. The refresh counter is suspended while the SDRAMs are in self-refresh; the SDRAM controls the refresh period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Refresh count"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - Refresh timing"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Initiate self-refresh command."]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Refresh count"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W {
        RC_W { w: self }
    }
    #[doc = "Bits 9:10 - Refresh timing"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 11 - Initiate self-refresh command."]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
}
