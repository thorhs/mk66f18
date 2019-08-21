#[doc = "Reader of register OTGSC"]
pub type R = crate::R<u32, super::OTGSC>;
#[doc = "Writer for register OTGSC"]
pub type W = crate::W<u32, super::OTGSC>;
#[doc = "Register OTGSC `reset()`'s with value 0x1020"]
impl crate::ResetValue for super::OTGSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1020
    }
}
#[doc = "Reader of field `VD`"]
pub type VD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VD`"]
pub struct VD_W<'a> {
    w: &'a mut W,
}
impl<'a> VD_W<'a> {
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
#[doc = "Reader of field `VC`"]
pub type VC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC`"]
pub struct VC_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_W<'a> {
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
#[doc = "Hardware Assist Auto-Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAAR_A {
    #[doc = "0: Disabled."]
    _0,
    #[doc = "1: Enable automatic reset after connect on host port."]
    _1,
}
impl From<HAAR_A> for bool {
    #[inline(always)]
    fn from(variant: HAAR_A) -> Self {
        match variant {
            HAAR_A::_0 => false,
            HAAR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HAAR`"]
pub type HAAR_R = crate::R<bool, HAAR_A>;
impl HAAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAAR_A {
        match self.bits {
            false => HAAR_A::_0,
            true => HAAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HAAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HAAR_A::_1
    }
}
#[doc = "Write proxy for field `HAAR`"]
pub struct HAAR_W<'a> {
    w: &'a mut W,
}
impl<'a> HAAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HAAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HAAR_A::_0)
    }
    #[doc = "Enable automatic reset after connect on host port."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HAAR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "OTG Termination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OT_A {
    #[doc = "0: Disable pull-down on DM"]
    _0,
    #[doc = "1: Enable pull-down on DM"]
    _1,
}
impl From<OT_A> for bool {
    #[inline(always)]
    fn from(variant: OT_A) -> Self {
        match variant {
            OT_A::_0 => false,
            OT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `OT`"]
pub type OT_R = crate::R<bool, OT_A>;
impl OT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OT_A {
        match self.bits {
            false => OT_A::_0,
            true => OT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OT_A::_1
    }
}
#[doc = "Write proxy for field `OT`"]
pub struct OT_W<'a> {
    w: &'a mut W,
}
impl<'a> OT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable pull-down on DM"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OT_A::_0)
    }
    #[doc = "Enable pull-down on DM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OT_A::_1)
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
#[doc = "Data Pulsing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_A {
    #[doc = "0: The pull-up on DP is not asserted"]
    _0,
    #[doc = "1: The pull-up on DP is asserted for data pulsing during SRP"]
    _1,
}
impl From<DP_A> for bool {
    #[inline(always)]
    fn from(variant: DP_A) -> Self {
        match variant {
            DP_A::_0 => false,
            DP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DP`"]
pub type DP_R = crate::R<bool, DP_A>;
impl DP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_A {
        match self.bits {
            false => DP_A::_0,
            true => DP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DP_A::_1
    }
}
#[doc = "Write proxy for field `DP`"]
pub struct DP_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The pull-up on DP is not asserted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DP_A::_0)
    }
    #[doc = "The pull-up on DP is asserted for data pulsing during SRP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DP_A::_1)
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
#[doc = "ID Pull-Up\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDPU_A {
    #[doc = "0: Disable pull-up. ID input not sampled."]
    _0,
    #[doc = "1: Enable pull-up"]
    _1,
}
impl From<IDPU_A> for bool {
    #[inline(always)]
    fn from(variant: IDPU_A) -> Self {
        match variant {
            IDPU_A::_0 => false,
            IDPU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IDPU`"]
pub type IDPU_R = crate::R<bool, IDPU_A>;
impl IDPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPU_A {
        match self.bits {
            false => IDPU_A::_0,
            true => IDPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPU_A::_1
    }
}
#[doc = "Write proxy for field `IDPU`"]
pub struct IDPU_W<'a> {
    w: &'a mut W,
}
impl<'a> IDPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable pull-up. ID input not sampled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPU_A::_0)
    }
    #[doc = "Enable pull-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPU_A::_1)
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
#[doc = "Hardware Assist B-Disconnect to A-connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HABA_A {
    #[doc = "0: Disabled."]
    _0,
    #[doc = "1: Enable automatic B-disconnect to A-connect sequence."]
    _1,
}
impl From<HABA_A> for bool {
    #[inline(always)]
    fn from(variant: HABA_A) -> Self {
        match variant {
            HABA_A::_0 => false,
            HABA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HABA`"]
pub type HABA_R = crate::R<bool, HABA_A>;
impl HABA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HABA_A {
        match self.bits {
            false => HABA_A::_0,
            true => HABA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HABA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HABA_A::_1
    }
}
#[doc = "Write proxy for field `HABA`"]
pub struct HABA_W<'a> {
    w: &'a mut W,
}
impl<'a> HABA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HABA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HABA_A::_0)
    }
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HABA_A::_1)
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
#[doc = "USB ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID_A {
    #[doc = "0: A device"]
    _0,
    #[doc = "1: B device"]
    _1,
}
impl From<ID_A> for bool {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        match variant {
            ID_A::_0 => false,
            ID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            false => ID_A::_0,
            true => ID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
}
#[doc = "A VBus Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVV_A {
    #[doc = "0: VBus is below A VBus valid threshold"]
    _0,
    #[doc = "1: VBus is above A VBus valid threshold"]
    _1,
}
impl From<AVV_A> for bool {
    #[inline(always)]
    fn from(variant: AVV_A) -> Self {
        match variant {
            AVV_A::_0 => false,
            AVV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVV`"]
pub type AVV_R = crate::R<bool, AVV_A>;
impl AVV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVV_A {
        match self.bits {
            false => AVV_A::_0,
            true => AVV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVV_A::_1
    }
}
#[doc = "A Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASV_A {
    #[doc = "0: VBus is below A session valid threshold"]
    _0,
    #[doc = "1: VBus is above A session valid threshold"]
    _1,
}
impl From<ASV_A> for bool {
    #[inline(always)]
    fn from(variant: ASV_A) -> Self {
        match variant {
            ASV_A::_0 => false,
            ASV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ASV`"]
pub type ASV_R = crate::R<bool, ASV_A>;
impl ASV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASV_A {
        match self.bits {
            false => ASV_A::_0,
            true => ASV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASV_A::_1
    }
}
#[doc = "B Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSV_A {
    #[doc = "0: VBus is below B session valid threshold"]
    _0,
    #[doc = "1: VBus is above B session valid threshold"]
    _1,
}
impl From<BSV_A> for bool {
    #[inline(always)]
    fn from(variant: BSV_A) -> Self {
        match variant {
            BSV_A::_0 => false,
            BSV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BSV`"]
pub type BSV_R = crate::R<bool, BSV_A>;
impl BSV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSV_A {
        match self.bits {
            false => BSV_A::_0,
            true => BSV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSV_A::_1
    }
}
#[doc = "B Session End\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSE_A {
    #[doc = "0: VBus is above B session end threshold"]
    _0,
    #[doc = "1: VBus is below B session end threshold"]
    _1,
}
impl From<BSE_A> for bool {
    #[inline(always)]
    fn from(variant: BSE_A) -> Self {
        match variant {
            BSE_A::_0 => false,
            BSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BSE`"]
pub type BSE_R = crate::R<bool, BSE_A>;
impl BSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSE_A {
        match self.bits {
            false => BSE_A::_0,
            true => BSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSE_A::_1
    }
}
#[doc = "Reader of field `MST`"]
pub type MST_R = crate::R<bool, bool>;
#[doc = "Data bus Pulsing Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPS_A {
    #[doc = "0: No pulsing on port"]
    _0,
    #[doc = "1: Pulsing detected on port"]
    _1,
}
impl From<DPS_A> for bool {
    #[inline(always)]
    fn from(variant: DPS_A) -> Self {
        match variant {
            DPS_A::_0 => false,
            DPS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPS`"]
pub type DPS_R = crate::R<bool, DPS_A>;
impl DPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPS_A {
        match self.bits {
            false => DPS_A::_0,
            true => DPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPS_A::_1
    }
}
#[doc = "Reader of field `IDIS`"]
pub type IDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIS`"]
pub struct IDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AVVIS`"]
pub type AVVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVVIS`"]
pub struct AVVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVVIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ASVIS`"]
pub type ASVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASVIS`"]
pub struct ASVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASVIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `BSVIS`"]
pub type BSVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSVIS`"]
pub struct BSVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BSVIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `BSEIS`"]
pub type BSEIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSEIS`"]
pub struct BSEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BSEIS_W<'a> {
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
#[doc = "Reader of field `MSS`"]
pub type MSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSS`"]
pub struct MSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSS_W<'a> {
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
#[doc = "Reader of field `DPIS`"]
pub type DPIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPIS`"]
pub struct DPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPIS_W<'a> {
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
#[doc = "USB ID Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<IDIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDIE_A) -> Self {
        match variant {
            IDIE_A::_0 => false,
            IDIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IDIE`"]
pub type IDIE_R = crate::R<bool, IDIE_A>;
impl IDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDIE_A {
        match self.bits {
            false => IDIE_A::_0,
            true => IDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDIE_A::_1
    }
}
#[doc = "Write proxy for field `IDIE`"]
pub struct IDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDIE_A::_1)
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
#[doc = "A VBUS Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVVIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<AVVIE_A> for bool {
    #[inline(always)]
    fn from(variant: AVVIE_A) -> Self {
        match variant {
            AVVIE_A::_0 => false,
            AVVIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVVIE`"]
pub type AVVIE_R = crate::R<bool, AVVIE_A>;
impl AVVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVVIE_A {
        match self.bits {
            false => AVVIE_A::_0,
            true => AVVIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVVIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVVIE_A::_1
    }
}
#[doc = "Write proxy for field `AVVIE`"]
pub struct AVVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVVIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVVIE_A::_1)
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
#[doc = "A Session Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASVIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<ASVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ASVIE_A) -> Self {
        match variant {
            ASVIE_A::_0 => false,
            ASVIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ASVIE`"]
pub type ASVIE_R = crate::R<bool, ASVIE_A>;
impl ASVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASVIE_A {
        match self.bits {
            false => ASVIE_A::_0,
            true => ASVIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASVIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASVIE_A::_1
    }
}
#[doc = "Write proxy for field `ASVIE`"]
pub struct ASVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASVIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASVIE_A::_1)
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
#[doc = "B Session Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSVIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<BSVIE_A> for bool {
    #[inline(always)]
    fn from(variant: BSVIE_A) -> Self {
        match variant {
            BSVIE_A::_0 => false,
            BSVIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BSVIE`"]
pub type BSVIE_R = crate::R<bool, BSVIE_A>;
impl BSVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSVIE_A {
        match self.bits {
            false => BSVIE_A::_0,
            true => BSVIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSVIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSVIE_A::_1
    }
}
#[doc = "Write proxy for field `BSVIE`"]
pub struct BSVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSVIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSVIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "B Session End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSEIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<BSEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BSEIE_A) -> Self {
        match variant {
            BSEIE_A::_0 => false,
            BSEIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BSEIE`"]
pub type BSEIE_R = crate::R<bool, BSEIE_A>;
impl BSEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSEIE_A {
        match self.bits {
            false => BSEIE_A::_0,
            true => BSEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSEIE_A::_1
    }
}
#[doc = "Write proxy for field `BSEIE`"]
pub struct BSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSEIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "1 Milli-Second timer interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<MSE_A> for bool {
    #[inline(always)]
    fn from(variant: MSE_A) -> Self {
        match variant {
            MSE_A::_0 => false,
            MSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MSE`"]
pub type MSE_R = crate::R<bool, MSE_A>;
impl MSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSE_A {
        match self.bits {
            false => MSE_A::_0,
            true => MSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSE_A::_1
    }
}
#[doc = "Write proxy for field `MSE`"]
pub struct MSE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Data Pulse Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIE_A {
    #[doc = "0: Disable"]
    _0,
    #[doc = "1: Enable"]
    _1,
}
impl From<DPIE_A> for bool {
    #[inline(always)]
    fn from(variant: DPIE_A) -> Self {
        match variant {
            DPIE_A::_0 => false,
            DPIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPIE`"]
pub type DPIE_R = crate::R<bool, DPIE_A>;
impl DPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPIE_A {
        match self.bits {
            false => DPIE_A::_0,
            true => DPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPIE_A::_1
    }
}
#[doc = "Write proxy for field `DPIE`"]
pub struct DPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPIE_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VBUS Discharge"]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Charge"]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hardware Assist Auto-Reset"]
    #[inline(always)]
    pub fn haar(&self) -> HAAR_R {
        HAAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG Termination"]
    #[inline(always)]
    pub fn ot(&self) -> OT_R {
        OT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Pulsing"]
    #[inline(always)]
    pub fn dp(&self) -> DP_R {
        DP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ID Pull-Up"]
    #[inline(always)]
    pub fn idpu(&self) -> IDPU_R {
        IDPU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hardware Assist B-Disconnect to A-connect"]
    #[inline(always)]
    pub fn haba(&self) -> HABA_R {
        HABA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A VBus Valid"]
    #[inline(always)]
    pub fn avv(&self) -> AVV_R {
        AVV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A Session Valid"]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B Session Valid"]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B Session End"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 1 Milli-Second timer Toggle"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data bus Pulsing Status"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB ID Interrupt Status"]
    #[inline(always)]
    pub fn idis(&self) -> IDIS_R {
        IDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - A VBUS Valid Interrupt Status"]
    #[inline(always)]
    pub fn avvis(&self) -> AVVIS_R {
        AVVIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status"]
    #[inline(always)]
    pub fn asvis(&self) -> ASVIS_R {
        ASVIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status"]
    #[inline(always)]
    pub fn bsvis(&self) -> BSVIS_R {
        BSVIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B Session End Interrupt Status"]
    #[inline(always)]
    pub fn bseis(&self) -> BSEIS_R {
        BSEIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1 Milli-Second timer interrupt Status"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Pulse interrupt Status"]
    #[inline(always)]
    pub fn dpis(&self) -> DPIS_R {
        DPIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable"]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - A VBUS Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avvie(&self) -> AVVIE_R {
        AVVIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable"]
    #[inline(always)]
    pub fn asvie(&self) -> ASVIE_R {
        ASVIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable"]
    #[inline(always)]
    pub fn bsvie(&self) -> BSVIE_R {
        BSVIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable"]
    #[inline(always)]
    pub fn bseie(&self) -> BSEIE_R {
        BSEIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1 Milli-Second timer interrupt Enable"]
    #[inline(always)]
    pub fn mse(&self) -> MSE_R {
        MSE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn dpie(&self) -> DPIE_R {
        DPIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Discharge"]
    #[inline(always)]
    pub fn vd(&mut self) -> VD_W {
        VD_W { w: self }
    }
    #[doc = "Bit 1 - VBUS Charge"]
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W {
        VC_W { w: self }
    }
    #[doc = "Bit 2 - Hardware Assist Auto-Reset"]
    #[inline(always)]
    pub fn haar(&mut self) -> HAAR_W {
        HAAR_W { w: self }
    }
    #[doc = "Bit 3 - OTG Termination"]
    #[inline(always)]
    pub fn ot(&mut self) -> OT_W {
        OT_W { w: self }
    }
    #[doc = "Bit 4 - Data Pulsing"]
    #[inline(always)]
    pub fn dp(&mut self) -> DP_W {
        DP_W { w: self }
    }
    #[doc = "Bit 5 - ID Pull-Up"]
    #[inline(always)]
    pub fn idpu(&mut self) -> IDPU_W {
        IDPU_W { w: self }
    }
    #[doc = "Bit 7 - Hardware Assist B-Disconnect to A-connect"]
    #[inline(always)]
    pub fn haba(&mut self) -> HABA_W {
        HABA_W { w: self }
    }
    #[doc = "Bit 16 - USB ID Interrupt Status"]
    #[inline(always)]
    pub fn idis(&mut self) -> IDIS_W {
        IDIS_W { w: self }
    }
    #[doc = "Bit 17 - A VBUS Valid Interrupt Status"]
    #[inline(always)]
    pub fn avvis(&mut self) -> AVVIS_W {
        AVVIS_W { w: self }
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status"]
    #[inline(always)]
    pub fn asvis(&mut self) -> ASVIS_W {
        ASVIS_W { w: self }
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status"]
    #[inline(always)]
    pub fn bsvis(&mut self) -> BSVIS_W {
        BSVIS_W { w: self }
    }
    #[doc = "Bit 20 - B Session End Interrupt Status"]
    #[inline(always)]
    pub fn bseis(&mut self) -> BSEIS_W {
        BSEIS_W { w: self }
    }
    #[doc = "Bit 21 - 1 Milli-Second timer interrupt Status"]
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W {
        MSS_W { w: self }
    }
    #[doc = "Bit 22 - Data Pulse interrupt Status"]
    #[inline(always)]
    pub fn dpis(&mut self) -> DPIS_W {
        DPIS_W { w: self }
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable"]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W {
        IDIE_W { w: self }
    }
    #[doc = "Bit 25 - A VBUS Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avvie(&mut self) -> AVVIE_W {
        AVVIE_W { w: self }
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable"]
    #[inline(always)]
    pub fn asvie(&mut self) -> ASVIE_W {
        ASVIE_W { w: self }
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable"]
    #[inline(always)]
    pub fn bsvie(&mut self) -> BSVIE_W {
        BSVIE_W { w: self }
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable"]
    #[inline(always)]
    pub fn bseie(&mut self) -> BSEIE_W {
        BSEIE_W { w: self }
    }
    #[doc = "Bit 29 - 1 Milli-Second timer interrupt Enable"]
    #[inline(always)]
    pub fn mse(&mut self) -> MSE_W {
        MSE_W { w: self }
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn dpie(&mut self) -> DPIE_W {
        DPIE_W { w: self }
    }
}
