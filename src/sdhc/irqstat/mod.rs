#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSTAT {
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
#[doc = "Possible values of the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCR {
    #[doc = "Command not complete."]
    _0,
    #[doc = "Command complete."]
    _1,
}
impl CCR {
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
            CCR::_0 => false,
            CCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCR {
        match value {
            false => CCR::_0,
            true => CCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCR::_1
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transfer not complete."]
    _0,
    #[doc = "Transfer complete."]
    _1,
}
impl TCR {
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
            TCR::_0 => false,
            TCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::_0,
            true => TCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCR::_1
    }
}
#[doc = "Possible values of the field `BGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGER {
    #[doc = "No block gap event."]
    _0,
    #[doc = "Transaction stopped at block gap."]
    _1,
}
impl BGER {
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
            BGER::_0 => false,
            BGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGER {
        match value {
            false => BGER::_0,
            true => BGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGER::_1
    }
}
#[doc = "Possible values of the field `DINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTR {
    #[doc = "No DMA Interrupt."]
    _0,
    #[doc = "DMA Interrupt is generated."]
    _1,
}
impl DINTR {
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
            DINTR::_0 => false,
            DINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTR {
        match value {
            false => DINTR::_0,
            true => DINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DINTR::_1
    }
}
#[doc = "Possible values of the field `BWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRR {
    #[doc = "Not ready to write buffer."]
    _0,
    #[doc = "Ready to write buffer."]
    _1,
}
impl BWRR {
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
            BWRR::_0 => false,
            BWRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRR {
        match value {
            false => BWRR::_0,
            true => BWRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWRR::_1
    }
}
#[doc = "Possible values of the field `BRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRR {
    #[doc = "Not ready to read buffer."]
    _0,
    #[doc = "Ready to read buffer."]
    _1,
}
impl BRRR {
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
            BRRR::_0 => false,
            BRRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRR {
        match value {
            false => BRRR::_0,
            true => BRRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRRR::_1
    }
}
#[doc = "Possible values of the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSR {
    #[doc = "Card state unstable or removed."]
    _0,
    #[doc = "Card inserted."]
    _1,
}
impl CINSR {
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
            CINSR::_0 => false,
            CINSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSR {
        match value {
            false => CINSR::_0,
            true => CINSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINSR::_1
    }
}
#[doc = "Possible values of the field `CRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMR {
    #[doc = "Card state unstable or inserted."]
    _0,
    #[doc = "Card removed."]
    _1,
}
impl CRMR {
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
            CRMR::_0 => false,
            CRMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMR {
        match value {
            false => CRMR::_0,
            true => CRMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRMR::_1
    }
}
#[doc = "Possible values of the field `CINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTR {
    #[doc = "No Card Interrupt."]
    _0,
    #[doc = "Generate Card Interrupt."]
    _1,
}
impl CINTR {
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
            CINTR::_0 => false,
            CINTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTR {
        match value {
            false => CINTR::_0,
            true => CINTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINTR::_1
    }
}
#[doc = "Possible values of the field `CTOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOER {
    #[doc = "No error."]
    _0,
    #[doc = "Time out."]
    _1,
}
impl CTOER {
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
            CTOER::_0 => false,
            CTOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOER {
        match value {
            false => CTOER::_0,
            true => CTOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTOER::_1
    }
}
#[doc = "Possible values of the field `CCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCER {
    #[doc = "No error."]
    _0,
    #[doc = "CRC Error generated."]
    _1,
}
impl CCER {
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
            CCER::_0 => false,
            CCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCER {
        match value {
            false => CCER::_0,
            true => CCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCER::_1
    }
}
#[doc = "Possible values of the field `CEBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBER {
    #[doc = "No error."]
    _0,
    #[doc = "End Bit Error generated."]
    _1,
}
impl CEBER {
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
            CEBER::_0 => false,
            CEBER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBER {
        match value {
            false => CEBER::_0,
            true => CEBER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CEBER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CEBER::_1
    }
}
#[doc = "Possible values of the field `CIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl CIER {
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
            CIER::_0 => false,
            CIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIER {
        match value {
            false => CIER::_0,
            true => CIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CIER::_1
    }
}
#[doc = "Possible values of the field `DTOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOER {
    #[doc = "No error."]
    _0,
    #[doc = "Time out."]
    _1,
}
impl DTOER {
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
            DTOER::_0 => false,
            DTOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOER {
        match value {
            false => DTOER::_0,
            true => DTOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTOER::_1
    }
}
#[doc = "Possible values of the field `DCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DCER {
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
            DCER::_0 => false,
            DCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCER {
        match value {
            false => DCER::_0,
            true => DCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCER::_1
    }
}
#[doc = "Possible values of the field `DEBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DEBER {
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
            DEBER::_0 => false,
            DEBER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBER {
        match value {
            false => DEBER::_0,
            true => DEBER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DEBER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DEBER::_1
    }
}
#[doc = "Possible values of the field `AC12E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl AC12ER {
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
            AC12ER::_0 => false,
            AC12ER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ER {
        match value {
            false => AC12ER::_0,
            true => AC12ER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12ER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12ER::_1
    }
}
#[doc = "Possible values of the field `DMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DMAER {
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
            DMAER::_0 => false,
            DMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAER {
        match value {
            false => DMAER::_0,
            true => DMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAER::_1
    }
}
#[doc = "Values that can be written to the field `CC`"]
pub enum CCW {
    #[doc = "Command not complete."]
    _0,
    #[doc = "Command complete."]
    _1,
}
impl CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCW::_0 => false,
            CCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command not complete."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCW::_0)
    }
    #[doc = "Command complete."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCW::_1)
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
#[doc = "Values that can be written to the field `TC`"]
pub enum TCW {
    #[doc = "Transfer not complete."]
    _0,
    #[doc = "Transfer complete."]
    _1,
}
impl TCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCW::_0 => false,
            TCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer not complete."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCW::_0)
    }
    #[doc = "Transfer complete."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCW::_1)
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
#[doc = "Values that can be written to the field `BGE`"]
pub enum BGEW {
    #[doc = "No block gap event."]
    _0,
    #[doc = "Transaction stopped at block gap."]
    _1,
}
impl BGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGEW::_0 => false,
            BGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No block gap event."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEW::_0)
    }
    #[doc = "Transaction stopped at block gap."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEW::_1)
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
#[doc = "Values that can be written to the field `DINT`"]
pub enum DINTW {
    #[doc = "No DMA Interrupt."]
    _0,
    #[doc = "DMA Interrupt is generated."]
    _1,
}
impl DINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTW::_0 => false,
            DINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA Interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTW::_0)
    }
    #[doc = "DMA Interrupt is generated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTW::_1)
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
#[doc = "Values that can be written to the field `BWR`"]
pub enum BWRW {
    #[doc = "Not ready to write buffer."]
    _0,
    #[doc = "Ready to write buffer."]
    _1,
}
impl BWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRW::_0 => false,
            BWRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready to write buffer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRW::_0)
    }
    #[doc = "Ready to write buffer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRW::_1)
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
#[doc = "Values that can be written to the field `BRR`"]
pub enum BRRW {
    #[doc = "Not ready to read buffer."]
    _0,
    #[doc = "Ready to read buffer."]
    _1,
}
impl BRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRW::_0 => false,
            BRRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready to read buffer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRW::_0)
    }
    #[doc = "Ready to read buffer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRW::_1)
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
#[doc = "Values that can be written to the field `CINS`"]
pub enum CINSW {
    #[doc = "Card state unstable or removed."]
    _0,
    #[doc = "Card inserted."]
    _1,
}
impl CINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSW::_0 => false,
            CINSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card state unstable or removed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSW::_0)
    }
    #[doc = "Card inserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRM`"]
