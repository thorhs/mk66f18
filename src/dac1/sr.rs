#[doc = "Reader of register SR"]
pub type R = crate::R<u8, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u8, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "DAC Buffer Read Pointer Bottom Position Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPBF_A {
    #[doc = "0: The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    _0,
    #[doc = "1: The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    _1,
}
impl From<DACBFRPBF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPBF_A) -> Self {
        match variant {
            DACBFRPBF_A::_0 => false,
            DACBFRPBF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBFRPBF`"]
pub type DACBFRPBF_R = crate::R<bool, DACBFRPBF_A>;
impl DACBFRPBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPBF_A {
        match self.bits {
            false => DACBFRPBF_A::_0,
            true => DACBFRPBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPBF_A::_1
    }
}
#[doc = "Write proxy for field `DACBFRPBF`"]
pub struct DACBFRPBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRPBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFRPBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_1)
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
#[doc = "DAC Buffer Read Pointer Top Position Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPTF_A {
    #[doc = "0: The DAC buffer read pointer is not zero."]
    _0,
    #[doc = "1: The DAC buffer read pointer is zero."]
    _1,
}
impl From<DACBFRPTF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPTF_A) -> Self {
        match variant {
            DACBFRPTF_A::_0 => false,
            DACBFRPTF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBFRPTF`"]
pub type DACBFRPTF_R = crate::R<bool, DACBFRPTF_A>;
impl DACBFRPTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPTF_A {
        match self.bits {
            false => DACBFRPTF_A::_0,
            true => DACBFRPTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPTF_A::_1
    }
}
#[doc = "Write proxy for field `DACBFRPTF`"]
pub struct DACBFRPTF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRPTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFRPTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "DAC Buffer Watermark Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFWMF_A {
    #[doc = "0: The DAC buffer read pointer has not reached the watermark level."]
    _0,
    #[doc = "1: The DAC buffer read pointer has reached the watermark level."]
    _1,
}
impl From<DACBFWMF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFWMF_A) -> Self {
        match variant {
            DACBFWMF_A::_0 => false,
            DACBFWMF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBFWMF`"]
pub type DACBFWMF_R = crate::R<bool, DACBFWMF_A>;
impl DACBFWMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFWMF_A {
        match self.bits {
            false => DACBFWMF_A::_0,
            true => DACBFWMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFWMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFWMF_A::_1
    }
}
#[doc = "Write proxy for field `DACBFWMF`"]
pub struct DACBFWMF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFWMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFWMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DAC buffer read pointer has not reached the watermark level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFWMF_A::_0)
    }
    #[doc = "The DAC buffer read pointer has reached the watermark level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFWMF_A::_1)
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
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&self) -> DACBFRPBF_R {
        DACBFRPBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&self) -> DACBFRPTF_R {
        DACBFRPTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline(always)]
    pub fn dacbfwmf(&self) -> DACBFWMF_R {
        DACBFWMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&mut self) -> DACBFRPBF_W {
        DACBFRPBF_W { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&mut self) -> DACBFRPTF_W {
        DACBFRPTF_W { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline(always)]
    pub fn dacbfwmf(&mut self) -> DACBFWMF_W {
        DACBFWMF_W { w: self }
    }
}
