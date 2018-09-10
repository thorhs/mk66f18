#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCSR {
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
#[doc = "Possible values of the field `FRDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDER {
    #[doc = "Disables the DMA request."]
    _0,
    #[doc = "Enables the DMA request."]
    _1,
}
impl FRDER {
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
            FRDER::_0 => false,
            FRDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRDER {
        match value {
            false => FRDER::_0,
            true => FRDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRDER::_1
    }
}
#[doc = "Possible values of the field `FWDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWDER {
    #[doc = "Disables the DMA request."]
    _0,
    #[doc = "Enables the DMA request."]
    _1,
}
impl FWDER {
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
            FWDER::_0 => false,
            FWDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWDER {
        match value {
            false => FWDER::_0,
            true => FWDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FWDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FWDER::_1
    }
}
#[doc = "Possible values of the field `FRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIER {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FRIER {
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
            FRIER::_0 => false,
            FRIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRIER {
        match value {
            false => FRIER::_0,
            true => FRIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRIER::_1
    }
}
#[doc = "Possible values of the field `FWIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWIER {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FWIER {
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
            FWIER::_0 => false,
            FWIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWIER {
        match value {
            false => FWIER::_0,
            true => FWIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FWIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FWIER::_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FEIER {
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
            FEIER::_0 => false,
            FEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::_0,
            true => FEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEIER::_1
    }
}
#[doc = "Possible values of the field `SEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIER {
    #[doc = "Disables interrupt."]
    _0,
    #[doc = "Enables interrupt."]
    _1,
}
impl SEIER {
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
            SEIER::_0 => false,
            SEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEIER {
        match value {
            false => SEIER::_0,
            true => SEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEIER::_1
    }
}
#[doc = "Possible values of the field `WSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSIER {
    #[doc = "Disables interrupt."]
    _0,
    #[doc = "Enables interrupt."]
    _1,
}
impl WSIER {
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
            WSIER::_0 => false,
            WSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSIER {
        match value {
            false => WSIER::_0,
            true => WSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WSIER::_1
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "Receive FIFO watermark not reached."]
    _0,
    #[doc = "Receive FIFO watermark has been reached."]
    _1,
}
impl FRFR {
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
            FRFR::_0 => false,
            FRFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRFR {
        match value {
            false => FRFR::_0,
            true => FRFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRFR::_1
    }
}
#[doc = "Possible values of the field `FWF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWFR {
    #[doc = "No enabled receive FIFO is full."]
    _0,
    #[doc = "Enabled receive FIFO is full."]
    _1,
}
impl FWFR {
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
            FWFR::_0 => false,
            FWFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWFR {
        match value {
            false => FWFR::_0,
            true => FWFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FWFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FWFR::_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "Receive overflow not detected."]
    _0,
    #[doc = "Receive overflow detected."]
    _1,
}
impl FEFR {
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
            FEFR::_0 => false,
            FEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::_0,
            true => FEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEFR::_1
    }
}
#[doc = "Possible values of the field `SEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEFR {
    #[doc = "Sync error not detected."]
    _0,
    #[doc = "Frame sync error detected."]
    _1,
}
impl SEFR {
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
            SEFR::_0 => false,
            SEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEFR {
        match value {
            false => SEFR::_0,
            true => SEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEFR::_1
    }
}
#[doc = "Possible values of the field `WSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSFR {
    #[doc = "Start of word not detected."]
    _0,
    #[doc = "Start of word detected."]
    _1,
}
impl WSFR {
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
            WSFR::_0 => false,
            WSFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSFR {
        match value {
            false => WSFR::_0,
            true => WSFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WSFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WSFR::_1
    }
}
#[doc = "Possible values of the field `SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR {
    #[doc = "No effect."]
    _0,
    #[doc = "Software reset."]
    _1,
}
impl SRR {
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
            SRR::_0 => false,
            SRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRR {
        match value {
            false => SRR::_0,
            true => SRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRR::_1
    }
}
#[doc = "Possible values of the field `BCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCER {
    #[doc = "Receive bit clock is disabled."]
    _0,
    #[doc = "Receive bit clock is enabled."]
    _1,
}
impl BCER {
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
            BCER::_0 => false,
            BCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCER {
        match value {
            false => BCER::_0,
            true => BCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCER::_1
    }
}
#[doc = "Possible values of the field `DBGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGER {
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    _0,
    #[doc = "Receiver is enabled in Debug mode."]
    _1,
}
impl DBGER {
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
            DBGER::_0 => false,
            DBGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGER {
        match value {
            false => DBGER::_0,
            true => DBGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBGER::_1
    }
}
#[doc = "Possible values of the field `STOPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPER {
    #[doc = "Receiver disabled in Stop mode."]
    _0,
    #[doc = "Receiver enabled in Stop mode."]
    _1,
}
impl STOPER {
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
            STOPER::_0 => false,
            STOPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPER {
        match value {
            false => STOPER::_0,
            true => STOPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPER::_1
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver is disabled."]
    _0,
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    _1,
}
impl RER {
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
            RER::_0 => false,
            RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::_0,
            true => RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RER::_1
    }
}
#[doc = "Values that can be written to the field `FRDE`"]
pub enum FRDEW {
    #[doc = "Disables the DMA request."]
    _0,
    #[doc = "Enables the DMA request."]
    _1,
}
impl FRDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRDEW::_0 => false,
            FRDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRDEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDEW::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FWDE`"]
pub enum FWDEW {
    #[doc = "Disables the DMA request."]
    _0,
    #[doc = "Enables the DMA request."]
    _1,
}
impl FWDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWDEW::_0 => false,
            FWDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWDEW<'a> {
    w: &'a mut W,
}
impl<'a> _FWDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWDEW::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWDEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRIE`"]
pub enum FRIEW {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRIEW::_0 => false,
            FRIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIEW::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIEW::_1)
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
#[doc = "Values that can be written to the field `FWIE`"]
pub enum FWIEW {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FWIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWIEW::_0 => false,
            FWIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FWIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWIEW::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWIEW::_1)
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
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "Disables the interrupt."]
    _0,
    #[doc = "Enables the interrupt."]
    _1,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::_0 => false,
            FEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIEW::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIEW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEIE`"]
pub enum SEIEW {
    #[doc = "Disables interrupt."]
    _0,
    #[doc = "Enables interrupt."]
    _1,
}
impl SEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEIEW::_0 => false,
            SEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIEW::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WSIE`"]
pub enum WSIEW {
    #[doc = "Disables interrupt."]
    _0,
    #[doc = "Enables interrupt."]
    _1,
}
impl WSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSIEW::_0 => false,
            WSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSIEW::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSIEW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "Receive overflow not detected."]
    _0,
    #[doc = "Receive overflow detected."]
    _1,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::_0 => false,
            FEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive overflow not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEFW::_0)
    }
    #[doc = "Receive overflow detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEFW::_1)
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
#[doc = "Values that can be written to the field `SEF`"]
pub enum SEFW {
    #[doc = "Sync error not detected."]
    _0,
    #[doc = "Frame sync error detected."]
    _1,
}
impl SEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEFW::_0 => false,
            SEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEFW<'a> {
    w: &'a mut W,
}
impl<'a> _SEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sync error not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEFW::_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEFW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WSF`"]
pub enum WSFW {
    #[doc = "Start of word not detected."]
    _0,
    #[doc = "Start of word detected."]
    _1,
}
impl WSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSFW::_0 => false,
            WSFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSFW<'a> {
    w: &'a mut W,
}
impl<'a> _WSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start of word not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSFW::_0)
    }
    #[doc = "Start of word detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSFW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "No effect."]
    _0,
    #[doc = "Software reset."]
    _1,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::_0 => false,
            SRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRW::_0)
    }
    #[doc = "Software reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FR`"]
