#[doc = "Writer for register PSOR"]
pub type W = crate::W<u32, super::PSOR>;
#[doc = "Register PSOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO0_AW) -> Self {
        match variant {
            PTSO0_AW::_0 => false,
            PTSO0_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO0`"]
pub struct PTSO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO0_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO1_AW) -> Self {
        match variant {
            PTSO1_AW::_0 => false,
            PTSO1_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO1`"]
pub struct PTSO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO1_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO2_AW) -> Self {
        match variant {
            PTSO2_AW::_0 => false,
            PTSO2_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO2`"]
pub struct PTSO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO2_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO3_AW) -> Self {
        match variant {
            PTSO3_AW::_0 => false,
            PTSO3_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO3`"]
pub struct PTSO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO3_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO4_AW) -> Self {
        match variant {
            PTSO4_AW::_0 => false,
            PTSO4_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO4`"]
pub struct PTSO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO4_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO5_AW) -> Self {
        match variant {
            PTSO5_AW::_0 => false,
            PTSO5_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO5`"]
pub struct PTSO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO5_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO6_AW) -> Self {
        match variant {
            PTSO6_AW::_0 => false,
            PTSO6_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO6`"]
pub struct PTSO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO6_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO7_AW) -> Self {
        match variant {
            PTSO7_AW::_0 => false,
            PTSO7_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO7`"]
pub struct PTSO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO7_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO8_AW) -> Self {
        match variant {
            PTSO8_AW::_0 => false,
            PTSO8_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO8`"]
pub struct PTSO8_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO8_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO9_AW) -> Self {
        match variant {
            PTSO9_AW::_0 => false,
            PTSO9_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO9`"]
pub struct PTSO9_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO9_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO10_AW) -> Self {
        match variant {
            PTSO10_AW::_0 => false,
            PTSO10_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO10`"]
pub struct PTSO10_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO10_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO11_AW) -> Self {
        match variant {
            PTSO11_AW::_0 => false,
            PTSO11_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO11`"]
pub struct PTSO11_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO11_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO12_AW) -> Self {
        match variant {
            PTSO12_AW::_0 => false,
            PTSO12_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO12`"]
pub struct PTSO12_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO12_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO13_AW) -> Self {
        match variant {
            PTSO13_AW::_0 => false,
            PTSO13_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO13`"]
pub struct PTSO13_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO13_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO14_AW) -> Self {
        match variant {
            PTSO14_AW::_0 => false,
            PTSO14_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO14`"]
pub struct PTSO14_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO14_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO15_AW) -> Self {
        match variant {
            PTSO15_AW::_0 => false,
            PTSO15_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO15`"]
pub struct PTSO15_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO15_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO16_AW) -> Self {
        match variant {
            PTSO16_AW::_0 => false,
            PTSO16_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO16`"]
pub struct PTSO16_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO16_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO17_AW) -> Self {
        match variant {
            PTSO17_AW::_0 => false,
            PTSO17_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO17`"]
pub struct PTSO17_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO17_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO18_AW) -> Self {
        match variant {
            PTSO18_AW::_0 => false,
            PTSO18_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO18`"]
pub struct PTSO18_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO18_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO19_AW) -> Self {
        match variant {
            PTSO19_AW::_0 => false,
            PTSO19_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO19`"]
pub struct PTSO19_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO19_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO20_AW) -> Self {
        match variant {
            PTSO20_AW::_0 => false,
            PTSO20_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO20`"]
pub struct PTSO20_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO20_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO21_AW) -> Self {
        match variant {
            PTSO21_AW::_0 => false,
            PTSO21_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO21`"]
pub struct PTSO21_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO21_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO22_AW) -> Self {
        match variant {
            PTSO22_AW::_0 => false,
            PTSO22_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO22`"]
pub struct PTSO22_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO22_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO23_AW) -> Self {
        match variant {
            PTSO23_AW::_0 => false,
            PTSO23_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO23`"]
pub struct PTSO23_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO23_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO24_AW) -> Self {
        match variant {
            PTSO24_AW::_0 => false,
            PTSO24_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO24`"]
pub struct PTSO24_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO24_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO25_AW) -> Self {
        match variant {
            PTSO25_AW::_0 => false,
            PTSO25_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO25`"]
pub struct PTSO25_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO25_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO26_AW) -> Self {
        match variant {
            PTSO26_AW::_0 => false,
            PTSO26_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO26`"]
pub struct PTSO26_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO26_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO27_AW) -> Self {
        match variant {
            PTSO27_AW::_0 => false,
            PTSO27_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO27`"]
pub struct PTSO27_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO27_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO28_AW) -> Self {
        match variant {
            PTSO28_AW::_0 => false,
            PTSO28_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO28`"]
pub struct PTSO28_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO28_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO29_AW) -> Self {
        match variant {
            PTSO29_AW::_0 => false,
            PTSO29_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO29`"]
pub struct PTSO29_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO29_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO30_AW) -> Self {
        match variant {
            PTSO30_AW::_0 => false,
            PTSO30_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO30`"]
pub struct PTSO30_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO30_AW::_1)
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
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl From<PTSO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO31_AW) -> Self {
        match variant {
            PTSO31_AW::_0 => false,
            PTSO31_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTSO31`"]
pub struct PTSO31_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTSO31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO31_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Port Set Output"]
    #[inline(always)]
    pub fn ptso0(&mut self) -> PTSO0_W {
        PTSO0_W { w: self }
    }
    #[doc = "Bit 1 - Port Set Output"]
    #[inline(always)]
    pub fn ptso1(&mut self) -> PTSO1_W {
        PTSO1_W { w: self }
    }
    #[doc = "Bit 2 - Port Set Output"]
    #[inline(always)]
    pub fn ptso2(&mut self) -> PTSO2_W {
        PTSO2_W { w: self }
    }
    #[doc = "Bit 3 - Port Set Output"]
    #[inline(always)]
    pub fn ptso3(&mut self) -> PTSO3_W {
        PTSO3_W { w: self }
    }
    #[doc = "Bit 4 - Port Set Output"]
    #[inline(always)]
    pub fn ptso4(&mut self) -> PTSO4_W {
        PTSO4_W { w: self }
    }
    #[doc = "Bit 5 - Port Set Output"]
    #[inline(always)]
    pub fn ptso5(&mut self) -> PTSO5_W {
        PTSO5_W { w: self }
    }
    #[doc = "Bit 6 - Port Set Output"]
    #[inline(always)]
    pub fn ptso6(&mut self) -> PTSO6_W {
        PTSO6_W { w: self }
    }
    #[doc = "Bit 7 - Port Set Output"]
    #[inline(always)]
    pub fn ptso7(&mut self) -> PTSO7_W {
        PTSO7_W { w: self }
    }
    #[doc = "Bit 8 - Port Set Output"]
    #[inline(always)]
    pub fn ptso8(&mut self) -> PTSO8_W {
        PTSO8_W { w: self }
    }
    #[doc = "Bit 9 - Port Set Output"]
    #[inline(always)]
    pub fn ptso9(&mut self) -> PTSO9_W {
        PTSO9_W { w: self }
    }
    #[doc = "Bit 10 - Port Set Output"]
    #[inline(always)]
    pub fn ptso10(&mut self) -> PTSO10_W {
        PTSO10_W { w: self }
    }
    #[doc = "Bit 11 - Port Set Output"]
    #[inline(always)]
    pub fn ptso11(&mut self) -> PTSO11_W {
        PTSO11_W { w: self }
    }
    #[doc = "Bit 12 - Port Set Output"]
    #[inline(always)]
    pub fn ptso12(&mut self) -> PTSO12_W {
        PTSO12_W { w: self }
    }
    #[doc = "Bit 13 - Port Set Output"]
    #[inline(always)]
    pub fn ptso13(&mut self) -> PTSO13_W {
        PTSO13_W { w: self }
    }
    #[doc = "Bit 14 - Port Set Output"]
    #[inline(always)]
    pub fn ptso14(&mut self) -> PTSO14_W {
        PTSO14_W { w: self }
    }
    #[doc = "Bit 15 - Port Set Output"]
    #[inline(always)]
    pub fn ptso15(&mut self) -> PTSO15_W {
        PTSO15_W { w: self }
    }
    #[doc = "Bit 16 - Port Set Output"]
    #[inline(always)]
    pub fn ptso16(&mut self) -> PTSO16_W {
        PTSO16_W { w: self }
    }
    #[doc = "Bit 17 - Port Set Output"]
    #[inline(always)]
    pub fn ptso17(&mut self) -> PTSO17_W {
        PTSO17_W { w: self }
    }
    #[doc = "Bit 18 - Port Set Output"]
    #[inline(always)]
    pub fn ptso18(&mut self) -> PTSO18_W {
        PTSO18_W { w: self }
    }
    #[doc = "Bit 19 - Port Set Output"]
    #[inline(always)]
    pub fn ptso19(&mut self) -> PTSO19_W {
        PTSO19_W { w: self }
    }
    #[doc = "Bit 20 - Port Set Output"]
    #[inline(always)]
    pub fn ptso20(&mut self) -> PTSO20_W {
        PTSO20_W { w: self }
    }
    #[doc = "Bit 21 - Port Set Output"]
    #[inline(always)]
    pub fn ptso21(&mut self) -> PTSO21_W {
        PTSO21_W { w: self }
    }
    #[doc = "Bit 22 - Port Set Output"]
    #[inline(always)]
    pub fn ptso22(&mut self) -> PTSO22_W {
        PTSO22_W { w: self }
    }
    #[doc = "Bit 23 - Port Set Output"]
    #[inline(always)]
    pub fn ptso23(&mut self) -> PTSO23_W {
        PTSO23_W { w: self }
    }
    #[doc = "Bit 24 - Port Set Output"]
    #[inline(always)]
    pub fn ptso24(&mut self) -> PTSO24_W {
        PTSO24_W { w: self }
    }
    #[doc = "Bit 25 - Port Set Output"]
    #[inline(always)]
    pub fn ptso25(&mut self) -> PTSO25_W {
        PTSO25_W { w: self }
    }
    #[doc = "Bit 26 - Port Set Output"]
    #[inline(always)]
    pub fn ptso26(&mut self) -> PTSO26_W {
        PTSO26_W { w: self }
    }
    #[doc = "Bit 27 - Port Set Output"]
    #[inline(always)]
    pub fn ptso27(&mut self) -> PTSO27_W {
        PTSO27_W { w: self }
    }
    #[doc = "Bit 28 - Port Set Output"]
    #[inline(always)]
    pub fn ptso28(&mut self) -> PTSO28_W {
        PTSO28_W { w: self }
    }
    #[doc = "Bit 29 - Port Set Output"]
    #[inline(always)]
    pub fn ptso29(&mut self) -> PTSO29_W {
        PTSO29_W { w: self }
    }
    #[doc = "Bit 30 - Port Set Output"]
    #[inline(always)]
    pub fn ptso30(&mut self) -> PTSO30_W {
        PTSO30_W { w: self }
    }
    #[doc = "Bit 31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso31(&mut self) -> PTSO31_W {
        PTSO31_W { w: self }
    }
}
