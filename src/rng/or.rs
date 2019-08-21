#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Random Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANDOUT_A {
    #[doc = "0: Invalid data (if you read this field when it is 0 and SR\\[OREG_LVL\\] is 0, RNGA then writes 1 to SR\\[ERRI\\], SR\\[ORU\\], and SR\\[LRS\\]; when the error interrupt is not masked (CR\\[INTM\\]=0), RNGA also asserts an error interrupt request to the interrupt controller)."]
    _0,
}
impl From<RANDOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: RANDOUT_A) -> Self {
        match variant {
            RANDOUT_A::_0 => 0,
        }
    }
}
#[doc = "Reader of field `RANDOUT`"]
pub type RANDOUT_R = crate::R<u32, RANDOUT_A>;
impl RANDOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RANDOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RANDOUT_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RANDOUT_A::_0
    }
}
impl R {
    #[doc = "Bits 0:31 - Random Output"]
    #[inline(always)]
    pub fn randout(&self) -> RANDOUT_R {
        RANDOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
