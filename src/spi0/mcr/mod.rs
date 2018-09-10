#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "Start transfers."]
    _0,
    #[doc = "Stop transfers."]
    _1,
}
impl HALTR {
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
            HALTR::_0 => false,
            HALTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::_0,
            true => HALTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HALTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HALTR::_1
    }
}
#[doc = "Possible values of the field `SMPL_PT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPL_PTR {
    #[doc = "0 protocol clock cycles between SCK edge and SIN sample"]
    _00,
    #[doc = "1 protocol clock cycle between SCK edge and SIN sample"]
    _01,
    #[doc = "2 protocol clock cycles between SCK edge and SIN sample"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMPL_PTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMPL_PTR::_00 => 0,
            SMPL_PTR::_01 => 1,
            SMPL_PTR::_10 => 2,
            SMPL_PTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMPL_PTR {
        match value {
            0 => SMPL_PTR::_00,
            1 => SMPL_PTR::_01,
            2 => SMPL_PTR::_10,
            i => SMPL_PTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SMPL_PTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SMPL_PTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SMPL_PTR::_10
    }
}
#[doc = "Possible values of the field `DIS_RXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_RXFR {
    #[doc = "RX FIFO is enabled."]
    _0,
    #[doc = "RX FIFO is disabled."]
    _1,
}
impl DIS_RXFR {
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
            DIS_RXFR::_0 => false,
            DIS_RXFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIS_RXFR {
        match value {
            false => DIS_RXFR::_0,
            true => DIS_RXFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIS_RXFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIS_RXFR::_1
    }
}
#[doc = "Possible values of the field `DIS_TXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_TXFR {
    #[doc = "TX FIFO is enabled."]
    _0,
    #[doc = "TX FIFO is disabled."]
    _1,
}
impl DIS_TXFR {
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
            DIS_TXFR::_0 => false,
            DIS_TXFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIS_TXFR {
        match value {
            false => DIS_TXFR::_0,
            true => DIS_TXFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIS_TXFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIS_TXFR::_1
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Enables the module clocks."]
    _0,
    #[doc = "Allows external logic to disable the module clocks."]
    _1,
}
impl MDISR {
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
            MDISR::_0 => false,
            MDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::_0,
            true => MDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDISR::_1
    }
}
#[doc = "Possible values of the field `DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZER {
    #[doc = "Doze mode has no effect on the module."]
    _0,
    #[doc = "Doze mode disables the module."]
    _1,
}
impl DOZER {
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
            DOZER::_0 => false,
            DOZER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZER {
        match value {
            false => DOZER::_0,
            true => DOZER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZER::_1
    }
}
#[doc = "Possible values of the field `PCSIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSISR {
    #[doc = "The inactive state of PCSx is low."]
    _0,
    #[doc = "The inactive state of PCSx is high."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PCSISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSISR::_0 => 0,
            PCSISR::_1 => 1,
            PCSISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSISR {
        match value {
            0 => PCSISR::_0,
            1 => PCSISR::_1,
            i => PCSISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCSISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCSISR::_1
    }
}
#[doc = "Possible values of the field `ROOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROOER {
    #[doc = "Incoming data is ignored."]
    _0,
    #[doc = "Incoming data is shifted into the shift register."]
    _1,
}
impl ROOER {
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
            ROOER::_0 => false,
            ROOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROOER {
        match value {
            false => ROOER::_0,
            true => ROOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ROOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ROOER::_1
    }
}
#[doc = "Possible values of the field `PCSSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSSER {
    #[doc = "PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\] signal."]
    _0,
    #[doc = "PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    _1,
}
impl PCSSER {
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
            PCSSER::_0 => false,
            PCSSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCSSER {
        match value {
            false => PCSSER::_0,
            true => PCSSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCSSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCSSER::_1
    }
}
#[doc = "Possible values of the field `MTFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTFER {
    #[doc = "Modified SPI transfer format disabled."]
    _0,
    #[doc = "Modified SPI transfer format enabled."]
    _1,
}
impl MTFER {
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
            MTFER::_0 => false,
            MTFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTFER {
        match value {
            false => MTFER::_0,
            true => MTFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTFER::_1
    }
}
#[doc = "Possible values of the field `FRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZR {
    #[doc = "Do not halt serial transfers in Debug mode."]
    _0,
    #[doc = "Halt serial transfers in Debug mode."]
    _1,
}
impl FRZR {
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
            FRZR::_0 => false,
            FRZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZR {
        match value {
            false => FRZR::_0,
            true => FRZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRZR::_1
    }
}
#[doc = "Possible values of the field `DCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCONFR {
    #[doc = "SPI"]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCONFR::_00 => 0,
            DCONFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCONFR {
        match value {
            0 => DCONFR::_00,
            i => DCONFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DCONFR::_00
    }
}
#[doc = "Possible values of the field `CONT_SCKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_SCKER {
    #[doc = "Continuous SCK disabled."]
    _0,
    #[doc = "Continuous SCK enabled."]
    _1,
}
impl CONT_SCKER {
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
            CONT_SCKER::_0 => false,
            CONT_SCKER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONT_SCKER {
        match value {
            false => CONT_SCKER::_0,
            true => CONT_SCKER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CONT_SCKER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CONT_SCKER::_1
    }
}
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "Enables Slave mode"]
    _0,
    #[doc = "Enables Master mode"]
    _1,
}
impl MSTRR {
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
            MSTRR::_0 => false,
            MSTRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            false => MSTRR::_0,
            true => MSTRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSTRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSTRR::_1
    }
}
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "Start transfers."]
    _0,
    #[doc = "Stop transfers."]
    _1,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::_0 => false,
            HALTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start transfers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALTW::_0)
    }
    #[doc = "Stop transfers."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALTW::_1)
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
#[doc = "Values that can be written to the field `SMPL_PT`"]
pub enum SMPL_PTW {
    #[doc = "0 protocol clock cycles between SCK edge and SIN sample"]
    _00,
    #[doc = "1 protocol clock cycle between SCK edge and SIN sample"]
    _01,
    #[doc = "2 protocol clock cycles between SCK edge and SIN sample"]
    _10,
}
impl SMPL_PTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMPL_PTW::_00 => 0,
            SMPL_PTW::_01 => 1,
            SMPL_PTW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPL_PTW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPL_PTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPL_PTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 protocol clock cycles between SCK edge and SIN sample"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SMPL_PTW::_00)
    }
    #[doc = "1 protocol clock cycle between SCK edge and SIN sample"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SMPL_PTW::_01)
    }
    #[doc = "2 protocol clock cycles between SCK edge and SIN sample"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMPL_PTW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_RXF`"]
