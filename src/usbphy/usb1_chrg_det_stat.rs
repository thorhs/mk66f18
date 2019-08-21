#[doc = "Reader of register USB1_CHRG_DET_STAT"]
pub type R = crate::R<u32, super::USB1_CHRG_DET_STAT>;
#[doc = "Battery Charging Data Contact Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUG_CONTACT_A {
    #[doc = "0: No USB cable attachment has been detected"]
    _0,
    #[doc = "1: A USB cable attachment between the device and host has been detected"]
    _1,
}
impl From<PLUG_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_CONTACT_A) -> Self {
        match variant {
            PLUG_CONTACT_A::_0 => false,
            PLUG_CONTACT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PLUG_CONTACT`"]
pub type PLUG_CONTACT_R = crate::R<bool, PLUG_CONTACT_A>;
impl PLUG_CONTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLUG_CONTACT_A {
        match self.bits {
            false => PLUG_CONTACT_A::_0,
            true => PLUG_CONTACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLUG_CONTACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLUG_CONTACT_A::_1
    }
}
#[doc = "Battery Charging Primary Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRG_DETECTED_A {
    #[doc = "0: Standard Downstream Port (SDP) has been detected"]
    _0,
    #[doc = "1: Charging Port has been detected"]
    _1,
}
impl From<CHRG_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: CHRG_DETECTED_A) -> Self {
        match variant {
            CHRG_DETECTED_A::_0 => false,
            CHRG_DETECTED_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CHRG_DETECTED`"]
pub type CHRG_DETECTED_R = crate::R<bool, CHRG_DETECTED_A>;
impl CHRG_DETECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRG_DETECTED_A {
        match self.bits {
            false => CHRG_DETECTED_A::_0,
            true => CHRG_DETECTED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHRG_DETECTED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHRG_DETECTED_A::_1
    }
}
#[doc = "Single ended receiver output for the USB_DM pin, from charger detection circuits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_STATE_A {
    #[doc = "0: USB_DM pin voltage is < 0.8V"]
    _0,
    #[doc = "1: USB_DM pin voltage is > 2.0V"]
    _1,
}
impl From<DM_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DM_STATE_A) -> Self {
        match variant {
            DM_STATE_A::_0 => false,
            DM_STATE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DM_STATE`"]
pub type DM_STATE_R = crate::R<bool, DM_STATE_A>;
impl DM_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_STATE_A {
        match self.bits {
            false => DM_STATE_A::_0,
            true => DM_STATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DM_STATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DM_STATE_A::_1
    }
}
#[doc = "Single ended receiver output for the USB_DP pin, from charger detection circuits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_STATE_A {
    #[doc = "0: USB_DP pin voltage is < 0.8V"]
    _0,
    #[doc = "1: USB_DP pin voltage is > 2.0V"]
    _1,
}
impl From<DP_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DP_STATE_A) -> Self {
        match variant {
            DP_STATE_A::_0 => false,
            DP_STATE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DP_STATE`"]
pub type DP_STATE_R = crate::R<bool, DP_STATE_A>;
impl DP_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_STATE_A {
        match self.bits {
            false => DP_STATE_A::_0,
            true => DP_STATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DP_STATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DP_STATE_A::_1
    }
}
#[doc = "Battery Charging Secondary Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECDET_DCP_A {
    #[doc = "0: Charging Downstream Port (CDP) has been detected"]
    _0,
    #[doc = "1: Downstream Charging Port (DCP) has been detected"]
    _1,
}
impl From<SECDET_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: SECDET_DCP_A) -> Self {
        match variant {
            SECDET_DCP_A::_0 => false,
            SECDET_DCP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SECDET_DCP`"]
pub type SECDET_DCP_R = crate::R<bool, SECDET_DCP_A>;
impl SECDET_DCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECDET_DCP_A {
        match self.bits {
            false => SECDET_DCP_A::_0,
            true => SECDET_DCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECDET_DCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECDET_DCP_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Battery Charging Data Contact Detection phase output"]
    #[inline(always)]
    pub fn plug_contact(&self) -> PLUG_CONTACT_R {
        PLUG_CONTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Battery Charging Primary Detection phase output"]
    #[inline(always)]
    pub fn chrg_detected(&self) -> CHRG_DETECTED_R {
        CHRG_DETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dm_state(&self) -> DM_STATE_R {
        DM_STATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dp_state(&self) -> DP_STATE_R {
        DP_STATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Battery Charging Secondary Detection phase output"]
    #[inline(always)]
    pub fn secdet_dcp(&self) -> SECDET_DCP_R {
        SECDET_DCP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
