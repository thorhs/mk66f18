#[doc = "Reader of register CH%sC1"]
pub type R = crate::R<u32, super::CHC1>;
#[doc = "Writer for register CH%sC1"]
pub type W = crate::W<u32, super::CHC1>;
#[doc = "Register CH%sC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EN0_A) -> Self {
        match variant {
            EN0_A::_0 => false,
            EN0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN0`"]
pub type EN0_R = crate::R<bool, EN0_A>;
impl EN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN0_A {
        match self.bits {
            false => EN0_A::_0,
            true => EN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN0_A::_1
    }
}
#[doc = "Write proxy for field `EN0`"]
pub struct EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN0_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        match variant {
            EN1_A::_0 => false,
            EN1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN1`"]
pub type EN1_R = crate::R<bool, EN1_A>;
impl EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::_0,
            true => EN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1_A::_1
    }
}
#[doc = "Write proxy for field `EN1`"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        match variant {
            EN2_A::_0 => false,
            EN2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN2`"]
pub type EN2_R = crate::R<bool, EN2_A>;
impl EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::_0,
            true => EN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN2_A::_1
    }
}
#[doc = "Write proxy for field `EN2`"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN2_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EN3_A) -> Self {
        match variant {
            EN3_A::_0 => false,
            EN3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN3`"]
