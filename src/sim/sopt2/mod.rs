#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT2 {
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
#[doc = "Possible values of the field `USBSLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSLSRCR {
    #[doc = "MCGIRCLK"]
    _0,
    #[doc = "RTC 32.768kHz clock"]
    _1,
}
impl USBSLSRCR {
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
            USBSLSRCR::_0 => false,
            USBSLSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSLSRCR {
        match value {
            false => USBSLSRCR::_0,
            true => USBSLSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSLSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSLSRCR::_1
    }
}
#[doc = "Possible values of the field `USBREGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREGENR {
    #[doc = "USB PHY PLL Regulator disabled."]
    _0,
    #[doc = "USB PHY PLL Regulator enabled."]
    _1,
}
impl USBREGENR {
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
            USBREGENR::_0 => false,
            USBREGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBREGENR {
        match value {
            false => USBREGENR::_0,
            true => USBREGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBREGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBREGENR::_1
    }
}
#[doc = "Possible values of the field `RTCCLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSELR {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELR {
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
            RTCCLKOUTSELR::_0 => false,
            RTCCLKOUTSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCCLKOUTSELR {
        match value {
            false => RTCCLKOUTSELR::_0,
            true => RTCCLKOUTSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSELR::_1
    }
}
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "FlexBus CLKOUT"]
    _000,
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "RTC 32.768kHz clock"]
    _101,
    #[doc = "OSCERCLK0"]
    _110,
    #[doc = "IRC 48 MHz clock"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_000 => 0,
            CLKOUTSELR::_010 => 2,
            CLKOUTSELR::_011 => 3,
            CLKOUTSELR::_100 => 4,
            CLKOUTSELR::_101 => 5,
            CLKOUTSELR::_110 => 6,
            CLKOUTSELR::_111 => 7,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::_000,
            2 => CLKOUTSELR::_010,
            3 => CLKOUTSELR::_011,
            4 => CLKOUTSELR::_100,
            5 => CLKOUTSELR::_101,
            6 => CLKOUTSELR::_110,
            7 => CLKOUTSELR::_111,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSELR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTSELR::_111
    }
}
#[doc = "Possible values of the field `FBSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBSLR {
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _00,
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _01,
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10,
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    _11,
}
impl FBSLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FBSLR::_00 => 0,
            FBSLR::_01 => 1,
            FBSLR::_10 => 2,
            FBSLR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FBSLR {
        match value {
            0 => FBSLR::_00,
            1 => FBSLR::_01,
            2 => FBSLR::_10,
            3 => FBSLR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FBSLR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FBSLR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FBSLR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FBSLR::_11
    }
}
#[doc = "Possible values of the field `TRACECLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLKSELR {
    #[doc = "MCGOUTCLK, divided by the TRACECLK fractional divider as configured by SIM_CLKDIV4\\[TRACEFRAC, TRACEDIV\\]"]
    _0,
    #[doc = "Core/system clock"]
    _1,
}
impl TRACECLKSELR {
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
            TRACECLKSELR::_0 => false,
            TRACECLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACECLKSELR {
        match value {
            false => TRACECLKSELR::_0,
            true => TRACECLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSELR::_1
    }
}
#[doc = "Possible values of the field `PLLFLLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSELR {
    #[doc = "MCGFLLCLK clock"]
    _00,
    #[doc = "MCGPLLCLK clock"]
    _01,
    #[doc = "USB1 PFD clock"]
    _10,
    #[doc = "IRC48 MHz clock"]
    _11,
}
impl PLLFLLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLFLLSELR::_00 => 0,
            PLLFLLSELR::_01 => 1,
            PLLFLLSELR::_10 => 2,
            PLLFLLSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLFLLSELR {
        match value {
            0 => PLLFLLSELR::_00,
            1 => PLLFLLSELR::_01,
            2 => PLLFLLSELR::_10,
            3 => PLLFLLSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PLLFLLSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PLLFLLSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSELR::_11
    }
}
#[doc = "Possible values of the field `USBSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRCR {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1,
}
impl USBSRCR {
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
            USBSRCR::_0 => false,
            USBSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSRCR {
        match value {
            false => USBSRCR::_0,
            true => USBSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSRCR::_1
    }
}
#[doc = "Possible values of the field `RMIISRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMIISRCR {
    #[doc = "EXTAL clock"]
    _0,
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    _1,
}
impl RMIISRCR {
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
            RMIISRCR::_0 => false,
            RMIISRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMIISRCR {
        match value {
            false => RMIISRCR::_0,
            true => RMIISRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RMIISRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RMIISRCR::_1
    }
}
#[doc = "Possible values of the field `TIMESRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMESRCR {
    #[doc = "Core/system clock."]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    _11,
}
impl TIMESRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMESRCR::_00 => 0,
            TIMESRCR::_01 => 1,
            TIMESRCR::_10 => 2,
            TIMESRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMESRCR {
        match value {
            0 => TIMESRCR::_00,
            1 => TIMESRCR::_01,
            2 => TIMESRCR::_10,
            3 => TIMESRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TIMESRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TIMESRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMESRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMESRCR::_11
    }
}
#[doc = "Possible values of the field `TPMSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPMSRCR::_00 => 0,
            TPMSRCR::_01 => 1,
            TPMSRCR::_10 => 2,
            TPMSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPMSRCR {
        match value {
            0 => TPMSRCR::_00,
            1 => TPMSRCR::_01,
            2 => TPMSRCR::_10,
            3 => TPMSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPMSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPMSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPMSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TPMSRCR::_11
    }
}
#[doc = "Possible values of the field `LPUARTSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUARTSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUARTSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUARTSRCR::_00 => 0,
            LPUARTSRCR::_01 => 1,
            LPUARTSRCR::_10 => 2,
            LPUARTSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUARTSRCR {
        match value {
            0 => LPUARTSRCR::_00,
            1 => LPUARTSRCR::_01,
            2 => LPUARTSRCR::_10,
            3 => LPUARTSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUARTSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUARTSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUARTSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LPUARTSRCR::_11
    }
}
#[doc = "Possible values of the field `SDHCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHCSRCR {
    #[doc = "Core/system clock."]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "External bypass clock (SDHC0_CLKIN)"]
    _11,
}
impl SDHCSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDHCSRCR::_00 => 0,
            SDHCSRCR::_01 => 1,
            SDHCSRCR::_10 => 2,
            SDHCSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDHCSRCR {
        match value {
            0 => SDHCSRCR::_00,
            1 => SDHCSRCR::_01,
            2 => SDHCSRCR::_10,
            3 => SDHCSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SDHCSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SDHCSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SDHCSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SDHCSRCR::_11
    }
}
#[doc = "Values that can be written to the field `USBSLSRC`"]
pub enum USBSLSRCW {
    #[doc = "MCGIRCLK"]
    _0,
    #[doc = "RTC 32.768kHz clock"]
    _1,
}
impl USBSLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSLSRCW::_0 => false,
            USBSLSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSLSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSLSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSLSRCW::_0)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSLSRCW::_1)
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
#[doc = "Values that can be written to the field `USBREGEN`"]
pub enum USBREGENW {
    #[doc = "USB PHY PLL Regulator disabled."]
    _0,
    #[doc = "USB PHY PLL Regulator enabled."]
    _1,
}
impl USBREGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBREGENW::_0 => false,
            USBREGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBREGENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBREGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBREGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB PHY PLL Regulator disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBREGENW::_0)
    }
    #[doc = "USB PHY PLL Regulator enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBREGENW::_1)
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
#[doc = "Values that can be written to the field `RTCCLKOUTSEL`"]
pub enum RTCCLKOUTSELW {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCCLKOUTSELW::_0 => false,
            RTCCLKOUTSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCLKOUTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSELW::_1)
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
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "FlexBus CLKOUT"]
    _000,
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "RTC 32.768kHz clock"]
    _101,
    #[doc = "OSCERCLK0"]
    _110,
    #[doc = "IRC 48 MHz clock"]
    _111,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_000 => 0,
            CLKOUTSELW::_010 => 2,
            CLKOUTSELW::_011 => 3,
            CLKOUTSELW::_100 => 4,
            CLKOUTSELW::_101 => 5,
            CLKOUTSELW::_110 => 6,
            CLKOUTSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FlexBus CLKOUT"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_000)
    }
    #[doc = "Flash clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_100)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_101)
    }
    #[doc = "OSCERCLK0"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_111)
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
#[doc = "Values that can be written to the field `FBSL`"]
pub enum FBSLW {
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _00,
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _01,
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10,
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    _11,
}
impl FBSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FBSLW::_00 => 0,
            FBSLW::_01 => 1,
            FBSLW::_10 => 2,
            FBSLW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBSLW<'a> {
    w: &'a mut W,
}
impl<'a> _FBSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBSLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FBSLW::_00)
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FBSLW::_01)
    }
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FBSLW::_10)
    }
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FBSLW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACECLKSEL`"]
pub enum TRACECLKSELW {
    #[doc = "MCGOUTCLK, divided by the TRACECLK fractional divider as configured by SIM_CLKDIV4\\[TRACEFRAC, TRACEDIV\\]"]
    _0,
    #[doc = "Core/system clock"]
    _1,
}
impl TRACECLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACECLKSELW::_0 => false,
            TRACECLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACECLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACECLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACECLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGOUTCLK, divided by the TRACECLK fractional divider as configured by SIM_CLKDIV4\\[TRACEFRAC, TRACEDIV\\]"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSELW::_0)
    }
    #[doc = "Core/system clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSELW::_1)
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
#[doc = "Values that can be written to the field `PLLFLLSEL`"]
pub enum PLLFLLSELW {
    #[doc = "MCGFLLCLK clock"]
    _00,
    #[doc = "MCGPLLCLK clock"]
    _01,
    #[doc = "USB1 PFD clock"]
    _10,
    #[doc = "IRC48 MHz clock"]
    _11,
}
impl PLLFLLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLFLLSELW::_00 => 0,
            PLLFLLSELW::_01 => 1,
            PLLFLLSELW::_10 => 2,
            PLLFLLSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLFLLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFLLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLFLLSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_00)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_01)
    }
    #[doc = "USB1 PFD clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_10)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBSRC`"]
