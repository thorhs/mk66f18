#[doc = "Reader of register RGDAAC%s"]
pub type R = crate::R<u32, super::RGDAAC>;
#[doc = "Writer for register RGDAAC%s"]
pub type W = crate::W<u32, super::RGDAAC>;
#[doc = "Register RGDAAC%s `reset()`'s with value 0x0061_f7df"]
impl crate::ResetValue for super::RGDAAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0061_f7df
    }
}
#[doc = "Reader of field `M0UM`"]
pub type M0UM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M0UM`"]
pub struct M0UM_W<'a> {
    w: &'a mut W,
}
impl<'a> M0UM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `M0SM`"]
pub type M0SM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M0SM`"]
pub struct M0SM_W<'a> {
    w: &'a mut W,
}
impl<'a> M0SM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `M0PE`"]
pub type M0PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M0PE`"]
pub struct M0PE_W<'a> {
    w: &'a mut W,
}
impl<'a> M0PE_W<'a> {
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
#[doc = "Reader of field `M1UM`"]
pub type M1UM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M1UM`"]
pub struct M1UM_W<'a> {
    w: &'a mut W,
}
impl<'a> M1UM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `M1SM`"]
pub type M1SM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M1SM`"]
pub struct M1SM_W<'a> {
    w: &'a mut W,
}
impl<'a> M1SM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `M1PE`"]
pub type M1PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M1PE`"]
pub struct M1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> M1PE_W<'a> {
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
#[doc = "Reader of field `M2UM`"]
pub type M2UM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M2UM`"]
pub struct M2UM_W<'a> {
    w: &'a mut W,
}
impl<'a> M2UM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `M2SM`"]
pub type M2SM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M2SM`"]
pub struct M2SM_W<'a> {
    w: &'a mut W,
}
impl<'a> M2SM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `M2PE`"]
pub type M2PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M2PE`"]
pub struct M2PE_W<'a> {
    w: &'a mut W,
}
impl<'a> M2PE_W<'a> {
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
#[doc = "Bus Master 3 User Mode Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3UM_A {
    #[doc = "0: An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    _0,
    #[doc = "1: Allows the given access type to occur"]
    _1,
}
impl From<M3UM_A> for u8 {
    #[inline(always)]
    fn from(variant: M3UM_A) -> Self {
        match variant {
            M3UM_A::_0 => 0,
            M3UM_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `M3UM`"]
pub type M3UM_R = crate::R<u8, M3UM_A>;
impl M3UM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, M3UM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(M3UM_A::_0),
            1 => Val(M3UM_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3UM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3UM_A::_1
    }
}
#[doc = "Write proxy for field `M3UM`"]
pub struct M3UM_W<'a> {
    w: &'a mut W,
}
impl<'a> M3UM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3UM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3UM_A::_0)
    }
    #[doc = "Allows the given access type to occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3UM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Bus Master 3 Supervisor Mode Access Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3SM_A {
    #[doc = "0: r/w/x; read, write and execute allowed"]
    _00,
    #[doc = "1: r/x; read and execute allowed, but no write"]
    _01,
    #[doc = "2: r/w; read and write allowed, but no execute"]
    _10,
    #[doc = "3: Same as User mode defined in M3UM"]
    _11,
}
impl From<M3SM_A> for u8 {
    #[inline(always)]
    fn from(variant: M3SM_A) -> Self {
        match variant {
            M3SM_A::_00 => 0,
            M3SM_A::_01 => 1,
            M3SM_A::_10 => 2,
            M3SM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M3SM`"]
