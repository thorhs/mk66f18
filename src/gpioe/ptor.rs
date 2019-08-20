#[doc = "Writer for register PTOR"]
pub type W = crate::W<u32, super::PTOR>;
#[doc = "Register PTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO0_AW) -> Self {
        match variant {
            PTTO0_AW::_0 => false,
            PTTO0_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO0`"]
pub struct PTTO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO0_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO1_AW) -> Self {
        match variant {
            PTTO1_AW::_0 => false,
            PTTO1_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO1`"]
pub struct PTTO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO1_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO2_AW) -> Self {
        match variant {
            PTTO2_AW::_0 => false,
            PTTO2_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO2`"]
pub struct PTTO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO2_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO3_AW) -> Self {
        match variant {
            PTTO3_AW::_0 => false,
            PTTO3_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO3`"]
pub struct PTTO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO3_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO4_AW) -> Self {
        match variant {
            PTTO4_AW::_0 => false,
            PTTO4_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO4`"]
pub struct PTTO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO4_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO5_AW) -> Self {
        match variant {
            PTTO5_AW::_0 => false,
            PTTO5_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO5`"]
pub struct PTTO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO5_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO6_AW) -> Self {
        match variant {
            PTTO6_AW::_0 => false,
            PTTO6_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO6`"]
pub struct PTTO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO6_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO7_AW) -> Self {
        match variant {
            PTTO7_AW::_0 => false,
            PTTO7_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO7`"]
pub struct PTTO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO7_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO8_AW) -> Self {
        match variant {
            PTTO8_AW::_0 => false,
            PTTO8_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO8`"]
pub struct PTTO8_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO8_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO9_AW) -> Self {
        match variant {
            PTTO9_AW::_0 => false,
            PTTO9_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO9`"]
pub struct PTTO9_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO9_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO10_AW) -> Self {
        match variant {
            PTTO10_AW::_0 => false,
            PTTO10_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO10`"]
pub struct PTTO10_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO10_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO11_AW) -> Self {
        match variant {
            PTTO11_AW::_0 => false,
            PTTO11_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO11`"]
pub struct PTTO11_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO11_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO12_AW) -> Self {
        match variant {
            PTTO12_AW::_0 => false,
            PTTO12_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO12`"]
pub struct PTTO12_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO12_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO13_AW) -> Self {
        match variant {
            PTTO13_AW::_0 => false,
            PTTO13_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO13`"]
pub struct PTTO13_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO13_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO14_AW) -> Self {
        match variant {
            PTTO14_AW::_0 => false,
            PTTO14_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO14`"]
pub struct PTTO14_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO14_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO15_AW) -> Self {
        match variant {
            PTTO15_AW::_0 => false,
            PTTO15_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO15`"]
pub struct PTTO15_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO15_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO16_AW) -> Self {
        match variant {
            PTTO16_AW::_0 => false,
            PTTO16_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO16`"]
pub struct PTTO16_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO16_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO17_AW) -> Self {
        match variant {
            PTTO17_AW::_0 => false,
            PTTO17_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO17`"]
pub struct PTTO17_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO17_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO18_AW) -> Self {
        match variant {
            PTTO18_AW::_0 => false,
            PTTO18_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO18`"]
pub struct PTTO18_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO18_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO19_AW) -> Self {
        match variant {
            PTTO19_AW::_0 => false,
            PTTO19_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO19`"]
pub struct PTTO19_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO19_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO20_AW) -> Self {
        match variant {
            PTTO20_AW::_0 => false,
            PTTO20_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO20`"]
pub struct PTTO20_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO20_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO21_AW) -> Self {
        match variant {
            PTTO21_AW::_0 => false,
            PTTO21_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO21`"]
pub struct PTTO21_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO21_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO22_AW) -> Self {
        match variant {
            PTTO22_AW::_0 => false,
            PTTO22_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO22`"]
pub struct PTTO22_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO22_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO23_AW) -> Self {
        match variant {
            PTTO23_AW::_0 => false,
            PTTO23_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO23`"]
pub struct PTTO23_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO23_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO24_AW) -> Self {
        match variant {
            PTTO24_AW::_0 => false,
            PTTO24_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO24`"]
pub struct PTTO24_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO24_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO25_AW) -> Self {
        match variant {
            PTTO25_AW::_0 => false,
            PTTO25_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO25`"]
pub struct PTTO25_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO25_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO26_AW) -> Self {
        match variant {
            PTTO26_AW::_0 => false,
            PTTO26_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO26`"]
pub struct PTTO26_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO26_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO27_AW) -> Self {
        match variant {
            PTTO27_AW::_0 => false,
            PTTO27_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO27`"]
pub struct PTTO27_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO27_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO28_AW) -> Self {
        match variant {
            PTTO28_AW::_0 => false,
            PTTO28_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO28`"]
pub struct PTTO28_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO28_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO29_AW) -> Self {
        match variant {
            PTTO29_AW::_0 => false,
            PTTO29_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO29`"]
pub struct PTTO29_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO29_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO30_AW) -> Self {
        match variant {
            PTTO30_AW::_0 => false,
            PTTO30_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO30`"]
pub struct PTTO30_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO30_AW::_1)
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
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTTO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl From<PTTO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO31_AW) -> Self {
        match variant {
            PTTO31_AW::_0 => false,
            PTTO31_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTTO31`"]
pub struct PTTO31_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO31_AW::_1)
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
    #[doc = "Bit 0 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto0(&mut self) -> PTTO0_W {
        PTTO0_W { w: self }
    }
    #[doc = "Bit 1 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto1(&mut self) -> PTTO1_W {
        PTTO1_W { w: self }
    }
    #[doc = "Bit 2 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto2(&mut self) -> PTTO2_W {
        PTTO2_W { w: self }
    }
    #[doc = "Bit 3 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto3(&mut self) -> PTTO3_W {
        PTTO3_W { w: self }
    }
    #[doc = "Bit 4 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto4(&mut self) -> PTTO4_W {
        PTTO4_W { w: self }
    }
    #[doc = "Bit 5 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto5(&mut self) -> PTTO5_W {
        PTTO5_W { w: self }
    }
    #[doc = "Bit 6 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto6(&mut self) -> PTTO6_W {
        PTTO6_W { w: self }
    }
    #[doc = "Bit 7 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto7(&mut self) -> PTTO7_W {
        PTTO7_W { w: self }
    }
    #[doc = "Bit 8 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto8(&mut self) -> PTTO8_W {
        PTTO8_W { w: self }
    }
    #[doc = "Bit 9 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto9(&mut self) -> PTTO9_W {
        PTTO9_W { w: self }
    }
    #[doc = "Bit 10 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto10(&mut self) -> PTTO10_W {
        PTTO10_W { w: self }
    }
    #[doc = "Bit 11 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto11(&mut self) -> PTTO11_W {
        PTTO11_W { w: self }
    }
    #[doc = "Bit 12 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto12(&mut self) -> PTTO12_W {
        PTTO12_W { w: self }
    }
    #[doc = "Bit 13 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto13(&mut self) -> PTTO13_W {
        PTTO13_W { w: self }
    }
    #[doc = "Bit 14 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto14(&mut self) -> PTTO14_W {
        PTTO14_W { w: self }
    }
    #[doc = "Bit 15 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto15(&mut self) -> PTTO15_W {
        PTTO15_W { w: self }
    }
    #[doc = "Bit 16 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto16(&mut self) -> PTTO16_W {
        PTTO16_W { w: self }
    }
    #[doc = "Bit 17 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto17(&mut self) -> PTTO17_W {
        PTTO17_W { w: self }
    }
    #[doc = "Bit 18 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto18(&mut self) -> PTTO18_W {
        PTTO18_W { w: self }
    }
    #[doc = "Bit 19 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto19(&mut self) -> PTTO19_W {
        PTTO19_W { w: self }
    }
    #[doc = "Bit 20 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto20(&mut self) -> PTTO20_W {
        PTTO20_W { w: self }
    }
    #[doc = "Bit 21 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto21(&mut self) -> PTTO21_W {
        PTTO21_W { w: self }
    }
    #[doc = "Bit 22 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto22(&mut self) -> PTTO22_W {
        PTTO22_W { w: self }
    }
    #[doc = "Bit 23 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto23(&mut self) -> PTTO23_W {
        PTTO23_W { w: self }
    }
    #[doc = "Bit 24 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto24(&mut self) -> PTTO24_W {
        PTTO24_W { w: self }
    }
    #[doc = "Bit 25 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto25(&mut self) -> PTTO25_W {
        PTTO25_W { w: self }
    }
    #[doc = "Bit 26 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto26(&mut self) -> PTTO26_W {
        PTTO26_W { w: self }
    }
    #[doc = "Bit 27 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto27(&mut self) -> PTTO27_W {
        PTTO27_W { w: self }
    }
    #[doc = "Bit 28 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto28(&mut self) -> PTTO28_W {
        PTTO28_W { w: self }
    }
    #[doc = "Bit 29 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto29(&mut self) -> PTTO29_W {
        PTTO29_W { w: self }
    }
    #[doc = "Bit 30 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto30(&mut self) -> PTTO30_W {
        PTTO30_W { w: self }
    }
    #[doc = "Bit 31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto31(&mut self) -> PTTO31_W {
        PTTO31_W { w: self }
    }
}
