#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OTGSC {
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
#[doc = r" Value of the field"]
pub struct VDR {
    bits: bool,
}
impl VDR {
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
#[doc = r" Value of the field"]
pub struct VCR {
    bits: bool,
}
impl VCR {
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
#[doc = "Possible values of the field `HAAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAARR {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enable automatic reset after connect on host port."]
    _1,
}
impl HAARR {
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
            HAARR::_0 => false,
            HAARR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HAARR {
        match value {
            false => HAARR::_0,
            true => HAARR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HAARR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HAARR::_1
    }
}
#[doc = "Possible values of the field `OT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTR {
    #[doc = "Disable pull-down on DM"]
    _0,
    #[doc = "Enable pull-down on DM"]
    _1,
}
impl OTR {
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
            OTR::_0 => false,
            OTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTR {
        match value {
            false => OTR::_0,
            true => OTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OTR::_1
    }
}
#[doc = "Possible values of the field `DP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPR {
    #[doc = "The pull-up on DP is not asserted"]
    _0,
    #[doc = "The pull-up on DP is asserted for data pulsing during SRP"]
    _1,
}
impl DPR {
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
            DPR::_0 => false,
            DPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPR {
        match value {
            false => DPR::_0,
            true => DPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPR::_1
    }
}
#[doc = "Possible values of the field `IDPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDPUR {
    #[doc = "Disable pull-up. ID input not sampled."]
    _0,
    #[doc = "Enable pull-up"]
    _1,
}
impl IDPUR {
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
            IDPUR::_0 => false,
            IDPUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDPUR {
        match value {
            false => IDPUR::_0,
            true => IDPUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDPUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDPUR::_1
    }
}
#[doc = "Possible values of the field `HABA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HABAR {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    _1,
}
impl HABAR {
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
            HABAR::_0 => false,
            HABAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HABAR {
        match value {
            false => HABAR::_0,
            true => HABAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HABAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HABAR::_1
    }
}
#[doc = "Possible values of the field `ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR {
    #[doc = "A device"]
    _0,
    #[doc = "B device"]
    _1,
}
impl IDR {
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
            IDR::_0 => false,
            IDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDR {
        match value {
            false => IDR::_0,
            true => IDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDR::_1
    }
}
#[doc = "Possible values of the field `AVV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVVR {
    #[doc = "VBus is below A VBus valid threshold"]
    _0,
    #[doc = "VBus is above A VBus valid threshold"]
    _1,
}
impl AVVR {
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
            AVVR::_0 => false,
            AVVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVVR {
        match value {
            false => AVVR::_0,
            true => AVVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVVR::_1
    }
}
#[doc = "Possible values of the field `ASV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASVR {
    #[doc = "VBus is below A session valid threshold"]
    _0,
    #[doc = "VBus is above A session valid threshold"]
    _1,
}
impl ASVR {
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
            ASVR::_0 => false,
            ASVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASVR {
        match value {
            false => ASVR::_0,
            true => ASVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASVR::_1
    }
}
#[doc = "Possible values of the field `BSV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSVR {
    #[doc = "VBus is below B session valid threshold"]
    _0,
    #[doc = "VBus is above B session valid threshold"]
    _1,
}
impl BSVR {
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
            BSVR::_0 => false,
            BSVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSVR {
        match value {
            false => BSVR::_0,
            true => BSVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSVR::_1
    }
}
#[doc = "Possible values of the field `BSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSER {
    #[doc = "VBus is above B session end threshold"]
    _0,
    #[doc = "VBus is below B session end threshold"]
    _1,
}
impl BSER {
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
            BSER::_0 => false,
            BSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSER {
        match value {
            false => BSER::_0,
            true => BSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSER::_1
    }
}
#[doc = r" Value of the field"]
pub struct MSTR {
    bits: bool,
}
impl MSTR {
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
#[doc = "Possible values of the field `DPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSR {
    #[doc = "No pulsing on port"]
    _0,
    #[doc = "Pulsing detected on port"]
    _1,
}
impl DPSR {
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
            DPSR::_0 => false,
            DPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPSR {
        match value {
            false => DPSR::_0,
            true => DPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct IDISR {
    bits: bool,
}
impl IDISR {
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
#[doc = r" Value of the field"]
pub struct AVVISR {
    bits: bool,
}
impl AVVISR {
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
#[doc = r" Value of the field"]
pub struct ASVISR {
    bits: bool,
}
impl ASVISR {
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
#[doc = r" Value of the field"]
pub struct BSVISR {
    bits: bool,
}
impl BSVISR {
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
#[doc = r" Value of the field"]
pub struct BSEISR {
    bits: bool,
}
impl BSEISR {
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
#[doc = r" Value of the field"]
pub struct MSSR {
    bits: bool,
}
impl MSSR {
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
#[doc = r" Value of the field"]
pub struct DPISR {
    bits: bool,
}
impl DPISR {
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
#[doc = "Possible values of the field `IDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl IDIER {
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
            IDIER::_0 => false,
            IDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDIER {
        match value {
            false => IDIER::_0,
            true => IDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDIER::_1
    }
}
#[doc = "Possible values of the field `AVVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVVIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl AVVIER {
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
            AVVIER::_0 => false,
            AVVIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVVIER {
        match value {
            false => AVVIER::_0,
            true => AVVIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVVIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVVIER::_1
    }
}
#[doc = "Possible values of the field `ASVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASVIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl ASVIER {
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
            ASVIER::_0 => false,
            ASVIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASVIER {
        match value {
            false => ASVIER::_0,
            true => ASVIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASVIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASVIER::_1
    }
}
#[doc = "Possible values of the field `BSVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSVIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BSVIER {
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
            BSVIER::_0 => false,
            BSVIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSVIER {
        match value {
            false => BSVIER::_0,
            true => BSVIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSVIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSVIER::_1
    }
}
#[doc = "Possible values of the field `BSEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSEIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BSEIER {
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
            BSEIER::_0 => false,
            BSEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSEIER {
        match value {
            false => BSEIER::_0,
            true => BSEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSEIER::_1
    }
}
#[doc = "Possible values of the field `MSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl MSER {
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
            MSER::_0 => false,
            MSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSER {
        match value {
            false => MSER::_0,
            true => MSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSER::_1
    }
}
#[doc = "Possible values of the field `DPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIER {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl DPIER {
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
            DPIER::_0 => false,
            DPIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPIER {
        match value {
            false => DPIER::_0,
            true => DPIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPIER::_1
    }
}
#[doc = r" Proxy"]
pub struct _VDW<'a> {
    w: &'a mut W,
}
impl<'a> _VDW<'a> {
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
#[doc = r" Proxy"]
pub struct _VCW<'a> {
    w: &'a mut W,
}
impl<'a> _VCW<'a> {
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
#[doc = "Values that can be written to the field `HAAR`"]
pub enum HAARW {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enable automatic reset after connect on host port."]
    _1,
}
impl HAARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAARW::_0 => false,
            HAARW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAARW<'a> {
    w: &'a mut W,
}
impl<'a> _HAARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HAARW::_0)
    }
    #[doc = "Enable automatic reset after connect on host port."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HAARW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT`"]
pub enum OTW {
    #[doc = "Disable pull-down on DM"]
    _0,
    #[doc = "Enable pull-down on DM"]
    _1,
}
impl OTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTW::_0 => false,
            OTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable pull-down on DM"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTW::_0)
    }
    #[doc = "Enable pull-down on DM"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DP`"]
pub enum DPW {
    #[doc = "The pull-up on DP is not asserted"]
    _0,
    #[doc = "The pull-up on DP is asserted for data pulsing during SRP"]
    _1,
}
impl DPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPW::_0 => false,
            DPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPW<'a> {
    w: &'a mut W,
}
impl<'a> _DPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The pull-up on DP is not asserted"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPW::_0)
    }
    #[doc = "The pull-up on DP is asserted for data pulsing during SRP"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDPU`"]
pub enum IDPUW {
    #[doc = "Disable pull-up. ID input not sampled."]
    _0,
    #[doc = "Enable pull-up"]
    _1,
}
impl IDPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDPUW::_0 => false,
            IDPUW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDPUW<'a> {
    w: &'a mut W,
}
impl<'a> _IDPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDPUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable pull-up. ID input not sampled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPUW::_0)
    }
    #[doc = "Enable pull-up"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPUW::_1)
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
#[doc = "Values that can be written to the field `HABA`"]
pub enum HABAW {
    #[doc = "Disabled."]
    _0,
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    _1,
}
impl HABAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HABAW::_0 => false,
            HABAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HABAW<'a> {
    w: &'a mut W,
}
impl<'a> _HABAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HABAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HABAW::_0)
    }
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HABAW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDISW<'a> {
    w: &'a mut W,
}
impl<'a> _IDISW<'a> {
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
#[doc = r" Proxy"]
pub struct _AVVISW<'a> {
    w: &'a mut W,
}
impl<'a> _AVVISW<'a> {
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
#[doc = r" Proxy"]
pub struct _ASVISW<'a> {
    w: &'a mut W,
}
impl<'a> _ASVISW<'a> {
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
pub struct _BSVISW<'a> {
    w: &'a mut W,
}
impl<'a> _BSVISW<'a> {
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
#[doc = r" Proxy"]
pub struct _BSEISW<'a> {
    w: &'a mut W,
}
impl<'a> _BSEISW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSSW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPISW<'a> {
    w: &'a mut W,
}
impl<'a> _DPISW<'a> {
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
#[doc = "Values that can be written to the field `IDIE`"]
pub enum IDIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl IDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDIEW::_0 => false,
            IDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDIEW::_1)
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
#[doc = "Values that can be written to the field `AVVIE`"]
pub enum AVVIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl AVVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVVIEW::_0 => false,
            AVVIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVVIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVVIEW::_1)
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
#[doc = "Values that can be written to the field `ASVIE`"]
pub enum ASVIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl ASVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASVIEW::_0 => false,
            ASVIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASVIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASVIEW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BSVIE`"]
pub enum BSVIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BSVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSVIEW::_0 => false,
            BSVIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSVIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSVIEW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BSEIE`"]
pub enum BSEIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BSEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSEIEW::_0 => false,
            BSEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSEIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSEIEW::_1)
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
#[doc = "Values that can be written to the field `MSE`"]
pub enum MSEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl MSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSEW::_0 => false,
            MSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSEW::_1)
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
#[doc = "Values that can be written to the field `DPIE`"]
pub enum DPIEW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl DPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPIEW::_0 => false,
            DPIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPIEW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPIEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VBUS Discharge"]
    #[inline]
    pub fn vd(&self) -> VDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDR { bits }
    }
    #[doc = "Bit 1 - VBUS Charge"]
    #[inline]
    pub fn vc(&self) -> VCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VCR { bits }
    }
    #[doc = "Bit 2 - Hardware Assist Auto-Reset"]
    #[inline]
    pub fn haar(&self) -> HAARR {
        HAARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OTG Termination"]
    #[inline]
    pub fn ot(&self) -> OTR {
        OTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data Pulsing"]
    #[inline]
    pub fn dp(&self) -> DPR {
        DPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - ID Pull-Up"]
    #[inline]
    pub fn idpu(&self) -> IDPUR {
        IDPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Hardware Assist B-Disconnect to A-connect"]
    #[inline]
    pub fn haba(&self) -> HABAR {
        HABAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - USB ID"]
    #[inline]
    pub fn id(&self) -> IDR {
        IDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - A VBus Valid"]
    #[inline]
    pub fn avv(&self) -> AVVR {
        AVVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - A Session Valid"]
    #[inline]
    pub fn asv(&self) -> ASVR {
        ASVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - B Session Valid"]
    #[inline]
    pub fn bsv(&self) -> BSVR {
        BSVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - B Session End"]
    #[inline]
    pub fn bse(&self) -> BSER {
        BSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - 1 Milli-Second timer Toggle"]
    #[inline]
    pub fn mst(&self) -> MSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSTR { bits }
    }
    #[doc = "Bit 14 - Data bus Pulsing Status"]
    #[inline]
    pub fn dps(&self) -> DPSR {
        DPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB ID Interrupt Status"]
    #[inline]
    pub fn idis(&self) -> IDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDISR { bits }
    }
    #[doc = "Bit 17 - A VBUS Valid Interrupt Status"]
    #[inline]
    pub fn avvis(&self) -> AVVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AVVISR { bits }
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status"]
    #[inline]
    pub fn asvis(&self) -> ASVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASVISR { bits }
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status"]
    #[inline]
    pub fn bsvis(&self) -> BSVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSVISR { bits }
    }
    #[doc = "Bit 20 - B Session End Interrupt Status"]
    #[inline]
    pub fn bseis(&self) -> BSEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSEISR { bits }
    }
    #[doc = "Bit 21 - 1 Milli-Second timer interrupt Status"]
    #[inline]
    pub fn mss(&self) -> MSSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSSR { bits }
    }
    #[doc = "Bit 22 - Data Pulse interrupt Status"]
    #[inline]
    pub fn dpis(&self) -> DPISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPISR { bits }
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable"]
    #[inline]
    pub fn idie(&self) -> IDIER {
        IDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - A VBUS Valid Interrupt Enable"]
    #[inline]
    pub fn avvie(&self) -> AVVIER {
        AVVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable"]
    #[inline]
    pub fn asvie(&self) -> ASVIER {
        ASVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable"]
    #[inline]
    pub fn bsvie(&self) -> BSVIER {
        BSVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable"]
    #[inline]
    pub fn bseie(&self) -> BSEIER {
        BSEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - 1 Milli-Second timer interrupt Enable"]
    #[inline]
    pub fn mse(&self) -> MSER {
        MSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline]
    pub fn dpie(&self) -> DPIER {
        DPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - VBUS Discharge"]
    #[inline]
    pub fn vd(&mut self) -> _VDW {
        _VDW { w: self }
    }
    #[doc = "Bit 1 - VBUS Charge"]
    #[inline]
    pub fn vc(&mut self) -> _VCW {
        _VCW { w: self }
    }
    #[doc = "Bit 2 - Hardware Assist Auto-Reset"]
    #[inline]
    pub fn haar(&mut self) -> _HAARW {
        _HAARW { w: self }
    }
    #[doc = "Bit 3 - OTG Termination"]
    #[inline]
    pub fn ot(&mut self) -> _OTW {
        _OTW { w: self }
    }
    #[doc = "Bit 4 - Data Pulsing"]
    #[inline]
    pub fn dp(&mut self) -> _DPW {
        _DPW { w: self }
    }
    #[doc = "Bit 5 - ID Pull-Up"]
    #[inline]
    pub fn idpu(&mut self) -> _IDPUW {
        _IDPUW { w: self }
    }
    #[doc = "Bit 7 - Hardware Assist B-Disconnect to A-connect"]
    #[inline]
    pub fn haba(&mut self) -> _HABAW {
        _HABAW { w: self }
    }
    #[doc = "Bit 16 - USB ID Interrupt Status"]
    #[inline]
    pub fn idis(&mut self) -> _IDISW {
        _IDISW { w: self }
    }
    #[doc = "Bit 17 - A VBUS Valid Interrupt Status"]
    #[inline]
    pub fn avvis(&mut self) -> _AVVISW {
        _AVVISW { w: self }
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status"]
    #[inline]
    pub fn asvis(&mut self) -> _ASVISW {
        _ASVISW { w: self }
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status"]
    #[inline]
    pub fn bsvis(&mut self) -> _BSVISW {
        _BSVISW { w: self }
    }
    #[doc = "Bit 20 - B Session End Interrupt Status"]
    #[inline]
    pub fn bseis(&mut self) -> _BSEISW {
        _BSEISW { w: self }
    }
    #[doc = "Bit 21 - 1 Milli-Second timer interrupt Status"]
    #[inline]
    pub fn mss(&mut self) -> _MSSW {
        _MSSW { w: self }
    }
    #[doc = "Bit 22 - Data Pulse interrupt Status"]
    #[inline]
    pub fn dpis(&mut self) -> _DPISW {
        _DPISW { w: self }
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable"]
    #[inline]
    pub fn idie(&mut self) -> _IDIEW {
        _IDIEW { w: self }
    }
    #[doc = "Bit 25 - A VBUS Valid Interrupt Enable"]
    #[inline]
    pub fn avvie(&mut self) -> _AVVIEW {
        _AVVIEW { w: self }
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable"]
    #[inline]
    pub fn asvie(&mut self) -> _ASVIEW {
        _ASVIEW { w: self }
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable"]
    #[inline]
    pub fn bsvie(&mut self) -> _BSVIEW {
        _BSVIEW { w: self }
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable"]
    #[inline]
    pub fn bseie(&mut self) -> _BSEIEW {
        _BSEIEW { w: self }
    }
    #[doc = "Bit 29 - 1 Milli-Second timer interrupt Enable"]
    #[inline]
    pub fn mse(&mut self) -> _MSEW {
        _MSEW { w: self }
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline]
    pub fn dpie(&mut self) -> _DPIEW {
        _DPIEW { w: self }
    }
}
