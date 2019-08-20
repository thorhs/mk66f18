#[doc = "Reader of register PMPROT"]
pub type R = crate::R<u8, super::PMPROT>;
#[doc = "Writer for register PMPROT"]
pub type W = crate::W<u8, super::PMPROT>;
#[doc = "Register PMPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::PMPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Allow Very-Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLLS_A {
    #[doc = "0: Any VLLSx mode is not allowed"]
    _0,
    #[doc = "1: Any VLLSx mode is allowed"]
    _1,
}
impl From<AVLLS_A> for bool {
    #[inline(always)]
    fn from(variant: AVLLS_A) -> Self {
        match variant {
            AVLLS_A::_0 => false,
            AVLLS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVLLS`"]
pub type AVLLS_R = crate::R<bool, AVLLS_A>;
impl AVLLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLLS_A {
        match self.bits {
            false => AVLLS_A::_0,
            true => AVLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVLLS_A::_1
    }
}
#[doc = "Write proxy for field `AVLLS`"]
pub struct AVLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLLS_A::_0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLLS_A::_1)
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
#[doc = "Allow Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLS_A {
    #[doc = "0: Any LLSx mode is not allowed"]
    _0,
    #[doc = "1: Any LLSx mode is allowed"]
    _1,
}
impl From<ALLS_A> for bool {
    #[inline(always)]
    fn from(variant: ALLS_A) -> Self {
        match variant {
            ALLS_A::_0 => false,
            ALLS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ALLS`"]
pub type ALLS_R = crate::R<bool, ALLS_A>;
impl ALLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLS_A {
        match self.bits {
            false => ALLS_A::_0,
            true => ALLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALLS_A::_1
    }
}
#[doc = "Write proxy for field `ALLS`"]
pub struct ALLS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Any LLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLS_A::_0)
    }
    #[doc = "Any LLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLS_A::_1)
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
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLP_A {
    #[doc = "0: VLPR, VLPW, and VLPS are not allowed."]
    _0,
    #[doc = "1: VLPR, VLPW, and VLPS are allowed."]
    _1,
}
impl From<AVLP_A> for bool {
    #[inline(always)]
    fn from(variant: AVLP_A) -> Self {
        match variant {
            AVLP_A::_0 => false,
            AVLP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVLP`"]
pub type AVLP_R = crate::R<bool, AVLP_A>;
impl AVLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLP_A {
        match self.bits {
            false => AVLP_A::_0,
            true => AVLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVLP_A::_1
    }
}
#[doc = "Write proxy for field `AVLP`"]
pub struct AVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLP_A::_0)
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLP_A::_1)
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
#[doc = "Allow High Speed Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHSRUN_A {
    #[doc = "0: HSRUN is not allowed"]
    _0,
    #[doc = "1: HSRUN is allowed"]
    _1,
}
impl From<AHSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: AHSRUN_A) -> Self {
        match variant {
            AHSRUN_A::_0 => false,
            AHSRUN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AHSRUN`"]
pub type AHSRUN_R = crate::R<bool, AHSRUN_A>;
impl AHSRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHSRUN_A {
        match self.bits {
            false => AHSRUN_A::_0,
            true => AHSRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AHSRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AHSRUN_A::_1
    }
}
#[doc = "Write proxy for field `AHSRUN`"]
pub struct AHSRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHSRUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSRUN is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHSRUN_A::_0)
    }
    #[doc = "HSRUN is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHSRUN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&self) -> AVLLS_R {
        AVLLS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&self) -> ALLS_R {
        ALLS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AVLP_R {
        AVLP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&self) -> AHSRUN_R {
        AHSRUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&mut self) -> AVLLS_W {
        AVLLS_W { w: self }
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&mut self) -> ALLS_W {
        ALLS_W { w: self }
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&mut self) -> AVLP_W {
        AVLP_W { w: self }
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&mut self) -> AHSRUN_W {
        AHSRUN_W { w: self }
    }
}
