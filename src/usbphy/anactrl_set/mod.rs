#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANACTRL_SET {
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
pub struct TESTCLK_SELR {
    bits: bool,
}
impl TESTCLK_SELR {
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
#[doc = "Possible values of the field `PFD_CLKGATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFD_CLKGATER {
    #[doc = "PFD clock output is enabled"]
    _0,
    #[doc = "PFD clock output is gated (Default)"]
    _1,
}
impl PFD_CLKGATER {
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
            PFD_CLKGATER::_0 => false,
            PFD_CLKGATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFD_CLKGATER {
        match value {
            false => PFD_CLKGATER::_0,
            true => PFD_CLKGATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PFD_CLKGATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PFD_CLKGATER::_1
    }
}
#[doc = "Possible values of the field `PFD_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFD_CLK_SELR {
    #[doc = "USB1PFDCLK is the same frequency as the xtal clock (Default)"]
    _00,
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 4"]
    _01,
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 2"]
    _10,
    #[doc = "USB1PFDCLK frequency is the same as pfd_clk frequency"]
    _11,
}
impl PFD_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PFD_CLK_SELR::_00 => 0,
            PFD_CLK_SELR::_01 => 1,
            PFD_CLK_SELR::_10 => 2,
            PFD_CLK_SELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PFD_CLK_SELR {
        match value {
            0 => PFD_CLK_SELR::_00,
            1 => PFD_CLK_SELR::_01,
            2 => PFD_CLK_SELR::_10,
            3 => PFD_CLK_SELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PFD_CLK_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PFD_CLK_SELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PFD_CLK_SELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PFD_CLK_SELR::_11
    }
}
#[doc = r" Value of the field"]
pub struct PFD_FRACR {
    bits: u8,
}
impl PFD_FRACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEV_PULLDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWNR {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    _0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    _1,
}
impl DEV_PULLDOWNR {
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
            DEV_PULLDOWNR::_0 => false,
            DEV_PULLDOWNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_PULLDOWNR {
        match value {
            false => DEV_PULLDOWNR::_0,
            true => DEV_PULLDOWNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DEV_PULLDOWNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DEV_PULLDOWNR::_1
    }
}
#[doc = "Possible values of the field `EMPH_PULSE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_PULSE_CTRLR {
    #[doc = "Minimum duration of pre-emphasis current after each data transition"]
    _00,
    #[doc = "Maximum duration of pre-emphasis current after each data transition"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EMPH_PULSE_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMPH_PULSE_CTRLR::_00 => 0,
            EMPH_PULSE_CTRLR::_11 => 3,
            EMPH_PULSE_CTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMPH_PULSE_CTRLR {
        match value {
            0 => EMPH_PULSE_CTRLR::_00,
            3 => EMPH_PULSE_CTRLR::_11,
            i => EMPH_PULSE_CTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == EMPH_PULSE_CTRLR::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == EMPH_PULSE_CTRLR::_11
    }
}
#[doc = "Possible values of the field `EMPH_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_ENR {
    #[doc = "No pre-emphasis is used on HS TX output drivers"]
    _0,
    #[doc = "Enables pre-emphasis for HS TX output drivers"]
    _1,
}
impl EMPH_ENR {
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
            EMPH_ENR::_0 => false,
            EMPH_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMPH_ENR {
        match value {
            false => EMPH_ENR::_0,
            true => EMPH_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EMPH_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EMPH_ENR::_1
    }
}
#[doc = "Possible values of the field `EMPH_CUR_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_CUR_CTRLR {
    #[doc = "No pre-emphasis current is enabled for the HS TX drivers"]
    _00,
    #[doc = "One unit of pre-emphasis current is enabled for the HS TX drivers"]
    _01,
    #[doc = "Two units of pre-emphasis current are enabled for the HS TX drivers"]
    _10,
    #[doc = "Three units of pre-emphasis current are enabled for the HS TX drivers"]
    _11,
}
impl EMPH_CUR_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMPH_CUR_CTRLR::_00 => 0,
            EMPH_CUR_CTRLR::_01 => 1,
            EMPH_CUR_CTRLR::_10 => 2,
            EMPH_CUR_CTRLR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMPH_CUR_CTRLR {
        match value {
            0 => EMPH_CUR_CTRLR::_00,
            1 => EMPH_CUR_CTRLR::_01,
            2 => EMPH_CUR_CTRLR::_10,
            3 => EMPH_CUR_CTRLR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == EMPH_CUR_CTRLR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == EMPH_CUR_CTRLR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == EMPH_CUR_CTRLR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == EMPH_CUR_CTRLR::_11
    }
}
#[doc = r" Value of the field"]
pub struct PFD_STABLER {
    bits: bool,
}
impl PFD_STABLER {
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
#[doc = r" Proxy"]
pub struct _TESTCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTCLK_SELW<'a> {
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
#[doc = "Values that can be written to the field `PFD_CLKGATE`"]
pub enum PFD_CLKGATEW {
    #[doc = "PFD clock output is enabled"]
    _0,
    #[doc = "PFD clock output is gated (Default)"]
    _1,
}
impl PFD_CLKGATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFD_CLKGATEW::_0 => false,
            PFD_CLKGATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFD_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_CLKGATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFD_CLKGATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PFD clock output is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFD_CLKGATEW::_0)
    }
    #[doc = "PFD clock output is gated (Default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFD_CLKGATEW::_1)
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
#[doc = "Values that can be written to the field `PFD_CLK_SEL`"]
pub enum PFD_CLK_SELW {
    #[doc = "USB1PFDCLK is the same frequency as the xtal clock (Default)"]
    _00,
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 4"]
    _01,
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 2"]
    _10,
    #[doc = "USB1PFDCLK frequency is the same as pfd_clk frequency"]
    _11,
}
impl PFD_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PFD_CLK_SELW::_00 => 0,
            PFD_CLK_SELW::_01 => 1,
            PFD_CLK_SELW::_10 => 2,
            PFD_CLK_SELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFD_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFD_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USB1PFDCLK is the same frequency as the xtal clock (Default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PFD_CLK_SELW::_00)
    }
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 4"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PFD_CLK_SELW::_01)
    }
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PFD_CLK_SELW::_10)
    }
    #[doc = "USB1PFDCLK frequency is the same as pfd_clk frequency"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PFD_CLK_SELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFD_FRACW<'a> {
    w: &'a mut W,
}
impl<'a> _PFD_FRACW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEV_PULLDOWN`"]
pub enum DEV_PULLDOWNW {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    _0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    _1,
}
impl DEV_PULLDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEV_PULLDOWNW::_0 => false,
            DEV_PULLDOWNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEV_PULLDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_PULLDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEV_PULLDOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWNW::_0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWNW::_1)
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
#[doc = "Values that can be written to the field `EMPH_PULSE_CTRL`"]
pub enum EMPH_PULSE_CTRLW {
    #[doc = "Minimum duration of pre-emphasis current after each data transition"]
    _00,
    #[doc = "Maximum duration of pre-emphasis current after each data transition"]
    _11,
}
impl EMPH_PULSE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMPH_PULSE_CTRLW::_00 => 0,
            EMPH_PULSE_CTRLW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMPH_PULSE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPH_PULSE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMPH_PULSE_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Minimum duration of pre-emphasis current after each data transition"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMPH_PULSE_CTRLW::_00)
    }
    #[doc = "Maximum duration of pre-emphasis current after each data transition"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMPH_PULSE_CTRLW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMPH_EN`"]