pub enum CLR_RXFW {
    #[doc = "Do not clear the RX FIFO counter."]
    _0,
    #[doc = "Clear the RX FIFO counter."]
    _1,
}
impl CLR_RXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_RXFW::_0 => false,
            CLR_RXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_RXFW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_RXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_RXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear the RX FIFO counter."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXFW::_0)
    }
    #[doc = "Clear the RX FIFO counter."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXFW::_1)
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
#[doc = "Values that can be written to the field `CLR_TXF`"]
pub enum CLR_TXFW {
    #[doc = "Do not clear the TX FIFO counter."]
    _0,
    #[doc = "Clear the TX FIFO counter."]
    _1,
}
impl CLR_TXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_TXFW::_0 => false,
            CLR_TXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_TXFW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_TXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_TXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear the TX FIFO counter."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXFW::_0)
    }
    #[doc = "Clear the TX FIFO counter."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXFW::_1)
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
#[doc = "Values that can be written to the field `DIS_RXF`"]
pub enum DIS_RXFW {
    #[doc = "RX FIFO is enabled."]
    _0,
    #[doc = "RX FIFO is disabled."]
    _1,
}
impl DIS_RXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIS_RXFW::_0 => false,
            DIS_RXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIS_RXFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_RXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIS_RXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX FIFO is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_RXFW::_0)
    }
    #[doc = "RX FIFO is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_RXFW::_1)
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
#[doc = "Values that can be written to the field `DIS_TXF`"]
pub enum DIS_TXFW {
    #[doc = "TX FIFO is enabled."]
    _0,
    #[doc = "TX FIFO is disabled."]
    _1,
}
impl DIS_TXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIS_TXFW::_0 => false,
            DIS_TXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIS_TXFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_TXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIS_TXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX FIFO is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_TXFW::_0)
    }
    #[doc = "TX FIFO is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_TXFW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Enables the module clocks."]
    _0,
    #[doc = "Allows external logic to disable the module clocks."]
    _1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::_0 => false,
            MDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables the module clocks."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDISW::_0)
    }
    #[doc = "Allows external logic to disable the module clocks."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDISW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZE`"]
pub enum DOZEW {
    #[doc = "Doze mode has no effect on the module."]
    _0,
    #[doc = "Doze mode disables the module."]
    _1,
}
impl DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEW::_0 => false,
            DOZEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Doze mode has no effect on the module."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEW::_0)
    }
    #[doc = "Doze mode disables the module."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCSIS`"]
