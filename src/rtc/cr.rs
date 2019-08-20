#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    _1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        match variant {
            SWR_A::_0 => false,
            SWR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, SWR_A>;
impl SWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWR_A {
        match self.bits {
            false => SWR_A::_0,
            true => SWR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWR_A::_1
    }
}
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWR_A::_0)
    }
    #[doc = "Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWR_A::_1)
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
#[doc = "Wakeup Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPE_A {
    #[doc = "0: Wakeup pin is disabled."]
    _0,
    #[doc = "1: Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    _1,
}
impl From<WPE_A> for bool {
    #[inline(always)]
    fn from(variant: WPE_A) -> Self {
        match variant {
            WPE_A::_0 => false,
            WPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WPE`"]
pub type WPE_R = crate::R<bool, WPE_A>;
impl WPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPE_A {
        match self.bits {
            false => WPE_A::_0,
            true => WPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPE_A::_1
    }
}
#[doc = "Write proxy for field `WPE`"]
pub struct WPE_W<'a> {
    w: &'a mut W,
}
impl<'a> WPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPE_A::_0)
    }
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPE_A::_1)
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
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUP_A {
    #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
    _0,
    #[doc = "1: Non-supervisor mode write accesses are supported."]
    _1,
}
impl From<SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SUP_A) -> Self {
        match variant {
            SUP_A::_0 => false,
            SUP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SUP`"]
pub type SUP_R = crate::R<bool, SUP_A>;
impl SUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUP_A {
        match self.bits {
            false => SUP_A::_0,
            true => SUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUP_A::_1
    }
}
#[doc = "Write proxy for field `SUP`"]
pub struct SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUP_A::_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUP_A::_1)
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
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UM_A {
    #[doc = "0: Registers cannot be written when locked."]
    _0,
    #[doc = "1: Registers can be written when locked under limited conditions."]
    _1,
}
impl From<UM_A> for bool {
    #[inline(always)]
    fn from(variant: UM_A) -> Self {
        match variant {
            UM_A::_0 => false,
            UM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UM`"]
pub type UM_R = crate::R<bool, UM_A>;
impl UM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UM_A {
        match self.bits {
            false => UM_A::_0,
            true => UM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UM_A::_1
    }
}
#[doc = "Write proxy for field `UM`"]
pub struct UM_W<'a> {
    w: &'a mut W,
}
impl<'a> UM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UM_A::_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UM_A::_1)
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
#[doc = "Wakeup Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPS_A {
    #[doc = "0: Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    _0,
    #[doc = "1: Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    _1,
}
impl From<WPS_A> for bool {
    #[inline(always)]
    fn from(variant: WPS_A) -> Self {
        match variant {
            WPS_A::_0 => false,
            WPS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WPS`"]
pub type WPS_R = crate::R<bool, WPS_A>;
impl WPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPS_A {
        match self.bits {
            false => WPS_A::_0,
            true => WPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPS_A::_1
    }
}
#[doc = "Write proxy for field `WPS`"]
pub struct WPS_W<'a> {
    w: &'a mut W,
}
impl<'a> WPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPS_A::_0)
    }
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPS_A::_1)
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
#[doc = "Oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCE_A {
    #[doc = "0: 32.768 kHz oscillator is disabled."]
    _0,
    #[doc = "1: 32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    _1,
}
impl From<OSCE_A> for bool {
    #[inline(always)]
    fn from(variant: OSCE_A) -> Self {
        match variant {
            OSCE_A::_0 => false,
            OSCE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `OSCE`"]
pub type OSCE_R = crate::R<bool, OSCE_A>;
impl OSCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCE_A {
        match self.bits {
            false => OSCE_A::_0,
            true => OSCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSCE_A::_1
    }
}
#[doc = "Write proxy for field `OSCE`"]
pub struct OSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "32.768 kHz oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCE_A::_0)
    }
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCE_A::_1)
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
#[doc = "Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO_A {
    #[doc = "0: The 32 kHz clock is output to other peripherals."]
    _0,
    #[doc = "1: The 32 kHz clock is not output to other peripherals."]
    _1,
}
impl From<CLKO_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO_A) -> Self {
        match variant {
            CLKO_A::_0 => false,
            CLKO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CLKO`"]
pub type CLKO_R = crate::R<bool, CLKO_A>;
impl CLKO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO_A {
        match self.bits {
            false => CLKO_A::_0,
            true => CLKO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKO_A::_1
    }
}
#[doc = "Write proxy for field `CLKO`"]
pub struct CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKO_A::_0)
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKO_A::_1)
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
#[doc = "Oscillator 16pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16P_A {
    #[doc = "0: Disable the load."]
    _0,
    #[doc = "1: Enable the additional load."]
    _1,
}
impl From<SC16P_A> for bool {
    #[inline(always)]
    fn from(variant: SC16P_A) -> Self {
        match variant {
            SC16P_A::_0 => false,
            SC16P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC16P`"]
pub type SC16P_R = crate::R<bool, SC16P_A>;
impl SC16P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC16P_A {
        match self.bits {
            false => SC16P_A::_0,
            true => SC16P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC16P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC16P_A::_1
    }
}
#[doc = "Write proxy for field `SC16P`"]
pub struct SC16P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC16P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC16P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC16P_A::_1)
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
#[doc = "Oscillator 8pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC8P_A {
    #[doc = "0: Disable the load."]
    _0,
    #[doc = "1: Enable the additional load."]
    _1,
}
impl From<SC8P_A> for bool {
    #[inline(always)]
    fn from(variant: SC8P_A) -> Self {
        match variant {
            SC8P_A::_0 => false,
            SC8P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC8P`"]
pub type SC8P_R = crate::R<bool, SC8P_A>;
impl SC8P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC8P_A {
        match self.bits {
            false => SC8P_A::_0,
            true => SC8P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC8P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC8P_A::_1
    }
}
#[doc = "Write proxy for field `SC8P`"]
pub struct SC8P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC8P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC8P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC8P_A::_1)
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
#[doc = "Oscillator 4pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC4P_A {
    #[doc = "0: Disable the load."]
    _0,
    #[doc = "1: Enable the additional load."]
    _1,
}
impl From<SC4P_A> for bool {
    #[inline(always)]
    fn from(variant: SC4P_A) -> Self {
        match variant {
            SC4P_A::_0 => false,
            SC4P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC4P`"]
pub type SC4P_R = crate::R<bool, SC4P_A>;
impl SC4P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC4P_A {
        match self.bits {
            false => SC4P_A::_0,
            true => SC4P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC4P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC4P_A::_1
    }
}
#[doc = "Write proxy for field `SC4P`"]
pub struct SC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC4P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC4P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC4P_A::_1)
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
#[doc = "Oscillator 2pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC2P_A {
    #[doc = "0: Disable the load."]
    _0,
    #[doc = "1: Enable the additional load."]
    _1,
}
impl From<SC2P_A> for bool {
    #[inline(always)]
    fn from(variant: SC2P_A) -> Self {
        match variant {
            SC2P_A::_0 => false,
            SC2P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC2P`"]
pub type SC2P_R = crate::R<bool, SC2P_A>;
impl SC2P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC2P_A {
        match self.bits {
            false => SC2P_A::_0,
            true => SC2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC2P_A::_1
    }
}
#[doc = "Write proxy for field `SC2P`"]
pub struct SC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC2P_A::_1)
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
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    pub fn wpe(&self) -> WPE_R {
        WPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&self) -> SUP_R {
        SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&self) -> UM_R {
        UM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    pub fn wps(&self) -> WPS_R {
        WPS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    pub fn osce(&self) -> OSCE_R {
        OSCE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> SC16P_R {
        SC16P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> SC8P_R {
        SC8P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> SC4P_R {
        SC4P_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> SC2P_R {
        SC2P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    pub fn wpe(&mut self) -> WPE_W {
        WPE_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&mut self) -> SUP_W {
        SUP_W { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&mut self) -> UM_W {
        UM_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    pub fn wps(&mut self) -> WPS_W {
        WPS_W { w: self }
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    pub fn osce(&mut self) -> OSCE_W {
        OSCE_W { w: self }
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    pub fn sc16p(&mut self) -> SC16P_W {
        SC16P_W { w: self }
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    pub fn sc8p(&mut self) -> SC8P_W {
        SC8P_W { w: self }
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    pub fn sc4p(&mut self) -> SC4P_W {
        SC4P_W { w: self }
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    pub fn sc2p(&mut self) -> SC2P_W {
        SC2P_W { w: self }
    }
}
