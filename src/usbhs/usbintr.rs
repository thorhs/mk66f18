#[doc = "Reader of register USBINTR"]
pub type R = crate::R<u32, super::USBINTR>;
#[doc = "Writer for register USBINTR"]
pub type W = crate::W<u32, super::USBINTR>;
#[doc = "Register USBINTR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBINTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        match variant {
            UE_A::_0 => false,
            UE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UE`"]
pub type UE_R = crate::R<bool, UE_A>;
impl UE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::_0,
            true => UE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UE_A::_1
    }
}
#[doc = "Write proxy for field `UE`"]
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UE_A::_1)
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
#[doc = "USB Error interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<UEE_A> for bool {
    #[inline(always)]
    fn from(variant: UEE_A) -> Self {
        match variant {
            UEE_A::_0 => false,
            UEE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UEE`"]
pub type UEE_R = crate::R<bool, UEE_A>;
impl UEE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UEE_A {
        match self.bits {
            false => UEE_A::_0,
            true => UEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UEE_A::_1
    }
}
#[doc = "Write proxy for field `UEE`"]
pub struct UEE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UEE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UEE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UEE_A::_1)
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
#[doc = "Port Change detect Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        match variant {
            PCE_A::_0 => false,
            PCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PCE`"]
pub type PCE_R = crate::R<bool, PCE_A>;
impl PCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::_0,
            true => PCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCE_A::_1
    }
}
#[doc = "Write proxy for field `PCE`"]
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCE_A::_1)
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
#[doc = "Frame list Rollover Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<FRE_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_A) -> Self {
        match variant {
            FRE_A::_0 => false,
            FRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRE`"]
