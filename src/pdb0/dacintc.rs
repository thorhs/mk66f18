#[doc = "Reader of register DACINTC%s"]
pub type R = crate::R<u32, super::DACINTC>;
#[doc = "Writer for register DACINTC%s"]
pub type W = crate::W<u32, super::DACINTC>;
#[doc = "Register DACINTC%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DACINTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC Interval Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOE_A {
    #[doc = "0: DAC interval trigger disabled."]
    _0,
    #[doc = "1: DAC interval trigger enabled."]
    _1,
}
impl From<TOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOE_A) -> Self {
        match variant {
            TOE_A::_0 => false,
            TOE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOE`"]
pub type TOE_R = crate::R<bool, TOE_A>;
impl TOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOE_A {
        match self.bits {
            false => TOE_A::_0,
            true => TOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOE_A::_1
    }
}
#[doc = "Write proxy for field `TOE`"]
pub struct TOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC interval trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOE_A::_0)
    }
    #[doc = "DAC interval trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DAC External Trigger Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_A {
    #[doc = "0: DAC external trigger input disabled. DAC interval counter is reset and counting starts when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "1: DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    _1,
}
impl From<EXT_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_A) -> Self {
        match variant {
            EXT_A::_0 => false,
            EXT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, EXT_A>;
impl EXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_A {
        match self.bits {
            false => EXT_A::_0,
            true => EXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXT_A::_1
    }
}
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC external trigger input disabled. DAC interval counter is reset and counting starts when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXT_A::_0)
    }
    #[doc = "DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXT_A::_1)
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
impl R {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    pub fn toe(&self) -> TOE_R {
        TOE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    pub fn toe(&mut self) -> TOE_W {
        TOE_W { w: self }
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
}
