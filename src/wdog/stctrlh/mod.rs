#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::STCTRLH {
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
#[doc = "Possible values of the field `WDOGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOGENR {
    #[doc = "WDOG is disabled."]
    _0,
    #[doc = "WDOG is enabled."]
    _1,
}
impl WDOGENR {
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
            WDOGENR::_0 => false,
            WDOGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOGENR {
        match value {
            false => WDOGENR::_0,
            true => WDOGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WDOGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WDOGENR::_1
    }
}
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "WDOG clock sourced from LPO ."]
    _0,
    #[doc = "WDOG clock sourced from alternate clock source."]
    _1,
}
impl CLKSRCR {
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
            CLKSRCR::_0 => false,
            CLKSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSRCR {
        match value {
            false => CLKSRCR::_0,
            true => CLKSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKSRCR::_1
    }
}
#[doc = "Possible values of the field `IRQRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQRSTENR {
    #[doc = "WDOG time-out generates reset only."]
    _0,
    #[doc = "WDOG time-out initially generates an interrupt. After WCT, it generates a reset."]
    _1,
}
impl IRQRSTENR {
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
            IRQRSTENR::_0 => false,
            IRQRSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQRSTENR {
        match value {
            false => IRQRSTENR::_0,
            true => IRQRSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRQRSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRQRSTENR::_1
    }
}
#[doc = "Possible values of the field `WINEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINENR {
    #[doc = "Windowing mode is disabled."]
    _0,
    #[doc = "Windowing mode is enabled."]
    _1,
}
impl WINENR {
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
            WINENR::_0 => false,
            WINENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WINENR {
        match value {
            false => WINENR::_0,
            true => WINENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WINENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WINENR::_1
    }
}
#[doc = "Possible values of the field `ALLOWUPDATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLOWUPDATER {
    #[doc = "No further updates allowed to WDOG write-once registers."]
    _0,
    #[doc = "WDOG write-once registers can be unlocked for updating."]
    _1,
}
impl ALLOWUPDATER {
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
            ALLOWUPDATER::_0 => false,
            ALLOWUPDATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALLOWUPDATER {
        match value {
            false => ALLOWUPDATER::_0,
            true => ALLOWUPDATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALLOWUPDATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALLOWUPDATER::_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "WDOG is disabled in CPU Debug mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Debug mode."]
    _1,
}
impl DBGENR {
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
            DBGENR::_0 => false,
            DBGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::_0,
            true => DBGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBGENR::_1
    }
}
#[doc = "Possible values of the field `STOPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPENR {
    #[doc = "WDOG is disabled in CPU Stop mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Stop mode."]
    _1,
}
impl STOPENR {
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
            STOPENR::_0 => false,
            STOPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPENR {
        match value {
            false => STOPENR::_0,
            true => STOPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPENR::_1
    }
}
#[doc = "Possible values of the field `WAITEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITENR {
    #[doc = "WDOG is disabled in CPU Wait mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Wait mode."]
    _1,
}
impl WAITENR {
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
            WAITENR::_0 => false,
            WAITENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITENR {
        match value {
            false => WAITENR::_0,
            true => WAITENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAITENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAITENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TESTWDOGR {
    bits: bool,
}
impl TESTWDOGR {
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
#[doc = "Possible values of the field `TESTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTSELR {
    #[doc = "Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test."]
    _0,
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL\\[1:0\\] for testing."]
    _1,
}
impl TESTSELR {
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
            TESTSELR::_0 => false,
            TESTSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TESTSELR {
        match value {
            false => TESTSELR::_0,
            true => TESTSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TESTSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TESTSELR::_1
    }
}
#[doc = "Possible values of the field `BYTESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTESELR {
    #[doc = "Byte 0 selected"]
    _00,
    #[doc = "Byte 1 selected"]
    _01,
    #[doc = "Byte 2 selected"]
    _10,
    #[doc = "Byte 3 selected"]
    _11,
}
impl BYTESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTESELR::_00 => 0,
            BYTESELR::_01 => 1,
            BYTESELR::_10 => 2,
            BYTESELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTESELR {
        match value {
            0 => BYTESELR::_00,
            1 => BYTESELR::_01,
            2 => BYTESELR::_10,
            3 => BYTESELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == BYTESELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == BYTESELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BYTESELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == BYTESELR::_11
    }
}
#[doc = "Possible values of the field `DISTESTWDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISTESTWDOGR {
    #[doc = "WDOG functional test mode is not disabled."]
    _0,
    #[doc = "WDOG functional test mode is disabled permanently until reset."]
    _1,
}
impl DISTESTWDOGR {
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
            DISTESTWDOGR::_0 => false,
            DISTESTWDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISTESTWDOGR {
        match value {
            false => DISTESTWDOGR::_0,
            true => DISTESTWDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DISTESTWDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DISTESTWDOGR::_1
    }
}
#[doc = "Values that can be written to the field `WDOGEN`"]
pub enum WDOGENW {
    #[doc = "WDOG is disabled."]
    _0,
    #[doc = "WDOG is enabled."]
    _1,
}
impl WDOGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOGENW::_0 => false,
            WDOGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDOGENW::_0)
    }
    #[doc = "WDOG is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDOGENW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "WDOG clock sourced from LPO ."]
    _0,
    #[doc = "WDOG clock sourced from alternate clock source."]
    _1,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSRCW::_0 => false,
            CLKSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG clock sourced from LPO ."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRCW::_0)
    }
    #[doc = "WDOG clock sourced from alternate clock source."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRCW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQRSTEN`"]