pub type EN3_R = crate::R<bool, EN3_A>;
impl EN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN3_A {
        match self.bits {
            false => EN3_A::_0,
            true => EN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN3_A::_1
    }
}
#[doc = "Write proxy for field `EN3`"]
pub struct EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN3_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EN4_A) -> Self {
        match variant {
            EN4_A::_0 => false,
            EN4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN4`"]
pub type EN4_R = crate::R<bool, EN4_A>;
impl EN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN4_A {
        match self.bits {
            false => EN4_A::_0,
            true => EN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN4_A::_1
    }
}
#[doc = "Write proxy for field `EN4`"]
pub struct EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN4_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EN5_A) -> Self {
        match variant {
            EN5_A::_0 => false,
            EN5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN5`"]
pub type EN5_R = crate::R<bool, EN5_A>;
impl EN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN5_A {
        match self.bits {
            false => EN5_A::_0,
            true => EN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN5_A::_1
    }
}
#[doc = "Write proxy for field `EN5`"]
pub struct EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN5_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN6_A> for bool {
    #[inline(always)]
    fn from(variant: EN6_A) -> Self {
        match variant {
            EN6_A::_0 => false,
            EN6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN6`"]
pub type EN6_R = crate::R<bool, EN6_A>;
impl EN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN6_A {
        match self.bits {
            false => EN6_A::_0,
            true => EN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN6_A::_1
    }
}
#[doc = "Write proxy for field `EN6`"]
pub struct EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN6_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl From<EN7_A> for bool {
    #[inline(always)]
    fn from(variant: EN7_A) -> Self {
        match variant {
            EN7_A::_0 => false,
            EN7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN7`"]
pub type EN7_R = crate::R<bool, EN7_A>;
impl EN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN7_A {
        match self.bits {
            false => EN7_A::_0,
            true => EN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN7_A::_1
    }
}
#[doc = "Write proxy for field `EN7`"]
pub struct EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN7_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS0_A> for bool {
    #[inline(always)]
    fn from(variant: TOS0_A) -> Self {
        match variant {
            TOS0_A::_0 => false,
            TOS0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS0`"]
pub type TOS0_R = crate::R<bool, TOS0_A>;
impl TOS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS0_A {
        match self.bits {
            false => TOS0_A::_0,
            true => TOS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS0_A::_1
    }
}
#[doc = "Write proxy for field `TOS0`"]
pub struct TOS0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS0_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS1_A> for bool {
    #[inline(always)]
    fn from(variant: TOS1_A) -> Self {
        match variant {
            TOS1_A::_0 => false,
            TOS1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS1`"]
pub type TOS1_R = crate::R<bool, TOS1_A>;
impl TOS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS1_A {
        match self.bits {
            false => TOS1_A::_0,
            true => TOS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS1_A::_1
    }
}
#[doc = "Write proxy for field `TOS1`"]
pub struct TOS1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS1_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS2_A> for bool {
    #[inline(always)]
    fn from(variant: TOS2_A) -> Self {
        match variant {
            TOS2_A::_0 => false,
            TOS2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS2`"]
pub type TOS2_R = crate::R<bool, TOS2_A>;
impl TOS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS2_A {
        match self.bits {
            false => TOS2_A::_0,
            true => TOS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS2_A::_1
    }
}
#[doc = "Write proxy for field `TOS2`"]
pub struct TOS2_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS2_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS3_A> for bool {
    #[inline(always)]
    fn from(variant: TOS3_A) -> Self {
        match variant {
            TOS3_A::_0 => false,
            TOS3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS3`"]
pub type TOS3_R = crate::R<bool, TOS3_A>;
impl TOS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS3_A {
        match self.bits {
            false => TOS3_A::_0,
            true => TOS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS3_A::_1
    }
}
#[doc = "Write proxy for field `TOS3`"]
pub struct TOS3_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS3_A::_1)
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
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS4_A> for bool {
    #[inline(always)]
    fn from(variant: TOS4_A) -> Self {
        match variant {
            TOS4_A::_0 => false,
            TOS4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS4`"]
pub type TOS4_R = crate::R<bool, TOS4_A>;
impl TOS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS4_A {
        match self.bits {
            false => TOS4_A::_0,
            true => TOS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS4_A::_1
    }
}
#[doc = "Write proxy for field `TOS4`"]
pub struct TOS4_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS5_A> for bool {
    #[inline(always)]
    fn from(variant: TOS5_A) -> Self {
        match variant {
            TOS5_A::_0 => false,
            TOS5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS5`"]
pub type TOS5_R = crate::R<bool, TOS5_A>;
impl TOS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS5_A {
        match self.bits {
            false => TOS5_A::_0,
            true => TOS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS5_A::_1
    }
}
#[doc = "Write proxy for field `TOS5`"]
pub struct TOS5_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS6_A> for bool {
    #[inline(always)]
    fn from(variant: TOS6_A) -> Self {
        match variant {
            TOS6_A::_0 => false,
            TOS6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS6`"]
pub type TOS6_R = crate::R<bool, TOS6_A>;
impl TOS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS6_A {
        match self.bits {
            false => TOS6_A::_0,
            true => TOS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS6_A::_1
    }
}
#[doc = "Write proxy for field `TOS6`"]
pub struct TOS6_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl From<TOS7_A> for bool {
    #[inline(always)]
    fn from(variant: TOS7_A) -> Self {
        match variant {
            TOS7_A::_0 => false,
            TOS7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOS7`"]
pub type TOS7_R = crate::R<bool, TOS7_A>;
impl TOS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS7_A {
        match self.bits {
            false => TOS7_A::_0,
            true => TOS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS7_A::_1
    }
}
#[doc = "Write proxy for field `TOS7`"]
pub struct TOS7_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB0_A> for bool {
    #[inline(always)]
    fn from(variant: BB0_A) -> Self {
        match variant {
            BB0_A::_0 => false,
            BB0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB0`"]
pub type BB0_R = crate::R<bool, BB0_A>;
impl BB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB0_A {
        match self.bits {
            false => BB0_A::_0,
            true => BB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB0_A::_1
    }
}
#[doc = "Write proxy for field `BB0`"]
pub struct BB0_W<'a> {
    w: &'a mut W,
}
impl<'a> BB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB1_A> for bool {
    #[inline(always)]
    fn from(variant: BB1_A) -> Self {
        match variant {
            BB1_A::_0 => false,
            BB1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB1`"]
pub type BB1_R = crate::R<bool, BB1_A>;
impl BB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB1_A {
        match self.bits {
            false => BB1_A::_0,
            true => BB1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB1_A::_1
    }
}
#[doc = "Write proxy for field `BB1`"]
pub struct BB1_W<'a> {
    w: &'a mut W,
}
impl<'a> BB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB2_A> for bool {
    #[inline(always)]
    fn from(variant: BB2_A) -> Self {
        match variant {
            BB2_A::_0 => false,
            BB2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB2`"]
pub type BB2_R = crate::R<bool, BB2_A>;
impl BB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB2_A {
        match self.bits {
            false => BB2_A::_0,
            true => BB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB2_A::_1
    }
}
#[doc = "Write proxy for field `BB2`"]
pub struct BB2_W<'a> {
    w: &'a mut W,
}
impl<'a> BB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB3_A> for bool {
    #[inline(always)]
    fn from(variant: BB3_A) -> Self {
        match variant {
            BB3_A::_0 => false,
            BB3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB3`"]
pub type BB3_R = crate::R<bool, BB3_A>;
impl BB3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB3_A {
        match self.bits {
            false => BB3_A::_0,
            true => BB3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB3_A::_1
    }
}
#[doc = "Write proxy for field `BB3`"]
pub struct BB3_W<'a> {
    w: &'a mut W,
}
impl<'a> BB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB4_A> for bool {
    #[inline(always)]
    fn from(variant: BB4_A) -> Self {
        match variant {
            BB4_A::_0 => false,
            BB4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB4`"]
pub type BB4_R = crate::R<bool, BB4_A>;
impl BB4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB4_A {
        match self.bits {
            false => BB4_A::_0,
            true => BB4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB4_A::_1
    }
}
#[doc = "Write proxy for field `BB4`"]
pub struct BB4_W<'a> {
    w: &'a mut W,
}
impl<'a> BB4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB5_A> for bool {
    #[inline(always)]
    fn from(variant: BB5_A) -> Self {
        match variant {
            BB5_A::_0 => false,
            BB5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB5`"]
pub type BB5_R = crate::R<bool, BB5_A>;
impl BB5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB5_A {
        match self.bits {
            false => BB5_A::_0,
            true => BB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB5_A::_1
    }
}
#[doc = "Write proxy for field `BB5`"]
pub struct BB5_W<'a> {
    w: &'a mut W,
}
impl<'a> BB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB6_A> for bool {
    #[inline(always)]
    fn from(variant: BB6_A) -> Self {
        match variant {
            BB6_A::_0 => false,
            BB6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB6`"]
pub type BB6_R = crate::R<bool, BB6_A>;
impl BB6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB6_A {
        match self.bits {
            false => BB6_A::_0,
            true => BB6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB6_A::_1
    }
}
#[doc = "Write proxy for field `BB6`"]
pub struct BB6_W<'a> {
    w: &'a mut W,
}
impl<'a> BB6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl From<BB7_A> for bool {
    #[inline(always)]
    fn from(variant: BB7_A) -> Self {
        match variant {
            BB7_A::_0 => false,
            BB7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BB7`"]
pub type BB7_R = crate::R<bool, BB7_A>;
impl BB7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB7_A {
        match self.bits {
            false => BB7_A::_0,
            true => BB7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB7_A::_1
    }
}
#[doc = "Write proxy for field `BB7`"]
pub struct BB7_W<'a> {
    w: &'a mut W,
}
impl<'a> BB7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos0(&self) -> TOS0_R {
        TOS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos1(&self) -> TOS1_R {
        TOS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos2(&self) -> TOS2_R {
        TOS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos3(&self) -> TOS3_R {
        TOS3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos4(&self) -> TOS4_R {
        TOS4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos5(&self) -> TOS5_R {
        TOS5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos6(&self) -> TOS6_R {
        TOS6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos7(&self) -> TOS7_R {
        TOS7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb0(&self) -> BB0_R {
        BB0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb1(&self) -> BB1_R {
        BB1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb2(&self) -> BB2_R {
        BB2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb3(&self) -> BB3_R {
        BB3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb4(&self) -> BB4_R {
        BB4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb5(&self) -> BB5_R {
        BB5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb6(&self) -> BB6_R {
        BB6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb7(&self) -> BB7_R {
        BB7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W {
        EN0_W { w: self }
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W {
        EN3_W { w: self }
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W {
        EN4_W { w: self }
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W {
        EN5_W { w: self }
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W {
        EN6_W { w: self }
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W {
        EN7_W { w: self }
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos0(&mut self) -> TOS0_W {
        TOS0_W { w: self }
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos1(&mut self) -> TOS1_W {
        TOS1_W { w: self }
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos2(&mut self) -> TOS2_W {
        TOS2_W { w: self }
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos3(&mut self) -> TOS3_W {
        TOS3_W { w: self }
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos4(&mut self) -> TOS4_W {
        TOS4_W { w: self }
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos5(&mut self) -> TOS5_W {
        TOS5_W { w: self }
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos6(&mut self) -> TOS6_W {
        TOS6_W { w: self }
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos7(&mut self) -> TOS7_W {
        TOS7_W { w: self }
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb0(&mut self) -> BB0_W {
        BB0_W { w: self }
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb1(&mut self) -> BB1_W {
        BB1_W { w: self }
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb2(&mut self) -> BB2_W {
        BB2_W { w: self }
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb3(&mut self) -> BB3_W {
        BB3_W { w: self }
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb4(&mut self) -> BB4_W {
        BB4_W { w: self }
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb5(&mut self) -> BB5_W {
        BB5_W { w: self }
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb6(&mut self) -> BB6_W {
        BB6_W { w: self }
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb7(&mut self) -> BB7_W {
        BB7_W { w: self }
    }
}
