#[doc = "Reader of register PORTSC1"]
pub type R = crate::R<u32, super::PORTSC1>;
#[doc = "Writer for register PORTSC1"]
pub type W = crate::W<u32, super::PORTSC1>;
#[doc = "Register PORTSC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTSC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCS_A {
    #[doc = "0: No device present (host mode) or attached (device mode)"]
    _0,
    #[doc = "1: Device is present (host mode) or attached (device mode)"]
    _1,
}
impl From<CCS_A> for bool {
    #[inline(always)]
    fn from(variant: CCS_A) -> Self {
        match variant {
            CCS_A::_0 => false,
            CCS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCS`"]
pub type CCS_R = crate::R<bool, CCS_A>;
impl CCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::_0,
            true => CCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCS_A::_1
    }
}
#[doc = "Connect Change Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSC_A {
    #[doc = "0: No change"]
    _0,
    #[doc = "1: Connect status has changed"]
    _1,
}
impl From<CSC_A> for bool {
    #[inline(always)]
    fn from(variant: CSC_A) -> Self {
        match variant {
            CSC_A::_0 => false,
            CSC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CSC`"]
pub type CSC_R = crate::R<bool, CSC_A>;
impl CSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSC_A {
        match self.bits {
            false => CSC_A::_0,
            true => CSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSC_A::_1
    }
}
#[doc = "Write proxy for field `CSC`"]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSC_A::_0)
    }
    #[doc = "Connect status has changed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Port Enable/disable Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEC_A {
    #[doc = "0: No change"]
    _0,
    #[doc = "1: Port disabled"]
    _1,
}
impl From<PEC_A> for bool {
    #[inline(always)]
    fn from(variant: PEC_A) -> Self {
        match variant {
            PEC_A::_0 => false,
            PEC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<bool, PEC_A>;
impl PEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEC_A {
        match self.bits {
            false => PEC_A::_0,
            true => PEC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEC_A::_1
    }
}
#[doc = "Write proxy for field `PEC`"]
pub struct PEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEC_A::_0)
    }
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEC_A::_1)
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
#[doc = "Over-current active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCA_A {
    #[doc = "0: Port not in over-current condition"]
    _0,
    #[doc = "1: Port currently in over-current condition"]
    _1,
}
impl From<OCA_A> for bool {
    #[inline(always)]
    fn from(variant: OCA_A) -> Self {
        match variant {
            OCA_A::_0 => false,
            OCA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `OCA`"]
pub type OCA_R = crate::R<bool, OCA_A>;
impl OCA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCA_A {
        match self.bits {
            false => OCA_A::_0,
            true => OCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCA_A::_1
    }
}
#[doc = "Over-Current Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCC_A {
    #[doc = "0: No over-current"]
    _0,
    #[doc = "1: Over-current detect"]
    _1,
}
impl From<OCC_A> for bool {
    #[inline(always)]
    fn from(variant: OCC_A) -> Self {
        match variant {
            OCC_A::_0 => false,
            OCC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `OCC`"]
pub type OCC_R = crate::R<bool, OCC_A>;
impl OCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCC_A {
        match self.bits {
            false => OCC_A::_0,
            true => OCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCC_A::_1
    }
}
#[doc = "Write proxy for field `OCC`"]
pub struct OCC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No over-current"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCC_A::_0)
    }
    #[doc = "Over-current detect"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCC_A::_1)
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
#[doc = "Force Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPR_A {
    #[doc = "0: No resume (K-state) detected/driven on port"]
    _0,
    #[doc = "1: Resume detected/driven on port"]
    _1,
}
impl From<FPR_A> for bool {
    #[inline(always)]
    fn from(variant: FPR_A) -> Self {
        match variant {
            FPR_A::_0 => false,
            FPR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FPR`"]
pub type FPR_R = crate::R<bool, FPR_A>;
impl FPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPR_A {
        match self.bits {
            false => FPR_A::_0,
            true => FPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPR_A::_1
    }
}
#[doc = "Write proxy for field `FPR`"]
pub struct FPR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No resume (K-state) detected/driven on port"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPR_A::_0)
    }
    #[doc = "Resume detected/driven on port"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPR_A::_1)
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
#[doc = "Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "0: Port not in suspend state"]
    _0,
    #[doc = "1: Port in suspend state"]
    _1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        match variant {
            SUSP_A::_0 => false,
            SUSP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, SUSP_A>;
impl SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::_0,
            true => SUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSP_A::_1
    }
}
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port not in suspend state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSP_A::_0)
    }
    #[doc = "Port in suspend state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
    #[doc = "0: Port is not in reset"]
    _0,
    #[doc = "1: Port is in reset"]
    _1,
}
impl From<PR_A> for bool {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        match variant {
            PR_A::_0 => false,
            PR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, PR_A>;
impl PR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            false => PR_A::_0,
            true => PR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PR_A::_1
    }
}
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port is not in reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PR_A::_0)
    }
    #[doc = "Port is in reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PR_A::_1)
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
#[doc = "High Speed Port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSP_A {
    #[doc = "0: FS or LS"]
    _0,
    #[doc = "1: HS"]
    _1,
}
impl From<HSP_A> for bool {
    #[inline(always)]
    fn from(variant: HSP_A) -> Self {
        match variant {
            HSP_A::_0 => false,
            HSP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HSP`"]
pub type HSP_R = crate::R<bool, HSP_A>;
impl HSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSP_A {
        match self.bits {
            false => HSP_A::_0,
            true => HSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSP_A::_1
    }
}
#[doc = "Line Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LS_A {
    #[doc = "0: SE0"]
    _00,
    #[doc = "1: J-state"]
    _01,
    #[doc = "2: K-state"]
    _10,
    #[doc = "3: Undefined"]
    _11,
}
impl From<LS_A> for u8 {
    #[inline(always)]
    fn from(variant: LS_A) -> Self {
        match variant {
            LS_A::_00 => 0,
            LS_A::_01 => 1,
            LS_A::_10 => 2,
            LS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `LS`"]
pub type LS_R = crate::R<u8, LS_A>;
impl LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LS_A {
        match self.bits {
            0 => LS_A::_00,
            1 => LS_A::_01,
            2 => LS_A::_10,
            3 => LS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LS_A::_11
    }
}
#[doc = "Reader of field `PP`"]
pub type PP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PP`"]
pub struct PP_W<'a> {
    w: &'a mut W,
}
impl<'a> PP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PO`"]
pub type PO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PO`"]
pub struct PO_W<'a> {
    w: &'a mut W,
}
impl<'a> PO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PIC`"]
pub type PIC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIC`"]
pub struct PIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTC_A {
    #[doc = "0: Not enabled"]
    _0000,
    #[doc = "1: J_STATE"]
    _0001,
    #[doc = "2: K_STATE"]
    _0010,
    #[doc = "3: SE0_NAK"]
    _0011,
    #[doc = "4: Packet"]
    _0100,
    #[doc = "5: FORCE_ENABLE_HS"]
    _0101,
    #[doc = "6: FORCE_ENABLE_FS"]
    _0110,
    #[doc = "7: FORCE_ENABLE_LS"]
    _0111,
}
impl From<PTC_A> for u8 {
    #[inline(always)]
    fn from(variant: PTC_A) -> Self {
        match variant {
            PTC_A::_0000 => 0,
            PTC_A::_0001 => 1,
            PTC_A::_0010 => 2,
            PTC_A::_0011 => 3,
            PTC_A::_0100 => 4,
            PTC_A::_0101 => 5,
            PTC_A::_0110 => 6,
            PTC_A::_0111 => 7,
        }
    }
}
#[doc = "Reader of field `PTC`"]
pub type PTC_R = crate::R<u8, PTC_A>;
impl PTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTC_A::_0000),
            1 => Val(PTC_A::_0001),
            2 => Val(PTC_A::_0010),
            3 => Val(PTC_A::_0011),
            4 => Val(PTC_A::_0100),
            5 => Val(PTC_A::_0101),
            6 => Val(PTC_A::_0110),
            7 => Val(PTC_A::_0111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PTC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PTC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PTC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PTC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PTC_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PTC_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PTC_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PTC_A::_0111
    }
}
#[doc = "Write proxy for field `PTC`"]
pub struct PTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not enabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PTC_A::_0000)
    }
    #[doc = "J_STATE"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PTC_A::_0001)
    }
    #[doc = "K_STATE"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PTC_A::_0010)
    }
    #[doc = "SE0_NAK"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PTC_A::_0011)
    }
    #[doc = "Packet"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PTC_A::_0100)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PTC_A::_0101)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PTC_A::_0110)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PTC_A::_0111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WKCN`"]
pub type WKCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKCN`"]
pub struct WKCN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKCN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `WKDS`"]
pub type WKDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKDS`"]
pub struct WKDS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `WKOC`"]
pub type WKOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKOC`"]
pub struct WKOC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKOC_W<'a> {
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
#[doc = "Reader of field `PHCD`"]
pub type PHCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHCD`"]
pub struct PHCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PHCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port force Full-Speed Connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSC_A {
    #[doc = "0: Allow the port to identify itself as high speed"]
    _0,
    #[doc = "1: Force the port to only connect at full speed"]
    _1,
}
impl From<PFSC_A> for bool {
    #[inline(always)]
    fn from(variant: PFSC_A) -> Self {
        match variant {
            PFSC_A::_0 => false,
            PFSC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PFSC`"]
pub type PFSC_R = crate::R<bool, PFSC_A>;
impl PFSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSC_A {
        match self.bits {
            false => PFSC_A::_0,
            true => PFSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFSC_A::_1
    }
}
#[doc = "Write proxy for field `PFSC`"]
pub struct PFSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow the port to identify itself as high speed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFSC_A::_0)
    }
    #[doc = "Force the port to only connect at full speed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFSC_A::_1)
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
#[doc = "Reader of field `PTS2`"]
pub type PTS2_R = crate::R<bool, bool>;
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSPD_A {
    #[doc = "0: Full speed"]
    _00,
    #[doc = "1: Low speed"]
    _01,
    #[doc = "2: High speed"]
    _10,
    #[doc = "3: Undefined"]
    _11,
}
impl From<PSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PSPD_A) -> Self {
        match variant {
            PSPD_A::_00 => 0,
            PSPD_A::_01 => 1,
            PSPD_A::_10 => 2,
            PSPD_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PSPD`"]
pub type PSPD_R = crate::R<u8, PSPD_A>;
impl PSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSPD_A {
        match self.bits {
            0 => PSPD_A::_00,
            1 => PSPD_A::_01,
            2 => PSPD_A::_10,
            3 => PSPD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PSPD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PSPD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PSPD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PSPD_A::_11
    }
}
#[doc = "Port Transceiver Select \\[1:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTS_A {
    #[doc = "0: Use UTMI transceiver interface."]
    _000,
}
impl From<PTS_A> for u8 {
    #[inline(always)]
    fn from(variant: PTS_A) -> Self {
        match variant {
            PTS_A::_000 => 0,
        }
    }
}
#[doc = "Reader of field `PTS`"]
pub type PTS_R = crate::R<u8, PTS_A>;
impl PTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTS_A::_000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PTS_A::_000
    }
}
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connect Change Status"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/disabled"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/disable Change"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Over-current active"]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Over-Current Change"]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - High Speed Port."]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn po(&self) -> PO_R {
        PO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect enable"]
    #[inline(always)]
    pub fn wkcn(&self) -> WKCN_R {
        WKCN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect enable"]
    #[inline(always)]
    pub fn wkds(&self) -> WKDS_R {
        WKDS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Wake on Over-Current enable"]
    #[inline(always)]
    pub fn wkoc(&self) -> WKOC_R {
        WKOC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PHY low power suspend"]
    #[inline(always)]
    pub fn phcd(&self) -> PHCD_R {
        PHCD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port force Full-Speed Connect"]
    #[inline(always)]
    pub fn pfsc(&self) -> PFSC_R {
        PFSC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Transceiver Select \\[2\\]"]
    #[inline(always)]
    pub fn pts2(&self) -> PTS2_R {
        PTS2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - Port Speed"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port Transceiver Select \\[1:0\\]"]
    #[inline(always)]
    pub fn pts(&self) -> PTS_R {
        PTS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Change Status"]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/disabled"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/disable Change"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W { w: self }
    }
    #[doc = "Bit 5 - Over-Current Change"]
    #[inline(always)]
    pub fn occ(&mut self) -> OCC_W {
        OCC_W { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn fpr(&mut self) -> FPR_W {
        FPR_W { w: self }
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn pp(&mut self) -> PP_W {
        PP_W { w: self }
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn po(&mut self) -> PO_W {
        PO_W { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline(always)]
    pub fn pic(&mut self) -> PIC_W {
        PIC_W { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn ptc(&mut self) -> PTC_W {
        PTC_W { w: self }
    }
    #[doc = "Bit 20 - Wake on Connect enable"]
    #[inline(always)]
    pub fn wkcn(&mut self) -> WKCN_W {
        WKCN_W { w: self }
    }
    #[doc = "Bit 21 - Wake on Disconnect enable"]
    #[inline(always)]
    pub fn wkds(&mut self) -> WKDS_W {
        WKDS_W { w: self }
    }
    #[doc = "Bit 22 - Wake on Over-Current enable"]
    #[inline(always)]
    pub fn wkoc(&mut self) -> WKOC_W {
        WKOC_W { w: self }
    }
    #[doc = "Bit 23 - PHY low power suspend"]
    #[inline(always)]
    pub fn phcd(&mut self) -> PHCD_W {
        PHCD_W { w: self }
    }
    #[doc = "Bit 24 - Port force Full-Speed Connect"]
    #[inline(always)]
    pub fn pfsc(&mut self) -> PFSC_W {
        PFSC_W { w: self }
    }
}
