#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
pub struct GTSR {
    bits: bool,
}
impl GTSR {
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
pub struct FDENR {
    bits: bool,
}
impl FDENR {
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
#[doc = "Possible values of the field `TFC_PAUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_PAUSER {
    #[doc = "No PAUSE frame transmitted."]
    _0,
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    _1,
}
impl TFC_PAUSER {
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
            TFC_PAUSER::_0 => false,
            TFC_PAUSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFC_PAUSER {
        match value {
            false => TFC_PAUSER::_0,
            true => TFC_PAUSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFC_PAUSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFC_PAUSER::_1
    }
}
#[doc = r" Value of the field"]
pub struct RFC_PAUSER {
    bits: bool,
}
impl RFC_PAUSER {
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
#[doc = "Possible values of the field `ADDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDSELR {
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    _000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDSELR::_000 => 0,
            ADDSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDSELR {
        match value {
            0 => ADDSELR::_000,
            i => ADDSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ADDSELR::_000
    }
}
#[doc = "Possible values of the field `ADDINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDINSR {
    #[doc = "The source MAC address is not modified by the MAC."]
    _0,
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    _1,
}
impl ADDINSR {
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
            ADDINSR::_0 => false,
            ADDINSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDINSR {
        match value {
            false => ADDINSR::_0,
            true => ADDINSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADDINSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADDINSR::_1
    }
}
#[doc = "Possible values of the field `CRCFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWDR {
    #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    _0,
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    _1,
}
impl CRCFWDR {
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
            CRCFWDR::_0 => false,
            CRCFWDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCFWDR {
        match value {
            false => CRCFWDR::_0,
            true => CRCFWDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCFWDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCFWDR::_1
    }
}
#[doc = r" Proxy"]
pub struct _GTSW<'a> {
    w: &'a mut W,
}
impl<'a> _GTSW<'a> {
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
pub struct _FDENW<'a> {
    w: &'a mut W,
}
impl<'a> _FDENW<'a> {
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
#[doc = "Values that can be written to the field `TFC_PAUSE`"]
pub enum TFC_PAUSEW {
    #[doc = "No PAUSE frame transmitted."]
    _0,
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    _1,
}
impl TFC_PAUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFC_PAUSEW::_0 => false,
            TFC_PAUSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFC_PAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFC_PAUSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFC_PAUSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No PAUSE frame transmitted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_PAUSEW::_0)
    }
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_PAUSEW::_1)
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
#[doc = "Values that can be written to the field `ADDSEL`"]
pub enum ADDSELW {
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    _000,
}
impl ADDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDSELW::_000 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDSELW::_000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDINS`"]
pub enum ADDINSW {
    #[doc = "The source MAC address is not modified by the MAC."]
    _0,
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    _1,
}
impl ADDINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDINSW::_0 => false,
            ADDINSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDINSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The source MAC address is not modified by the MAC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDINSW::_0)
    }
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDINSW::_1)
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
#[doc = "Values that can be written to the field `CRCFWD`"]
pub enum CRCFWDW {
    #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    _0,
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    _1,
}
impl CRCFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCFWDW::_0 => false,
            CRCFWDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWDW::_0)
    }
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCFWDW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline]
    pub fn gts(&self) -> GTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GTSR { bits }
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline]
    pub fn fden(&self) -> FDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FDENR { bits }
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline]
    pub fn tfc_pause(&self) -> TFC_PAUSER {
        TFC_PAUSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Frame Control Pause"]
    #[inline]
    pub fn rfc_pause(&self) -> RFC_PAUSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFC_PAUSER { bits }
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline]
    pub fn addsel(&self) -> ADDSELR {
        ADDSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline]
    pub fn addins(&self) -> ADDINSR {
        ADDINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline]
    pub fn crcfwd(&self) -> CRCFWDR {
        CRCFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline]
    pub fn gts(&mut self) -> _GTSW {
        _GTSW { w: self }
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline]
    pub fn fden(&mut self) -> _FDENW {
        _FDENW { w: self }
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline]
    pub fn tfc_pause(&mut self) -> _TFC_PAUSEW {
        _TFC_PAUSEW { w: self }
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline]
    pub fn addsel(&mut self) -> _ADDSELW {
        _ADDSELW { w: self }
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline]
    pub fn addins(&mut self) -> _ADDINSW {
        _ADDINSW { w: self }
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline]
    pub fn crcfwd(&mut self) -> _CRCFWDW {
        _CRCFWDW { w: self }
    }
}
