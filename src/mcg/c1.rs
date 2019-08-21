#[doc = "Reader of register C1"]
pub type R = crate::R<u8, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u8, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0x04"]
impl crate::ResetValue for super::C1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTEN_A {
    #[doc = "0: Internal reference clock is disabled in Stop mode."]
    _0,
    #[doc = "1: Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    _1,
}
impl From<IREFSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IREFSTEN_A) -> Self {
        match variant {
            IREFSTEN_A::_0 => false,
            IREFSTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IREFSTEN`"]
pub type IREFSTEN_R = crate::R<bool, IREFSTEN_A>;
impl IREFSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFSTEN_A {
        match self.bits {
            false => IREFSTEN_A::_0,
            true => IREFSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFSTEN_A::_1
    }
}
#[doc = "Write proxy for field `IREFSTEN`"]
pub struct IREFSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREFSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREFSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_0)
    }
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_1)
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
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCLKEN_A {
    #[doc = "0: MCGIRCLK inactive."]
    _0,
    #[doc = "1: MCGIRCLK active."]
    _1,
}
impl From<IRCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRCLKEN_A) -> Self {
        match variant {
            IRCLKEN_A::_0 => false,
            IRCLKEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IRCLKEN`"]
pub type IRCLKEN_R = crate::R<bool, IRCLKEN_A>;
impl IRCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCLKEN_A {
        match self.bits {
            false => IRCLKEN_A::_0,
            true => IRCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `IRCLKEN`"]
pub struct IRCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGIRCLK inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_0)
    }
    #[doc = "MCGIRCLK active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_1)
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
#[doc = "Internal Reference Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFS_A {
    #[doc = "0: External reference clock is selected."]
    _0,
    #[doc = "1: The slow internal reference clock is selected."]
    _1,
}
impl From<IREFS_A> for bool {
    #[inline(always)]
    fn from(variant: IREFS_A) -> Self {
        match variant {
            IREFS_A::_0 => false,
            IREFS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IREFS`"]
pub type IREFS_R = crate::R<bool, IREFS_A>;
impl IREFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFS_A {
        match self.bits {
            false => IREFS_A::_0,
            true => IREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFS_A::_1
    }
}
#[doc = "Write proxy for field `IREFS`"]
pub struct IREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFS_A::_0)
    }
    #[doc = "The slow internal reference clock is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFS_A::_1)
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
#[doc = "FLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDIV_A {
    #[doc = "0: If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    _000,
    #[doc = "1: If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    _001,
    #[doc = "2: If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    _010,
    #[doc = "3: If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    _011,
    #[doc = "4: If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    _100,
    #[doc = "5: If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    _101,
    #[doc = "6: If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    _110,
    #[doc = "7: If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    _111,
}
impl From<FRDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FRDIV_A) -> Self {
        match variant {
            FRDIV_A::_000 => 0,
            FRDIV_A::_001 => 1,
            FRDIV_A::_010 => 2,
            FRDIV_A::_011 => 3,
            FRDIV_A::_100 => 4,
            FRDIV_A::_101 => 5,
            FRDIV_A::_110 => 6,
            FRDIV_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `FRDIV`"]
pub type FRDIV_R = crate::R<u8, FRDIV_A>;
impl FRDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDIV_A {
        match self.bits {
            0 => FRDIV_A::_000,
            1 => FRDIV_A::_001,
            2 => FRDIV_A::_010,
            3 => FRDIV_A::_011,
            4 => FRDIV_A::_100,
            5 => FRDIV_A::_101,
            6 => FRDIV_A::_110,
            7 => FRDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FRDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FRDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FRDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FRDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FRDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FRDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FRDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FRDIV_A::_111
    }
}
#[doc = "Write proxy for field `FRDIV`"]
pub struct FRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FRDIV_A::_000)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FRDIV_A::_001)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FRDIV_A::_010)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FRDIV_A::_011)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FRDIV_A::_100)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FRDIV_A::_101)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FRDIV_A::_110)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FRDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKS_A {
    #[doc = "0: Encoding 0 - Output of FLL or PLLCS is selected (depends on PLLS control bit)."]
    _00,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = "3: Encoding 3 - Reserved."]
    _11,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        match variant {
            CLKS_A::_00 => 0,
            CLKS_A::_01 => 1,
            CLKS_A::_10 => 2,
            CLKS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CLKS`"]
pub type CLKS_R = crate::R<u8, CLKS_A>;
impl CLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::_00,
            1 => CLKS_A::_01,
            2 => CLKS_A::_10,
            3 => CLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKS_A::_11
    }
}
#[doc = "Write proxy for field `CLKS`"]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Encoding 0 - Output of FLL or PLLCS is selected (depends on PLLS control bit)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKS_A::_00)
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKS_A::_01)
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKS_A::_10)
    }
    #[doc = "Encoding 3 - Reserved."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IREFSTEN_R {
        IREFSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IRCLKEN_R {
        IRCLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&self) -> IREFS_R {
        IREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FRDIV_R {
        FRDIV_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&mut self) -> IREFSTEN_W {
        IREFSTEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&mut self) -> IRCLKEN_W {
        IRCLKEN_W { w: self }
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&mut self) -> IREFS_W {
        IREFS_W { w: self }
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&mut self) -> FRDIV_W {
        FRDIV_W { w: self }
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
}