pub enum USBSRCW {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1,
}
impl USBSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSRCW::_0 => false,
            USBSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRCW::_0)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRCW::_1)
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
#[doc = "Values that can be written to the field `RMIISRC`"]
pub enum RMIISRCW {
    #[doc = "EXTAL clock"]
    _0,
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    _1,
}
impl RMIISRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMIISRCW::_0 => false,
            RMIISRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMIISRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RMIISRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMIISRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EXTAL clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMIISRCW::_0)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMIISRCW::_1)
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
#[doc = "Values that can be written to the field `TIMESRC`"]
pub enum TIMESRCW {
    #[doc = "Core/system clock."]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    _11,
}
impl TIMESRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMESRCW::_00 => 0,
            TIMESRCW::_01 => 1,
            TIMESRCW::_10 => 2,
            TIMESRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMESRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMESRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMESRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Core/system clock."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIMESRCW::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIMESRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMESRCW::_10)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMESRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPMSRC`"]
pub enum TPMSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPMSRCW::_00 => 0,
            TPMSRCW::_01 => 1,
            TPMSRCW::_10 => 2,
            TPMSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPMSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPMSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRCW::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUARTSRC`"]
pub enum LPUARTSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUARTSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUARTSRCW::_00 => 0,
            LPUARTSRCW::_01 => 1,
            LPUARTSRCW::_10 => 2,
            LPUARTSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUARTSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUARTSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUARTSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDHCSRC`"]
