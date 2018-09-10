#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `DOZEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEENR {
    #[doc = "Internal TPM counter continues in Doze mode."]
    _0,
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1,
}
impl DOZEENR {
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
            DOZEENR::_0 => false,
            DOZEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZEENR {
        match value {
            false => DOZEENR::_0,
            true => DOZEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZEENR::_1
    }
}
#[doc = "Possible values of the field `DBGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGMODER {
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00,
    #[doc = "TPM counter continues in debug mode."]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBGMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBGMODER::_00 => 0,
            DBGMODER::_11 => 3,
            DBGMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBGMODER {
        match value {
            0 => DBGMODER::_00,
            3 => DBGMODER::_11,
            i => DBGMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DBGMODER::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DBGMODER::_11
    }
}
#[doc = "Possible values of the field `GTBSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBSYNCR {
    #[doc = "Global timebase synchronization disabled."]
    _0,
    #[doc = "Global timebase synchronization enabled."]
    _1,
}
impl GTBSYNCR {
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
            GTBSYNCR::_0 => false,
            GTBSYNCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTBSYNCR {
        match value {
            false => GTBSYNCR::_0,
            true => GTBSYNCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTBSYNCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTBSYNCR::_1
    }
}
#[doc = "Possible values of the field `GTBEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEENR {
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    _0,
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    _1,
}
impl GTBEENR {
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
            GTBEENR::_0 => false,
            GTBEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTBEENR {
        match value {
            false => GTBEENR::_0,
            true => GTBEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTBEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTBEENR::_1
    }
}
#[doc = "Possible values of the field `CSOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOTR {
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    _0,
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1,
}
impl CSOTR {
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
            CSOTR::_0 => false,
            CSOTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSOTR {
        match value {
            false => CSOTR::_0,
            true => CSOTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSOTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSOTR::_1
    }
}
#[doc = "Possible values of the field `CSOO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOOR {
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    _0,
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    _1,
}
impl CSOOR {
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
            CSOOR::_0 => false,
            CSOOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSOOR {
        match value {
            false => CSOOR::_0,
            true => CSOOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSOOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSOOR::_1
    }
}
#[doc = "Possible values of the field `CROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROTR {
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0,
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1,
}
impl CROTR {
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
            CROTR::_0 => false,
            CROTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CROTR {
        match value {
            false => CROTR::_0,
            true => CROTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CROTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CROTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CPOTR {
    bits: bool,
}
impl CPOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TRGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOLR {
    #[doc = "Trigger is active high."]
    _0,
    #[doc = "Trigger is active low."]
    _1,
}
impl TRGPOLR {
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
            TRGPOLR::_0 => false,
            TRGPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGPOLR {
        match value {
            false => TRGPOLR::_0,
            true => TRGPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGPOLR::_1
    }
}
#[doc = "Possible values of the field `TRGSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRCR {
    #[doc = "Trigger source selected by TRGSEL is external."]
    _0,
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    _1,
}
impl TRGSRCR {
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
            TRGSRCR::_0 => false,
            TRGSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGSRCR {
        match value {
            false => TRGSRCR::_0,
            true => TRGSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGSRCR::_1
    }
}
#[doc = "Possible values of the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELR {
    #[doc = "Channel 0 pin input capture"]
    _0001,
    #[doc = "Channel 1 pin input capture"]
    _0010,
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    _0011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSELR::_0001 => 1,
            TRGSELR::_0010 => 2,
            TRGSELR::_0011 => 3,
            TRGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSELR {
        match value {
            1 => TRGSELR::_0001,
            2 => TRGSELR::_0010,
            3 => TRGSELR::_0011,
            i => TRGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == TRGSELR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == TRGSELR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == TRGSELR::_0011
    }
}
#[doc = "Values that can be written to the field `DOZEEN`"]
pub enum DOZEENW {
    #[doc = "Internal TPM counter continues in Doze mode."]
    _0,
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1,
}
impl DOZEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEENW::_0 => false,
            DOZEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal TPM counter continues in Doze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEENW::_0)
    }
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGMODE`"]
pub enum DBGMODEW {
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00,
    #[doc = "TPM counter continues in debug mode."]
    _11,
}
impl DBGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBGMODEW::_00 => 0,
            DBGMODEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DBGMODEW::_00)
    }
    #[doc = "TPM counter continues in debug mode."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DBGMODEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTBSYNC`"]
pub enum GTBSYNCW {
    #[doc = "Global timebase synchronization disabled."]
    _0,
    #[doc = "Global timebase synchronization enabled."]
    _1,
}
impl GTBSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTBSYNCW::_0 => false,
            GTBSYNCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTBSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTBSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Global timebase synchronization disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBSYNCW::_0)
    }
    #[doc = "Global timebase synchronization enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBSYNCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTBEEN`"]
