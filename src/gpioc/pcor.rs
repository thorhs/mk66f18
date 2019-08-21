#[doc = "Writer for register PCOR"]
pub type W = crate::W<u32, super::PCOR>;
#[doc = "Register PCOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO0_AW) -> Self {
        match variant {
            PTCO0_AW::_0 => false,
            PTCO0_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO0`"]
pub struct PTCO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO0_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO1_AW) -> Self {
        match variant {
            PTCO1_AW::_0 => false,
            PTCO1_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO1`"]
pub struct PTCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO1_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO2_AW) -> Self {
        match variant {
            PTCO2_AW::_0 => false,
            PTCO2_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO2`"]
pub struct PTCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO2_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO3_AW) -> Self {
        match variant {
            PTCO3_AW::_0 => false,
            PTCO3_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO3`"]
pub struct PTCO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO3_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO4_AW) -> Self {
        match variant {
            PTCO4_AW::_0 => false,
            PTCO4_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO4`"]
pub struct PTCO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO4_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO5_AW) -> Self {
        match variant {
            PTCO5_AW::_0 => false,
            PTCO5_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO5`"]
pub struct PTCO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO5_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO6_AW) -> Self {
        match variant {
            PTCO6_AW::_0 => false,
            PTCO6_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO6`"]
pub struct PTCO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO6_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO7_AW) -> Self {
        match variant {
            PTCO7_AW::_0 => false,
            PTCO7_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO7`"]
pub struct PTCO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO7_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO8_AW) -> Self {
        match variant {
            PTCO8_AW::_0 => false,
            PTCO8_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO8`"]
pub struct PTCO8_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO8_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO9_AW) -> Self {
        match variant {
            PTCO9_AW::_0 => false,
            PTCO9_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO9`"]
pub struct PTCO9_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO9_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO10_AW) -> Self {
        match variant {
            PTCO10_AW::_0 => false,
            PTCO10_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO10`"]
pub struct PTCO10_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO10_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO11_AW) -> Self {
        match variant {
            PTCO11_AW::_0 => false,
            PTCO11_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO11`"]
pub struct PTCO11_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO11_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO12_AW) -> Self {
        match variant {
            PTCO12_AW::_0 => false,
            PTCO12_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO12`"]
pub struct PTCO12_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO12_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO13_AW) -> Self {
        match variant {
            PTCO13_AW::_0 => false,
            PTCO13_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO13`"]
pub struct PTCO13_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO13_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO14_AW) -> Self {
        match variant {
            PTCO14_AW::_0 => false,
            PTCO14_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO14`"]
pub struct PTCO14_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO14_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO15_AW) -> Self {
        match variant {
            PTCO15_AW::_0 => false,
            PTCO15_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO15`"]
pub struct PTCO15_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO15_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO16_AW) -> Self {
        match variant {
            PTCO16_AW::_0 => false,
            PTCO16_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO16`"]
pub struct PTCO16_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO16_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO17_AW) -> Self {
        match variant {
            PTCO17_AW::_0 => false,
            PTCO17_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO17`"]
pub struct PTCO17_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO17_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO17_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO18_AW) -> Self {
        match variant {
            PTCO18_AW::_0 => false,
            PTCO18_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO18`"]
pub struct PTCO18_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO18_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO19_AW) -> Self {
        match variant {
            PTCO19_AW::_0 => false,
            PTCO19_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO19`"]
pub struct PTCO19_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO19_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO20_AW) -> Self {
        match variant {
            PTCO20_AW::_0 => false,
            PTCO20_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO20`"]
pub struct PTCO20_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO20_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO21_AW) -> Self {
        match variant {
            PTCO21_AW::_0 => false,
            PTCO21_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO21`"]
pub struct PTCO21_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO21_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO22_AW) -> Self {
        match variant {
            PTCO22_AW::_0 => false,
            PTCO22_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO22`"]
pub struct PTCO22_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO22_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO23_AW) -> Self {
        match variant {
            PTCO23_AW::_0 => false,
            PTCO23_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO23`"]
pub struct PTCO23_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO23_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO23_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO24_AW) -> Self {
        match variant {
            PTCO24_AW::_0 => false,
            PTCO24_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO24`"]
pub struct PTCO24_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO24_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO24_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO25_AW) -> Self {
        match variant {
            PTCO25_AW::_0 => false,
            PTCO25_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO25`"]
pub struct PTCO25_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO25_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO25_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO26_AW) -> Self {
        match variant {
            PTCO26_AW::_0 => false,
            PTCO26_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO26`"]
pub struct PTCO26_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO26_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO26_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO27_AW) -> Self {
        match variant {
            PTCO27_AW::_0 => false,
            PTCO27_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO27`"]
pub struct PTCO27_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO27_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO27_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO28_AW) -> Self {
        match variant {
            PTCO28_AW::_0 => false,
            PTCO28_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO28`"]
pub struct PTCO28_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO28_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO28_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO29_AW) -> Self {
        match variant {
            PTCO29_AW::_0 => false,
            PTCO29_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO29`"]
pub struct PTCO29_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO29_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO29_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO30_AW) -> Self {
        match variant {
            PTCO30_AW::_0 => false,
            PTCO30_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO30`"]
pub struct PTCO30_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO30_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO30_AW::_1)
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
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl From<PTCO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO31_AW) -> Self {
        match variant {
            PTCO31_AW::_0 => false,
            PTCO31_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `PTCO31`"]
pub struct PTCO31_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCO31_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO31_AW::_1)
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
    #[doc = "Bit 0 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco0(&mut self) -> PTCO0_W {
        PTCO0_W { w: self }
    }
    #[doc = "Bit 1 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco1(&mut self) -> PTCO1_W {
        PTCO1_W { w: self }
    }
    #[doc = "Bit 2 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco2(&mut self) -> PTCO2_W {
        PTCO2_W { w: self }
    }
    #[doc = "Bit 3 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco3(&mut self) -> PTCO3_W {
        PTCO3_W { w: self }
    }
    #[doc = "Bit 4 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco4(&mut self) -> PTCO4_W {
        PTCO4_W { w: self }
    }
    #[doc = "Bit 5 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco5(&mut self) -> PTCO5_W {
        PTCO5_W { w: self }
    }
    #[doc = "Bit 6 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco6(&mut self) -> PTCO6_W {
        PTCO6_W { w: self }
    }
    #[doc = "Bit 7 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco7(&mut self) -> PTCO7_W {
        PTCO7_W { w: self }
    }
    #[doc = "Bit 8 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco8(&mut self) -> PTCO8_W {
        PTCO8_W { w: self }
    }
    #[doc = "Bit 9 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco9(&mut self) -> PTCO9_W {
        PTCO9_W { w: self }
    }
    #[doc = "Bit 10 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco10(&mut self) -> PTCO10_W {
        PTCO10_W { w: self }
    }
    #[doc = "Bit 11 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco11(&mut self) -> PTCO11_W {
        PTCO11_W { w: self }
    }
    #[doc = "Bit 12 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco12(&mut self) -> PTCO12_W {
        PTCO12_W { w: self }
    }
    #[doc = "Bit 13 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco13(&mut self) -> PTCO13_W {
        PTCO13_W { w: self }
    }
    #[doc = "Bit 14 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco14(&mut self) -> PTCO14_W {
        PTCO14_W { w: self }
    }
    #[doc = "Bit 15 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco15(&mut self) -> PTCO15_W {
        PTCO15_W { w: self }
    }
    #[doc = "Bit 16 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco16(&mut self) -> PTCO16_W {
        PTCO16_W { w: self }
    }
    #[doc = "Bit 17 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco17(&mut self) -> PTCO17_W {
        PTCO17_W { w: self }
    }
    #[doc = "Bit 18 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco18(&mut self) -> PTCO18_W {
        PTCO18_W { w: self }
    }
    #[doc = "Bit 19 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco19(&mut self) -> PTCO19_W {
        PTCO19_W { w: self }
    }
    #[doc = "Bit 20 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco20(&mut self) -> PTCO20_W {
        PTCO20_W { w: self }
    }
    #[doc = "Bit 21 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco21(&mut self) -> PTCO21_W {
        PTCO21_W { w: self }
    }
    #[doc = "Bit 22 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco22(&mut self) -> PTCO22_W {
        PTCO22_W { w: self }
    }
    #[doc = "Bit 23 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco23(&mut self) -> PTCO23_W {
        PTCO23_W { w: self }
    }
    #[doc = "Bit 24 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco24(&mut self) -> PTCO24_W {
        PTCO24_W { w: self }
    }
    #[doc = "Bit 25 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco25(&mut self) -> PTCO25_W {
        PTCO25_W { w: self }
    }
    #[doc = "Bit 26 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco26(&mut self) -> PTCO26_W {
        PTCO26_W { w: self }
    }
    #[doc = "Bit 27 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco27(&mut self) -> PTCO27_W {
        PTCO27_W { w: self }
    }
    #[doc = "Bit 28 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco28(&mut self) -> PTCO28_W {
        PTCO28_W { w: self }
    }
    #[doc = "Bit 29 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco29(&mut self) -> PTCO29_W {
        PTCO29_W { w: self }
    }
    #[doc = "Bit 30 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco30(&mut self) -> PTCO30_W {
        PTCO30_W { w: self }
    }
    #[doc = "Bit 31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco31(&mut self) -> PTCO31_W {
        PTCO31_W { w: self }
    }
}
