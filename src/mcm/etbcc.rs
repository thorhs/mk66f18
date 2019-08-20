#[doc = "Reader of register ETBCC"]
pub type R = crate::R<u32, super::ETBCC>;
#[doc = "Writer for register ETBCC"]
pub type W = crate::W<u32, super::ETBCC>;
#[doc = "Register ETBCC `reset()`'s with value 0"]
impl crate::ResetValue for super::ETBCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEN_A {
    #[doc = "0: ETB counter disabled"]
    _0,
    #[doc = "1: ETB counter enabled"]
    _1,
}
impl From<CNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN_A) -> Self {
        match variant {
            CNTEN_A::_0 => false,
            CNTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CNTEN`"]
pub type CNTEN_R = crate::R<bool, CNTEN_A>;
impl CNTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN_A {
        match self.bits {
            false => CNTEN_A::_0,
            true => CNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTEN_A::_1
    }
}
#[doc = "Write proxy for field `CNTEN`"]
pub struct CNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ETB counter disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN_A::_0)
    }
    #[doc = "ETB counter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPT_A {
    #[doc = "0: No response when the ETB count expires"]
    _00,
    #[doc = "1: Generate a normal interrupt when the ETB count expires"]
    _01,
    #[doc = "2: Generate an NMI when the ETB count expires"]
    _10,
    #[doc = "3: Generate a debug halt when the ETB count expires"]
    _11,
}
impl From<RSPT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPT_A) -> Self {
        match variant {
            RSPT_A::_00 => 0,
            RSPT_A::_01 => 1,
            RSPT_A::_10 => 2,
            RSPT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `RSPT`"]
pub type RSPT_R = crate::R<u8, RSPT_A>;
impl RSPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPT_A {
        match self.bits {
            0 => RSPT_A::_00,
            1 => RSPT_A::_01,
            2 => RSPT_A::_10,
            3 => RSPT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSPT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSPT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSPT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSPT_A::_11
    }
}
#[doc = "Write proxy for field `RSPT`"]
pub struct RSPT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSPT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No response when the ETB count expires"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPT_A::_00)
    }
    #[doc = "Generate a normal interrupt when the ETB count expires"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPT_A::_01)
    }
    #[doc = "Generate an NMI when the ETB count expires"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPT_A::_10)
    }
    #[doc = "Generate a debug halt when the ETB count expires"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reload Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLRQ_A {
    #[doc = "0: No effect"]
    _0,
    #[doc = "1: Clears pending debug halt, NMI, or IRQ interrupt requests"]
    _1,
}
impl From<RLRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RLRQ_A) -> Self {
        match variant {
            RLRQ_A::_0 => false,
            RLRQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RLRQ`"]
pub type RLRQ_R = crate::R<bool, RLRQ_A>;
impl RLRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLRQ_A {
        match self.bits {
            false => RLRQ_A::_0,
            true => RLRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RLRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RLRQ_A::_1
    }
}
#[doc = "Write proxy for field `RLRQ`"]
pub struct RLRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RLRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLRQ_A::_0)
    }
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLRQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "ETM-To-TPIU Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETDIS_A {
    #[doc = "0: ETM-to-TPIU trace path enabled"]
    _0,
    #[doc = "1: ETM-to-TPIU trace path disabled"]
    _1,
}
impl From<ETDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ETDIS_A) -> Self {
        match variant {
            ETDIS_A::_0 => false,
            ETDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ETDIS`"]
pub type ETDIS_R = crate::R<bool, ETDIS_A>;
impl ETDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETDIS_A {
        match self.bits {
            false => ETDIS_A::_0,
            true => ETDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETDIS_A::_1
    }
}
#[doc = "Write proxy for field `ETDIS`"]
pub struct ETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ETM-to-TPIU trace path enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETDIS_A::_0)
    }
    #[doc = "ETM-to-TPIU trace path disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "ITM-To-TPIU Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITDIS_A {
    #[doc = "0: ITM-to-TPIU trace path enabled"]
    _0,
    #[doc = "1: ITM-to-TPIU trace path disabled"]
    _1,
}
impl From<ITDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ITDIS_A) -> Self {
        match variant {
            ITDIS_A::_0 => false,
            ITDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ITDIS`"]
pub type ITDIS_R = crate::R<bool, ITDIS_A>;
impl ITDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITDIS_A {
        match self.bits {
            false => ITDIS_A::_0,
            true => ITDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITDIS_A::_1
    }
}
#[doc = "Write proxy for field `ITDIS`"]
pub struct ITDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ITM-to-TPIU trace path enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITDIS_A::_0)
    }
    #[doc = "ITM-to-TPIU trace path disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline(always)]
    pub fn rspt(&self) -> RSPT_R {
        RSPT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline(always)]
    pub fn rlrq(&self) -> RLRQ_R {
        RLRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline(always)]
    pub fn etdis(&self) -> ETDIS_R {
        ETDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline(always)]
    pub fn itdis(&self) -> ITDIS_R {
        ITDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CNTEN_W {
        CNTEN_W { w: self }
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline(always)]
    pub fn rspt(&mut self) -> RSPT_W {
        RSPT_W { w: self }
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline(always)]
    pub fn rlrq(&mut self) -> RLRQ_W {
        RLRQ_W { w: self }
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline(always)]
    pub fn etdis(&mut self) -> ETDIS_W {
        ETDIS_W { w: self }
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline(always)]
    pub fn itdis(&mut self) -> ITDIS_W {
        ITDIS_W { w: self }
    }
}
