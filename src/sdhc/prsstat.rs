#[doc = "Reader of register PRSSTAT"]
pub type R = crate::R<u32, super::PRSSTAT>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIHB_A {
    #[doc = "0: Can issue command using only CMD line."]
    _0,
    #[doc = "1: Cannot issue command."]
    _1,
}
impl From<CIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CIHB_A) -> Self {
        match variant {
            CIHB_A::_0 => false,
            CIHB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CIHB`"]
pub type CIHB_R = crate::R<bool, CIHB_A>;
impl CIHB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIHB_A {
        match self.bits {
            false => CIHB_A::_0,
            true => CIHB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIHB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIHB_A::_1
    }
}
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIHB_A {
    #[doc = "0: Can issue command which uses the DAT line."]
    _0,
    #[doc = "1: Cannot issue command which uses the DAT line."]
    _1,
}
impl From<CDIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CDIHB_A) -> Self {
        match variant {
            CDIHB_A::_0 => false,
            CDIHB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CDIHB`"]
pub type CDIHB_R = crate::R<bool, CDIHB_A>;
impl CDIHB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIHB_A {
        match self.bits {
            false => CDIHB_A::_0,
            true => CDIHB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDIHB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDIHB_A::_1
    }
}
#[doc = "Data Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLA_A {
    #[doc = "0: DAT line inactive."]
    _0,
    #[doc = "1: DAT line active."]
    _1,
}
impl From<DLA_A> for bool {
    #[inline(always)]
    fn from(variant: DLA_A) -> Self {
        match variant {
            DLA_A::_0 => false,
            DLA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DLA`"]
pub type DLA_R = crate::R<bool, DLA_A>;
impl DLA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLA_A {
        match self.bits {
            false => DLA_A::_0,
            true => DLA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLA_A::_1
    }
}
#[doc = "SD Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDSTB_A {
    #[doc = "0: Clock is changing frequency and not stable."]
    _0,
    #[doc = "1: Clock is stable."]
    _1,
}
impl From<SDSTB_A> for bool {
    #[inline(always)]
    fn from(variant: SDSTB_A) -> Self {
        match variant {
            SDSTB_A::_0 => false,
            SDSTB_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SDSTB`"]
pub type SDSTB_R = crate::R<bool, SDSTB_A>;
impl SDSTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDSTB_A {
        match self.bits {
            false => SDSTB_A::_0,
            true => SDSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDSTB_A::_1
    }
}
#[doc = "Bus Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGOFF_A {
    #[doc = "0: Bus clock is active."]
    _0,
    #[doc = "1: Bus clock is gated off."]
    _1,
}
impl From<IPGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: IPGOFF_A) -> Self {
        match variant {
            IPGOFF_A::_0 => false,
            IPGOFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IPGOFF`"]
pub type IPGOFF_R = crate::R<bool, IPGOFF_A>;
impl IPGOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGOFF_A {
        match self.bits {
            false => IPGOFF_A::_0,
            true => IPGOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPGOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPGOFF_A::_1
    }
}
#[doc = "System Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKOFF_A {
    #[doc = "0: System clock is active."]
    _0,
    #[doc = "1: System clock is gated off."]
    _1,
}
impl From<HCKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: HCKOFF_A) -> Self {
        match variant {
            HCKOFF_A::_0 => false,
            HCKOFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HCKOFF`"]
pub type HCKOFF_R = crate::R<bool, HCKOFF_A>;
impl HCKOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKOFF_A {
        match self.bits {
            false => HCKOFF_A::_0,
            true => HCKOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCKOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCKOFF_A::_1
    }
}
#[doc = "SDHC clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEROFF_A {
    #[doc = "0: SDHC clock is active."]
    _0,
    #[doc = "1: SDHC clock is gated off."]
    _1,
}
impl From<PEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: PEROFF_A) -> Self {
        match variant {
            PEROFF_A::_0 => false,
            PEROFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PEROFF`"]
pub type PEROFF_R = crate::R<bool, PEROFF_A>;
impl PEROFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEROFF_A {
        match self.bits {
            false => PEROFF_A::_0,
            true => PEROFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEROFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEROFF_A::_1
    }
}
#[doc = "SD Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDOFF_A {
    #[doc = "0: SD clock is active."]
    _0,
    #[doc = "1: SD clock is gated off."]
    _1,
}
impl From<SDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SDOFF_A) -> Self {
        match variant {
            SDOFF_A::_0 => false,
            SDOFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SDOFF`"]
pub type SDOFF_R = crate::R<bool, SDOFF_A>;
impl SDOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOFF_A {
        match self.bits {
            false => SDOFF_A::_0,
            true => SDOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDOFF_A::_1
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTA_A {
    #[doc = "0: No valid data."]
    _0,
    #[doc = "1: Transferring data."]
    _1,
}
impl From<WTA_A> for bool {
    #[inline(always)]
    fn from(variant: WTA_A) -> Self {
        match variant {
            WTA_A::_0 => false,
            WTA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WTA`"]
pub type WTA_R = crate::R<bool, WTA_A>;
impl WTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTA_A {
        match self.bits {
            false => WTA_A::_0,
            true => WTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTA_A::_1
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTA_A {
    #[doc = "0: No valid data."]
    _0,
    #[doc = "1: Transferring data."]
    _1,
}
impl From<RTA_A> for bool {
    #[inline(always)]
    fn from(variant: RTA_A) -> Self {
        match variant {
            RTA_A::_0 => false,
            RTA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RTA`"]
pub type RTA_R = crate::R<bool, RTA_A>;
impl RTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTA_A {
        match self.bits {
            false => RTA_A::_0,
            true => RTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTA_A::_1
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWEN_A {
    #[doc = "0: Write disable, the buffer can hold valid data less than the write watermark level."]
    _0,
    #[doc = "1: Write enable, the buffer can hold valid data greater than the write watermark level."]
    _1,
}
impl From<BWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWEN_A) -> Self {
        match variant {
            BWEN_A::_0 => false,
            BWEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BWEN`"]
pub type BWEN_R = crate::R<bool, BWEN_A>;
impl BWEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWEN_A {
        match self.bits {
            false => BWEN_A::_0,
            true => BWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWEN_A::_1
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREN_A {
    #[doc = "0: Read disable, valid data less than the watermark level exist in the buffer."]
    _0,
    #[doc = "1: Read enable, valid data greater than the watermark level exist in the buffer."]
    _1,
}
impl From<BREN_A> for bool {
    #[inline(always)]
    fn from(variant: BREN_A) -> Self {
        match variant {
            BREN_A::_0 => false,
            BREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BREN`"]
pub type BREN_R = crate::R<bool, BREN_A>;
impl BREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREN_A {
        match self.bits {
            false => BREN_A::_0,
            true => BREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BREN_A::_1
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINS_A {
    #[doc = "0: Power on reset or no card."]
    _0,
    #[doc = "1: Card inserted."]
    _1,
}
impl From<CINS_A> for bool {
    #[inline(always)]
    fn from(variant: CINS_A) -> Self {
        match variant {
            CINS_A::_0 => false,
            CINS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CINS`"]
pub type CINS_R = crate::R<bool, CINS_A>;
impl CINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINS_A {
        match self.bits {
            false => CINS_A::_0,
            true => CINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINS_A::_1
    }
}
#[doc = "Reader of field `CLSL`"]
pub type CLSL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLSL`"]
pub type DLSL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cihb(&self) -> CIHB_R {
        CIHB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cdihb(&self) -> CDIHB_R {
        CDIHB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline(always)]
    pub fn dla(&self) -> DLA_R {
        DLA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline(always)]
    pub fn sdstb(&self) -> SDSTB_R {
        SDSTB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bus Clock Gated Off Internally"]
    #[inline(always)]
    pub fn ipgoff(&self) -> IPGOFF_R {
        IPGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Clock Gated Off Internally"]
    #[inline(always)]
    pub fn hckoff(&self) -> HCKOFF_R {
        HCKOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SDHC clock Gated Off Internally"]
    #[inline(always)]
    pub fn peroff(&self) -> PEROFF_R {
        PEROFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline(always)]
    pub fn sdoff(&self) -> SDOFF_R {
        SDOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wta(&self) -> WTA_R {
        WTA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rta(&self) -> RTA_R {
        RTA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bwen(&self) -> BWEN_R {
        BWEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn clsl(&self) -> CLSL_R {
        CLSL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - DAT Line Signal Level"]
    #[inline(always)]
    pub fn dlsl(&self) -> DLSL_R {
        DLSL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
