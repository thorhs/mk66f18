#[doc = "Reader of register CPACR"]
pub type R = crate::R<u32, super::CPACR>;
#[doc = "Writer for register CPACR"]
pub type W = crate::W<u32, super::CPACR>;
#[doc = "Register CPACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access privileges for coprocessor 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "2: Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "3: Full access."]
    _11,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        match variant {
            CP10_A::_00 => 0,
            CP10_A::_01 => 1,
            CP10_A::_10 => 2,
            CP10_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CP10`"]
pub type CP10_R = crate::R<u8, CP10_A>;
impl CP10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CP10_A {
        match self.bits {
            0 => CP10_A::_00,
            1 => CP10_A::_01,
            2 => CP10_A::_10,
            3 => CP10_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CP10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CP10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CP10_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CP10_A::_11
    }
}
#[doc = "Write proxy for field `CP10`"]
pub struct CP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CP10_A::_00)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CP10_A::_01)
    }
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CP10_A::_10)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CP10_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "2: Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "3: Full access."]
    _11,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        match variant {
            CP11_A::_00 => 0,
            CP11_A::_01 => 1,
            CP11_A::_10 => 2,
            CP11_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `CP11`"]
pub type CP11_R = crate::R<u8, CP11_A>;
impl CP11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CP11_A {
        match self.bits {
            0 => CP11_A::_00,
            1 => CP11_A::_01,
            2 => CP11_A::_10,
            3 => CP11_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CP11_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CP11_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CP11_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CP11_A::_11
    }
}
#[doc = "Write proxy for field `CP11`"]
pub struct CP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CP11_A::_00)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CP11_A::_01)
    }
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CP11_A::_10)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CP11_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&mut self) -> CP10_W {
        CP10_W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&mut self) -> CP11_W {
        CP11_W { w: self }
    }
}
