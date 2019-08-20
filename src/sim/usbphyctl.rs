#[doc = "Reader of register USBPHYCTL"]
pub type R = crate::R<u32, super::USBPHYCTL>;
#[doc = "Writer for register USBPHYCTL"]
pub type W = crate::W<u32, super::USBPHYCTL>;
#[doc = "Register USBPHYCTL `reset()`'s with value 0x0060_0000"]
impl crate::ResetValue for super::USBPHYCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0060_0000
    }
}
#[doc = "Selects the default input voltage source to the USB Regulator in case both VREG_IN0 and VREG_IN1 are powered\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVREGSEL_A {
    #[doc = "0: VREG_IN0 will be selected if both regulator inputs are powered"]
    _0,
    #[doc = "1: VREG_IN1 will be selected if both regulator inputs are powered"]
    _1,
}
impl From<USBVREGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBVREGSEL_A) -> Self {
        match variant {
            USBVREGSEL_A::_0 => false,
            USBVREGSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBVREGSEL`"]
pub type USBVREGSEL_R = crate::R<bool, USBVREGSEL_A>;
impl USBVREGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBVREGSEL_A {
        match self.bits {
            false => USBVREGSEL_A::_0,
            true => USBVREGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBVREGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBVREGSEL_A::_1
    }
}
#[doc = "Write proxy for field `USBVREGSEL`"]
pub struct USBVREGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBVREGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBVREGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VREG_IN0 will be selected if both regulator inputs are powered"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVREGSEL_A::_0)
    }
    #[doc = "VREG_IN1 will be selected if both regulator inputs are powered"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVREGSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enables the pulldown on the output of the USB Regulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVREGPD_A {
    #[doc = "0: Regulator output pulldown resistor is not enabled"]
    _0,
    #[doc = "1: Regulator output pulldown resistor is enabled"]
    _1,
}
impl From<USBVREGPD_A> for bool {
    #[inline(always)]
    fn from(variant: USBVREGPD_A) -> Self {
        match variant {
            USBVREGPD_A::_0 => false,
            USBVREGPD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBVREGPD`"]
pub type USBVREGPD_R = crate::R<bool, USBVREGPD_A>;
impl USBVREGPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBVREGPD_A {
        match self.bits {
            false => USBVREGPD_A::_0,
            true => USBVREGPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBVREGPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBVREGPD_A::_1
    }
}
#[doc = "Write proxy for field `USBVREGPD`"]
pub struct USBVREGPD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBVREGPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBVREGPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regulator output pulldown resistor is not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVREGPD_A::_0)
    }
    #[doc = "Regulator output pulldown resistor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVREGPD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "USB 3.3V Output Target\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB3VOUTTRG_A {
    #[doc = "0: 2.733V"]
    _000,
    #[doc = "1: 3.020V"]
    _001,
    #[doc = "2: 3.074V"]
    _010,
    #[doc = "3: 3.130V"]
    _011,
    #[doc = "4: 3.188V"]
    _100,
    #[doc = "5: 3.248V"]
    _101,
    #[doc = "6: 3.310V (default)"]
    _110,
    #[doc = "7: 3.662V (For Freescale use only, not for customer use)"]
    _111,
}
impl From<USB3VOUTTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: USB3VOUTTRG_A) -> Self {
        match variant {
            USB3VOUTTRG_A::_000 => 0,
            USB3VOUTTRG_A::_001 => 1,
            USB3VOUTTRG_A::_010 => 2,
            USB3VOUTTRG_A::_011 => 3,
            USB3VOUTTRG_A::_100 => 4,
            USB3VOUTTRG_A::_101 => 5,
            USB3VOUTTRG_A::_110 => 6,
            USB3VOUTTRG_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `USB3VOUTTRG`"]
pub type USB3VOUTTRG_R = crate::R<u8, USB3VOUTTRG_A>;
impl USB3VOUTTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB3VOUTTRG_A {
        match self.bits {
            0 => USB3VOUTTRG_A::_000,
            1 => USB3VOUTTRG_A::_001,
            2 => USB3VOUTTRG_A::_010,
            3 => USB3VOUTTRG_A::_011,
            4 => USB3VOUTTRG_A::_100,
            5 => USB3VOUTTRG_A::_101,
            6 => USB3VOUTTRG_A::_110,
            7 => USB3VOUTTRG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == USB3VOUTTRG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == USB3VOUTTRG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == USB3VOUTTRG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == USB3VOUTTRG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == USB3VOUTTRG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == USB3VOUTTRG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == USB3VOUTTRG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == USB3VOUTTRG_A::_111
    }
}
#[doc = "Write proxy for field `USB3VOUTTRG`"]
pub struct USB3VOUTTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> USB3VOUTTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB3VOUTTRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2.733V"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_000)
    }
    #[doc = "3.020V"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_001)
    }
    #[doc = "3.074V"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_010)
    }
    #[doc = "3.130V"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_011)
    }
    #[doc = "3.188V"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_100)
    }
    #[doc = "3.248V"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_101)
    }
    #[doc = "3.310V (default)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_110)
    }
    #[doc = "3.662V (For Freescale use only, not for customer use)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(USB3VOUTTRG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "USB Disable Inrush Current Limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDISILIM_A {
    #[doc = "0: The current limiter for the USB Voltage Regulator is enabled"]
    _0,
    #[doc = "1: The current limiter for the USB Voltage Regulator is disabled"]
    _1,
}
impl From<USBDISILIM_A> for bool {
    #[inline(always)]
    fn from(variant: USBDISILIM_A) -> Self {
        match variant {
            USBDISILIM_A::_0 => false,
            USBDISILIM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBDISILIM`"]
pub type USBDISILIM_R = crate::R<bool, USBDISILIM_A>;
impl USBDISILIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDISILIM_A {
        match self.bits {
            false => USBDISILIM_A::_0,
            true => USBDISILIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBDISILIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBDISILIM_A::_1
    }
}
#[doc = "Write proxy for field `USBDISILIM`"]
pub struct USBDISILIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDISILIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDISILIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The current limiter for the USB Voltage Regulator is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDISILIM_A::_0)
    }
    #[doc = "The current limiter for the USB Voltage Regulator is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDISILIM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Selects the default input voltage source to the USB Regulator in case both VREG_IN0 and VREG_IN1 are powered"]
    #[inline(always)]
    pub fn usbvregsel(&self) -> USBVREGSEL_R {
        USBVREGSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the pulldown on the output of the USB Regulator."]
    #[inline(always)]
    pub fn usbvregpd(&self) -> USBVREGPD_R {
        USBVREGPD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - USB 3.3V Output Target"]
    #[inline(always)]
    pub fn usb3vouttrg(&self) -> USB3VOUTTRG_R {
        USB3VOUTTRG_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - USB Disable Inrush Current Limit"]
    #[inline(always)]
    pub fn usbdisilim(&self) -> USBDISILIM_R {
        USBDISILIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Selects the default input voltage source to the USB Regulator in case both VREG_IN0 and VREG_IN1 are powered"]
    #[inline(always)]
    pub fn usbvregsel(&mut self) -> USBVREGSEL_W {
        USBVREGSEL_W { w: self }
    }
    #[doc = "Bit 9 - Enables the pulldown on the output of the USB Regulator."]
    #[inline(always)]
    pub fn usbvregpd(&mut self) -> USBVREGPD_W {
        USBVREGPD_W { w: self }
    }
    #[doc = "Bits 20:22 - USB 3.3V Output Target"]
    #[inline(always)]
    pub fn usb3vouttrg(&mut self) -> USB3VOUTTRG_W {
        USB3VOUTTRG_W { w: self }
    }
    #[doc = "Bit 23 - USB Disable Inrush Current Limit"]
    #[inline(always)]
    pub fn usbdisilim(&mut self) -> USBDISILIM_W {
        USBDISILIM_W { w: self }
    }
}
