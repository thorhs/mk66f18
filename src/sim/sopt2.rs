#[doc = "Reader of register SOPT2"]
pub type R = crate::R<u32, super::SOPT2>;
#[doc = "Writer for register SOPT2"]
pub type W = crate::W<u32, super::SOPT2>;
#[doc = "Register SOPT2 `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::SOPT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "USB Slow Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSLSRC_A {
    #[doc = "0: MCGIRCLK"]
    _0,
    #[doc = "1: RTC 32.768kHz clock"]
    _1,
}
impl From<USBSLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSLSRC_A) -> Self {
        match variant {
            USBSLSRC_A::_0 => false,
            USBSLSRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBSLSRC`"]
pub type USBSLSRC_R = crate::R<bool, USBSLSRC_A>;
impl USBSLSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSLSRC_A {
        match self.bits {
            false => USBSLSRC_A::_0,
            true => USBSLSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSLSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSLSRC_A::_1
    }
}
#[doc = "Write proxy for field `USBSLSRC`"]
pub struct USBSLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSLSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSLSRC_A::_0)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSLSRC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "USB PHY PLL Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREGEN_A {
    #[doc = "0: USB PHY PLL Regulator disabled."]
    _0,
    #[doc = "1: USB PHY PLL Regulator enabled."]
    _1,
}
impl From<USBREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBREGEN_A) -> Self {
        match variant {
            USBREGEN_A::_0 => false,
            USBREGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBREGEN`"]
pub type USBREGEN_R = crate::R<bool, USBREGEN_A>;
impl USBREGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBREGEN_A {
        match self.bits {
            false => USBREGEN_A::_0,
            true => USBREGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBREGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBREGEN_A::_1
    }
}
#[doc = "Write proxy for field `USBREGEN`"]
pub struct USBREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBREGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBREGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB PHY PLL Regulator disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBREGEN_A::_0)
    }
    #[doc = "USB PHY PLL Regulator enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBREGEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSEL_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "1: RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl From<RTCCLKOUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTSEL_A) -> Self {
        match variant {
            RTCCLKOUTSEL_A::_0 => false,
            RTCCLKOUTSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RTCCLKOUTSEL`"]
pub type RTCCLKOUTSEL_R = crate::R<bool, RTCCLKOUTSEL_A>;
impl RTCCLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTSEL_A {
        match self.bits {
            false => RTCCLKOUTSEL_A::_0,
            true => RTCCLKOUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_1
    }
}
#[doc = "Write proxy for field `RTCCLKOUTSEL`"]
pub struct RTCCLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKOUTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL_A {
    #[doc = "0: FlexBus CLKOUT"]
    _000,
    #[doc = "2: Flash clock"]
    _010,
    #[doc = "3: LPO clock (1 kHz)"]
    _011,
    #[doc = "4: MCGIRCLK"]
    _100,
    #[doc = "5: RTC 32.768kHz clock"]
    _101,
    #[doc = "6: OSCERCLK0"]
    _110,
    #[doc = "7: IRC 48 MHz clock"]
    _111,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        match variant {
            CLKOUTSEL_A::_000 => 0,
            CLKOUTSEL_A::_010 => 2,
            CLKOUTSEL_A::_011 => 3,
            CLKOUTSEL_A::_100 => 4,
            CLKOUTSEL_A::_101 => 5,
            CLKOUTSEL_A::_110 => 6,
            CLKOUTSEL_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL_A::_000),
            2 => Val(CLKOUTSEL_A::_010),
            3 => Val(CLKOUTSEL_A::_011),
            4 => Val(CLKOUTSEL_A::_100),
            5 => Val(CLKOUTSEL_A::_101),
            6 => Val(CLKOUTSEL_A::_110),
            7 => Val(CLKOUTSEL_A::_111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTSEL_A::_111
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FlexBus CLKOUT"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_000)
    }
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_101)
    }
    #[doc = "OSCERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "FlexBus security level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBSL_A {
    #[doc = "0: All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _00,
    #[doc = "1: All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    _01,
    #[doc = "2: Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10,
    #[doc = "3: Off-chip instruction accesses and data accesses are allowed."]
    _11,
}
impl From<FBSL_A> for u8 {
    #[inline(always)]
    fn from(variant: FBSL_A) -> Self {
        match variant {
            FBSL_A::_00 => 0,
            FBSL_A::_01 => 1,
            FBSL_A::_10 => 2,
            FBSL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FBSL`"]
pub type FBSL_R = crate::R<u8, FBSL_A>;
impl FBSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBSL_A {
        match self.bits {
            0 => FBSL_A::_00,
            1 => FBSL_A::_01,
            2 => FBSL_A::_10,
            3 => FBSL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FBSL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FBSL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FBSL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FBSL_A::_11
    }
}
#[doc = "Write proxy for field `FBSL`"]
pub struct FBSL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBSL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FBSL_A::_00)
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus or SDRAM are disallowed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FBSL_A::_01)
    }
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FBSL_A::_10)
    }
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FBSL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Debug trace clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLKSEL_A {
    #[doc = "0: MCGOUTCLK, divided by the TRACECLK fractional divider as configured by SIM_CLKDIV4\\[TRACEFRAC, TRACEDIV\\]"]
    _0,
    #[doc = "1: Core/system clock"]
    _1,
}
impl From<TRACECLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLKSEL_A) -> Self {
        match variant {
            TRACECLKSEL_A::_0 => false,
            TRACECLKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TRACECLKSEL`"]
pub type TRACECLKSEL_R = crate::R<bool, TRACECLKSEL_A>;
impl TRACECLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLKSEL_A {
        match self.bits {
            false => TRACECLKSEL_A::_0,
            true => TRACECLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TRACECLKSEL`"]
pub struct TRACECLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGOUTCLK, divided by the TRACECLK fractional divider as configured by SIM_CLKDIV4\\[TRACEFRAC, TRACEDIV\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_0)
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _00,
    #[doc = "1: MCGPLLCLK clock"]
    _01,
    #[doc = "2: USB1 PFD clock"]
    _10,
    #[doc = "3: IRC48 MHz clock"]
    _11,
}
impl From<PLLFLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        match variant {
            PLLFLLSEL_A::_00 => 0,
            PLLFLLSEL_A::_01 => 1,
            PLLFLLSEL_A::_10 => 2,
            PLLFLLSEL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PLLFLLSEL`"]
pub type PLLFLLSEL_R = crate::R<u8, PLLFLLSEL_A>;
impl PLLFLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLFLLSEL_A {
        match self.bits {
            0 => PLLFLLSEL_A::_00,
            1 => PLLFLLSEL_A::_01,
            2 => PLLFLLSEL_A::_10,
            3 => PLLFLLSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLLFLLSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLLFLLSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSEL_A::_11
    }
}
#[doc = "Write proxy for field `PLLFLLSEL`"]
pub struct PLLFLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLFLLSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_00)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_01)
    }
    #[doc = "USB1 PFD clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_10)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRC_A {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1,
}
impl From<USBSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSRC_A) -> Self {
        match variant {
            USBSRC_A::_0 => false,
            USBSRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBSRC`"]
pub type USBSRC_R = crate::R<bool, USBSRC_A>;
impl USBSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSRC_A {
        match self.bits {
            false => USBSRC_A::_0,
            true => USBSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSRC_A::_1
    }
}
#[doc = "Write proxy for field `USBSRC`"]
pub struct USBSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRC_A::_0)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "RMII clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMIISRC_A {
    #[doc = "0: EXTAL clock"]
    _0,
    #[doc = "1: External bypass clock (ENET_1588_CLKIN)."]
    _1,
}
impl From<RMIISRC_A> for bool {
    #[inline(always)]
    fn from(variant: RMIISRC_A) -> Self {
        match variant {
            RMIISRC_A::_0 => false,
            RMIISRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RMIISRC`"]
pub type RMIISRC_R = crate::R<bool, RMIISRC_A>;
impl RMIISRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMIISRC_A {
        match self.bits {
            false => RMIISRC_A::_0,
            true => RMIISRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMIISRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMIISRC_A::_1
    }
}
#[doc = "Write proxy for field `RMIISRC`"]
pub struct RMIISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIISRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMIISRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EXTAL clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMIISRC_A::_0)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMIISRC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "IEEE 1588 timestamp clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMESRC_A {
    #[doc = "0: Core/system clock."]
    _00,
    #[doc = "1: MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "2: OSCERCLK clock"]
    _10,
    #[doc = "3: External bypass clock (ENET_1588_CLKIN)."]
    _11,
}
impl From<TIMESRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMESRC_A) -> Self {
        match variant {
            TIMESRC_A::_00 => 0,
            TIMESRC_A::_01 => 1,
            TIMESRC_A::_10 => 2,
            TIMESRC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TIMESRC`"]
pub type TIMESRC_R = crate::R<u8, TIMESRC_A>;
impl TIMESRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMESRC_A {
        match self.bits {
            0 => TIMESRC_A::_00,
            1 => TIMESRC_A::_01,
            2 => TIMESRC_A::_10,
            3 => TIMESRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIMESRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIMESRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TIMESRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TIMESRC_A::_11
    }
}
#[doc = "Write proxy for field `TIMESRC`"]
pub struct TIMESRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMESRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Core/system clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIMESRC_A::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIMESRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMESRC_A::_10)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMESRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "TPM clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMSRC_A {
    #[doc = "0: Clock disabled"]
    _00,
    #[doc = "1: MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "2: OSCERCLK clock"]
    _10,
    #[doc = "3: MCGIRCLK clock"]
    _11,
}
impl From<TPMSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPMSRC_A) -> Self {
        match variant {
            TPMSRC_A::_00 => 0,
            TPMSRC_A::_01 => 1,
            TPMSRC_A::_10 => 2,
            TPMSRC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TPMSRC`"]
pub type TPMSRC_R = crate::R<u8, TPMSRC_A>;
impl TPMSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPMSRC_A {
        match self.bits {
            0 => TPMSRC_A::_00,
            1 => TPMSRC_A::_01,
            2 => TPMSRC_A::_10,
            3 => TPMSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPMSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPMSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPMSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPMSRC_A::_11
    }
}
#[doc = "Write proxy for field `TPMSRC`"]
pub struct TPMSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPMSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPMSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRC_A::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "LPUART clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUARTSRC_A {
    #[doc = "0: Clock disabled"]
    _00,
    #[doc = "1: MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    _01,
    #[doc = "2: OSCERCLK clock"]
    _10,
    #[doc = "3: MCGIRCLK clock"]
    _11,
}
impl From<LPUARTSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUARTSRC_A) -> Self {
        match variant {
            LPUARTSRC_A::_00 => 0,
            LPUARTSRC_A::_01 => 1,
            LPUARTSRC_A::_10 => 2,
            LPUARTSRC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `LPUARTSRC`"]
pub type LPUARTSRC_R = crate::R<u8, LPUARTSRC_A>;
impl LPUARTSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUARTSRC_A {
        match self.bits {
            0 => LPUARTSRC_A::_00,
            1 => LPUARTSRC_A::_01,
            2 => LPUARTSRC_A::_10,
            3 => LPUARTSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUARTSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUARTSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUARTSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LPUARTSRC_A::_11
    }
}
#[doc = "Write proxy for field `LPUARTSRC`"]
pub struct LPUARTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUARTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUARTSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC, PLLFLLDIV\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "SDHC clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHCSRC_A {
    #[doc = "0: Core/system clock."]
    _00,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "2: OSCERCLK clock"]
    _10,
    #[doc = "3: External bypass clock (SDHC0_CLKIN)"]
    _11,
}
impl From<SDHCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SDHCSRC_A) -> Self {
        match variant {
            SDHCSRC_A::_00 => 0,
            SDHCSRC_A::_01 => 1,
            SDHCSRC_A::_10 => 2,
            SDHCSRC_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `SDHCSRC`"]
pub type SDHCSRC_R = crate::R<u8, SDHCSRC_A>;
impl SDHCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHCSRC_A {
        match self.bits {
            0 => SDHCSRC_A::_00,
            1 => SDHCSRC_A::_01,
            2 => SDHCSRC_A::_10,
            3 => SDHCSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SDHCSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SDHCSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SDHCSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SDHCSRC_A::_11
    }
}
#[doc = "Write proxy for field `SDHCSRC`"]
pub struct SDHCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDHCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDHCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Core/system clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M , or USB1 PFD clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_10)
    }
    #[doc = "External bypass clock (SDHC0_CLKIN)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB Slow Clock Source"]
    #[inline(always)]
    pub fn usbslsrc(&self) -> USBSLSRC_R {
        USBSLSRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB PHY PLL Regulator Enable"]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSEL_R {
        RTCCLKOUTSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    pub fn fbsl(&self) -> FBSL_R {
        FBSL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&self) -> TRACECLKSEL_R {
        TRACECLKSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline(always)]
    pub fn rmiisrc(&self) -> RMIISRC_R {
        RMIISRC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline(always)]
    pub fn timesrc(&self) -> TIMESRC_R {
        TIMESRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&self) -> TPMSRC_R {
        TPMSRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline(always)]
    pub fn lpuartsrc(&self) -> LPUARTSRC_R {
        LPUARTSRC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline(always)]
    pub fn sdhcsrc(&self) -> SDHCSRC_R {
        SDHCSRC_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Slow Clock Source"]
    #[inline(always)]
    pub fn usbslsrc(&mut self) -> USBSLSRC_W {
        USBSLSRC_W { w: self }
    }
    #[doc = "Bit 1 - USB PHY PLL Regulator Enable"]
    #[inline(always)]
    pub fn usbregen(&mut self) -> USBREGEN_W {
        USBREGEN_W { w: self }
    }
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&mut self) -> RTCCLKOUTSEL_W {
        RTCCLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    pub fn fbsl(&mut self) -> FBSL_W {
        FBSL_W { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&mut self) -> TRACECLKSEL_W {
        TRACECLKSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W {
        PLLFLLSEL_W { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&mut self) -> USBSRC_W {
        USBSRC_W { w: self }
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline(always)]
    pub fn rmiisrc(&mut self) -> RMIISRC_W {
        RMIISRC_W { w: self }
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline(always)]
    pub fn timesrc(&mut self) -> TIMESRC_W {
        TIMESRC_W { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&mut self) -> TPMSRC_W {
        TPMSRC_W { w: self }
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline(always)]
    pub fn lpuartsrc(&mut self) -> LPUARTSRC_W {
        LPUARTSRC_W { w: self }
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline(always)]
    pub fn sdhcsrc(&mut self) -> SDHCSRC_W {
        SDHCSRC_W { w: self }
    }
}
