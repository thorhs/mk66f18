#[doc = "Reader of register SSRS1"]
pub type R = crate::R<u8, super::SSRS1>;
#[doc = "Writer for register SSRS1"]
pub type W = crate::W<u8, super::SSRS1>;
#[doc = "Register SSRS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SSRS1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sticky JTAG Generated Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SJTAG_A {
    #[doc = "0: Reset not caused by JTAG"]
    _0,
    #[doc = "1: Reset caused by JTAG"]
    _1,
}
impl From<SJTAG_A> for bool {
    #[inline(always)]
    fn from(variant: SJTAG_A) -> Self {
        match variant {
            SJTAG_A::_0 => false,
            SJTAG_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SJTAG`"]
pub type SJTAG_R = crate::R<bool, SJTAG_A>;
impl SJTAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJTAG_A {
        match self.bits {
            false => SJTAG_A::_0,
            true => SJTAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SJTAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SJTAG_A::_1
    }
}
#[doc = "Write proxy for field `SJTAG`"]
pub struct SJTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SJTAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SJTAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SJTAG_A::_0)
    }
    #[doc = "Reset caused by JTAG"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SJTAG_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Sticky Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    _1,
}
impl From<SLOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: SLOCKUP_A) -> Self {
        match variant {
            SLOCKUP_A::_0 => false,
            SLOCKUP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLOCKUP`"]
pub type SLOCKUP_R = crate::R<bool, SLOCKUP_A>;
impl SLOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOCKUP_A {
        match self.bits {
            false => SLOCKUP_A::_0,
            true => SLOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOCKUP_A::_1
    }
}
#[doc = "Write proxy for field `SLOCKUP`"]
pub struct SLOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Sticky Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl From<SSW_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_A) -> Self {
        match variant {
            SSW_A::_0 => false,
            SSW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SSW`"]
pub type SSW_R = crate::R<bool, SSW_A>;
impl SSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_A {
        match self.bits {
            false => SSW_A::_0,
            true => SSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSW_A::_1
    }
}
#[doc = "Write proxy for field `SSW`"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSW_A::_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSW_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Sticky MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_AP_A {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl From<SMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: SMDM_AP_A) -> Self {
        match variant {
            SMDM_AP_A::_0 => false,
            SMDM_AP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SMDM_AP`"]
pub type SMDM_AP_R = crate::R<bool, SMDM_AP_A>;
impl SMDM_AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMDM_AP_A {
        match self.bits {
            false => SMDM_AP_A::_0,
            true => SMDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMDM_AP_A::_1
    }
}
#[doc = "Write proxy for field `SMDM_AP`"]
pub struct SMDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMDM_AP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_0)
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Sticky EzPort Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEZPT_A {
    #[doc = "0: Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0,
    #[doc = "1: Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1,
}
impl From<SEZPT_A> for bool {
    #[inline(always)]
    fn from(variant: SEZPT_A) -> Self {
        match variant {
            SEZPT_A::_0 => false,
            SEZPT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SEZPT`"]
pub type SEZPT_R = crate::R<bool, SEZPT_A>;
impl SEZPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEZPT_A {
        match self.bits {
            false => SEZPT_A::_0,
            true => SEZPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEZPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEZPT_A::_1
    }
}
#[doc = "Write proxy for field `SEZPT`"]
pub struct SEZPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEZPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEZPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEZPT_A::_0)
    }
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEZPT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Sticky Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl From<SSACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SSACKERR_A) -> Self {
        match variant {
            SSACKERR_A::_0 => false,
            SSACKERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SSACKERR`"]
pub type SSACKERR_R = crate::R<bool, SSACKERR_A>;
impl SSACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACKERR_A {
        match self.bits {
            false => SSACKERR_A::_0,
            true => SSACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSACKERR_A::_1
    }
}
#[doc = "Write proxy for field `SSACKERR`"]
pub struct SSACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACKERR_A::_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACKERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sticky JTAG Generated Reset"]
    #[inline(always)]
    pub fn sjtag(&self) -> SJTAG_R {
        SJTAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&self) -> SLOCKUP_R {
        SLOCKUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&self) -> SMDM_AP_R {
        SMDM_AP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sticky EzPort Reset"]
    #[inline(always)]
    pub fn sezpt(&self) -> SEZPT_R {
        SEZPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn ssackerr(&self) -> SSACKERR_R {
        SSACKERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky JTAG Generated Reset"]
    #[inline(always)]
    pub fn sjtag(&mut self) -> SJTAG_W {
        SJTAG_W { w: self }
    }
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&mut self) -> SLOCKUP_W {
        SLOCKUP_W { w: self }
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&mut self) -> SMDM_AP_W {
        SMDM_AP_W { w: self }
    }
    #[doc = "Bit 4 - Sticky EzPort Reset"]
    #[inline(always)]
    pub fn sezpt(&mut self) -> SEZPT_W {
        SEZPT_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn ssackerr(&mut self) -> SSACKERR_W {
        SSACKERR_W { w: self }
    }
}