pub enum PCSISW {
    #[doc = "The inactive state of PCSx is low."]
    _0,
    #[doc = "The inactive state of PCSx is high."]
    _1,
}
impl PCSISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSISW::_0 => 0,
            PCSISW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSISW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The inactive state of PCSx is low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSISW::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSISW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROOE`"]
pub enum ROOEW {
    #[doc = "Incoming data is ignored."]
    _0,
    #[doc = "Incoming data is shifted into the shift register."]
    _1,
}
impl ROOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROOEW::_0 => false,
            ROOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROOEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Incoming data is ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROOEW::_0)
    }
    #[doc = "Incoming data is shifted into the shift register."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROOEW::_1)
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
#[doc = "Values that can be written to the field `PCSSE`"]
pub enum PCSSEW {
    #[doc = "PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\] signal."]
    _0,
    #[doc = "PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    _1,
}
impl PCSSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCSSEW::_0 => false,
            PCSSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSSEW::_0)
    }
    #[doc = "PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSSEW::_1)
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
#[doc = "Values that can be written to the field `MTFE`"]
pub enum MTFEW {
    #[doc = "Modified SPI transfer format disabled."]
    _0,
    #[doc = "Modified SPI transfer format enabled."]
    _1,
}
impl MTFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTFEW::_0 => false,
            MTFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTFEW<'a> {
    w: &'a mut W,
}
impl<'a> _MTFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Modified SPI transfer format disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTFEW::_0)
    }
    #[doc = "Modified SPI transfer format enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTFEW::_1)
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
#[doc = "Values that can be written to the field `FRZ`"]
pub enum FRZW {
    #[doc = "Do not halt serial transfers in Debug mode."]
    _0,
    #[doc = "Halt serial transfers in Debug mode."]
    _1,
}
impl FRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRZW::_0 => false,
            FRZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not halt serial transfers in Debug mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZW::_0)
    }
    #[doc = "Halt serial transfers in Debug mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZW::_1)
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
#[doc = "Values that can be written to the field `CONT_SCKE`"]
pub enum CONT_SCKEW {
    #[doc = "Continuous SCK disabled."]
    _0,
    #[doc = "Continuous SCK enabled."]
    _1,
}
impl CONT_SCKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONT_SCKEW::_0 => false,
            CONT_SCKEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONT_SCKEW<'a> {
    w: &'a mut W,
}
impl<'a> _CONT_SCKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONT_SCKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous SCK disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_SCKEW::_0)
    }
    #[doc = "Continuous SCK enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_SCKEW::_1)
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
#[doc = "Values that can be written to the field `MSTR`"]
pub enum MSTRW {
    #[doc = "Enables Slave mode"]
    _0,
    #[doc = "Enables Master mode"]
    _1,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::_0 => false,
            MSTRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enables Slave mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTRW::_0)
    }
    #[doc = "Enables Master mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTRW::_1)
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
    #[doc = "Bit 0 - Halt"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline]
    pub fn smpl_pt(&self) -> SMPL_PTR {
        SMPL_PTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline]
    pub fn dis_rxf(&self) -> DIS_RXFR {
        DIS_RXFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline]
    pub fn dis_txf(&self) -> DIS_TXFR {
        DIS_TXFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline]
    pub fn doze(&self) -> DOZER {
        DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline]
    pub fn pcsis(&self) -> PCSISR {
        PCSISR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline]
    pub fn rooe(&self) -> ROOER {
        ROOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline]
    pub fn pcsse(&self) -> PCSSER {
        PCSSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Modified Transfer Format Enable"]
    #[inline]
    pub fn mtfe(&self) -> MTFER {
        MTFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline]
    pub fn frz(&self) -> FRZR {
        FRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:29 - SPI Configuration."]
    #[inline]
    pub fn dconf(&self) -> DCONFR {
        DCONFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline]
    pub fn cont_scke(&self) -> CONT_SCKER {
        CONT_SCKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
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
        W { bits: 16385 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Halt"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline]
    pub fn smpl_pt(&mut self) -> _SMPL_PTW {
        _SMPL_PTW { w: self }
    }
    #[doc = "Bit 10 - CLR_RXF"]
    #[inline]
    pub fn clr_rxf(&mut self) -> _CLR_RXFW {
        _CLR_RXFW { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO"]
    #[inline]
    pub fn clr_txf(&mut self) -> _CLR_TXFW {
        _CLR_TXFW { w: self }
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline]
    pub fn dis_rxf(&mut self) -> _DIS_RXFW {
        _DIS_RXFW { w: self }
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline]
    pub fn dis_txf(&mut self) -> _DIS_TXFW {
        _DIS_TXFW { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline]
    pub fn doze(&mut self) -> _DOZEW {
        _DOZEW { w: self }
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline]
    pub fn pcsis(&mut self) -> _PCSISW {
        _PCSISW { w: self }
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline]
    pub fn rooe(&mut self) -> _ROOEW {
        _ROOEW { w: self }
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline]
    pub fn pcsse(&mut self) -> _PCSSEW {
        _PCSSEW { w: self }
    }
    #[doc = "Bit 26 - Modified Transfer Format Enable"]
    #[inline]
    pub fn mtfe(&mut self) -> _MTFEW {
        _MTFEW { w: self }
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline]
    pub fn frz(&mut self) -> _FRZW {
        _FRZW { w: self }
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline]
    pub fn cont_scke(&mut self) -> _CONT_SCKEW {
        _CONT_SCKEW { w: self }
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
}
