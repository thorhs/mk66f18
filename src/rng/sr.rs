#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Security Violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECV_A {
    #[doc = "0: No security violation"]
    _0,
    #[doc = "1: Security violation"]
    _1,
}
impl From<SECV_A> for bool {
    #[inline(always)]
    fn from(variant: SECV_A) -> Self {
        match variant {
            SECV_A::_0 => false,
            SECV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SECV`"]
pub type SECV_R = crate::R<bool, SECV_A>;
impl SECV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECV_A {
        match self.bits {
            false => SECV_A::_0,
            true => SECV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECV_A::_1
    }
}
#[doc = "Last Read Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRS_A {
    #[doc = "0: No underflow"]
    _0,
    #[doc = "1: Underflow"]
    _1,
}
impl From<LRS_A> for bool {
    #[inline(always)]
    fn from(variant: LRS_A) -> Self {
        match variant {
            LRS_A::_0 => false,
            LRS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LRS`"]
pub type LRS_R = crate::R<bool, LRS_A>;
impl LRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRS_A {
        match self.bits {
            false => LRS_A::_0,
            true => LRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRS_A::_1
    }
}
#[doc = "Output Register Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORU_A {
    #[doc = "0: No underflow"]
    _0,
    #[doc = "1: Underflow"]
    _1,
}
impl From<ORU_A> for bool {
    #[inline(always)]
    fn from(variant: ORU_A) -> Self {
        match variant {
            ORU_A::_0 => false,
            ORU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ORU`"]
pub type ORU_R = crate::R<bool, ORU_A>;
impl ORU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORU_A {
        match self.bits {
            false => ORU_A::_0,
            true => ORU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORU_A::_1
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRI_A {
    #[doc = "0: No underflow"]
    _0,
    #[doc = "1: Underflow"]
    _1,
}
impl From<ERRI_A> for bool {
    #[inline(always)]
    fn from(variant: ERRI_A) -> Self {
        match variant {
            ERRI_A::_0 => false,
            ERRI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERRI`"]
pub type ERRI_R = crate::R<bool, ERRI_A>;
impl ERRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRI_A {
        match self.bits {
            false => ERRI_A::_0,
            true => ERRI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRI_A::_1
    }
}
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLP_A {
    #[doc = "0: Normal mode"]
    _0,
    #[doc = "1: Sleep (low-power) mode"]
    _1,
}
impl From<SLP_A> for bool {
    #[inline(always)]
    fn from(variant: SLP_A) -> Self {
        match variant {
            SLP_A::_0 => false,
            SLP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLP`"]
pub type SLP_R = crate::R<bool, SLP_A>;
impl SLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_A {
        match self.bits {
            false => SLP_A::_0,
            true => SLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLP_A::_1
    }
}
#[doc = "Output Register Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OREG_LVL_A {
    #[doc = "0: No words (empty)"]
    _0,
    #[doc = "1: One word (valid)"]
    _1,
}
impl From<OREG_LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: OREG_LVL_A) -> Self {
        match variant {
            OREG_LVL_A::_0 => 0,
            OREG_LVL_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `OREG_LVL`"]
pub type OREG_LVL_R = crate::R<u8, OREG_LVL_A>;
impl OREG_LVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OREG_LVL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OREG_LVL_A::_0),
            1 => Val(OREG_LVL_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OREG_LVL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OREG_LVL_A::_1
    }
}
#[doc = "Output Register Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OREG_SIZE_A {
    #[doc = "1: One word (this value is fixed)"]
    _1,
}
impl From<OREG_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: OREG_SIZE_A) -> Self {
        match variant {
            OREG_SIZE_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `OREG_SIZE`"]
pub type OREG_SIZE_R = crate::R<u8, OREG_SIZE_A>;
impl OREG_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OREG_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(OREG_SIZE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OREG_SIZE_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation"]
    #[inline(always)]
    pub fn secv(&self) -> SECV_R {
        SECV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Last Read Status"]
    #[inline(always)]
    pub fn lrs(&self) -> LRS_R {
        LRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Register Underflow"]
    #[inline(always)]
    pub fn oru(&self) -> ORU_R {
        ORU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    pub fn slp(&self) -> SLP_R {
        SLP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Output Register Level"]
    #[inline(always)]
    pub fn oreg_lvl(&self) -> OREG_LVL_R {
        OREG_LVL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output Register Size"]
    #[inline(always)]
    pub fn oreg_size(&self) -> OREG_SIZE_R {
        OREG_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
