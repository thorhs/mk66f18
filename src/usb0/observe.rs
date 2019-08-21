#[doc = "Reader of register OBSERVE"]
pub type R = crate::R<u8, super::OBSERVE>;
#[doc = "Provides observability of the D- Pulldown enable at the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMPD_A {
    #[doc = "0: D- pulldown disabled."]
    _0,
    #[doc = "1: D- pulldown enabled."]
    _1,
}
impl From<DMPD_A> for bool {
    #[inline(always)]
    fn from(variant: DMPD_A) -> Self {
        match variant {
            DMPD_A::_0 => false,
            DMPD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMPD`"]
pub type DMPD_R = crate::R<bool, DMPD_A>;
impl DMPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMPD_A {
        match self.bits {
            false => DMPD_A::_0,
            true => DMPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMPD_A::_1
    }
}
#[doc = "Provides observability of the D+ Pulldown enable at the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPD_A {
    #[doc = "0: D+ pulldown disabled."]
    _0,
    #[doc = "1: D+ pulldown enabled."]
    _1,
}
impl From<DPPD_A> for bool {
    #[inline(always)]
    fn from(variant: DPPD_A) -> Self {
        match variant {
            DPPD_A::_0 => false,
            DPPD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPPD`"]
pub type DPPD_R = crate::R<bool, DPPD_A>;
impl DPPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPD_A {
        match self.bits {
            false => DPPD_A::_0,
            true => DPPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPPD_A::_1
    }
}
#[doc = "Provides observability of the D+ Pullup enable at the USB transceiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_A {
    #[doc = "0: D+ pullup disabled."]
    _0,
    #[doc = "1: D+ pullup enabled."]
    _1,
}
impl From<DPPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_A) -> Self {
        match variant {
            DPPU_A::_0 => false,
            DPPU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DPPU`"]
pub type DPPU_R = crate::R<bool, DPPU_A>;
impl DPPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPU_A {
        match self.bits {
            false => DPPU_A::_0,
            true => DPPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPPU_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - Provides observability of the D- Pulldown enable at the USB transceiver."]
    #[inline(always)]
    pub fn dmpd(&self) -> DMPD_R {
        DMPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Provides observability of the D+ Pulldown enable at the USB transceiver."]
    #[inline(always)]
    pub fn dppd(&self) -> DPPD_R {
        DPPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Provides observability of the D+ Pullup enable at the USB transceiver."]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