pub type M3SM_R = crate::R<u8, M3SM_A>;
impl M3SM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3SM_A {
        match self.bits {
            0 => M3SM_A::_00,
            1 => M3SM_A::_01,
            2 => M3SM_A::_10,
            3 => M3SM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M3SM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M3SM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M3SM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M3SM_A::_11
    }
}
#[doc = "Write proxy for field `M3SM`"]
pub struct M3SM_W<'a> {
    w: &'a mut W,
}
impl<'a> M3SM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3SM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "r/w/x; read, write and execute allowed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3SM_A::_00)
    }
    #[doc = "r/x; read and execute allowed, but no write"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3SM_A::_01)
    }
    #[doc = "r/w; read and write allowed, but no execute"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3SM_A::_10)
    }
    #[doc = "Same as User mode defined in M3UM"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3SM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Bus Master 3 Process Identifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3PE_A {
    #[doc = "0: Do not include the process identifier in the evaluation"]
    _0,
    #[doc = "1: Include the process identifier and mask (RGDn.RGDAAC) in the region hit evaluation"]
    _1,
}
impl From<M3PE_A> for bool {
    #[inline(always)]
    fn from(variant: M3PE_A) -> Self {
        match variant {
            M3PE_A::_0 => false,
            M3PE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M3PE`"]
pub type M3PE_R = crate::R<bool, M3PE_A>;
impl M3PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3PE_A {
        match self.bits {
            false => M3PE_A::_0,
            true => M3PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3PE_A::_1
    }
}
#[doc = "Write proxy for field `M3PE`"]
pub struct M3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> M3PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not include the process identifier in the evaluation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PE_A::_0)
    }
    #[doc = "Include the process identifier and mask (RGDn.RGDAAC) in the region hit evaluation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PE_A::_1)
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
#[doc = "Bus Master 4 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4WE_A {
    #[doc = "0: Bus master 4 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "1: Bus master 4 writes allowed"]
    _1,
}
impl From<M4WE_A> for bool {
    #[inline(always)]
    fn from(variant: M4WE_A) -> Self {
        match variant {
            M4WE_A::_0 => false,
            M4WE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M4WE`"]
pub type M4WE_R = crate::R<bool, M4WE_A>;
impl M4WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4WE_A {
        match self.bits {
            false => M4WE_A::_0,
            true => M4WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4WE_A::_1
    }
}
#[doc = "Write proxy for field `M4WE`"]
pub struct M4WE_W<'a> {
    w: &'a mut W,
}
impl<'a> M4WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 4 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4WE_A::_0)
    }
    #[doc = "Bus master 4 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4WE_A::_1)
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
#[doc = "Bus Master 4 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4RE_A {
    #[doc = "0: Bus master 4 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "1: Bus master 4 reads allowed"]
    _1,
}
impl From<M4RE_A> for bool {
    #[inline(always)]
    fn from(variant: M4RE_A) -> Self {
        match variant {
            M4RE_A::_0 => false,
            M4RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M4RE`"]
pub type M4RE_R = crate::R<bool, M4RE_A>;
impl M4RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4RE_A {
        match self.bits {
            false => M4RE_A::_0,
            true => M4RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4RE_A::_1
    }
}
#[doc = "Write proxy for field `M4RE`"]
pub struct M4RE_W<'a> {
    w: &'a mut W,
}
impl<'a> M4RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 4 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4RE_A::_0)
    }
    #[doc = "Bus master 4 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4RE_A::_1)
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
#[doc = "Bus Master 5 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5WE_A {
    #[doc = "0: Bus master 5 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "1: Bus master 5 writes allowed"]
    _1,
}
impl From<M5WE_A> for bool {
    #[inline(always)]
    fn from(variant: M5WE_A) -> Self {
        match variant {
            M5WE_A::_0 => false,
            M5WE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M5WE`"]
pub type M5WE_R = crate::R<bool, M5WE_A>;
impl M5WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5WE_A {
        match self.bits {
            false => M5WE_A::_0,
            true => M5WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5WE_A::_1
    }
}
#[doc = "Write proxy for field `M5WE`"]
pub struct M5WE_W<'a> {
    w: &'a mut W,
}
impl<'a> M5WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M5WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 5 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5WE_A::_0)
    }
    #[doc = "Bus master 5 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5WE_A::_1)
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
#[doc = "Bus Master 5 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5RE_A {
    #[doc = "0: Bus master 5 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "1: Bus master 5 reads allowed"]
    _1,
}
impl From<M5RE_A> for bool {
    #[inline(always)]
    fn from(variant: M5RE_A) -> Self {
        match variant {
            M5RE_A::_0 => false,
            M5RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M5RE`"]
pub type M5RE_R = crate::R<bool, M5RE_A>;
impl M5RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5RE_A {
        match self.bits {
            false => M5RE_A::_0,
            true => M5RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5RE_A::_1
    }
}
#[doc = "Write proxy for field `M5RE`"]
pub struct M5RE_W<'a> {
    w: &'a mut W,
}
impl<'a> M5RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M5RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 5 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5RE_A::_0)
    }
    #[doc = "Bus master 5 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5RE_A::_1)
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
#[doc = "Bus Master 6 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6WE_A {
    #[doc = "0: Bus master 6 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "1: Bus master 6 writes allowed"]
    _1,
}
impl From<M6WE_A> for bool {
    #[inline(always)]
    fn from(variant: M6WE_A) -> Self {
        match variant {
            M6WE_A::_0 => false,
            M6WE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M6WE`"]
pub type M6WE_R = crate::R<bool, M6WE_A>;
impl M6WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6WE_A {
        match self.bits {
            false => M6WE_A::_0,
            true => M6WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6WE_A::_1
    }
}
#[doc = "Write proxy for field `M6WE`"]
pub struct M6WE_W<'a> {
    w: &'a mut W,
}
impl<'a> M6WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M6WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 6 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6WE_A::_0)
    }
    #[doc = "Bus master 6 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6WE_A::_1)
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
#[doc = "Bus Master 6 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6RE_A {
    #[doc = "0: Bus master 6 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "1: Bus master 6 reads allowed"]
    _1,
}
impl From<M6RE_A> for bool {
    #[inline(always)]
    fn from(variant: M6RE_A) -> Self {
        match variant {
            M6RE_A::_0 => false,
            M6RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M6RE`"]
pub type M6RE_R = crate::R<bool, M6RE_A>;
impl M6RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6RE_A {
        match self.bits {
            false => M6RE_A::_0,
            true => M6RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6RE_A::_1
    }
}
#[doc = "Write proxy for field `M6RE`"]
pub struct M6RE_W<'a> {
    w: &'a mut W,
}
impl<'a> M6RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M6RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 6 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6RE_A::_0)
    }
    #[doc = "Bus master 6 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6RE_A::_1)
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
#[doc = "Bus Master 7 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7WE_A {
    #[doc = "0: Bus master 7 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "1: Bus master 7 writes allowed"]
    _1,
}
impl From<M7WE_A> for bool {
    #[inline(always)]
    fn from(variant: M7WE_A) -> Self {
        match variant {
            M7WE_A::_0 => false,
            M7WE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M7WE`"]
pub type M7WE_R = crate::R<bool, M7WE_A>;
impl M7WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7WE_A {
        match self.bits {
            false => M7WE_A::_0,
            true => M7WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7WE_A::_1
    }
}
#[doc = "Write proxy for field `M7WE`"]
pub struct M7WE_W<'a> {
    w: &'a mut W,
}
impl<'a> M7WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 7 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7WE_A::_0)
    }
    #[doc = "Bus master 7 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7WE_A::_1)
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
#[doc = "Bus Master 7 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7RE_A {
    #[doc = "0: Bus master 7 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "1: Bus master 7 reads allowed"]
    _1,
}
impl From<M7RE_A> for bool {
    #[inline(always)]
    fn from(variant: M7RE_A) -> Self {
        match variant {
            M7RE_A::_0 => false,
            M7RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M7RE`"]
pub type M7RE_R = crate::R<bool, M7RE_A>;
impl M7RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7RE_A {
        match self.bits {
            false => M7RE_A::_0,
            true => M7RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7RE_A::_1
    }
}
#[doc = "Write proxy for field `M7RE`"]
pub struct M7RE_W<'a> {
    w: &'a mut W,
}
impl<'a> M7RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus master 7 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7RE_A::_0)
    }
    #[doc = "Bus master 7 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7RE_A::_1)
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
impl R {
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline(always)]
    pub fn m0um(&self) -> M0UM_R {
        M0UM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m0sm(&self) -> M0SM_R {
        M0SM_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier Enable"]
    #[inline(always)]
    pub fn m0pe(&self) -> M0PE_R {
        M0PE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline(always)]
    pub fn m1um(&self) -> M1UM_R {
        M1UM_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m1sm(&self) -> M1SM_R {
        M1SM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier Enable"]
    #[inline(always)]
    pub fn m1pe(&self) -> M1PE_R {
        M1PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access Control"]
    #[inline(always)]
    pub fn m2um(&self) -> M2UM_R {
        M2UM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m2sm(&self) -> M2SM_R {
        M2SM_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline(always)]
    pub fn m2pe(&self) -> M2PE_R {
        M2PE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline(always)]
    pub fn m3um(&self) -> M3UM_R {
        M3UM_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m3sm(&self) -> M3SM_R {
        M3SM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline(always)]
    pub fn m3pe(&self) -> M3PE_R {
        M3PE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline(always)]
    pub fn m4we(&self) -> M4WE_R {
        M4WE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline(always)]
    pub fn m4re(&self) -> M4RE_R {
        M4RE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline(always)]
    pub fn m5we(&self) -> M5WE_R {
        M5WE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline(always)]
    pub fn m5re(&self) -> M5RE_R {
        M5RE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline(always)]
    pub fn m6we(&self) -> M6WE_R {
        M6WE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline(always)]
    pub fn m6re(&self) -> M6RE_R {
        M6RE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline(always)]
    pub fn m7we(&self) -> M7WE_R {
        M7WE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline(always)]
    pub fn m7re(&self) -> M7RE_R {
        M7RE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline(always)]
    pub fn m0um(&mut self) -> M0UM_W {
        M0UM_W { w: self }
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m0sm(&mut self) -> M0SM_W {
        M0SM_W { w: self }
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier Enable"]
    #[inline(always)]
    pub fn m0pe(&mut self) -> M0PE_W {
        M0PE_W { w: self }
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline(always)]
    pub fn m1um(&mut self) -> M1UM_W {
        M1UM_W { w: self }
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m1sm(&mut self) -> M1SM_W {
        M1SM_W { w: self }
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier Enable"]
    #[inline(always)]
    pub fn m1pe(&mut self) -> M1PE_W {
        M1PE_W { w: self }
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access Control"]
    #[inline(always)]
    pub fn m2um(&mut self) -> M2UM_W {
        M2UM_W { w: self }
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m2sm(&mut self) -> M2SM_W {
        M2SM_W { w: self }
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline(always)]
    pub fn m2pe(&mut self) -> M2PE_W {
        M2PE_W { w: self }
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline(always)]
    pub fn m3um(&mut self) -> M3UM_W {
        M3UM_W { w: self }
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m3sm(&mut self) -> M3SM_W {
        M3SM_W { w: self }
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline(always)]
    pub fn m3pe(&mut self) -> M3PE_W {
        M3PE_W { w: self }
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline(always)]
    pub fn m4we(&mut self) -> M4WE_W {
        M4WE_W { w: self }
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline(always)]
    pub fn m4re(&mut self) -> M4RE_W {
        M4RE_W { w: self }
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline(always)]
    pub fn m5we(&mut self) -> M5WE_W {
        M5WE_W { w: self }
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline(always)]
    pub fn m5re(&mut self) -> M5RE_W {
        M5RE_W { w: self }
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline(always)]
    pub fn m6we(&mut self) -> M6WE_W {
        M6WE_W { w: self }
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline(always)]
    pub fn m6re(&mut self) -> M6RE_W {
        M6RE_W { w: self }
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline(always)]
    pub fn m7we(&mut self) -> M7WE_W {
        M7WE_W { w: self }
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline(always)]
    pub fn m7re(&mut self) -> M7RE_W {
        M7RE_W { w: self }
    }
}
