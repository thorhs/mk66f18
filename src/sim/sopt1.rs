#[doc = "Reader of register SOPT1"]
pub type R = crate::R<u32, super::SOPT1>;
#[doc = "Writer for register SOPT1"]
pub type W = crate::W<u32, super::SOPT1>;
#[doc = "Register SOPT1 `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::SOPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMSIZE_A {
    #[doc = "1: 8 KB"]
    _0001,
    #[doc = "3: 16 KB"]
    _0011,
    #[doc = "4: 24 KB"]
    _0100,
    #[doc = "5: 32 KB"]
    _0101,
    #[doc = "6: 48 KB"]
    _0110,
    #[doc = "7: 64 KB"]
    _0111,
    #[doc = "8: 96 KB"]
    _1000,
    #[doc = "9: 128 KB"]
    _1001,
    #[doc = "11: 256 KB"]
    _1011,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        match variant {
            RAMSIZE_A::_0001 => 1,
            RAMSIZE_A::_0011 => 3,
            RAMSIZE_A::_0100 => 4,
            RAMSIZE_A::_0101 => 5,
            RAMSIZE_A::_0110 => 6,
            RAMSIZE_A::_0111 => 7,
            RAMSIZE_A::_1000 => 8,
            RAMSIZE_A::_1001 => 9,
            RAMSIZE_A::_1011 => 11,
        }
    }
}
#[doc = "Reader of field `RAMSIZE`"]
pub type RAMSIZE_R = crate::R<u8, RAMSIZE_A>;
impl RAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RAMSIZE_A::_0001),
            3 => Val(RAMSIZE_A::_0011),
            4 => Val(RAMSIZE_A::_0100),
            5 => Val(RAMSIZE_A::_0101),
            6 => Val(RAMSIZE_A::_0110),
            7 => Val(RAMSIZE_A::_0111),
            8 => Val(RAMSIZE_A::_1000),
            9 => Val(RAMSIZE_A::_1001),
            11 => Val(RAMSIZE_A::_1011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZE_A::_1011
    }
}
#[doc = "32K oscillator clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32KSEL_A {
    #[doc = "0: System oscillator (OSC32KCLK)"]
    _00,
    #[doc = "2: RTC 32.768kHz oscillator"]
    _10,
    #[doc = "3: LPO 1 kHz"]
    _11,
}
impl From<OSC32KSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KSEL_A) -> Self {
        match variant {
            OSC32KSEL_A::_00 => 0,
            OSC32KSEL_A::_10 => 2,
            OSC32KSEL_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `OSC32KSEL`"]
pub type OSC32KSEL_R = crate::R<u8, OSC32KSEL_A>;
impl OSC32KSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSC32KSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSC32KSEL_A::_00),
            2 => Val(OSC32KSEL_A::_10),
            3 => Val(OSC32KSEL_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSEL_A::_11
    }
}
#[doc = "Write proxy for field `OSC32KSEL`"]
pub struct OSC32KSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC32KSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_00)
    }
    #[doc = "RTC 32.768kHz oscillator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_10)
    }
    #[doc = "LPO 1 kHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "USB voltage regulator in standby mode during VLPR and VLPW modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVSTBY_A {
    #[doc = "0: USB voltage regulator not in standby during VLPR and VLPW modes."]
    _0,
    #[doc = "1: USB voltage regulator in standby during VLPR and VLPW modes."]
    _1,
}
impl From<USBVSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: USBVSTBY_A) -> Self {
        match variant {
            USBVSTBY_A::_0 => false,
            USBVSTBY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBVSTBY`"]
pub type USBVSTBY_R = crate::R<bool, USBVSTBY_A>;
impl USBVSTBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBVSTBY_A {
        match self.bits {
            false => USBVSTBY_A::_0,
            true => USBVSTBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBVSTBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBVSTBY_A::_1
    }
}
#[doc = "Write proxy for field `USBVSTBY`"]
pub struct USBVSTBY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBVSTBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBVSTBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVSTBY_A::_0)
    }
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVSTBY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSSTBY_A {
    #[doc = "0: USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    _0,
    #[doc = "1: USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    _1,
}
impl From<USBSSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: USBSSTBY_A) -> Self {
        match variant {
            USBSSTBY_A::_0 => false,
            USBSSTBY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBSSTBY`"]
pub type USBSSTBY_R = crate::R<bool, USBSSTBY_A>;
impl USBSSTBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSSTBY_A {
        match self.bits {
            false => USBSSTBY_A::_0,
            true => USBSSTBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSSTBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSSTBY_A::_1
    }
}
#[doc = "Write proxy for field `USBSSTBY`"]
pub struct USBSSTBY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSSTBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSSTBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSSTBY_A::_0)
    }
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSSTBY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "USB voltage regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREGEN_A {
    #[doc = "0: USB voltage regulator is disabled."]
    _0,
    #[doc = "1: USB voltage regulator is enabled."]
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
    #[doc = "USB voltage regulator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBREGEN_A::_0)
    }
    #[doc = "USB voltage regulator is enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> OSC32KSEL_R {
        OSC32KSEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    pub fn usbvstby(&self) -> USBVSTBY_R {
        USBVSTBY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn usbsstby(&self) -> USBSSTBY_R {
        USBSSTBY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&mut self) -> OSC32KSEL_W {
        OSC32KSEL_W { w: self }
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    pub fn usbvstby(&mut self) -> USBVSTBY_W {
        USBVSTBY_W { w: self }
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn usbsstby(&mut self) -> USBSSTBY_W {
        USBSSTBY_W { w: self }
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    pub fn usbregen(&mut self) -> USBREGEN_W {
        USBREGEN_W { w: self }
    }
}
