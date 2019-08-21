#[doc = "Reader of register LR"]
pub type R = crate::R<u32, super::LR>;
#[doc = "Writer for register LR"]
pub type W = crate::W<u32, super::LR>;
#[doc = "Register LR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Time Compensation Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCL_A {
    #[doc = "0: Time Compensation Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Time Compensation Register is not locked and writes complete as normal."]
    _1,
}
impl From<TCL_A> for bool {
    #[inline(always)]
    fn from(variant: TCL_A) -> Self {
        match variant {
            TCL_A::_0 => false,
            TCL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCL`"]
pub type TCL_R = crate::R<bool, TCL_A>;
impl TCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCL_A {
        match self.bits {
            false => TCL_A::_0,
            true => TCL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCL_A::_1
    }
}
#[doc = "Write proxy for field `TCL`"]
pub struct TCL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCL_A::_0)
    }
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCL_A::_1)
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
#[doc = "Control Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRL_A {
    #[doc = "0: Control Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Control Register is not locked and writes complete as normal."]
    _1,
}
impl From<CRL_A> for bool {
    #[inline(always)]
    fn from(variant: CRL_A) -> Self {
        match variant {
            CRL_A::_0 => false,
            CRL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRL`"]
pub type CRL_R = crate::R<bool, CRL_A>;
impl CRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRL_A {
        match self.bits {
            false => CRL_A::_0,
            true => CRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRL_A::_1
    }
}
#[doc = "Write proxy for field `CRL`"]
pub struct CRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Control Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRL_A::_0)
    }
    #[doc = "Control Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRL_A::_1)
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
#[doc = "Status Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRL_A {
    #[doc = "0: Status Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Status Register is not locked and writes complete as normal."]
    _1,
}
impl From<SRL_A> for bool {
    #[inline(always)]
    fn from(variant: SRL_A) -> Self {
        match variant {
            SRL_A::_0 => false,
            SRL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SRL`"]
pub type SRL_R = crate::R<bool, SRL_A>;
impl SRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRL_A {
        match self.bits {
            false => SRL_A::_0,
            true => SRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRL_A::_1
    }
}
#[doc = "Write proxy for field `SRL`"]
pub struct SRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Status Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRL_A::_0)
    }
    #[doc = "Status Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRL_A::_1)
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
#[doc = "Lock Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRL_A {
    #[doc = "0: Lock Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Lock Register is not locked and writes complete as normal."]
    _1,
}
impl From<LRL_A> for bool {
    #[inline(always)]
    fn from(variant: LRL_A) -> Self {
        match variant {
            LRL_A::_0 => false,
            LRL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LRL`"]
pub type LRL_R = crate::R<bool, LRL_A>;
impl LRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRL_A {
        match self.bits {
            false => LRL_A::_0,
            true => LRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRL_A::_1
    }
}
#[doc = "Write proxy for field `LRL`"]
pub struct LRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lock Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRL_A::_0)
    }
    #[doc = "Lock Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Tamper Time Seconds Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTSL_A {
    #[doc = "0: Tamper Time Seconds Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Tamper Time Seconds Register is not locked and writes complete as normal."]
    _1,
}
impl From<TTSL_A> for bool {
    #[inline(always)]
    fn from(variant: TTSL_A) -> Self {
        match variant {
            TTSL_A::_0 => false,
            TTSL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TTSL`"]
pub type TTSL_R = crate::R<bool, TTSL_A>;
impl TTSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTSL_A {
        match self.bits {
            false => TTSL_A::_0,
            true => TTSL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TTSL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TTSL_A::_1
    }
}
#[doc = "Write proxy for field `TTSL`"]
pub struct TTSL_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TTSL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper Time Seconds Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTSL_A::_0)
    }
    #[doc = "Tamper Time Seconds Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTSL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Monotonic Enable Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEL_A {
    #[doc = "0: Monotonic Enable Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Monotonic Enable Register is not locked and writes complete as normal."]
    _1,
}
impl From<MEL_A> for bool {
    #[inline(always)]
    fn from(variant: MEL_A) -> Self {
        match variant {
            MEL_A::_0 => false,
            MEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MEL`"]
pub type MEL_R = crate::R<bool, MEL_A>;
impl MEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEL_A {
        match self.bits {
            false => MEL_A::_0,
            true => MEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEL_A::_1
    }
}
#[doc = "Write proxy for field `MEL`"]
pub struct MEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Monotonic Enable Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEL_A::_0)
    }
    #[doc = "Monotonic Enable Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Monotonic Counter Low Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLL_A {
    #[doc = "0: Monotonic Counter Low Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Monotonic Counter Low Register is not locked and writes complete as normal."]
    _1,
}
impl From<MCLL_A> for bool {
    #[inline(always)]
    fn from(variant: MCLL_A) -> Self {
        match variant {
            MCLL_A::_0 => false,
            MCLL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MCLL`"]
pub type MCLL_R = crate::R<bool, MCLL_A>;
impl MCLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLL_A {
        match self.bits {
            false => MCLL_A::_0,
            true => MCLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCLL_A::_1
    }
}
#[doc = "Write proxy for field `MCLL`"]
pub struct MCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Monotonic Counter Low Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCLL_A::_0)
    }
    #[doc = "Monotonic Counter Low Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCLL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Monotonic Counter High Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCHL_A {
    #[doc = "0: Monotonic Counter High Register is locked and writes are ignored."]
    _0,
    #[doc = "1: Monotonic Counter High Register is not locked and writes complete as normal."]
    _1,
}
impl From<MCHL_A> for bool {
    #[inline(always)]
    fn from(variant: MCHL_A) -> Self {
        match variant {
            MCHL_A::_0 => false,
            MCHL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MCHL`"]
pub type MCHL_R = crate::R<bool, MCHL_A>;
impl MCHL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCHL_A {
        match self.bits {
            false => MCHL_A::_0,
            true => MCHL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCHL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCHL_A::_1
    }
}
#[doc = "Write proxy for field `MCHL`"]
pub struct MCHL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCHL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCHL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Monotonic Counter High Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCHL_A::_0)
    }
    #[doc = "Monotonic Counter High Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCHL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline(always)]
    pub fn crl(&self) -> CRL_R {
        CRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline(always)]
    pub fn lrl(&self) -> LRL_R {
        LRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tamper Time Seconds Lock"]
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Monotonic Enable Lock"]
    #[inline(always)]
    pub fn mel(&self) -> MEL_R {
        MEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Monotonic Counter Low Lock"]
    #[inline(always)]
    pub fn mcll(&self) -> MCLL_R {
        MCLL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Monotonic Counter High Lock"]
    #[inline(always)]
    pub fn mchl(&self) -> MCHL_R {
        MCHL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W {
        TCL_W { w: self }
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline(always)]
    pub fn crl(&mut self) -> CRL_W {
        CRL_W { w: self }
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W { w: self }
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline(always)]
    pub fn lrl(&mut self) -> LRL_W {
        LRL_W { w: self }
    }
    #[doc = "Bit 8 - Tamper Time Seconds Lock"]
    #[inline(always)]
    pub fn ttsl(&mut self) -> TTSL_W {
        TTSL_W { w: self }
    }
    #[doc = "Bit 9 - Monotonic Enable Lock"]
    #[inline(always)]
    pub fn mel(&mut self) -> MEL_W {
        MEL_W { w: self }
    }
    #[doc = "Bit 10 - Monotonic Counter Low Lock"]
    #[inline(always)]
    pub fn mcll(&mut self) -> MCLL_W {
        MCLL_W { w: self }
    }
    #[doc = "Bit 11 - Monotonic Counter High Lock"]
    #[inline(always)]
    pub fn mchl(&mut self) -> MCHL_W {
        MCHL_W { w: self }
    }
}
