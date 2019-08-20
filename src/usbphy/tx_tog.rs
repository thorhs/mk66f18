#[doc = "Reader of register TX_TOG"]
pub type R = crate::R<u32, super::TX_TOG>;
#[doc = "Writer for register TX_TOG"]
pub type W = crate::W<u32, super::TX_TOG>;
#[doc = "Register TX_TOG `reset()`'s with value 0x1006_0607"]
impl crate::ResetValue for super::TX_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1006_0607
    }
}
#[doc = "Decode to trim the nominal 17\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_CAL_A {
    #[doc = "0: Maximum current, approximately 19% above nominal."]
    _0000,
    #[doc = "7: Nominal"]
    _0111,
    #[doc = "15: Minimum current, approximately 19% below nominal."]
    _1111,
}
impl From<D_CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: D_CAL_A) -> Self {
        match variant {
            D_CAL_A::_0000 => 0,
            D_CAL_A::_0111 => 7,
            D_CAL_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `D_CAL`"]
pub type D_CAL_R = crate::R<u8, D_CAL_A>;
impl D_CAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, D_CAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(D_CAL_A::_0000),
            7 => Val(D_CAL_A::_0111),
            15 => Val(D_CAL_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == D_CAL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == D_CAL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == D_CAL_A::_1111
    }
}
#[doc = "Write proxy for field `D_CAL`"]
pub struct D_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D_CAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(D_CAL_A::_0000)
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(D_CAL_A::_0111)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(D_CAL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DM`"]
pub type TXCAL45DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DM`"]
pub struct TXCAL45DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DP`"]
pub type TXCAL45DP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DP`"]
pub struct TXCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `USBPHY_TX_EDGECTRL`"]
pub type USBPHY_TX_EDGECTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBPHY_TX_EDGECTRL`"]
pub struct USBPHY_TX_EDGECTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHY_TX_EDGECTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&self) -> TXCAL45DM_R {
        TXCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub fn usbphy_tx_edgectrl(&self) -> USBPHY_TX_EDGECTRL_R {
        USBPHY_TX_EDGECTRL_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&mut self) -> D_CAL_W {
        D_CAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&mut self) -> TXCAL45DM_W {
        TXCAL45DM_W { w: self }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W {
        TXCAL45DP_W { w: self }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub fn usbphy_tx_edgectrl(&mut self) -> USBPHY_TX_EDGECTRL_W {
        USBPHY_TX_EDGECTRL_W { w: self }
    }
}
