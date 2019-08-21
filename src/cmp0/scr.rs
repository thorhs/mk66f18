#[doc = "Reader of register SCR"]
pub type R = crate::R<u8, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u8, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUT`"]
pub type COUT_R = crate::R<bool, bool>;
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFF_A {
    #[doc = "0: Falling-edge on COUT has not been detected."]
    _0,
    #[doc = "1: Falling-edge on COUT has occurred."]
    _1,
}
impl From<CFF_A> for bool {
    #[inline(always)]
    fn from(variant: CFF_A) -> Self {
        match variant {
            CFF_A::_0 => false,
            CFF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CFF`"]
pub type CFF_R = crate::R<bool, CFF_A>;
impl CFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFF_A {
        match self.bits {
            false => CFF_A::_0,
            true => CFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFF_A::_1
    }
}
#[doc = "Write proxy for field `CFF`"]
pub struct CFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFF_A::_0)
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFF_A::_1)
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
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR_A {
    #[doc = "0: Rising-edge on COUT has not been detected."]
    _0,
    #[doc = "1: Rising-edge on COUT has occurred."]
    _1,
}
impl From<CFR_A> for bool {
    #[inline(always)]
    fn from(variant: CFR_A) -> Self {
        match variant {
            CFR_A::_0 => false,
            CFR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CFR`"]
pub type CFR_R = crate::R<bool, CFR_A>;
impl CFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFR_A {
        match self.bits {
            false => CFR_A::_0,
            true => CFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFR_A::_1
    }
}
#[doc = "Write proxy for field `CFR`"]
pub struct CFR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR_A::_0)
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR_A::_1)
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
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEF_A {
    #[doc = "0: Interrupt is disabled."]
    _0,
    #[doc = "1: Interrupt is enabled."]
    _1,
}
impl From<IEF_A> for bool {
    #[inline(always)]
    fn from(variant: IEF_A) -> Self {
        match variant {
            IEF_A::_0 => false,
            IEF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IEF`"]
pub type IEF_R = crate::R<bool, IEF_A>;
impl IEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEF_A {
        match self.bits {
            false => IEF_A::_0,
            true => IEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEF_A::_1
    }
}
#[doc = "Write proxy for field `IEF`"]
pub struct IEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEF_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEF_A::_1)
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
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IER_A {
    #[doc = "0: Interrupt is disabled."]
    _0,
    #[doc = "1: Interrupt is enabled."]
    _1,
}
impl From<IER_A> for bool {
    #[inline(always)]
    fn from(variant: IER_A) -> Self {
        match variant {
            IER_A::_0 => false,
            IER_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IER`"]
pub type IER_R = crate::R<bool, IER_A>;
impl IER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IER_A {
        match self.bits {
            false => IER_A::_0,
            true => IER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IER_A::_1
    }
}
#[doc = "Write proxy for field `IER`"]
pub struct IER_W<'a> {
    w: &'a mut W,
}
impl<'a> IER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IER_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IER_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0,
    #[doc = "1: DMA is enabled."]
    _1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        match variant {
            DMAEN_A::_0 => false,
            DMAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CFF_R {
        CFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CFR_R {
        CFR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IER_R {
        IER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&mut self) -> CFF_W {
        CFF_W { w: self }
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&mut self) -> CFR_W {
        CFR_W { w: self }
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&mut self) -> IEF_W {
        IEF_W { w: self }
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&mut self) -> IER_W {
        IER_W { w: self }
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
