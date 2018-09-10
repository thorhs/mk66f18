#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUD {
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
pub struct SBRR {
    bits: u16,
}
impl SBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SBNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNSR {
    #[doc = "One stop bit."]
    _0,
    #[doc = "Two stop bits."]
    _1,
}
impl SBNSR {
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
            SBNSR::_0 => false,
            SBNSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBNSR {
        match value {
            false => SBNSR::_0,
            true => SBNSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBNSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBNSR::_1
    }
}
#[doc = "Possible values of the field `RXEDGIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIER {
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\] disabled (use polling)."]
    _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\] flag is 1."]
    _1,
}
impl RXEDGIER {
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
            RXEDGIER::_0 => false,
            RXEDGIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIER {
        match value {
            false => RXEDGIER::_0,
            true => RXEDGIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIER::_1
    }
}
#[doc = "Possible values of the field `LBKDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIER {
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\] disabled (use polling)."]
    _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\] flag is 1."]
    _1,
}
impl LBKDIER {
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
            LBKDIER::_0 => false,
            LBKDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIER {
        match value {
            false => LBKDIER::_0,
            true => LBKDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDIER::_1
    }
}
#[doc = "Possible values of the field `RESYNCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDISR {
    #[doc = "Resynchronization during received data word is supported"]
    _0,
    #[doc = "Resynchronization during received data word is disabled"]
    _1,
}
impl RESYNCDISR {
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
            RESYNCDISR::_0 => false,
            RESYNCDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESYNCDISR {
        match value {
            false => RESYNCDISR::_0,
            true => RESYNCDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESYNCDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESYNCDISR::_1
    }
}
#[doc = "Possible values of the field `BOTHEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGER {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGER {
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
            BOTHEDGER::_0 => false,
            BOTHEDGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOTHEDGER {
        match value {
            false => BOTHEDGER::_0,
            true => BOTHEDGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOTHEDGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOTHEDGER::_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Address Match Wakeup"]
    _00,
    #[doc = "Idle Match Wakeup"]
    _01,
    #[doc = "Match On and Match Off"]
    _10,
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    _11,
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::_00 => 0,
            MATCFGR::_01 => 1,
            MATCFGR::_10 => 2,
            MATCFGR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::_00,
            1 => MATCFGR::_01,
            2 => MATCFGR::_10,
            3 => MATCFGR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MATCFGR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MATCFGR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MATCFGR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MATCFGR::_11
    }
}
#[doc = "Possible values of the field `RDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAER {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl RDMAER {
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
            RDMAER::_0 => false,
            RDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMAER {
        match value {
            false => RDMAER::_0,
            true => RDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDMAER::_1
    }
}
#[doc = "Possible values of the field `TDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAER {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl TDMAER {
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
            TDMAER::_0 => false,
            TDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDMAER {
        match value {
            false => TDMAER::_0,
            true => TDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDMAER::_1
    }
}
#[doc = r" Value of the field"]
pub struct OSRR {
    bits: u8,
}
impl OSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10R {
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    _0,
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    _1,
}
impl M10R {
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
            M10R::_0 => false,
            M10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M10R {
        match value {
            false => M10R::_0,
            true => M10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M10R::_1
    }
}
#[doc = "Possible values of the field `MAEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2R {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    _1,
}
impl MAEN2R {
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
            MAEN2R::_0 => false,
            MAEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN2R {
        match value {
            false => MAEN2R::_0,
            true => MAEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN2R::_1
    }
}
#[doc = "Possible values of the field `MAEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1R {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    _1,
}
impl MAEN1R {
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
            MAEN1R::_0 => false,
            MAEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN1R {
        match value {
            false => MAEN1R::_0,
            true => MAEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN1R::_1
    }
}
#[doc = r" Proxy"]
pub struct _SBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBNS`"]
pub enum SBNSW {
    #[doc = "One stop bit."]
    _0,
    #[doc = "Two stop bits."]
    _1,
}
impl SBNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBNSW::_0 => false,
            SBNSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBNSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBNSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One stop bit."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNSW::_0)
    }
    #[doc = "Two stop bits."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNSW::_1)
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
#[doc = "Values that can be written to the field `RXEDGIE`"]
pub enum RXEDGIEW {
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\] disabled (use polling)."]
    _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\] flag is 1."]
    _1,
}
impl RXEDGIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIEW::_0 => false,
            RXEDGIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\] disabled (use polling)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIEW::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\] flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIEW::_1)
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
#[doc = "Values that can be written to the field `LBKDIE`"]
pub enum LBKDIEW {
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\] disabled (use polling)."]
    _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\] flag is 1."]
    _1,
}
impl LBKDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIEW::_0 => false,
            LBKDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\] disabled (use polling)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIEW::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\] flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIEW::_1)
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
#[doc = "Values that can be written to the field `RESYNCDIS`"]
pub enum RESYNCDISW {
    #[doc = "Resynchronization during received data word is supported"]
    _0,
    #[doc = "Resynchronization during received data word is disabled"]
    _1,
}
impl RESYNCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESYNCDISW::_0 => false,
            RESYNCDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RESYNCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESYNCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESYNCDISW::_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESYNCDISW::_1)
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
#[doc = "Values that can be written to the field `BOTHEDGE`"]
pub enum BOTHEDGEW {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOTHEDGEW::_0 => false,
            BOTHEDGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOTHEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOTHEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOTHEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_1)
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
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Address Match Wakeup"]
    _00,
    #[doc = "Idle Match Wakeup"]
    _01,
    #[doc = "Match On and Match Off"]
    _10,
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    _11,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::_00 => 0,
            MATCFGW::_01 => 1,
            MATCFGW::_10 => 2,
            MATCFGW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address Match Wakeup"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MATCFGW::_00)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MATCFGW::_01)
    }
    #[doc = "Match On and Match Off"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MATCFGW::_10)
    }
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MATCFGW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDMAE`"]
pub enum RDMAEW {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl RDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMAEW::_0 => false,
            RDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAEW::_1)
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
#[doc = "Values that can be written to the field `TDMAE`"]
pub enum TDMAEW {
    #[doc = "DMA request disabled."]
    _0,
    #[doc = "DMA request enabled."]
    _1,
}
impl TDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDMAEW::_0 => false,
            TDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAEW::_1)
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
#[doc = r" Proxy"]
pub struct _OSRW<'a> {
    w: &'a mut W,
}
impl<'a> _OSRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M10`"]
pub enum M10W {
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    _0,
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    _1,
}
impl M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M10W::_0 => false,
            M10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M10W<'a> {
    w: &'a mut W,
}
impl<'a> _M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10W::_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10W::_1)
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
#[doc = "Values that can be written to the field `MAEN2`"]
pub enum MAEN2W {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    _1,
}
impl MAEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN2W::_0 => false,
            MAEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2W::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2W::_1)
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
#[doc = "Values that can be written to the field `MAEN1`"]
pub enum MAEN1W {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    _1,
}
impl MAEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN1W::_0 => false,
            MAEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1W::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1W::_1)
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
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&self) -> SBRR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SBRR { bits }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&self) -> SBNSR {
        SBNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&self) -> RXEDGIER {
        RXEDGIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&self) -> LBKDIER {
        LBKDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&self) -> RESYNCDISR {
        RESYNCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&self) -> BOTHEDGER {
        BOTHEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&self) -> RDMAER {
        RDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&self) -> TDMAER {
        TDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Over Sampling Ratio"]
    #[inline]
    pub fn osr(&self) -> OSRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSRR { bits }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&self) -> M10R {
        M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&self) -> MAEN2R {
        MAEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&self) -> MAEN1R {
        MAEN1R::_from({
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
        W { bits: 251658244 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&mut self) -> _SBRW {
        _SBRW { w: self }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&mut self) -> _SBNSW {
        _SBNSW { w: self }
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&mut self) -> _RXEDGIEW {
        _RXEDGIEW { w: self }
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&mut self) -> _LBKDIEW {
        _LBKDIEW { w: self }
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&mut self) -> _RESYNCDISW {
        _RESYNCDISW { w: self }
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&mut self) -> _BOTHEDGEW {
        _BOTHEDGEW { w: self }
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&mut self) -> _RDMAEW {
        _RDMAEW { w: self }
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&mut self) -> _TDMAEW {
        _TDMAEW { w: self }
    }
    #[doc = "Bits 24:28 - Over Sampling Ratio"]
    #[inline]
    pub fn osr(&mut self) -> _OSRW {
        _OSRW { w: self }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&mut self) -> _M10W {
        _M10W { w: self }
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&mut self) -> _MAEN2W {
        _MAEN2W { w: self }
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&mut self) -> _MAEN1W {
        _MAEN1W { w: self }
    }
}