pub enum CRMW {
    #[doc = "Card state unstable or inserted."]
    _0,
    #[doc = "Card removed."]
    _1,
}
impl CRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMW::_0 => false,
            CRMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card state unstable or inserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMW::_0)
    }
    #[doc = "Card removed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMW::_1)
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
#[doc = "Values that can be written to the field `CINT`"]
pub enum CINTW {
    #[doc = "No Card Interrupt."]
    _0,
    #[doc = "Generate Card Interrupt."]
    _1,
}
impl CINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTW::_0 => false,
            CINTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Card Interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTW::_0)
    }
    #[doc = "Generate Card Interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTW::_1)
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
#[doc = "Values that can be written to the field `CTOE`"]
pub enum CTOEW {
    #[doc = "No error."]
    _0,
    #[doc = "Time out."]
    _1,
}
impl CTOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOEW::_0 => false,
            CTOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOEW::_0)
    }
    #[doc = "Time out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOEW::_1)
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
#[doc = "Values that can be written to the field `CCE`"]
pub enum CCEW {
    #[doc = "No error."]
    _0,
    #[doc = "CRC Error generated."]
    _1,
}
impl CCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEW::_0 => false,
            CCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCEW::_0)
    }
    #[doc = "CRC Error generated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCEW::_1)
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
#[doc = "Values that can be written to the field `CEBE`"]
pub enum CEBEW {
    #[doc = "No error."]
    _0,
    #[doc = "End Bit Error generated."]
    _1,
}
impl CEBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBEW::_0 => false,
            CEBEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBEW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBEW::_0)
    }
    #[doc = "End Bit Error generated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBEW::_1)
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
#[doc = "Values that can be written to the field `CIE`"]
pub enum CIEW {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl CIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIEW::_0 => false,
            CIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIEW::_0)
    }
    #[doc = "Error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIEW::_1)
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
#[doc = "Values that can be written to the field `DTOE`"]
pub enum DTOEW {
    #[doc = "No error."]
    _0,
    #[doc = "Time out."]
    _1,
}
impl DTOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOEW::_0 => false,
            DTOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOEW::_0)
    }
    #[doc = "Time out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOEW::_1)
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
#[doc = "Values that can be written to the field `DCE`"]
pub enum DCEW {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEW::_0 => false,
            DCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCEW::_0)
    }
    #[doc = "Error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCEW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEBE`"]
