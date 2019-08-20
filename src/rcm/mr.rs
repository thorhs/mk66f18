#[doc = "Reader of register MR"]
pub type R = crate::R<u8, super::MR>;
#[doc = "EZP_MS_B pin state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZP_MS_A {
    #[doc = "0: Pin deasserted (logic 1)"]
    _0,
    #[doc = "1: Pin asserted (logic 0)"]
    _1,
}
impl From<EZP_MS_A> for bool {
    #[inline(always)]
    fn from(variant: EZP_MS_A) -> Self {
        match variant {
            EZP_MS_A::_0 => false,
            EZP_MS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EZP_MS`"]
pub type EZP_MS_R = crate::R<bool, EZP_MS_A>;
impl EZP_MS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZP_MS_A {
        match self.bits {
            false => EZP_MS_A::_0,
            true => EZP_MS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EZP_MS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EZP_MS_A::_1
    }
}
impl R {
    #[doc = "Bit 1 - EZP_MS_B pin state"]
    #[inline(always)]
    pub fn ezp_ms(&self) -> EZP_MS_R {
        EZP_MS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