pub enum SDHCSRCW {
    #[doc = "Core/system clock."]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "External bypass clock (SDHC0_CLKIN)"]
    _11,
}
impl SDHCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDHCSRCW::_00 => 0,
            SDHCSRCW::_01 => 1,
            SDHCSRCW::_10 => 2,
            SDHCSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDHCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SDHCSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDHCSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Core/system clock."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SDHCSRCW::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SDHCSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDHCSRCW::_10)
    }
    #[doc = "External bypass clock (SDHC0_CLKIN)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SDHCSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - USB Slow Clock Source"]
    #[inline]
    pub fn usbslsrc(&self) -> USBSLSRCR {
        USBSLSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB PHY PLL Regulator Enable"]
    #[inline]
    pub fn usbregen(&self) -> USBREGENR {
        USBREGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSELR {
        RTCCLKOUTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline]
    pub fn fbsl(&self) -> FBSLR {
        FBSLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclksel(&self) -> TRACECLKSELR {
        TRACECLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&self) -> PLLFLLSELR {
        PLLFLLSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&self) -> USBSRCR {
        USBSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline]
    pub fn rmiisrc(&self) -> RMIISRCR {
        RMIISRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline]
    pub fn timesrc(&self) -> TIMESRCR {
        TIMESRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&self) -> TPMSRCR {
        TPMSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline]
    pub fn lpuartsrc(&self) -> LPUARTSRCR {
        LPUARTSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline]
    pub fn sdhcsrc(&self) -> SDHCSRCR {
        SDHCSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB Slow Clock Source"]
    #[inline]
    pub fn usbslsrc(&mut self) -> _USBSLSRCW {
        _USBSLSRCW { w: self }
    }
    #[doc = "Bit 1 - USB PHY PLL Regulator Enable"]
    #[inline]
    pub fn usbregen(&mut self) -> _USBREGENW {
        _USBREGENW { w: self }
    }
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkoutsel(&mut self) -> _RTCCLKOUTSELW {
        _RTCCLKOUTSELW { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline]
    pub fn fbsl(&mut self) -> _FBSLW {
        _FBSLW { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclksel(&mut self) -> _TRACECLKSELW {
        _TRACECLKSELW { w: self }
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&mut self) -> _PLLFLLSELW {
        _PLLFLLSELW { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&mut self) -> _USBSRCW {
        _USBSRCW { w: self }
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline]
    pub fn rmiisrc(&mut self) -> _RMIISRCW {
        _RMIISRCW { w: self }
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline]
    pub fn timesrc(&mut self) -> _TIMESRCW {
        _TIMESRCW { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&mut self) -> _TPMSRCW {
        _TPMSRCW { w: self }
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline]
    pub fn lpuartsrc(&mut self) -> _LPUARTSRCW {
        _LPUARTSRCW { w: self }
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline]
    pub fn sdhcsrc(&mut self) -> _SDHCSRCW {
        _SDHCSRCW { w: self }
    }
}