pub enum GTBEENW {
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    _0,
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    _1,
}
impl GTBEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTBEENW::_0 => false,
            GTBEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTBEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTBEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEENW::_0)
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSOT`"]
pub enum CSOTW {
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    _0,
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1,
}
impl CSOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSOTW::_0 => false,
            CSOTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSOTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOTW::_0)
    }
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOTW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSOO`"]
pub enum CSOOW {
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    _0,
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    _1,
}
impl CSOOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSOOW::_0 => false,
            CSOOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSOOW<'a> {
    w: &'a mut W,
}
impl<'a> _CSOOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSOOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOOW::_0)
    }
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOOW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CROT`"]
pub enum CROTW {
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0,
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1,
}
impl CROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CROTW::_0 => false,
            CROTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CROTW<'a> {
    w: &'a mut W,
}
impl<'a> _CROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CROTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CROTW::_0)
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CROTW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPOTW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGPOL`"]
pub enum TRGPOLW {
    #[doc = "Trigger is active high."]
    _0,
    #[doc = "Trigger is active low."]
    _1,
}
impl TRGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGPOLW::_0 => false,
            TRGPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPOLW::_0)
    }
    #[doc = "Trigger is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPOLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGSRC`"]
pub enum TRGSRCW {
    #[doc = "Trigger source selected by TRGSEL is external."]
    _0,
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    _1,
}
impl TRGSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGSRCW::_0 => false,
            TRGSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger source selected by TRGSEL is external."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSRCW::_0)
    }
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSRCW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGSEL`"]
pub enum TRGSELW {
    #[doc = "Channel 0 pin input capture"]
    _0001,
    #[doc = "Channel 1 pin input capture"]
    _0010,
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    _0011,
}
impl TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSELW::_0001 => 1,
            TRGSELW::_0010 => 2,
            TRGSELW::_0011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Channel 0 pin input capture"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TRGSELW::_0001)
    }
    #[doc = "Channel 1 pin input capture"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TRGSELW::_0010)
    }
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TRGSELW::_0011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 5 - Doze Enable"]
    #[inline]
    pub fn dozeen(&self) -> DOZEENR {
        DOZEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn dbgmode(&self) -> DBGMODER {
        DBGMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline]
    pub fn gtbsync(&self) -> GTBSYNCR {
        GTBSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline]
    pub fn gtbeen(&self) -> GTBEENR {
        GTBEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline]
    pub fn csot(&self) -> CSOTR {
        CSOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline]
    pub fn csoo(&self) -> CSOOR {
        CSOOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline]
    pub fn crot(&self) -> CROTR {
        CROTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline]
    pub fn cpot(&self) -> CPOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOTR { bits }
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&self) -> TRGPOLR {
        TRGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&self) -> TRGSRCR {
        TRGSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        TRGSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Doze Enable"]
    #[inline]
    pub fn dozeen(&mut self) -> _DOZEENW {
        _DOZEENW { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn dbgmode(&mut self) -> _DBGMODEW {
        _DBGMODEW { w: self }
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline]
    pub fn gtbsync(&mut self) -> _GTBSYNCW {
        _GTBSYNCW { w: self }
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline]
    pub fn gtbeen(&mut self) -> _GTBEENW {
        _GTBEENW { w: self }
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline]
    pub fn csot(&mut self) -> _CSOTW {
        _CSOTW { w: self }
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline]
    pub fn csoo(&mut self) -> _CSOOW {
        _CSOOW { w: self }
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline]
    pub fn crot(&mut self) -> _CROTW {
        _CROTW { w: self }
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline]
    pub fn cpot(&mut self) -> _CPOTW {
        _CPOTW { w: self }
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&mut self) -> _TRGPOLW {
        _TRGPOLW { w: self }
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&mut self) -> _TRGSRCW {
        _TRGSRCW { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
