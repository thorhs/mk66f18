#[doc = "Reader of register USBSTS"]
pub type R = crate::R<u32, super::USBSTS>;
#[doc = "Writer for register USBSTS"]
pub type W = crate::W<u32, super::USBSTS>;
#[doc = "Register USBSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::USBSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UI`"]
pub type UI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UI`"]
pub struct UI_W<'a> {
    w: &'a mut W,
}
impl<'a> UI_W<'a> {
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
#[doc = "USB Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEI_A {
    #[doc = "0: No error"]
    _0,
    #[doc = "1: Error detected"]
    _1,
}
impl From<UEI_A> for bool {
    #[inline(always)]
    fn from(variant: UEI_A) -> Self {
        match variant {
            UEI_A::_0 => false,
            UEI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UEI`"]
pub type UEI_R = crate::R<bool, UEI_A>;
impl UEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UEI_A {
        match self.bits {
            false => UEI_A::_0,
            true => UEI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UEI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UEI_A::_1
    }
}
#[doc = "Write proxy for field `UEI`"]
pub struct UEI_W<'a> {
    w: &'a mut W,
}
impl<'a> UEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UEI_A::_0)
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UEI_A::_1)
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
#[doc = "Reader of field `PCI`"]
pub type PCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI`"]
pub struct PCI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI_W<'a> {
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
#[doc = "Reader of field `FRI`"]
pub type FRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRI`"]
pub struct FRI_W<'a> {
    w: &'a mut W,
}
impl<'a> FRI_W<'a> {
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
#[doc = "System Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEI_A {
    #[doc = "0: Normal operation"]
    _0,
    #[doc = "1: Error"]
    _1,
}
impl From<SEI_A> for bool {
    #[inline(always)]
    fn from(variant: SEI_A) -> Self {
        match variant {
            SEI_A::_0 => false,
            SEI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SEI`"]
pub type SEI_R = crate::R<bool, SEI_A>;
impl SEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEI_A {
        match self.bits {
            false => SEI_A::_0,
            true => SEI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEI_A::_1
    }
}
#[doc = "Write proxy for field `SEI`"]
pub struct SEI_W<'a> {
    w: &'a mut W,
}
impl<'a> SEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEI_A::_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEI_A::_1)
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
#[doc = "Interrupt on Async Advance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAI_A {
    #[doc = "0: No async advance interrupt"]
    _0,
    #[doc = "1: Async advance interrupt"]
    _1,
}
impl From<AAI_A> for bool {
    #[inline(always)]
    fn from(variant: AAI_A) -> Self {
        match variant {
            AAI_A::_0 => false,
            AAI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AAI`"]
pub type AAI_R = crate::R<bool, AAI_A>;
impl AAI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAI_A {
        match self.bits {
            false => AAI_A::_0,
            true => AAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAI_A::_1
    }
}
#[doc = "Write proxy for field `AAI`"]
pub struct AAI_W<'a> {
    w: &'a mut W,
}
impl<'a> AAI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AAI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No async advance interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAI_A::_0)
    }
    #[doc = "Async advance interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAI_A::_1)
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
#[doc = "USB Reset received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URI_A {
    #[doc = "0: No reset received"]
    _0,
    #[doc = "1: Reset received"]
    _1,
}
impl From<URI_A> for bool {
    #[inline(always)]
    fn from(variant: URI_A) -> Self {
        match variant {
            URI_A::_0 => false,
            URI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `URI`"]
pub type URI_R = crate::R<bool, URI_A>;
impl URI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URI_A {
        match self.bits {
            false => URI_A::_0,
            true => URI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == URI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == URI_A::_1
    }
}
#[doc = "Write proxy for field `URI`"]
pub struct URI_W<'a> {
    w: &'a mut W,
}
impl<'a> URI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: URI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(URI_A::_0)
    }
    #[doc = "Reset received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(URI_A::_1)
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
#[doc = "Reader of field `SRI`"]
pub type SRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRI`"]
pub struct SRI_W<'a> {
    w: &'a mut W,
}
impl<'a> SRI_W<'a> {
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
#[doc = "Device-controller suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLI_A {
    #[doc = "0: Active"]
    _0,
    #[doc = "1: Suspended"]
    _1,
}
impl From<SLI_A> for bool {
    #[inline(always)]
    fn from(variant: SLI_A) -> Self {
        match variant {
            SLI_A::_0 => false,
            SLI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLI`"]
pub type SLI_R = crate::R<bool, SLI_A>;
impl SLI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLI_A {
        match self.bits {
            false => SLI_A::_0,
            true => SLI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLI_A::_1
    }
}
#[doc = "Write proxy for field `SLI`"]
pub struct SLI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLI_A::_0)
    }
    #[doc = "Suspended"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLI_A::_1)
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
#[doc = "Host Controller Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCH_A {
    #[doc = "0: Running"]
    _0,
    #[doc = "1: Halted"]
    _1,
}
impl From<HCH_A> for bool {
    #[inline(always)]
    fn from(variant: HCH_A) -> Self {
        match variant {
            HCH_A::_0 => false,
            HCH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HCH`"]
pub type HCH_R = crate::R<bool, HCH_A>;
impl HCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCH_A {
        match self.bits {
            false => HCH_A::_0,
            true => HCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCH_A::_1
    }
}
#[doc = "Reclamation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCL_A {
    #[doc = "0: Non-empty asynchronous schedule"]
    _0,
    #[doc = "1: Empty asynchronous schedule"]
    _1,
}
impl From<RCL_A> for bool {
    #[inline(always)]
    fn from(variant: RCL_A) -> Self {
        match variant {
            RCL_A::_0 => false,
            RCL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RCL`"]
pub type RCL_R = crate::R<bool, RCL_A>;
impl RCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCL_A {
        match self.bits {
            false => RCL_A::_0,
            true => RCL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCL_A::_1
    }
}
#[doc = "Periodic schedule Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::_0 => false,
            PS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<bool, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::_0,
            true => PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS_A::_1
    }
}
#[doc = "Asynchronous schedule Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AS_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<AS_A> for bool {
    #[inline(always)]
    fn from(variant: AS_A) -> Self {
        match variant {
            AS_A::_0 => false,
            AS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AS`"]
pub type AS_R = crate::R<bool, AS_A>;
impl AS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AS_A {
        match self.bits {
            false => AS_A::_0,
            true => AS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AS_A::_1
    }
}
#[doc = "Reader of field `NAKI`"]
pub type NAKI_R = crate::R<bool, bool>;
#[doc = "Reader of field `UAI`"]
pub type UAI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UAI`"]
pub struct UAI_W<'a> {
    w: &'a mut W,
}
impl<'a> UAI_W<'a> {
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
#[doc = "Reader of field `UPI`"]
pub type UPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPI`"]
pub struct UPI_W<'a> {
    w: &'a mut W,
}
impl<'a> UPI_W<'a> {
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
#[doc = "General purpose Timer 0 Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI0_A {
    #[doc = "0: No interrupt"]
    _0,
    #[doc = "1: Interrupt occurred"]
    _1,
}
impl From<TI0_A> for bool {
    #[inline(always)]
    fn from(variant: TI0_A) -> Self {
        match variant {
            TI0_A::_0 => false,
            TI0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TI0`"]
pub type TI0_R = crate::R<bool, TI0_A>;
impl TI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI0_A {
        match self.bits {
            false => TI0_A::_0,
            true => TI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TI0_A::_1
    }
}
#[doc = "Write proxy for field `TI0`"]
pub struct TI0_W<'a> {
    w: &'a mut W,
}
impl<'a> TI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TI0_A::_0)
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TI0_A::_1)
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
#[doc = "General purpose Timer 1 Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1_A {
    #[doc = "0: No interrupt"]
    _0,
    #[doc = "1: Interrupt occurred"]
    _1,
}
impl From<TI1_A> for bool {
    #[inline(always)]
    fn from(variant: TI1_A) -> Self {
        match variant {
            TI1_A::_0 => false,
            TI1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TI1`"]
pub type TI1_R = crate::R<bool, TI1_A>;
impl TI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1_A {
        match self.bits {
            false => TI1_A::_0,
            true => TI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TI1_A::_1
    }
}
#[doc = "Write proxy for field `TI1`"]
pub struct TI1_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TI1_A::_0)
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TI1_A::_1)
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
    #[doc = "Bit 0 - USB Interrupt (USBINT)"]
    #[inline(always)]
    pub fn ui(&self) -> UI_R {
        UI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn uei(&self) -> UEI_R {
        UEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Change detect"]
    #[inline(always)]
    pub fn pci(&self) -> PCI_R {
        PCI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame-list Rollover"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System Error"]
    #[inline(always)]
    pub fn sei(&self) -> SEI_R {
        SEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB Reset received"]
    #[inline(always)]
    pub fn uri(&self) -> URI_R {
        URI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SOF Received"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Device-controller suspend"]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Host Controller Halted"]
    #[inline(always)]
    pub fn hch(&self) -> HCH_R {
        HCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline(always)]
    pub fn rcl(&self) -> RCL_R {
        RCL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Periodic schedule Status"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Asynchronous schedule Status"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK Interrupt"]
    #[inline(always)]
    pub fn naki(&self) -> NAKI_R {
        NAKI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt"]
    #[inline(always)]
    pub fn uai(&self) -> UAI_R {
        UAI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt"]
    #[inline(always)]
    pub fn upi(&self) -> UPI_R {
        UPI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt"]
    #[inline(always)]
    pub fn ti0(&self) -> TI0_R {
        TI0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt"]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (USBINT)"]
    #[inline(always)]
    pub fn ui(&mut self) -> UI_W {
        UI_W { w: self }
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn uei(&mut self) -> UEI_W {
        UEI_W { w: self }
    }
    #[doc = "Bit 2 - Port Change detect"]
    #[inline(always)]
    pub fn pci(&mut self) -> PCI_W {
        PCI_W { w: self }
    }
    #[doc = "Bit 3 - Frame-list Rollover"]
    #[inline(always)]
    pub fn fri(&mut self) -> FRI_W {
        FRI_W { w: self }
    }
    #[doc = "Bit 4 - System Error"]
    #[inline(always)]
    pub fn sei(&mut self) -> SEI_W {
        SEI_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline(always)]
    pub fn aai(&mut self) -> AAI_W {
        AAI_W { w: self }
    }
    #[doc = "Bit 6 - USB Reset received"]
    #[inline(always)]
    pub fn uri(&mut self) -> URI_W {
        URI_W { w: self }
    }
    #[doc = "Bit 7 - SOF Received"]
    #[inline(always)]
    pub fn sri(&mut self) -> SRI_W {
        SRI_W { w: self }
    }
    #[doc = "Bit 8 - Device-controller suspend"]
    #[inline(always)]
    pub fn sli(&mut self) -> SLI_W {
        SLI_W { w: self }
    }
    #[doc = "Bit 18 - USB host Asynchronous Interrupt"]
    #[inline(always)]
    pub fn uai(&mut self) -> UAI_W {
        UAI_W { w: self }
    }
    #[doc = "Bit 19 - USB host Periodic Interrupt"]
    #[inline(always)]
    pub fn upi(&mut self) -> UPI_W {
        UPI_W { w: self }
    }
    #[doc = "Bit 24 - General purpose Timer 0 Interrupt"]
    #[inline(always)]
    pub fn ti0(&mut self) -> TI0_W {
        TI0_W { w: self }
    }
    #[doc = "Bit 25 - General purpose Timer 1 Interrupt"]
    #[inline(always)]
    pub fn ti1(&mut self) -> TI1_W {
        TI1_W { w: self }
    }
}
