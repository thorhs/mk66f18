#[doc = "Reader of register USB1_VBUS_DET_STAT"]
pub type R = crate::R<u32, super::USB1_VBUS_DET_STAT>;
#[doc = "Session End indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESSEND_A {
    #[doc = "0: The VBUS voltage is above the Session Valid threshold"]
    _0,
    #[doc = "1: The VBUS voltage is below the Session Valid threshold"]
    _1,
}
impl From<SESSEND_A> for bool {
    #[inline(always)]
    fn from(variant: SESSEND_A) -> Self {
        match variant {
            SESSEND_A::_0 => false,
            SESSEND_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SESSEND`"]
pub type SESSEND_R = crate::R<bool, SESSEND_A>;
impl SESSEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSEND_A {
        match self.bits {
            false => SESSEND_A::_0,
            true => SESSEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SESSEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SESSEND_A::_1
    }
}
#[doc = "B-Device Session Valid status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BVALID_A {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    _0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    _1,
}
impl From<BVALID_A> for bool {
    #[inline(always)]
    fn from(variant: BVALID_A) -> Self {
        match variant {
            BVALID_A::_0 => false,
            BVALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BVALID`"]
pub type BVALID_R = crate::R<bool, BVALID_A>;
impl BVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALID_A {
        match self.bits {
            false => BVALID_A::_0,
            true => BVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BVALID_A::_1
    }
}
#[doc = "A-Device Session Valid status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVALID_A {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    _0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    _1,
}
impl From<AVALID_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_A) -> Self {
        match variant {
            AVALID_A::_0 => false,
            AVALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVALID`"]
pub type AVALID_R = crate::R<bool, AVALID_A>;
impl AVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_A {
        match self.bits {
            false => AVALID_A::_0,
            true => AVALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVALID_A::_1
    }
}
#[doc = "VBUS voltage status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_VALID_A {
    #[doc = "0: VBUS is below the comparator threshold"]
    _0,
    #[doc = "1: VBUS is above the comparator threshold"]
    _1,
}
impl From<VBUS_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_VALID_A) -> Self {
        match variant {
            VBUS_VALID_A::_0 => false,
            VBUS_VALID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VBUS_VALID`"]
pub type VBUS_VALID_R = crate::R<bool, VBUS_VALID_A>;
impl VBUS_VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_VALID_A {
        match self.bits {
            false => VBUS_VALID_A::_0,
            true => VBUS_VALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUS_VALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUS_VALID_A::_1
    }
}
#[doc = "VBUS_VALID_3V detector status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_VALID_3V_A {
    #[doc = "0: VBUS voltage is below VBUS_VALID_3V threshold"]
    _0,
    #[doc = "1: VBUS voltage is above VBUS_VALID_3V threshold"]
    _1,
}
impl From<VBUS_VALID_3V_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_VALID_3V_A) -> Self {
        match variant {
            VBUS_VALID_3V_A::_0 => false,
            VBUS_VALID_3V_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `VBUS_VALID_3V`"]
pub type VBUS_VALID_3V_R = crate::R<bool, VBUS_VALID_3V_A>;
impl VBUS_VALID_3V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_VALID_3V_A {
        match self.bits {
            false => VBUS_VALID_3V_A::_0,
            true => VBUS_VALID_3V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUS_VALID_3V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUS_VALID_3V_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Session End indicator"]
    #[inline(always)]
    pub fn sessend(&self) -> SESSEND_R {
        SESSEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B-Device Session Valid status"]
    #[inline(always)]
    pub fn bvalid(&self) -> BVALID_R {
        BVALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A-Device Session Valid status"]
    #[inline(always)]
    pub fn avalid(&self) -> AVALID_R {
        AVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBUS voltage status"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VBUS_VALID_3V detector status"]
    #[inline(always)]
    pub fn vbus_valid_3v(&self) -> VBUS_VALID_3V_R {
        VBUS_VALID_3V_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
