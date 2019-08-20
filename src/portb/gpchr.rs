#[doc = "Writer for register GPCHR"]
pub type W = crate::W<u32, super::GPCHR>;
#[doc = "Register GPCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPWD`"]
pub struct GPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE0_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE0_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE0_AW) -> Self {
        match variant {
            GPWE0_AW::_0 => false,
            GPWE0_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE0`"]
pub struct GPWE0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE0_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE0_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE1_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE1_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE1_AW) -> Self {
        match variant {
            GPWE1_AW::_0 => false,
            GPWE1_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE1`"]
pub struct GPWE1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE1_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE1_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE2_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE2_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE2_AW) -> Self {
        match variant {
            GPWE2_AW::_0 => false,
            GPWE2_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE2`"]
pub struct GPWE2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE2_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE2_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE3_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE3_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE3_AW) -> Self {
        match variant {
            GPWE3_AW::_0 => false,
            GPWE3_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE3`"]
pub struct GPWE3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE3_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE3_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE4_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE4_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE4_AW) -> Self {
        match variant {
            GPWE4_AW::_0 => false,
            GPWE4_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE4`"]
pub struct GPWE4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE4_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE4_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE5_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE5_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE5_AW) -> Self {
        match variant {
            GPWE5_AW::_0 => false,
            GPWE5_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE5`"]
pub struct GPWE5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE5_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE5_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE6_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE6_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE6_AW) -> Self {
        match variant {
            GPWE6_AW::_0 => false,
            GPWE6_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE6`"]
pub struct GPWE6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE6_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE6_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE7_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE7_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE7_AW) -> Self {
        match variant {
            GPWE7_AW::_0 => false,
            GPWE7_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE7`"]
pub struct GPWE7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE7_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE7_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE8_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE8_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE8_AW) -> Self {
        match variant {
            GPWE8_AW::_0 => false,
            GPWE8_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE8`"]
pub struct GPWE8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE8_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE8_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE9_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE9_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE9_AW) -> Self {
        match variant {
            GPWE9_AW::_0 => false,
            GPWE9_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE9`"]
pub struct GPWE9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE9_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE9_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE10_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE10_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE10_AW) -> Self {
        match variant {
            GPWE10_AW::_0 => false,
            GPWE10_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE10`"]
pub struct GPWE10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE10_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE10_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE11_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE11_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE11_AW) -> Self {
        match variant {
            GPWE11_AW::_0 => false,
            GPWE11_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE11`"]
pub struct GPWE11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE11_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE11_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE12_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE12_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE12_AW) -> Self {
        match variant {
            GPWE12_AW::_0 => false,
            GPWE12_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE12`"]
pub struct GPWE12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE12_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE12_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE13_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE13_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE13_AW) -> Self {
        match variant {
            GPWE13_AW::_0 => false,
            GPWE13_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE13`"]
pub struct GPWE13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE13_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE13_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE14_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE14_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE14_AW) -> Self {
        match variant {
            GPWE14_AW::_0 => false,
            GPWE14_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE14`"]
pub struct GPWE14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE14_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE14_AW::_1)
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
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPWE15_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl From<GPWE15_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE15_AW) -> Self {
        match variant {
            GPWE15_AW::_0 => false,
            GPWE15_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `GPWE15`"]
pub struct GPWE15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE15_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE15_AW::_1)
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
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    pub fn gpwd(&mut self) -> GPWD_W {
        GPWD_W { w: self }
    }
    #[doc = "Bit 16 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe0(&mut self) -> GPWE0_W {
        GPWE0_W { w: self }
    }
    #[doc = "Bit 17 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe1(&mut self) -> GPWE1_W {
        GPWE1_W { w: self }
    }
    #[doc = "Bit 18 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe2(&mut self) -> GPWE2_W {
        GPWE2_W { w: self }
    }
    #[doc = "Bit 19 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe3(&mut self) -> GPWE3_W {
        GPWE3_W { w: self }
    }
    #[doc = "Bit 20 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe4(&mut self) -> GPWE4_W {
        GPWE4_W { w: self }
    }
    #[doc = "Bit 21 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe5(&mut self) -> GPWE5_W {
        GPWE5_W { w: self }
    }
    #[doc = "Bit 22 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe6(&mut self) -> GPWE6_W {
        GPWE6_W { w: self }
    }
    #[doc = "Bit 23 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe7(&mut self) -> GPWE7_W {
        GPWE7_W { w: self }
    }
    #[doc = "Bit 24 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe8(&mut self) -> GPWE8_W {
        GPWE8_W { w: self }
    }
    #[doc = "Bit 25 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe9(&mut self) -> GPWE9_W {
        GPWE9_W { w: self }
    }
    #[doc = "Bit 26 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe10(&mut self) -> GPWE10_W {
        GPWE10_W { w: self }
    }
    #[doc = "Bit 27 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe11(&mut self) -> GPWE11_W {
        GPWE11_W { w: self }
    }
    #[doc = "Bit 28 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe12(&mut self) -> GPWE12_W {
        GPWE12_W { w: self }
    }
    #[doc = "Bit 29 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe13(&mut self) -> GPWE13_W {
        GPWE13_W { w: self }
    }
    #[doc = "Bit 30 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe14(&mut self) -> GPWE14_W {
        GPWE14_W { w: self }
    }
    #[doc = "Bit 31 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe15(&mut self) -> GPWE15_W {
        GPWE15_W { w: self }
    }
}
