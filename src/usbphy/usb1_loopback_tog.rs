#[doc = "Reader of register USB1_LOOPBACK_TOG"]
pub type R = crate::R<u32, super::USB1_LOOPBACK_TOG>;
#[doc = "Writer for register USB1_LOOPBACK_TOG"]
pub type W = crate::W<u32, super::USB1_LOOPBACK_TOG>;
#[doc = "Register USB1_LOOPBACK_TOG `reset()`'s with value 0x0055_0000"]
impl crate::ResetValue for super::USB1_LOOPBACK_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0055_0000
    }
}
#[doc = "Reader of field `UTMI_TESTSTART`"]
pub type UTMI_TESTSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTMI_TESTSTART`"]
pub struct UTMI_TESTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_TESTSTART_W<'a> {
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
#[doc = "Reader of field `UTMI_DIG_TST0`"]
pub type UTMI_DIG_TST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTMI_DIG_TST0`"]
pub struct UTMI_DIG_TST0_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_DIG_TST0_W<'a> {
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
#[doc = "Reader of field `UTMI_DIG_TST1`"]
pub type UTMI_DIG_TST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTMI_DIG_TST1`"]
pub struct UTMI_DIG_TST1_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_DIG_TST1_W<'a> {
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
#[doc = "Reader of field `TSTI_TX_HS_MODE`"]
pub type TSTI_TX_HS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTI_TX_HS_MODE`"]
pub struct TSTI_TX_HS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_TX_HS_MODE_W<'a> {
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
#[doc = "Reader of field `TSTI_TX_LS_MODE`"]
pub type TSTI_TX_LS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTI_TX_LS_MODE`"]
pub struct TSTI_TX_LS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_TX_LS_MODE_W<'a> {
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
#[doc = "Reader of field `TSTI_TX_EN`"]
pub type TSTI_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTI_TX_EN`"]
pub struct TSTI_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_TX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSTI_TX_HIZ`"]
pub type TSTI_TX_HIZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTI_TX_HIZ`"]
pub struct TSTI_TX_HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_TX_HIZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UTMO_DIG_TST0`"]
pub type UTMO_DIG_TST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `UTMO_DIG_TST1`"]
pub type UTMO_DIG_TST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTI_HSFS_MODE_EN`"]
pub type TSTI_HSFS_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTI_HSFS_MODE_EN`"]
pub struct TSTI_HSFS_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_HSFS_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TSTPKT`"]
pub type TSTPKT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSTPKT`"]
pub struct TSTPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTPKT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline(always)]
    pub fn utmi_teststart(&self) -> UTMI_TESTSTART_R {
        UTMI_TESTSTART_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst0(&self) -> UTMI_DIG_TST0_R {
        UTMI_DIG_TST0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst1(&self) -> UTMI_DIG_TST1_R {
        UTMI_DIG_TST1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub fn tsti_tx_hs_mode(&self) -> TSTI_TX_HS_MODE_R {
        TSTI_TX_HS_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub fn tsti_tx_ls_mode(&self) -> TSTI_TX_LS_MODE_R {
        TSTI_TX_LS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_en(&self) -> TSTI_TX_EN_R {
        TSTI_TX_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_hiz(&self) -> TSTI_TX_HIZ_R {
        TSTI_TX_HIZ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub fn utmo_dig_tst0(&self) -> UTMO_DIG_TST0_R {
        UTMO_DIG_TST0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub fn utmo_dig_tst1(&self) -> UTMO_DIG_TST1_R {
        UTMO_DIG_TST1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub fn tsti_hsfs_mode_en(&self) -> TSTI_HSFS_MODE_EN_R {
        TSTI_HSFS_MODE_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub fn tstpkt(&self) -> TSTPKT_R {
        TSTPKT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline(always)]
    pub fn utmi_teststart(&mut self) -> UTMI_TESTSTART_W {
        UTMI_TESTSTART_W { w: self }
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst0(&mut self) -> UTMI_DIG_TST0_W {
        UTMI_DIG_TST0_W { w: self }
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline(always)]
    pub fn utmi_dig_tst1(&mut self) -> UTMI_DIG_TST1_W {
        UTMI_DIG_TST1_W { w: self }
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub fn tsti_tx_hs_mode(&mut self) -> TSTI_TX_HS_MODE_W {
        TSTI_TX_HS_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub fn tsti_tx_ls_mode(&mut self) -> TSTI_TX_LS_MODE_W {
        TSTI_TX_LS_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_en(&mut self) -> TSTI_TX_EN_W {
        TSTI_TX_EN_W { w: self }
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_hiz(&mut self) -> TSTI_TX_HIZ_W {
        TSTI_TX_HIZ_W { w: self }
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub fn tsti_hsfs_mode_en(&mut self) -> TSTI_HSFS_MODE_EN_W {
        TSTI_HSFS_MODE_EN_W { w: self }
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub fn tstpkt(&mut self) -> TSTPKT_W {
        TSTPKT_W { w: self }
    }
}