pub enum IRQRSTENW {
    #[doc = "WDOG time-out generates reset only."]
    _0,
    #[doc = "WDOG time-out initially generates an interrupt. After WCT, it generates a reset."]
    _1,
}
impl IRQRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQRSTENW::_0 => false,
            IRQRSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG time-out generates reset only."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQRSTENW::_0)
    }
    #[doc = "WDOG time-out initially generates an interrupt. After WCT, it generates a reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQRSTENW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WINEN`"]
pub enum WINENW {
    #[doc = "Windowing mode is disabled."]
    _0,
    #[doc = "Windowing mode is enabled."]
    _1,
}
impl WINENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WINENW::_0 => false,
            WINENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINENW<'a> {
    w: &'a mut W,
}
impl<'a> _WINENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Windowing mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WINENW::_0)
    }
    #[doc = "Windowing mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WINENW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALLOWUPDATE`"]
pub enum ALLOWUPDATEW {
    #[doc = "No further updates allowed to WDOG write-once registers."]
    _0,
    #[doc = "WDOG write-once registers can be unlocked for updating."]
    _1,
}
impl ALLOWUPDATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALLOWUPDATEW::_0 => false,
            ALLOWUPDATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALLOWUPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLOWUPDATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALLOWUPDATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No further updates allowed to WDOG write-once registers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLOWUPDATEW::_0)
    }
    #[doc = "WDOG write-once registers can be unlocked for updating."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLOWUPDATEW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "WDOG is disabled in CPU Debug mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Debug mode."]
    _1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::_0 => false,
            DBGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG is disabled in CPU Debug mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGENW::_0)
    }
    #[doc = "WDOG is enabled in CPU Debug mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGENW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPEN`"]
pub enum STOPENW {
    #[doc = "WDOG is disabled in CPU Stop mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Stop mode."]
    _1,
}
impl STOPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPENW::_0 => false,
            STOPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG is disabled in CPU Stop mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPENW::_0)
    }
    #[doc = "WDOG is enabled in CPU Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPENW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAITEN`"]
pub enum WAITENW {
    #[doc = "WDOG is disabled in CPU Wait mode."]
    _0,
    #[doc = "WDOG is enabled in CPU Wait mode."]
    _1,
}
impl WAITENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITENW::_0 => false,
            WAITENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG is disabled in CPU Wait mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAITENW::_0)
    }
    #[doc = "WDOG is enabled in CPU Wait mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAITENW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TESTWDOGW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTWDOGW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TESTSEL`"]