pub enum DEBEW {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DEBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBEW::_0 => false,
            DEBEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBEW::_0)
    }
    #[doc = "Error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBEW::_1)
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
#[doc = "Values that can be written to the field `AC12E`"]
pub enum AC12EW {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl AC12EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12EW::_0 => false,
            AC12EW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12EW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EW::_0)
    }
    #[doc = "Error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EW::_1)
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
#[doc = "Values that can be written to the field `DMAE`"]
pub enum DMAEW {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl DMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEW::_0 => false,
            DMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEW::_0)
    }
    #[doc = "Error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cc(&self) -> CCR {
        CCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn bge(&self) -> BGER {
        BGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dint(&self) -> DINTR {
        DINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwr(&self) -> BWRR {
        BWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brr(&self) -> BRRR {
        BRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&self) -> CINSR {
        CINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crm(&self) -> CRMR {
        CRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn cint(&self) -> CINTR {
        CINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline]
    pub fn ctoe(&self) -> CTOER {
        CTOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline]
    pub fn cce(&self) -> CCER {
        CCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline]
    pub fn cebe(&self) -> CEBER {
        CEBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline]
    pub fn cie(&self) -> CIER {
        CIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline]
    pub fn dtoe(&self) -> DTOER {
        DTOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline]
    pub fn dce(&self) -> DCER {
        DCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline]
    pub fn debe(&self) -> DEBER {
        DEBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline]
    pub fn ac12e(&self) -> AC12ER {
        AC12ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline]
    pub fn dmae(&self) -> DMAER {
        DMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn tc(&mut self) -> _TCW {
        _TCW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn bge(&mut self) -> _BGEW {
        _BGEW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dint(&mut self) -> _DINTW {
        _DINTW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwr(&mut self) -> _BWRW {
        _BWRW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brr(&mut self) -> _BRRW {
        _BRRW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&mut self) -> _CINSW {
        _CINSW { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crm(&mut self) -> _CRMW {
        _CRMW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn cint(&mut self) -> _CINTW {
        _CINTW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline]
    pub fn ctoe(&mut self) -> _CTOEW {
        _CTOEW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline]
    pub fn cebe(&mut self) -> _CEBEW {
        _CEBEW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline]
    pub fn cie(&mut self) -> _CIEW {
        _CIEW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline]
    pub fn dtoe(&mut self) -> _DTOEW {
        _DTOEW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline]
    pub fn dce(&mut self) -> _DCEW {
        _DCEW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline]
    pub fn debe(&mut self) -> _DEBEW {
        _DEBEW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline]
    pub fn ac12e(&mut self) -> _AC12EW {
        _AC12EW { w: self }
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline]
    pub fn dmae(&mut self) -> _DMAEW {
        _DMAEW { w: self }
    }
}
