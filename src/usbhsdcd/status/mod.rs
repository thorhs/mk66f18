#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SEQ_RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_RESR {
    #[doc = "No results to report."]
    _00,
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    _01,
    #[doc = "Attached to a charging port. The exact meaning depends on bit 18: 0: Attached to either a CDP or a DCP. The charger type detection has not completed. 1: Attached to a CDP. The charger type detection has completed."]
    _10,
    #[doc = "Attached to a DCP."]
    _11,
}
impl SEQ_RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEQ_RESR::_00 => 0,
            SEQ_RESR::_01 => 1,
            SEQ_RESR::_10 => 2,
            SEQ_RESR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEQ_RESR {
        match value {
            0 => SEQ_RESR::_00,
            1 => SEQ_RESR::_01,
            2 => SEQ_RESR::_10,
            3 => SEQ_RESR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SEQ_RESR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SEQ_RESR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SEQ_RESR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SEQ_RESR::_11
    }
}
#[doc = "Possible values of the field `SEQ_STAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_STATR {
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    _00,
    #[doc = "Data pin contact detection is complete."]
    _01,
    #[doc = "Charging port detection is complete."]
    _10,
    #[doc = "Charger type detection is complete."]
    _11,
}
impl SEQ_STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEQ_STATR::_00 => 0,
            SEQ_STATR::_01 => 1,
            SEQ_STATR::_10 => 2,
            SEQ_STATR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEQ_STATR {
        match value {
            0 => SEQ_STATR::_00,
            1 => SEQ_STATR::_01,
            2 => SEQ_STATR::_10,
            3 => SEQ_STATR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SEQ_STATR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SEQ_STATR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SEQ_STATR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SEQ_STATR::_11
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "No sequence errors."]
    _0,
    #[doc = "Error in the detection sequence. See the SEQ_STAT field to determine the phase in which the error occurred."]
    _1,
}
impl ERRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ERRR::_0 => false,
            ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            false => ERRR::_0,
            true => ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRR::_1
    }
}
#[doc = "Possible values of the field `TO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOR {
    #[doc = "The detection sequence has not been running for over 1 s."]
    _0,
    #[doc = "It has been over 1 s since the data pin contact was detected and debounced."]
    _1,
}
impl TOR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TOR::_0 => false,
            TOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOR {
        match value {
            false => TOR::_0,
            true => TOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOR::_1
    }
}
#[doc = "Possible values of the field `ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVER {
    #[doc = "The sequence is not running."]
    _0,
    #[doc = "The sequence is running."]
    _1,
}
impl ACTIVER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACTIVER::_0 => false,
            ACTIVER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVER {
        match value {
            false => ACTIVER::_0,
            true => ACTIVER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACTIVER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACTIVER::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:17 - Charger Detection Sequence Results"]
    #[inline]
    pub fn seq_res(&self) -> SEQ_RESR {
        SEQ_RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Charger Detection Sequence Status"]
    #[inline]
    pub fn seq_stat(&self) -> SEQ_STATR {
        SEQ_STATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Error Flag"]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Timeout Flag"]
    #[inline]
    pub fn to(&self) -> TOR {
        TOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Active Status Indicator"]
    #[inline]
    pub fn active(&self) -> ACTIVER {
        ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