pub enum EMPH_ENW {
    #[doc = "No pre-emphasis is used on HS TX output drivers"]
    _0,
    #[doc = "Enables pre-emphasis for HS TX output drivers"]
    _1,
}
impl EMPH_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMPH_ENW::_0 => false,
            EMPH_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMPH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPH_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMPH_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pre-emphasis is used on HS TX output drivers"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMPH_ENW::_0)
    }
    #[doc = "Enables pre-emphasis for HS TX output drivers"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMPH_ENW::_1)
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
#[doc = "Values that can be written to the field `EMPH_CUR_CTRL`"]
pub enum EMPH_CUR_CTRLW {
    #[doc = "No pre-emphasis current is enabled for the HS TX drivers"]
    _00,
    #[doc = "One unit of pre-emphasis current is enabled for the HS TX drivers"]
    _01,
    #[doc = "Two units of pre-emphasis current are enabled for the HS TX drivers"]
    _10,
    #[doc = "Three units of pre-emphasis current are enabled for the HS TX drivers"]
    _11,
}
impl EMPH_CUR_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMPH_CUR_CTRLW::_00 => 0,
            EMPH_CUR_CTRLW::_01 => 1,
            EMPH_CUR_CTRLW::_10 => 2,
            EMPH_CUR_CTRLW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMPH_CUR_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPH_CUR_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMPH_CUR_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No pre-emphasis current is enabled for the HS TX drivers"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRLW::_00)
    }
    #[doc = "One unit of pre-emphasis current is enabled for the HS TX drivers"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRLW::_01)
    }
    #[doc = "Two units of pre-emphasis current are enabled for the HS TX drivers"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRLW::_10)
    }
    #[doc = "Three units of pre-emphasis current are enabled for the HS TX drivers"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRLW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Test clock selection to analog test"]
    #[inline]
    pub fn testclk_sel(&self) -> TESTCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TESTCLK_SELR { bits }
    }
    #[doc = "Bit 1 - This bit field controls clock gating (disabling) for the PFD pfd_clk output for power savings when the PFD is not used"]
    #[inline]
    pub fn pfd_clkgate(&self) -> PFD_CLKGATER {
        PFD_CLKGATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - This bit field for the PFD selects the frequency relationship between the local pfd_clk output and the exported USB1PFDCLK"]
    #[inline]
    pub fn pfd_clk_sel(&self) -> PFD_CLK_SELR {
        PFD_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:9 - PFD fractional divider setting used to select the pfd_clk output frequency"]
    #[inline]
    pub fn pfd_frac(&self) -> PFD_FRACR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PFD_FRACR { bits }
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWNR {
        DEV_PULLDOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:12 - Controls pre-emphasis time duration for the High Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline]
    pub fn emph_pulse_ctrl(&self) -> EMPH_PULSE_CTRLR {
        EMPH_PULSE_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Enables pre-emphasis for the High-Speed TX drivers"]
    #[inline]
    pub fn emph_en(&self) -> EMPH_ENR {
        EMPH_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Controls the amount of pre-emphasis current added for the High-Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline]
    pub fn emph_cur_ctrl(&self) -> EMPH_CUR_CTRLR {
        EMPH_CUR_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - PFD stable signal from the Phase Fractional Divider."]
    #[inline]
    pub fn pfd_stable(&self) -> PFD_STABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_STABLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1026 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Test clock selection to analog test"]
    #[inline]
    pub fn testclk_sel(&mut self) -> _TESTCLK_SELW {
        _TESTCLK_SELW { w: self }
    }
    #[doc = "Bit 1 - This bit field controls clock gating (disabling) for the PFD pfd_clk output for power savings when the PFD is not used"]
    #[inline]
    pub fn pfd_clkgate(&mut self) -> _PFD_CLKGATEW {
        _PFD_CLKGATEW { w: self }
    }
    #[doc = "Bits 2:3 - This bit field for the PFD selects the frequency relationship between the local pfd_clk output and the exported USB1PFDCLK"]
    #[inline]
    pub fn pfd_clk_sel(&mut self) -> _PFD_CLK_SELW {
        _PFD_CLK_SELW { w: self }
    }
    #[doc = "Bits 4:9 - PFD fractional divider setting used to select the pfd_clk output frequency"]
    #[inline]
    pub fn pfd_frac(&mut self) -> _PFD_FRACW {
        _PFD_FRACW { w: self }
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline]
    pub fn dev_pulldown(&mut self) -> _DEV_PULLDOWNW {
        _DEV_PULLDOWNW { w: self }
    }
    #[doc = "Bits 11:12 - Controls pre-emphasis time duration for the High Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline]
    pub fn emph_pulse_ctrl(&mut self) -> _EMPH_PULSE_CTRLW {
        _EMPH_PULSE_CTRLW { w: self }
    }
    #[doc = "Bit 13 - Enables pre-emphasis for the High-Speed TX drivers"]
    #[inline]
    pub fn emph_en(&mut self) -> _EMPH_ENW {
        _EMPH_ENW { w: self }
    }
    #[doc = "Bits 14:15 - Controls the amount of pre-emphasis current added for the High-Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline]
    pub fn emph_cur_ctrl(&mut self) -> _EMPH_CUR_CTRLW {
        _EMPH_CUR_CTRLW { w: self }
    }
}