pub enum TESTSELW {
    #[doc = "Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test."]
    _0,
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL\\[1:0\\] for testing."]
    _1,
}
impl TESTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TESTSELW::_0 => false,
            TESTSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TESTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TESTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TESTSELW::_0)
    }
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL\\[1:0\\] for testing."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TESTSELW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYTESEL`"]
pub enum BYTESELW {
    #[doc = "Byte 0 selected"]
    _00,
    #[doc = "Byte 1 selected"]
    _01,
    #[doc = "Byte 2 selected"]
    _10,
    #[doc = "Byte 3 selected"]
    _11,
}
impl BYTESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BYTESELW::_00 => 0,
            BYTESELW::_01 => 1,
            BYTESELW::_10 => 2,
            BYTESELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYTESELW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYTESELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Byte 0 selected"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(BYTESELW::_00)
    }
    #[doc = "Byte 1 selected"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(BYTESELW::_01)
    }
    #[doc = "Byte 2 selected"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(BYTESELW::_10)
    }
    #[doc = "Byte 3 selected"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(BYTESELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISTESTWDOG`"]
pub enum DISTESTWDOGW {
    #[doc = "WDOG functional test mode is not disabled."]
    _0,
    #[doc = "WDOG functional test mode is disabled permanently until reset."]
    _1,
}
impl DISTESTWDOGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISTESTWDOGW::_0 => false,
            DISTESTWDOGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISTESTWDOGW<'a> {
    w: &'a mut W,
}
impl<'a> _DISTESTWDOGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISTESTWDOGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDOG functional test mode is not disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DISTESTWDOGW::_0)
    }
    #[doc = "WDOG functional test mode is disabled permanently until reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DISTESTWDOGW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline]
    pub fn wdogen(&self) -> WDOGENR {
        WDOGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline]
    pub fn irqrsten(&self) -> IRQRSTENR {
        IRQRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Enables Windowing mode."]
    #[inline]
    pub fn winen(&self) -> WINENR {
        WINENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
    #[inline]
    pub fn allowupdate(&self) -> ALLOWUPDATER {
        ALLOWUPDATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Enables or disables WDOG in Stop mode."]
    #[inline]
    pub fn stopen(&self) -> STOPENR {
        STOPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Enables or disables WDOG in Wait mode."]
    #[inline]
    pub fn waiten(&self) -> WAITENR {
        WAITENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline]
    pub fn testwdog(&self) -> TESTWDOGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TESTWDOGR { bits }
    }
    #[doc = "Bit 11 - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
    #[inline]
    pub fn testsel(&self) -> TESTSELR {
        TESTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 12:13 - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
    #[inline]
    pub fn bytesel(&self) -> BYTESELR {
        BYTESELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline]
    pub fn distestwdog(&self) -> DISTESTWDOGR {
        DISTESTWDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 467 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline]
    pub fn wdogen(&mut self) -> _WDOGENW {
        _WDOGENW { w: self }
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline]
    pub fn irqrsten(&mut self) -> _IRQRSTENW {
        _IRQRSTENW { w: self }
    }
    #[doc = "Bit 3 - Enables Windowing mode."]
    #[inline]
    pub fn winen(&mut self) -> _WINENW {
        _WINENW { w: self }
    }
    #[doc = "Bit 4 - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
    #[inline]
    pub fn allowupdate(&mut self) -> _ALLOWUPDATEW {
        _ALLOWUPDATEW { w: self }
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 6 - Enables or disables WDOG in Stop mode."]
    #[inline]
    pub fn stopen(&mut self) -> _STOPENW {
        _STOPENW { w: self }
    }
    #[doc = "Bit 7 - Enables or disables WDOG in Wait mode."]
    #[inline]
    pub fn waiten(&mut self) -> _WAITENW {
        _WAITENW { w: self }
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline]
    pub fn testwdog(&mut self) -> _TESTWDOGW {
        _TESTWDOGW { w: self }
    }
    #[doc = "Bit 11 - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
    #[inline]
    pub fn testsel(&mut self) -> _TESTSELW {
        _TESTSELW { w: self }
    }
    #[doc = "Bits 12:13 - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
    #[inline]
    pub fn bytesel(&mut self) -> _BYTESELW {
        _BYTESELW { w: self }
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline]
    pub fn distestwdog(&mut self) -> _DISTESTWDOGW {
        _DISTESTWDOGW { w: self }
    }
}
