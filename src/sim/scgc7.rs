#[doc = "Reader of register SCGC7"]
pub type R = crate::R<u32, super::SCGC7>;
#[doc = "Writer for register SCGC7"]
pub type W = crate::W<u32, super::SCGC7>;
#[doc = "Register SCGC7 `reset()`'s with value 0x06"]
impl crate::ResetValue for super::SCGC7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "FlexBus Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXBUS_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<FLEXBUS_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXBUS_A) -> Self {
        match variant {
            FLEXBUS_A::_0 => false,
            FLEXBUS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXBUS`"]
pub type FLEXBUS_R = crate::R<bool, FLEXBUS_A>;
impl FLEXBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXBUS_A {
        match self.bits {
            false => FLEXBUS_A::_0,
            true => FLEXBUS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLEXBUS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLEXBUS_A::_1
    }
}
#[doc = "Write proxy for field `FLEXBUS`"]
pub struct FLEXBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXBUS_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXBUS_A::_1)
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
#[doc = "DMA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        match variant {
            DMA_A::_0 => false,
            DMA_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
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
#[doc = "MPU Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPU_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<MPU_A> for bool {
    #[inline(always)]
    fn from(variant: MPU_A) -> Self {
        match variant {
            MPU_A::_0 => false,
            MPU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MPU`"]
pub type MPU_R = crate::R<bool, MPU_A>;
impl MPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPU_A {
        match self.bits {
            false => MPU_A::_0,
            true => MPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPU_A::_1
    }
}
#[doc = "Write proxy for field `MPU`"]
pub struct MPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPU_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPU_A::_1)
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
#[doc = "SDRAMC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDRAMC_A {
    #[doc = "0: Clock disabled"]
    _0,
    #[doc = "1: Clock enabled"]
    _1,
}
impl From<SDRAMC_A> for bool {
    #[inline(always)]
    fn from(variant: SDRAMC_A) -> Self {
        match variant {
            SDRAMC_A::_0 => false,
            SDRAMC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SDRAMC`"]
pub type SDRAMC_R = crate::R<bool, SDRAMC_A>;
impl SDRAMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDRAMC_A {
        match self.bits {
            false => SDRAMC_A::_0,
            true => SDRAMC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDRAMC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDRAMC_A::_1
    }
}
#[doc = "Write proxy for field `SDRAMC`"]
pub struct SDRAMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDRAMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDRAMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDRAMC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDRAMC_A::_1)
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
impl R {
    #[doc = "Bit 0 - FlexBus Clock Gate Control"]
    #[inline(always)]
    pub fn flexbus(&self) -> FLEXBUS_R {
        FLEXBUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Clock Gate Control"]
    #[inline(always)]
    pub fn mpu(&self) -> MPU_R {
        MPU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDRAMC Clock Gate Control"]
    #[inline(always)]
    pub fn sdramc(&self) -> SDRAMC_R {
        SDRAMC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexBus Clock Gate Control"]
    #[inline(always)]
    pub fn flexbus(&mut self) -> FLEXBUS_W {
        FLEXBUS_W { w: self }
    }
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - MPU Clock Gate Control"]
    #[inline(always)]
    pub fn mpu(&mut self) -> MPU_W {
        MPU_W { w: self }
    }
    #[doc = "Bit 3 - SDRAMC Clock Gate Control"]
    #[inline(always)]
    pub fn sdramc(&mut self) -> SDRAMC_W {
        SDRAMC_W { w: self }
    }
}
