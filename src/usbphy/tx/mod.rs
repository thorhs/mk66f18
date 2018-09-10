#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX {
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
#[doc = "Possible values of the field `D_CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_CALR {
    #[doc = "Maximum current, approximately 19% above nominal."]
    _0000,
    #[doc = "Nominal"]
    _0111,
    #[doc = "Minimum current, approximately 19% below nominal."]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl D_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            D_CALR::_0000 => 0,
            D_CALR::_0111 => 7,
            D_CALR::_1111 => 15,
            D_CALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> D_CALR {
        match value {
            0 => D_CALR::_0000,
            7 => D_CALR::_0111,
            15 => D_CALR::_1111,
            i => D_CALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == D_CALR::_0000
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == D_CALR::_0111
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == D_CALR::_1111
    }
}
#[doc = r" Value of the field"]
pub struct TXCAL45DMR {
    bits: u8,
}
impl TXCAL45DMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCAL45DPR {
    bits: u8,
}
impl TXCAL45DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBPHY_TX_EDGECTRLR {
    bits: u8,
}
impl USBPHY_TX_EDGECTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `D_CAL`"]
pub enum D_CALW {
    #[doc = "Maximum current, approximately 19% above nominal."]
    _0000,
    #[doc = "Nominal"]
    _0111,
    #[doc = "Minimum current, approximately 19% below nominal."]
    _1111,
}
impl D_CALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            D_CALW::_0000 => 0,
            D_CALW::_0111 => 7,
            D_CALW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _D_CALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D_CALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(D_CALW::_0000)
    }
    #[doc = "Nominal"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(D_CALW::_0111)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(D_CALW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBPHY_TX_EDGECTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPHY_TX_EDGECTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline]
    pub fn d_cal(&self) -> D_CALR {
        D_CALR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline]
    pub fn txcal45dm(&self) -> TXCAL45DMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DMR { bits }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&self) -> TXCAL45DPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DPR { bits }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline]
    pub fn usbphy_tx_edgectrl(&self) -> USBPHY_TX_EDGECTRLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBPHY_TX_EDGECTRLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268830215 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline]
    pub fn d_cal(&mut self) -> _D_CALW {
        _D_CALW { w: self }
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline]
    pub fn txcal45dm(&mut self) -> _TXCAL45DMW {
        _TXCAL45DMW { w: self }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&mut self) -> _TXCAL45DPW {
        _TXCAL45DPW { w: self }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline]
    pub fn usbphy_tx_edgectrl(&mut self) -> _USBPHY_TX_EDGECTRLW {
        _USBPHY_TX_EDGECTRLW { w: self }
    }
}
