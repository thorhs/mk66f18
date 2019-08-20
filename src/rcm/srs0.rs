#[doc = "Reader of register SRS0"]
pub type R = crate::R<u8, super::SRS0>;
#[doc = "Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_A {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    _0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    _1,
}
impl From<WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_A) -> Self {
        match variant {
            WAKEUP_A::_0 => false,
            WAKEUP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, WAKEUP_A>;
impl WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_A {
        match self.bits {
            false => WAKEUP_A::_0,
            true => WAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKEUP_A::_1
    }
}
#[doc = "Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1,
}
impl From<LVD_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_A) -> Self {
        match variant {
            LVD_A::_0 => false,
            LVD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LVD`"]
pub type LVD_R = crate::R<bool, LVD_A>;
impl LVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_A {
        match self.bits {
            false => LVD_A::_0,
            true => LVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD_A::_1
    }
}
#[doc = "Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        match variant {
            LOC_A::_0 => false,
            LOC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOC`"]
pub type LOC_R = crate::R<bool, LOC_A>;
impl LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::_0,
            true => LOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOC_A::_1
    }
}
#[doc = "Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL"]
    _0,
    #[doc = "1: Reset caused by a loss of lock in the PLL"]
    _1,
}
impl From<LOL_A> for bool {
    #[inline(always)]
    fn from(variant: LOL_A) -> Self {
        match variant {
            LOL_A::_0 => false,
            LOL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOL`"]
pub type LOL_R = crate::R<bool, LOL_A>;
impl LOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOL_A {
        match self.bits {
            false => LOL_A::_0,
            true => LOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOL_A::_1
    }
}
#[doc = "Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        match variant {
            WDOG_A::_0 => false,
            WDOG_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WDOG`"]
pub type WDOG_R = crate::R<bool, WDOG_A>;
impl WDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::_0,
            true => WDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDOG_A::_1
    }
}
#[doc = "External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0,
    #[doc = "1: Reset caused by external reset pin"]
    _1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        match variant {
            PIN_A::_0 => false,
            PIN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<bool, PIN_A>;
impl PIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::_0,
            true => PIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIN_A::_1
    }
}
#[doc = "Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: Reset not caused by POR"]
    _0,
    #[doc = "1: Reset caused by POR"]
    _1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        match variant {
            POR_A::_0 => false,
            POR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, POR_A>;
impl POR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::_0,
            true => POR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn lol(&self) -> LOL_R {
        LOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
