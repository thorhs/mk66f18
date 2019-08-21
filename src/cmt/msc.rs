#[doc = "Reader of register MSC"]
pub type R = crate::R<u8, super::MSC>;
#[doc = "Writer for register MSC"]
pub type W = crate::W<u8, super::MSC>;
#[doc = "Register MSC `reset()`'s with value 0"]
impl crate::ResetValue for super::MSC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Modulator and Carrier Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCGEN_A {
    #[doc = "0: Modulator and carrier generator disabled"]
    _0,
    #[doc = "1: Modulator and carrier generator enabled"]
    _1,
}
impl From<MCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCGEN_A) -> Self {
        match variant {
            MCGEN_A::_0 => false,
            MCGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MCGEN`"]
pub type MCGEN_R = crate::R<bool, MCGEN_A>;
impl MCGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCGEN_A {
        match self.bits {
            false => MCGEN_A::_0,
            true => MCGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCGEN_A::_1
    }
}
#[doc = "Write proxy for field `MCGEN`"]
pub struct MCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Modulator and carrier generator disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCGEN_A::_0)
    }
    #[doc = "Modulator and carrier generator enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCGEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "End of Cycle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIE_A {
    #[doc = "0: CPU interrupt is disabled."]
    _0,
    #[doc = "1: CPU interrupt is enabled."]
    _1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        match variant {
            EOCIE_A::_0 => false,
            EOCIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOCIE`"]
pub type EOCIE_R = crate::R<bool, EOCIE_A>;
impl EOCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::_0,
            true => EOCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCIE_A::_1
    }
}
#[doc = "Write proxy for field `EOCIE`"]
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCIE_A::_0)
    }
    #[doc = "CPU interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "FSK Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSK_A {
    #[doc = "0: The CMT operates in Time or Baseband mode."]
    _0,
    #[doc = "1: The CMT operates in FSK mode."]
    _1,
}
impl From<FSK_A> for bool {
    #[inline(always)]
    fn from(variant: FSK_A) -> Self {
        match variant {
            FSK_A::_0 => false,
            FSK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSK`"]
pub type FSK_R = crate::R<bool, FSK_A>;
impl FSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSK_A {
        match self.bits {
            false => FSK_A::_0,
            true => FSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSK_A::_1
    }
}
#[doc = "Write proxy for field `FSK`"]
pub struct FSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CMT operates in Time or Baseband mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSK_A::_0)
    }
    #[doc = "The CMT operates in FSK mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Baseband Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASE_A {
    #[doc = "0: Baseband mode is disabled."]
    _0,
    #[doc = "1: Baseband mode is enabled."]
    _1,
}
impl From<BASE_A> for bool {
    #[inline(always)]
    fn from(variant: BASE_A) -> Self {
        match variant {
            BASE_A::_0 => false,
            BASE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BASE`"]
pub type BASE_R = crate::R<bool, BASE_A>;
impl BASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BASE_A {
        match self.bits {
            false => BASE_A::_0,
            true => BASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BASE_A::_1
    }
}
#[doc = "Write proxy for field `BASE`"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BASE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BASE_A::_0)
    }
    #[doc = "Baseband mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BASE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Extended Space Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXSPC_A {
    #[doc = "0: Extended space is disabled."]
    _0,
    #[doc = "1: Extended space is enabled."]
    _1,
}
impl From<EXSPC_A> for bool {
    #[inline(always)]
    fn from(variant: EXSPC_A) -> Self {
        match variant {
            EXSPC_A::_0 => false,
            EXSPC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EXSPC`"]
pub type EXSPC_R = crate::R<bool, EXSPC_A>;
impl EXSPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXSPC_A {
        match self.bits {
            false => EXSPC_A::_0,
            true => EXSPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXSPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXSPC_A::_1
    }
}
#[doc = "Write proxy for field `EXSPC`"]
pub struct EXSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXSPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXSPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Extended space is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXSPC_A::_0)
    }
    #[doc = "Extended space is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXSPC_A::_1)
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
#[doc = "CMT Clock Divide Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTDIV_A {
    #[doc = "0: IF * 1"]
    _00,
    #[doc = "1: IF * 2"]
    _01,
    #[doc = "2: IF * 4"]
    _10,
    #[doc = "3: IF * 8"]
    _11,
}
impl From<CMTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CMTDIV_A) -> Self {
        match variant {
            CMTDIV_A::_00 => 0,
            CMTDIV_A::_01 => 1,
            CMTDIV_A::_10 => 2,
            CMTDIV_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CMTDIV`"]
pub type CMTDIV_R = crate::R<u8, CMTDIV_A>;
impl CMTDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTDIV_A {
        match self.bits {
            0 => CMTDIV_A::_00,
            1 => CMTDIV_A::_01,
            2 => CMTDIV_A::_10,
            3 => CMTDIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMTDIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMTDIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMTDIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMTDIV_A::_11
    }
}
#[doc = "Write proxy for field `CMTDIV`"]
pub struct CMTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CMTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMTDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IF * 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMTDIV_A::_00)
    }
    #[doc = "IF * 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMTDIV_A::_01)
    }
    #[doc = "IF * 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMTDIV_A::_10)
    }
    #[doc = "IF * 8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMTDIV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u8) & 0x03) << 5);
        self.w
    }
}
#[doc = "End Of Cycle Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCF_A {
    #[doc = "0: End of modulation cycle has not occurred since the flag last cleared."]
    _0,
    #[doc = "1: End of modulator cycle has occurred."]
    _1,
}
impl From<EOCF_A> for bool {
    #[inline(always)]
    fn from(variant: EOCF_A) -> Self {
        match variant {
            EOCF_A::_0 => false,
            EOCF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EOCF`"]
pub type EOCF_R = crate::R<bool, EOCF_A>;
impl EOCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCF_A {
        match self.bits {
            false => EOCF_A::_0,
            true => EOCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline(always)]
    pub fn mcgen(&self) -> MCGEN_R {
        MCGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline(always)]
    pub fn fsk(&self) -> FSK_R {
        FSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline(always)]
    pub fn exspc(&self) -> EXSPC_R {
        EXSPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline(always)]
    pub fn cmtdiv(&self) -> CMTDIV_R {
        CMTDIV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - End Of Cycle Status Flag"]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline(always)]
    pub fn mcgen(&mut self) -> MCGEN_W {
        MCGEN_W { w: self }
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline(always)]
    pub fn fsk(&mut self) -> FSK_W {
        FSK_W { w: self }
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline(always)]
    pub fn exspc(&mut self) -> EXSPC_W {
        EXSPC_W { w: self }
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline(always)]
    pub fn cmtdiv(&mut self) -> CMTDIV_W {
        CMTDIV_W { w: self }
    }
}