pub type FRE_R = crate::R<bool, FRE_A>;
impl FRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_A {
        match self.bits {
            false => FRE_A::_0,
            true => FRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRE_A::_1
    }
}
#[doc = "Write proxy for field `FRE`"]
pub struct FRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRE_A::_1)
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
#[doc = "System Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<SEE_A> for bool {
    #[inline(always)]
    fn from(variant: SEE_A) -> Self {
        match variant {
            SEE_A::_0 => false,
            SEE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SEE`"]
pub type SEE_R = crate::R<bool, SEE_A>;
impl SEE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEE_A {
        match self.bits {
            false => SEE_A::_0,
            true => SEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEE_A::_1
    }
}
#[doc = "Write proxy for field `SEE`"]
pub struct SEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEE_A::_1)
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
#[doc = "Interrupt on Async advance Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<AAE_A> for bool {
    #[inline(always)]
    fn from(variant: AAE_A) -> Self {
        match variant {
            AAE_A::_0 => false,
            AAE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AAE`"]
pub type AAE_R = crate::R<bool, AAE_A>;
impl AAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAE_A {
        match self.bits {
            false => AAE_A::_0,
            true => AAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAE_A::_1
    }
}
#[doc = "Write proxy for field `AAE`"]
pub struct AAE_W<'a> {
    w: &'a mut W,
}
impl<'a> AAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAE_A::_1)
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
#[doc = "USB-Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<URE_A> for bool {
    #[inline(always)]
    fn from(variant: URE_A) -> Self {
        match variant {
            URE_A::_0 => false,
            URE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `URE`"]
pub type URE_R = crate::R<bool, URE_A>;
impl URE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URE_A {
        match self.bits {
            false => URE_A::_0,
            true => URE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == URE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == URE_A::_1
    }
}
#[doc = "Write proxy for field `URE`"]
pub struct URE_W<'a> {
    w: &'a mut W,
}
impl<'a> URE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: URE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(URE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(URE_A::_1)
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
#[doc = "SOF-Received Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        match variant {
            SRE_A::_0 => false,
            SRE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SRE`"]
pub type SRE_R = crate::R<bool, SRE_A>;
impl SRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::_0,
            true => SRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRE_A::_1
    }
}
#[doc = "Write proxy for field `SRE`"]
pub struct SRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRE_A::_1)
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
#[doc = "Sleep (DC suspend) Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<SLE_A> for bool {
    #[inline(always)]
    fn from(variant: SLE_A) -> Self {
        match variant {
            SLE_A::_0 => false,
            SLE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLE`"]
pub type SLE_R = crate::R<bool, SLE_A>;
impl SLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLE_A {
        match self.bits {
            false => SLE_A::_0,
            true => SLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLE_A::_1
    }
}
#[doc = "Write proxy for field `SLE`"]
pub struct SLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLE_A::_1)
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
#[doc = "NAK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAKE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<NAKE_A> for bool {
    #[inline(always)]
    fn from(variant: NAKE_A) -> Self {
        match variant {
            NAKE_A::_0 => false,
            NAKE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NAKE`"]
pub type NAKE_R = crate::R<bool, NAKE_A>;
impl NAKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAKE_A {
        match self.bits {
            false => NAKE_A::_0,
            true => NAKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NAKE_A::_1
    }
}
#[doc = "Write proxy for field `NAKE`"]
pub struct NAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NAKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NAKE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NAKE_A::_1)
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
#[doc = "Reader of field `UAIE`"]
pub type UAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UAIE`"]
pub struct UAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UAIE_W<'a> {
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
#[doc = "Reader of field `UPIE`"]
pub type UPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPIE`"]
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
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
#[doc = "General purpose Timer 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE0_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<TIE0_A> for bool {
    #[inline(always)]
    fn from(variant: TIE0_A) -> Self {
        match variant {
            TIE0_A::_0 => false,
            TIE0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TIE0`"]
pub type TIE0_R = crate::R<bool, TIE0_A>;
impl TIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE0_A {
        match self.bits {
            false => TIE0_A::_0,
            true => TIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE0_A::_1
    }
}
#[doc = "Write proxy for field `TIE0`"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE0_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE0_A::_1)
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
#[doc = "General purpose Timer 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE1_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<TIE1_A> for bool {
    #[inline(always)]
    fn from(variant: TIE1_A) -> Self {
        match variant {
            TIE1_A::_0 => false,
            TIE1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TIE1`"]
pub type TIE1_R = crate::R<bool, TIE1_A>;
impl TIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE1_A {
        match self.bits {
            false => TIE1_A::_0,
            true => TIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE1_A::_1
    }
}
#[doc = "Write proxy for field `TIE1`"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE1_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE1_A::_1)
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
impl R {
    #[doc = "Bit 0 - USB interrupt Enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Error interrupt Enable"]
    #[inline(always)]
    pub fn uee(&self) -> UEE_R {
        UEE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Change detect Enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame list Rollover Enable"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System Error Enable"]
    #[inline(always)]
    pub fn see(&self) -> SEE_R {
        SEE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async advance Enable"]
    #[inline(always)]
    pub fn aae(&self) -> AAE_R {
        AAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB-Reset Enable"]
    #[inline(always)]
    pub fn ure(&self) -> URE_R {
        URE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SOF-Received Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep (DC suspend) Enable"]
    #[inline(always)]
    pub fn sle(&self) -> SLE_R {
        SLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK Interrupt Enable"]
    #[inline(always)]
    pub fn nake(&self) -> NAKE_R {
        NAKE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn uaie(&self) -> UAIE_R {
        UAIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB interrupt Enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    #[doc = "Bit 1 - USB Error interrupt Enable"]
    #[inline(always)]
    pub fn uee(&mut self) -> UEE_W {
        UEE_W { w: self }
    }
    #[doc = "Bit 2 - Port Change detect Enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    #[doc = "Bit 3 - Frame list Rollover Enable"]
    #[inline(always)]
    pub fn fre(&mut self) -> FRE_W {
        FRE_W { w: self }
    }
    #[doc = "Bit 4 - System Error Enable"]
    #[inline(always)]
    pub fn see(&mut self) -> SEE_W {
        SEE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt on Async advance Enable"]
    #[inline(always)]
    pub fn aae(&mut self) -> AAE_W {
        AAE_W { w: self }
    }
    #[doc = "Bit 6 - USB-Reset Enable"]
    #[inline(always)]
    pub fn ure(&mut self) -> URE_W {
        URE_W { w: self }
    }
    #[doc = "Bit 7 - SOF-Received Enable"]
    #[inline(always)]
    pub fn sre(&mut self) -> SRE_W {
        SRE_W { w: self }
    }
    #[doc = "Bit 8 - Sleep (DC suspend) Enable"]
    #[inline(always)]
    pub fn sle(&mut self) -> SLE_W {
        SLE_W { w: self }
    }
    #[doc = "Bit 16 - NAK Interrupt Enable"]
    #[inline(always)]
    pub fn nake(&mut self) -> NAKE_W {
        NAKE_W { w: self }
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn uaie(&mut self) -> UAIE_W {
        UAIE_W { w: self }
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
}
