#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIM_OVERRIDE_EN_SET {
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
pub struct TRIM_DIV_SEL_OVERRIDER {
    bits: bool,
}
impl TRIM_DIV_SEL_OVERRIDER {
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
pub struct TRIM_ENV_TAIL_ADJ_VD_OVERRIDER {
    bits: bool,
}
impl TRIM_ENV_TAIL_ADJ_VD_OVERRIDER {
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
pub struct TRIM_TX_D_CAL_OVERRIDER {
    bits: bool,
}
impl TRIM_TX_D_CAL_OVERRIDER {
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
pub struct TRIM_TX_CAL45DP_OVERRIDER {
    bits: bool,
}
impl TRIM_TX_CAL45DP_OVERRIDER {
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
pub struct TRIM_TX_CAL45DM_OVERRIDER {
    bits: bool,
}
impl TRIM_TX_CAL45DM_OVERRIDER {
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
pub struct TRIM_PLL_CTRL0_DIV_SELR {
    bits: u8,
}
impl TRIM_PLL_CTRL0_DIV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
    bits: u8,
}
impl TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_D_CALR {
    bits: u8,
}
impl TRIM_USBPHY_TX_D_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_CAL45DPR {
    bits: u8,
}
impl TRIM_USBPHY_TX_CAL45DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_CAL45DMR {
    bits: u8,
}
impl TRIM_USBPHY_TX_CAL45DMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_DIV_SEL_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_DIV_SEL_OVERRIDEW<'a> {
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
pub struct _TRIM_ENV_TAIL_ADJ_VD_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_ENV_TAIL_ADJ_VD_OVERRIDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TRIM_TX_D_CAL_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_TX_D_CAL_OVERRIDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TRIM_TX_CAL45DP_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_TX_CAL45DP_OVERRIDEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TRIM_TX_CAL45DM_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_TX_CAL45DM_OVERRIDEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in USBPHY_PLL_SIC\\[1:0\\] will be used."]
    #[inline]
    pub fn trim_div_sel_override(&self) -> TRIM_DIV_SEL_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIM_DIV_SEL_OVERRIDER { bits }
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in USBPHY_DEBUG1\\[14:13\\] will be used"]
    #[inline]
    pub fn trim_env_tail_adj_vd_override(&self) -> TRIM_ENV_TAIL_ADJ_VD_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIM_ENV_TAIL_ADJ_VD_OVERRIDER { bits }
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in USBPHY_TX\\[3:0\\] will be used."]
    #[inline]
    pub fn trim_tx_d_cal_override(&self) -> TRIM_TX_D_CAL_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIM_TX_D_CAL_OVERRIDER { bits }
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in USBPHY_TX\\[19:16\\] will be used."]
    #[inline]
    pub fn trim_tx_cal45dp_override(&self) -> TRIM_TX_CAL45DP_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIM_TX_CAL45DP_OVERRIDER { bits }
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in USBPHY_TX\\[11:8\\] will be used."]
    #[inline]
    pub fn trim_tx_cal45dm_override(&self) -> TRIM_TX_CAL45DM_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRIM_TX_CAL45DM_OVERRIDER { bits }
    }
    #[doc = "Bits 16:17 - IFR value of PLL_DIV_SEL."]
    #[inline]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TRIM_PLL_CTRL0_DIV_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_PLL_CTRL0_DIV_SELR { bits }
    }
    #[doc = "Bits 18:19 - IFR value of ENV_TAIL_ADJ."]
    #[inline]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USB_REG_ENV_TAIL_ADJ_VDR { bits }
    }
    #[doc = "Bits 20:23 - IFR value of TX_D_CAL."]
    #[inline]
    pub fn trim_usbphy_tx_d_cal(&self) -> TRIM_USBPHY_TX_D_CALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_D_CALR { bits }
    }
    #[doc = "Bits 24:27 - IFR value of TX_CAL45DP."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TRIM_USBPHY_TX_CAL45DPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_CAL45DPR { bits }
    }
    #[doc = "Bits 28:31 - IFR value of TX_CAL45DM."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TRIM_USBPHY_TX_CAL45DMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_CAL45DMR { bits }
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
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in USBPHY_PLL_SIC\\[1:0\\] will be used."]
    #[inline]
    pub fn trim_div_sel_override(&mut self) -> _TRIM_DIV_SEL_OVERRIDEW {
        _TRIM_DIV_SEL_OVERRIDEW { w: self }
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in USBPHY_DEBUG1\\[14:13\\] will be used"]
    #[inline]
    pub fn trim_env_tail_adj_vd_override(&mut self) -> _TRIM_ENV_TAIL_ADJ_VD_OVERRIDEW {
        _TRIM_ENV_TAIL_ADJ_VD_OVERRIDEW { w: self }
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in USBPHY_TX\\[3:0\\] will be used."]
    #[inline]
    pub fn trim_tx_d_cal_override(&mut self) -> _TRIM_TX_D_CAL_OVERRIDEW {
        _TRIM_TX_D_CAL_OVERRIDEW { w: self }
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in USBPHY_TX\\[19:16\\] will be used."]
    #[inline]
    pub fn trim_tx_cal45dp_override(&mut self) -> _TRIM_TX_CAL45DP_OVERRIDEW {
        _TRIM_TX_CAL45DP_OVERRIDEW { w: self }
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in USBPHY_TX\\[11:8\\] will be used."]
    #[inline]
    pub fn trim_tx_cal45dm_override(&mut self) -> _TRIM_TX_CAL45DM_OVERRIDEW {
        _TRIM_TX_CAL45DM_OVERRIDEW { w: self }
    }
}
