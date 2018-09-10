#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C1 {
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
#[doc = "Possible values of the field `PT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTR {
    #[doc = "Even parity."]
    _0,
    #[doc = "Odd parity."]
    _1,
}
impl PTR {
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
            PTR::_0 => false,
            PTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTR {
        match value {
            false => PTR::_0,
            true => PTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTR::_1
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Parity function disabled."]
    _0,
    #[doc = "Parity function enabled."]
    _1,
}
impl PER {
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
            PER::_0 => false,
            PER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::_0,
            true => PER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PER::_1
    }
}
#[doc = "Possible values of the field `ILT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILTR {
    #[doc = "Idle character bit count starts after start bit."]
    _0,
    #[doc = "Idle character bit count starts after stop bit."]
    _1,
}
impl ILTR {
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
            ILTR::_0 => false,
            ILTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILTR {
        match value {
            false => ILTR::_0,
            true => ILTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ILTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ILTR::_1
    }
}
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "Idle line wakeup."]
    _0,
    #[doc = "Address mark wakeup."]
    _1,
}
impl WAKER {
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
            WAKER::_0 => false,
            WAKER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::_0,
            true => WAKER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKER::_1
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _0,
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _1,
}
impl MR {
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
            MR::_0 => false,
            MR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::_0,
            true => MR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MR::_1
    }
}
#[doc = "Possible values of the field `RSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRCR {
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    _0,
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    _1,
}
impl RSRCR {
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
            RSRCR::_0 => false,
            RSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSRCR {
        match value {
            false => RSRCR::_0,
            true => RSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSRCR::_1
    }
}
#[doc = "Possible values of the field `UARTSWAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTSWAIR {
    #[doc = "UART clock continues to run in Wait mode."]
    _0,
    #[doc = "UART clock freezes while CPU is in Wait mode."]
    _1,
}
impl UARTSWAIR {
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
            UARTSWAIR::_0 => false,
            UARTSWAIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTSWAIR {
        match value {
            false => UARTSWAIR::_0,
            true => UARTSWAIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UARTSWAIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UARTSWAIR::_1
    }
}
#[doc = "Possible values of the field `LOOPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    _1,
}
impl LOOPSR {
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
            LOOPSR::_0 => false,
            LOOPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSR {
        match value {
            false => LOOPSR::_0,
            true => LOOPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOOPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOOPSR::_1
    }
}
#[doc = "Values that can be written to the field `PT`"]
pub enum PTW {
    #[doc = "Even parity."]
    _0,
    #[doc = "Odd parity."]
    _1,
}
impl PTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTW::_0 => false,
            PTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTW<'a> {
    w: &'a mut W,
}
impl<'a> _PTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTW::_0)
    }
    #[doc = "Odd parity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Parity function disabled."]
    _0,
    #[doc = "Parity function enabled."]
    _1,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::_0 => false,
            PEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity function disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEW::_0)
    }
    #[doc = "Parity function enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ILT`"]
pub enum ILTW {
    #[doc = "Idle character bit count starts after start bit."]
    _0,
    #[doc = "Idle character bit count starts after stop bit."]
    _1,
}
impl ILTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILTW::_0 => false,
            ILTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILTW<'a> {
    w: &'a mut W,
}
impl<'a> _ILTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILTW::_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKE`"]
pub enum WAKEW {
    #[doc = "Idle line wakeup."]
    _0,
    #[doc = "Address mark wakeup."]
    _1,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::_0 => false,
            WAKEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle line wakeup."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKEW::_0)
    }
    #[doc = "Address mark wakeup."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M`"]
pub enum MW {
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _0,
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _1,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::_0 => false,
            MW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MW::_0)
    }
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSRC`"]
pub enum RSRCW {
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    _0,
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    _1,
}
impl RSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSRCW::_0 => false,
            RSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSRCW::_0)
    }
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSRCW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UARTSWAI`"]
pub enum UARTSWAIW {
    #[doc = "UART clock continues to run in Wait mode."]
    _0,
    #[doc = "UART clock freezes while CPU is in Wait mode."]
    _1,
}
impl UARTSWAIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTSWAIW::_0 => false,
            UARTSWAIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTSWAIW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTSWAIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTSWAIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART clock continues to run in Wait mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSWAIW::_0)
    }
    #[doc = "UART clock freezes while CPU is in Wait mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSWAIW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOPS`"]
pub enum LOOPSW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    _1,
}
impl LOOPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSW::_0 => false,
            LOOPSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPSW::_0)
    }
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPSW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&self) -> PTR {
        PTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&self) -> ILTR {
        ILTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&self) -> RSRCR {
        RSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline]
    pub fn uartswai(&self) -> UARTSWAIR {
        UARTSWAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&self) -> LOOPSR {
        LOOPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&mut self) -> _PTW {
        _PTW { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&mut self) -> _ILTW {
        _ILTW { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&mut self) -> _RSRCW {
        _RSRCW { w: self }
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline]
    pub fn uartswai(&mut self) -> _UARTSWAIW {
        _UARTSWAIW { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&mut self) -> _LOOPSW {
        _LOOPSW { w: self }
    }
}