pub enum FRW {
    #[doc = "No effect."]
    _0,
    #[doc = "FIFO reset."]
    _1,
}
impl FRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRW::_0 => false,
            FRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRW::_0)
    }
    #[doc = "FIFO reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCE`"]
pub enum BCEW {
    #[doc = "Receive bit clock is disabled."]
    _0,
    #[doc = "Receive bit clock is enabled."]
    _1,
}
impl BCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCEW::_0 => false,
            BCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCEW<'a> {
    w: &'a mut W,
}
impl<'a> _BCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive bit clock is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCEW::_0)
    }
    #[doc = "Receive bit clock is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCEW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGE`"]
pub enum DBGEW {
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    _0,
    #[doc = "Receiver is enabled in Debug mode."]
    _1,
}
impl DBGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGEW::_0 => false,
            DBGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEW::_0)
    }
    #[doc = "Receiver is enabled in Debug mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPE`"]
pub enum STOPEW {
    #[doc = "Receiver disabled in Stop mode."]
    _0,
    #[doc = "Receiver enabled in Stop mode."]
    _1,
}
impl STOPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPEW::_0 => false,
            STOPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPEW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver disabled in Stop mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPEW::_0)
    }
    #[doc = "Receiver enabled in Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPEW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "Receiver is disabled."]
    _0,
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    _1,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::_0 => false,
            REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REW::_0)
    }
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline]
    pub fn frde(&self) -> FRDER {
        FRDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline]
    pub fn fwde(&self) -> FWDER {
        FWDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline]
    pub fn frie(&self) -> FRIER {
        FRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline]
    pub fn fwie(&self) -> FWIER {
        FWIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline]
    pub fn seie(&self) -> SEIER {
        SEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline]
    pub fn wsie(&self) -> WSIER {
        WSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline]
    pub fn fwf(&self) -> FWFR {
        FWFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline]
    pub fn sef(&self) -> SEFR {
        SEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline]
    pub fn wsf(&self) -> WSFR {
        WSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline]
    pub fn bce(&self) -> BCER {
        BCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline]
    pub fn dbge(&self) -> DBGER {
        DBGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline]
    pub fn stope(&self) -> STOPER {
        STOPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline]
    pub fn frde(&mut self) -> _FRDEW {
        _FRDEW { w: self }
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline]
    pub fn fwde(&mut self) -> _FWDEW {
        _FWDEW { w: self }
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline]
    pub fn frie(&mut self) -> _FRIEW {
        _FRIEW { w: self }
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline]
    pub fn fwie(&mut self) -> _FWIEW {
        _FWIEW { w: self }
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline]
    pub fn seie(&mut self) -> _SEIEW {
        _SEIEW { w: self }
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline]
    pub fn wsie(&mut self) -> _WSIEW {
        _WSIEW { w: self }
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline]
    pub fn sef(&mut self) -> _SEFW {
        _SEFW { w: self }
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline]
    pub fn wsf(&mut self) -> _WSFW {
        _WSFW { w: self }
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline]
    pub fn fr(&mut self) -> _FRW {
        _FRW { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline]
    pub fn bce(&mut self) -> _BCEW {
        _BCEW { w: self }
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline]
    pub fn dbge(&mut self) -> _DBGEW {
        _DBGEW { w: self }
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline]
    pub fn stope(&mut self) -> _STOPEW {
        _STOPEW { w: self }
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
}
