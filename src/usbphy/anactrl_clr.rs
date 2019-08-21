#[doc = "Reader of register ANACTRL_CLR"]
pub type R = crate::R<u32, super::ANACTRL_CLR>;
#[doc = "Writer for register ANACTRL_CLR"]
pub type W = crate::W<u32, super::ANACTRL_CLR>;
#[doc = "Register ANACTRL_CLR `reset()`'s with value 0x0402"]
impl crate::ResetValue for super::ANACTRL_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0402
    }
}
#[doc = "Reader of field `TESTCLK_SEL`"]
pub type TESTCLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTCLK_SEL`"]
pub struct TESTCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTCLK_SEL_W<'a> {
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
#[doc = "This bit field controls clock gating (disabling) for the PFD pfd_clk output for power savings when the PFD is not used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFD_CLKGATE_A {
    #[doc = "0: PFD clock output is enabled"]
    _0,
    #[doc = "1: PFD clock output is gated (Default)"]
    _1,
}
impl From<PFD_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: PFD_CLKGATE_A) -> Self {
        match variant {
            PFD_CLKGATE_A::_0 => false,
            PFD_CLKGATE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PFD_CLKGATE`"]
pub type PFD_CLKGATE_R = crate::R<bool, PFD_CLKGATE_A>;
impl PFD_CLKGATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD_CLKGATE_A {
        match self.bits {
            false => PFD_CLKGATE_A::_0,
            true => PFD_CLKGATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFD_CLKGATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFD_CLKGATE_A::_1
    }
}
#[doc = "Write proxy for field `PFD_CLKGATE`"]
pub struct PFD_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_CLKGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFD_CLKGATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PFD clock output is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFD_CLKGATE_A::_0)
    }
    #[doc = "PFD clock output is gated (Default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFD_CLKGATE_A::_1)
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
#[doc = "This bit field for the PFD selects the frequency relationship between the local pfd_clk output and the exported USB1PFDCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFD_CLK_SEL_A {
    #[doc = "0: USB1PFDCLK is the same frequency as the xtal clock (Default)"]
    _00,
    #[doc = "1: USB1PFDCLK frequency is pfd_clk divided by 4"]
    _01,
    #[doc = "2: USB1PFDCLK frequency is pfd_clk divided by 2"]
    _10,
    #[doc = "3: USB1PFDCLK frequency is the same as pfd_clk frequency"]
    _11,
}
impl From<PFD_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PFD_CLK_SEL_A) -> Self {
        match variant {
            PFD_CLK_SEL_A::_00 => 0,
            PFD_CLK_SEL_A::_01 => 1,
            PFD_CLK_SEL_A::_10 => 2,
            PFD_CLK_SEL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PFD_CLK_SEL`"]
pub type PFD_CLK_SEL_R = crate::R<u8, PFD_CLK_SEL_A>;
impl PFD_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFD_CLK_SEL_A {
        match self.bits {
            0 => PFD_CLK_SEL_A::_00,
            1 => PFD_CLK_SEL_A::_01,
            2 => PFD_CLK_SEL_A::_10,
            3 => PFD_CLK_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PFD_CLK_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PFD_CLK_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PFD_CLK_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PFD_CLK_SEL_A::_11
    }
}
#[doc = "Write proxy for field `PFD_CLK_SEL`"]
pub struct PFD_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFD_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USB1PFDCLK is the same frequency as the xtal clock (Default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PFD_CLK_SEL_A::_00)
    }
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PFD_CLK_SEL_A::_01)
    }
    #[doc = "USB1PFDCLK frequency is pfd_clk divided by 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PFD_CLK_SEL_A::_10)
    }
    #[doc = "USB1PFDCLK frequency is the same as pfd_clk frequency"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PFD_CLK_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PFD_FRAC`"]
pub type PFD_FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD_FRAC`"]
pub struct PFD_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "0: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    _0,
    #[doc = "1: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    _1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        match variant {
            DEV_PULLDOWN_A::_0 => false,
            DEV_PULLDOWN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DEV_PULLDOWN`"]
pub type DEV_PULLDOWN_R = crate::R<bool, DEV_PULLDOWN_A>;
impl DEV_PULLDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::_0,
            true => DEV_PULLDOWN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEV_PULLDOWN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEV_PULLDOWN_A::_1
    }
}
#[doc = "Write proxy for field `DEV_PULLDOWN`"]
pub struct DEV_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PULLDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_PULLDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::_0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Controls pre-emphasis time duration for the High Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_PULSE_CTRL_A {
    #[doc = "0: Minimum duration of pre-emphasis current after each data transition"]
    _00,
    #[doc = "3: Maximum duration of pre-emphasis current after each data transition"]
    _11,
}
impl From<EMPH_PULSE_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: EMPH_PULSE_CTRL_A) -> Self {
        match variant {
            EMPH_PULSE_CTRL_A::_00 => 0,
            EMPH_PULSE_CTRL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `EMPH_PULSE_CTRL`"]
pub type EMPH_PULSE_CTRL_R = crate::R<u8, EMPH_PULSE_CTRL_A>;
impl EMPH_PULSE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EMPH_PULSE_CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EMPH_PULSE_CTRL_A::_00),
            3 => Val(EMPH_PULSE_CTRL_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EMPH_PULSE_CTRL_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EMPH_PULSE_CTRL_A::_11
    }
}
#[doc = "Write proxy for field `EMPH_PULSE_CTRL`"]
pub struct EMPH_PULSE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPH_PULSE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMPH_PULSE_CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Minimum duration of pre-emphasis current after each data transition"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMPH_PULSE_CTRL_A::_00)
    }
    #[doc = "Maximum duration of pre-emphasis current after each data transition"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMPH_PULSE_CTRL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Enables pre-emphasis for the High-Speed TX drivers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_EN_A {
    #[doc = "0: No pre-emphasis is used on HS TX output drivers"]
    _0,
    #[doc = "1: Enables pre-emphasis for HS TX output drivers"]
    _1,
}
impl From<EMPH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EMPH_EN_A) -> Self {
        match variant {
            EMPH_EN_A::_0 => false,
            EMPH_EN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EMPH_EN`"]
pub type EMPH_EN_R = crate::R<bool, EMPH_EN_A>;
impl EMPH_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPH_EN_A {
        match self.bits {
            false => EMPH_EN_A::_0,
            true => EMPH_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMPH_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMPH_EN_A::_1
    }
}
#[doc = "Write proxy for field `EMPH_EN`"]
pub struct EMPH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPH_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMPH_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No pre-emphasis is used on HS TX output drivers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMPH_EN_A::_0)
    }
    #[doc = "Enables pre-emphasis for HS TX output drivers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMPH_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Controls the amount of pre-emphasis current added for the High-Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPH_CUR_CTRL_A {
    #[doc = "0: No pre-emphasis current is enabled for the HS TX drivers"]
    _00,
    #[doc = "1: One unit of pre-emphasis current is enabled for the HS TX drivers"]
    _01,
    #[doc = "2: Two units of pre-emphasis current are enabled for the HS TX drivers"]
    _10,
    #[doc = "3: Three units of pre-emphasis current are enabled for the HS TX drivers"]
    _11,
}
impl From<EMPH_CUR_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: EMPH_CUR_CTRL_A) -> Self {
        match variant {
            EMPH_CUR_CTRL_A::_00 => 0,
            EMPH_CUR_CTRL_A::_01 => 1,
            EMPH_CUR_CTRL_A::_10 => 2,
            EMPH_CUR_CTRL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `EMPH_CUR_CTRL`"]
pub type EMPH_CUR_CTRL_R = crate::R<u8, EMPH_CUR_CTRL_A>;
impl EMPH_CUR_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPH_CUR_CTRL_A {
        match self.bits {
            0 => EMPH_CUR_CTRL_A::_00,
            1 => EMPH_CUR_CTRL_A::_01,
            2 => EMPH_CUR_CTRL_A::_10,
            3 => EMPH_CUR_CTRL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EMPH_CUR_CTRL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EMPH_CUR_CTRL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EMPH_CUR_CTRL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EMPH_CUR_CTRL_A::_11
    }
}
#[doc = "Write proxy for field `EMPH_CUR_CTRL`"]
pub struct EMPH_CUR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPH_CUR_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMPH_CUR_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No pre-emphasis current is enabled for the HS TX drivers"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRL_A::_00)
    }
    #[doc = "One unit of pre-emphasis current is enabled for the HS TX drivers"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRL_A::_01)
    }
    #[doc = "Two units of pre-emphasis current are enabled for the HS TX drivers"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRL_A::_10)
    }
    #[doc = "Three units of pre-emphasis current are enabled for the HS TX drivers"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMPH_CUR_CTRL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PFD_STABLE`"]
pub type PFD_STABLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Test clock selection to analog test"]
    #[inline(always)]
    pub fn testclk_sel(&self) -> TESTCLK_SEL_R {
        TESTCLK_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit field controls clock gating (disabling) for the PFD pfd_clk output for power savings when the PFD is not used"]
    #[inline(always)]
    pub fn pfd_clkgate(&self) -> PFD_CLKGATE_R {
        PFD_CLKGATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - This bit field for the PFD selects the frequency relationship between the local pfd_clk output and the exported USB1PFDCLK"]
    #[inline(always)]
    pub fn pfd_clk_sel(&self) -> PFD_CLK_SEL_R {
        PFD_CLK_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:9 - PFD fractional divider setting used to select the pfd_clk output frequency"]
    #[inline(always)]
    pub fn pfd_frac(&self) -> PFD_FRAC_R {
        PFD_FRAC_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Controls pre-emphasis time duration for the High Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline(always)]
    pub fn emph_pulse_ctrl(&self) -> EMPH_PULSE_CTRL_R {
        EMPH_PULSE_CTRL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Enables pre-emphasis for the High-Speed TX drivers"]
    #[inline(always)]
    pub fn emph_en(&self) -> EMPH_EN_R {
        EMPH_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Controls the amount of pre-emphasis current added for the High-Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline(always)]
    pub fn emph_cur_ctrl(&self) -> EMPH_CUR_CTRL_R {
        EMPH_CUR_CTRL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 31 - PFD stable signal from the Phase Fractional Divider."]
    #[inline(always)]
    pub fn pfd_stable(&self) -> PFD_STABLE_R {
        PFD_STABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test clock selection to analog test"]
    #[inline(always)]
    pub fn testclk_sel(&mut self) -> TESTCLK_SEL_W {
        TESTCLK_SEL_W { w: self }
    }
    #[doc = "Bit 1 - This bit field controls clock gating (disabling) for the PFD pfd_clk output for power savings when the PFD is not used"]
    #[inline(always)]
    pub fn pfd_clkgate(&mut self) -> PFD_CLKGATE_W {
        PFD_CLKGATE_W { w: self }
    }
    #[doc = "Bits 2:3 - This bit field for the PFD selects the frequency relationship between the local pfd_clk output and the exported USB1PFDCLK"]
    #[inline(always)]
    pub fn pfd_clk_sel(&mut self) -> PFD_CLK_SEL_W {
        PFD_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:9 - PFD fractional divider setting used to select the pfd_clk output frequency"]
    #[inline(always)]
    pub fn pfd_frac(&mut self) -> PFD_FRAC_W {
        PFD_FRAC_W { w: self }
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W {
        DEV_PULLDOWN_W { w: self }
    }
    #[doc = "Bits 11:12 - Controls pre-emphasis time duration for the High Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline(always)]
    pub fn emph_pulse_ctrl(&mut self) -> EMPH_PULSE_CTRL_W {
        EMPH_PULSE_CTRL_W { w: self }
    }
    #[doc = "Bit 13 - Enables pre-emphasis for the High-Speed TX drivers"]
    #[inline(always)]
    pub fn emph_en(&mut self) -> EMPH_EN_W {
        EMPH_EN_W { w: self }
    }
    #[doc = "Bits 14:15 - Controls the amount of pre-emphasis current added for the High-Speed TX drivers after each data transition when the USBPHY_ANACTRL\\[EMPH_EN\\] bit is set high to 1'b1"]
    #[inline(always)]
    pub fn emph_cur_ctrl(&mut self) -> EMPH_CUR_CTRL_W {
        EMPH_CUR_CTRL_W { w: self }
    }
}
