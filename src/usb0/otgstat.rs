#[doc = "Reader of register OTGSTAT"]
pub type R = crate::R<u8, super::OTGSTAT>;
#[doc = "Writer for register OTGSTAT"]
pub type W = crate::W<u8, super::OTGSTAT>;
#[doc = "Register OTGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGSTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "A VBUS Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVBUSVLD_A {
    #[doc = "0: The VBUS voltage is below the A VBUS Valid threshold."]
    _0,
    #[doc = "1: The VBUS voltage is above the A VBUS Valid threshold."]
    _1,
}
impl From<AVBUSVLD_A> for bool {
    #[inline(always)]
    fn from(variant: AVBUSVLD_A) -> Self {
        match variant {
            AVBUSVLD_A::_0 => false,
            AVBUSVLD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AVBUSVLD`"]
pub type AVBUSVLD_R = crate::R<bool, AVBUSVLD_A>;
impl AVBUSVLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVBUSVLD_A {
        match self.bits {
            false => AVBUSVLD_A::_0,
            true => AVBUSVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVBUSVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVBUSVLD_A::_1
    }
}
#[doc = "Write proxy for field `AVBUSVLD`"]
pub struct AVBUSVLD_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBUSVLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVBUSVLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VBUS voltage is below the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVBUSVLD_A::_0)
    }
    #[doc = "The VBUS voltage is above the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVBUSVLD_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "B Session End\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSESSEND_A {
    #[doc = "0: The VBUS voltage is above the B session end threshold."]
    _0,
    #[doc = "1: The VBUS voltage is below the B session end threshold."]
    _1,
}
impl From<BSESSEND_A> for bool {
    #[inline(always)]
    fn from(variant: BSESSEND_A) -> Self {
        match variant {
            BSESSEND_A::_0 => false,
            BSESSEND_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BSESSEND`"]
pub type BSESSEND_R = crate::R<bool, BSESSEND_A>;
impl BSESSEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSESSEND_A {
        match self.bits {
            false => BSESSEND_A::_0,
            true => BSESSEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSESSEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSESSEND_A::_1
    }
}
#[doc = "Write proxy for field `BSESSEND`"]
pub struct BSESSEND_W<'a> {
    w: &'a mut W,
}
impl<'a> BSESSEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSESSEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VBUS voltage is above the B session end threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSESSEND_A::_0)
    }
    #[doc = "The VBUS voltage is below the B session end threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSESSEND_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESS_VLD_A {
    #[doc = "0: The VBUS voltage is below the B session valid threshold"]
    _0,
    #[doc = "1: The VBUS voltage is above the B session valid threshold."]
    _1,
}
impl From<SESS_VLD_A> for bool {
    #[inline(always)]
    fn from(variant: SESS_VLD_A) -> Self {
        match variant {
            SESS_VLD_A::_0 => false,
            SESS_VLD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SESS_VLD`"]
pub type SESS_VLD_R = crate::R<bool, SESS_VLD_A>;
impl SESS_VLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESS_VLD_A {
        match self.bits {
            false => SESS_VLD_A::_0,
            true => SESS_VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SESS_VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SESS_VLD_A::_1
    }
}
#[doc = "Write proxy for field `SESS_VLD`"]
pub struct SESS_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SESS_VLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SESS_VLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VBUS voltage is below the B session valid threshold"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SESS_VLD_A::_0)
    }
    #[doc = "The VBUS voltage is above the B session valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SESS_VLD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATESTABLE_A {
    #[doc = "0: The LINE_STAT_CHG bit is not yet stable."]
    _0,
    #[doc = "1: The LINE_STAT_CHG bit has been debounced and is stable."]
    _1,
}
impl From<LINESTATESTABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATESTABLE_A) -> Self {
        match variant {
            LINESTATESTABLE_A::_0 => false,
            LINESTATESTABLE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LINESTATESTABLE`"]
pub type LINESTATESTABLE_R = crate::R<bool, LINESTATESTABLE_A>;
impl LINESTATESTABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATESTABLE_A {
        match self.bits {
            false => LINESTATESTABLE_A::_0,
            true => LINESTATESTABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATESTABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATESTABLE_A::_1
    }
}
#[doc = "Write proxy for field `LINESTATESTABLE`"]
pub struct LINESTATESTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATESTABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATESTABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ONEMSECEN`"]
pub type ONEMSECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONEMSECEN`"]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Indicates the current state of the ID pin on the USB connector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID_A {
    #[doc = "0: Indicates a Type A cable is plugged into the USB connector."]
    _0,
    #[doc = "1: Indicates no cable is attached or a Type B cable is plugged into the USB connector."]
    _1,
}
impl From<ID_A> for bool {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        match variant {
            ID_A::_0 => false,
            ID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            false => ID_A::_0,
            true => ID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicates a Type A cable is plugged into the USB connector."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ID_A::_0)
    }
    #[doc = "Indicates no cable is attached or a Type B cable is plugged into the USB connector."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ID_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    pub fn avbusvld(&self) -> AVBUSVLD_R {
        AVBUSVLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - B Session End"]
    #[inline(always)]
    pub fn bsessend(&self) -> BSESSEND_R {
        BSESSEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Session Valid"]
    #[inline(always)]
    pub fn sess_vld(&self) -> SESS_VLD_R {
        SESS_VLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&self) -> LINESTATESTABLE_R {
        LINESTATESTABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    pub fn avbusvld(&mut self) -> AVBUSVLD_W {
        AVBUSVLD_W { w: self }
    }
    #[doc = "Bit 2 - B Session End"]
    #[inline(always)]
    pub fn bsessend(&mut self) -> BSESSEND_W {
        BSESSEND_W { w: self }
    }
    #[doc = "Bit 3 - Session Valid"]
    #[inline(always)]
    pub fn sess_vld(&mut self) -> SESS_VLD_W {
        SESS_VLD_W { w: self }
    }
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&mut self) -> LINESTATESTABLE_W {
        LINESTATESTABLE_W { w: self }
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W {
        ONEMSECEN_W { w: self }
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
}
