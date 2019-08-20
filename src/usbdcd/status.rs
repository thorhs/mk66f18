#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Charger Detection Sequence Results\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_RES_A {
    #[doc = "0: No results to report."]
    _00,
    #[doc = "1: Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    _01,
    #[doc = "2: Attached to a charging port. The exact meaning depends on bit 18: 0: Attached to either a CDP or a DCP. The charger type detection has not completed. 1: Attached to a CDP. The charger type detection has completed."]
    _10,
    #[doc = "3: Attached to a DCP."]
    _11,
}
impl From<SEQ_RES_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQ_RES_A) -> Self {
        match variant {
            SEQ_RES_A::_00 => 0,
            SEQ_RES_A::_01 => 1,
            SEQ_RES_A::_10 => 2,
            SEQ_RES_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `SEQ_RES`"]
pub type SEQ_RES_R = crate::R<u8, SEQ_RES_A>;
impl SEQ_RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_RES_A {
        match self.bits {
            0 => SEQ_RES_A::_00,
            1 => SEQ_RES_A::_01,
            2 => SEQ_RES_A::_10,
            3 => SEQ_RES_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEQ_RES_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEQ_RES_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEQ_RES_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEQ_RES_A::_11
    }
}
#[doc = "Charger Detection Sequence Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_STAT_A {
    #[doc = "0: The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    _00,
    #[doc = "1: Data pin contact detection is complete."]
    _01,
    #[doc = "2: Charging port detection is complete."]
    _10,
    #[doc = "3: Charger type detection is complete."]
    _11,
}
impl From<SEQ_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQ_STAT_A) -> Self {
        match variant {
            SEQ_STAT_A::_00 => 0,
            SEQ_STAT_A::_01 => 1,
            SEQ_STAT_A::_10 => 2,
            SEQ_STAT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `SEQ_STAT`"]
pub type SEQ_STAT_R = crate::R<u8, SEQ_STAT_A>;
impl SEQ_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_STAT_A {
        match self.bits {
            0 => SEQ_STAT_A::_00,
            1 => SEQ_STAT_A::_01,
            2 => SEQ_STAT_A::_10,
            3 => SEQ_STAT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEQ_STAT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEQ_STAT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEQ_STAT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEQ_STAT_A::_11
    }
}
#[doc = "Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: No sequence errors."]
    _0,
    #[doc = "1: Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred."]
    _1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        match variant {
            ERR_A::_0 => false,
            ERR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::_0,
            true => ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_A::_1
    }
}
#[doc = "Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_A {
    #[doc = "0: The detection sequence has not been running for over 1 s."]
    _0,
    #[doc = "1: It has been over 1 s since the data pin contact was detected and debounced."]
    _1,
}
impl From<TO_A> for bool {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        match variant {
            TO_A::_0 => false,
            TO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<bool, TO_A>;
impl TO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_A {
        match self.bits {
            false => TO_A::_0,
            true => TO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TO_A::_1
    }
}
#[doc = "Active Status Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVE_A {
    #[doc = "0: The sequence is not running."]
    _0,
    #[doc = "1: The sequence is running."]
    _1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        match variant {
            ACTIVE_A::_0 => false,
            ACTIVE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, ACTIVE_A>;
impl ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::_0,
            true => ACTIVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACTIVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACTIVE_A::_1
    }
}
impl R {
    #[doc = "Bits 16:17 - Charger Detection Sequence Results"]
    #[inline(always)]
    pub fn seq_res(&self) -> SEQ_RES_R {
        SEQ_RES_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Charger Detection Sequence Status"]
    #[inline(always)]
    pub fn seq_stat(&self) -> SEQ_STAT_R {
        SEQ_STAT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Error Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timeout Flag"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Active Status Indicator"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
