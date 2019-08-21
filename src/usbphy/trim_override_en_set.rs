#[doc = "Reader of register TRIM_OVERRIDE_EN_SET"]
pub type R = crate::R<u32, super::TRIM_OVERRIDE_EN_SET>;
#[doc = "Writer for register TRIM_OVERRIDE_EN_SET"]
pub type W = crate::W<u32, super::TRIM_OVERRIDE_EN_SET>;
#[doc = "Register TRIM_OVERRIDE_EN_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM_OVERRIDE_EN_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIM_DIV_SEL_OVERRIDE`"]
pub type TRIM_DIV_SEL_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_DIV_SEL_OVERRIDE`"]
pub struct TRIM_DIV_SEL_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_DIV_SEL_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `TRIM_ENV_TAIL_ADJ_VD_OVERRIDE`"]
pub type TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_ENV_TAIL_ADJ_VD_OVERRIDE`"]
pub struct TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `TRIM_TX_D_CAL_OVERRIDE`"]
pub type TRIM_TX_D_CAL_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_TX_D_CAL_OVERRIDE`"]
pub struct TRIM_TX_D_CAL_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_TX_D_CAL_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TRIM_TX_CAL45DP_OVERRIDE`"]
pub type TRIM_TX_CAL45DP_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_TX_CAL45DP_OVERRIDE`"]
pub struct TRIM_TX_CAL45DP_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_TX_CAL45DP_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TRIM_TX_CAL45DM_OVERRIDE`"]
pub type TRIM_TX_CAL45DM_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIM_TX_CAL45DM_OVERRIDE`"]
pub struct TRIM_TX_CAL45DM_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_TX_CAL45DM_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `TRIM_PLL_CTRL0_DIV_SEL`"]
pub type TRIM_PLL_CTRL0_DIV_SEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_USB_REG_ENV_TAIL_ADJ_VD`"]
pub type TRIM_USB_REG_ENV_TAIL_ADJ_VD_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_USBPHY_TX_D_CAL`"]
pub type TRIM_USBPHY_TX_D_CAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_USBPHY_TX_CAL45DP`"]
pub type TRIM_USBPHY_TX_CAL45DP_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_USBPHY_TX_CAL45DM`"]
pub type TRIM_USBPHY_TX_CAL45DM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in USBPHY_PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub fn trim_div_sel_override(&self) -> TRIM_DIV_SEL_OVERRIDE_R {
        TRIM_DIV_SEL_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in USBPHY_DEBUG1\\[14:13\\] will be used"]
    #[inline(always)]
    pub fn trim_env_tail_adj_vd_override(&self) -> TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_R {
        TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in USBPHY_TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_d_cal_override(&self) -> TRIM_TX_D_CAL_OVERRIDE_R {
        TRIM_TX_D_CAL_OVERRIDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in USBPHY_TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dp_override(&self) -> TRIM_TX_CAL45DP_OVERRIDE_R {
        TRIM_TX_CAL45DP_OVERRIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in USBPHY_TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dm_override(&self) -> TRIM_TX_CAL45DM_OVERRIDE_R {
        TRIM_TX_CAL45DM_OVERRIDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TRIM_PLL_CTRL0_DIV_SEL_R {
        TRIM_PLL_CTRL0_DIV_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VD_R {
        TRIM_USB_REG_ENV_TAIL_ADJ_VD_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:23 - IFR value of TX_D_CAL."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TRIM_USBPHY_TX_D_CAL_R {
        TRIM_USBPHY_TX_D_CAL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TRIM_USBPHY_TX_CAL45DP_R {
        TRIM_USBPHY_TX_CAL45DP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TRIM_USBPHY_TX_CAL45DM_R {
        TRIM_USBPHY_TX_CAL45DM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in USBPHY_PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub fn trim_div_sel_override(&mut self) -> TRIM_DIV_SEL_OVERRIDE_W {
        TRIM_DIV_SEL_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in USBPHY_DEBUG1\\[14:13\\] will be used"]
    #[inline(always)]
    pub fn trim_env_tail_adj_vd_override(&mut self) -> TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_W {
        TRIM_ENV_TAIL_ADJ_VD_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in USBPHY_TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_d_cal_override(&mut self) -> TRIM_TX_D_CAL_OVERRIDE_W {
        TRIM_TX_D_CAL_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in USBPHY_TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dp_override(&mut self) -> TRIM_TX_CAL45DP_OVERRIDE_W {
        TRIM_TX_CAL45DP_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in USBPHY_TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dm_override(&mut self) -> TRIM_TX_CAL45DM_OVERRIDE_W {
        TRIM_TX_CAL45DM_OVERRIDE_W { w: self }
    }
}
