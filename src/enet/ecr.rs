#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0xf000_0000"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf000_0000
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Ethernet Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHEREN_A {
    #[doc = "0: Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    _0,
    #[doc = "1: MAC is enabled, and reception and transmission are possible."]
    _1,
}
impl From<ETHEREN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHEREN_A) -> Self {
        match variant {
            ETHEREN_A::_0 => false,
            ETHEREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ETHEREN`"]
pub type ETHEREN_R = crate::R<bool, ETHEREN_A>;
impl ETHEREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHEREN_A {
        match self.bits {
            false => ETHEREN_A::_0,
            true => ETHEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETHEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETHEREN_A::_1
    }
}
#[doc = "Write proxy for field `ETHEREN`"]
pub struct ETHEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHEREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETHEREN_A::_0)
    }
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETHEREN_A::_1)
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
#[doc = "Magic Packet Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAGICEN_A {
    #[doc = "0: Magic detection logic disabled."]
    _0,
    #[doc = "1: The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    _1,
}
impl From<MAGICEN_A> for bool {
    #[inline(always)]
    fn from(variant: MAGICEN_A) -> Self {
        match variant {
            MAGICEN_A::_0 => false,
            MAGICEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MAGICEN`"]
pub type MAGICEN_R = crate::R<bool, MAGICEN_A>;
impl MAGICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAGICEN_A {
        match self.bits {
            false => MAGICEN_A::_0,
            true => MAGICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAGICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAGICEN_A::_1
    }
}
#[doc = "Write proxy for field `MAGICEN`"]
pub struct MAGICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAGICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAGICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Magic detection logic disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAGICEN_A::_0)
    }
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAGICEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Sleep Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal operating mode."]
    _0,
    #[doc = "1: Sleep mode."]
    _1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        match variant {
            SLEEP_A::_0 => false,
            SLEEP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::_0,
            true => SLEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEEP_A::_1
    }
}
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operating mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEP_A::_0)
    }
    #[doc = "Sleep mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "EN1588 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1588_A {
    #[doc = "0: Legacy FEC buffer descriptors and functions enabled."]
    _0,
    #[doc = "1: Enhanced frame time-stamping functions enabled."]
    _1,
}
impl From<EN1588_A> for bool {
    #[inline(always)]
    fn from(variant: EN1588_A) -> Self {
        match variant {
            EN1588_A::_0 => false,
            EN1588_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EN1588`"]
pub type EN1588_R = crate::R<bool, EN1588_A>;
impl EN1588_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1588_A {
        match self.bits {
            false => EN1588_A::_0,
            true => EN1588_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1588_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1588_A::_1
    }
}
#[doc = "Write proxy for field `EN1588`"]
pub struct EN1588_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1588_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1588_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1588_A::_0)
    }
    #[doc = "Enhanced frame time-stamping functions enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1588_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: MAC continues operation in debug mode."]
    _0,
    #[doc = "1: MAC enters hardware freeze mode when the processor is in debug mode."]
    _1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        match variant {
            DBGEN_A::_0 => false,
            DBGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, DBGEN_A>;
impl DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MAC continues operation in debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `STOPEN`"]
pub type STOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPEN`"]
pub struct STOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPEN_W<'a> {
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
#[doc = "Descriptor Byte Swapping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBSWP_A {
    #[doc = "0: The buffer descriptor bytes are not swapped to support big-endian devices."]
    _0,
    #[doc = "1: The buffer descriptor bytes are swapped to support little-endian devices."]
    _1,
}
impl From<DBSWP_A> for bool {
    #[inline(always)]
    fn from(variant: DBSWP_A) -> Self {
        match variant {
            DBSWP_A::_0 => false,
            DBSWP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DBSWP`"]
pub type DBSWP_R = crate::R<bool, DBSWP_A>;
impl DBSWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBSWP_A {
        match self.bits {
            false => DBSWP_A::_0,
            true => DBSWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBSWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBSWP_A::_1
    }
}
#[doc = "Write proxy for field `DBSWP`"]
pub struct DBSWP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBSWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBSWP_A::_0)
    }
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBSWP_A::_1)
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
impl R {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&self) -> ETHEREN_R {
        ETHEREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&self) -> MAGICEN_R {
        MAGICEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&self) -> EN1588_R {
        EN1588_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - STOPEN Signal Control"]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&self) -> DBSWP_R {
        DBSWP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&mut self) -> ETHEREN_W {
        ETHEREN_W { w: self }
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&mut self) -> MAGICEN_W {
        MAGICEN_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&mut self) -> EN1588_W {
        EN1588_W { w: self }
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 7 - STOPEN Signal Control"]
    #[inline(always)]
    pub fn stopen(&mut self) -> STOPEN_W {
        STOPEN_W { w: self }
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&mut self) -> DBSWP_W {
        DBSWP_W { w: self }
    }
}
