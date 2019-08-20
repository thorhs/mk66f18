#[doc = "Writer for register FEVT"]
pub type W = crate::W<u32, super::FEVT>;
#[doc = "Register FEVT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEVT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AC12NE`"]
pub struct AC12NE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12NE_W<'a> {
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
#[doc = "Write proxy for field `AC12TOE`"]
pub struct AC12TOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12TOE_W<'a> {
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
#[doc = "Write proxy for field `AC12CE`"]
pub struct AC12CE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12CE_W<'a> {
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
#[doc = "Write proxy for field `AC12EBE`"]
pub struct AC12EBE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12EBE_W<'a> {
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
#[doc = "Write proxy for field `AC12IE`"]
pub struct AC12IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12IE_W<'a> {
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
#[doc = "Write proxy for field `CNIBAC12E`"]
pub struct CNIBAC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIBAC12E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `CTOE`"]
pub struct CTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `CEBE`"]
pub struct CEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEBE_W<'a> {
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
#[doc = "Write proxy for field `CIE`"]
pub struct CIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIE_W<'a> {
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
#[doc = "Write proxy for field `DTOE`"]
pub struct DTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `DCE`"]
pub struct DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `DEBE`"]
pub struct DEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `AC12E`"]
pub struct AC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `DMAE`"]
pub struct DMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `CINT`"]
pub struct CINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub fn ac12ne(&mut self) -> AC12NE_W {
        AC12NE_W { w: self }
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub fn ac12toe(&mut self) -> AC12TOE_W {
        AC12TOE_W { w: self }
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub fn ac12ce(&mut self) -> AC12CE_W {
        AC12CE_W { w: self }
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&mut self) -> AC12EBE_W {
        AC12EBE_W { w: self }
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub fn ac12ie(&mut self) -> AC12IE_W {
        AC12IE_W { w: self }
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&mut self) -> CNIBAC12E_W {
        CNIBAC12E_W { w: self }
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    pub fn ctoe(&mut self) -> CTOE_W {
        CTOE_W { w: self }
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&mut self) -> CEBE_W {
        CEBE_W { w: self }
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W {
        CIE_W { w: self }
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    pub fn dtoe(&mut self) -> DTOE_W {
        DTOE_W { w: self }
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W {
        DCE_W { w: self }
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&mut self) -> DEBE_W {
        DEBE_W { w: self }
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> AC12E_W {
        AC12E_W { w: self }
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W {
        DMAE_W { w: self }
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W { w: self }
    }
}
