#[doc = "Reader of register FACSN"]
pub type R = crate::R<u8, super::FACSN>;
#[doc = "Number of Segments Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMSG_A {
    #[doc = "48: Program flash memory is divided into 48 segments (768 Kbytes, 1.5 Mbytes)"]
    _110000,
    #[doc = "64: Program flash memory is divided into 64 segments (512 Kbytes, 1 Mbyte, 2 Mbytes)"]
    _1000000,
}
impl From<NUMSG_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMSG_A) -> Self {
        match variant {
            NUMSG_A::_110000 => 48,
            NUMSG_A::_1000000 => 64,
        }
    }
}
#[doc = "Reader of field `NUMSG`"]
pub type NUMSG_R = crate::R<u8, NUMSG_A>;
impl NUMSG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NUMSG_A> {
        use crate::Variant::*;
        match self.bits {
            48 => Val(NUMSG_A::_110000),
            64 => Val(NUMSG_A::_1000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_110000`"]
    #[inline(always)]
    pub fn is_110000(&self) -> bool {
        *self == NUMSG_A::_110000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == NUMSG_A::_1000000
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Segments Indicator"]
    #[inline(always)]
    pub fn numsg(&self) -> NUMSG_R {
        NUMSG_R::new((self.bits & 0xff) as u8)
    }
}
