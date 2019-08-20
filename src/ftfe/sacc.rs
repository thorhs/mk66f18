#[doc = "Reader of register SACC%s"]
pub type R = crate::R<u8, super::SACC>;
#[doc = "Supervisor-only access control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
    #[doc = "0: Associated segment is accessible in supervisor mode only"]
    _0,
    #[doc = "1: Associated segment is accessible in user or supervisor mode"]
    _1,
}
impl From<SA_A> for u8 {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        match variant {
            SA_A::_0 => 0,
            SA_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u8, SA_A>;
impl SA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SA_A::_0),
            1 => Val(SA_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SA_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Supervisor-only access control"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xff) as u8)
    }
}
