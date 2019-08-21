#[doc = "Reader of register ED"]
pub type R = crate::R<u8, super::ED>;
#[doc = "The current received dataword contained in D and C3\\[R8\\] was received with a parity error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYE_A {
    #[doc = "0: The dataword was received without a parity error."]
    _0,
    #[doc = "1: The dataword was received with a parity error."]
    _1,
}
impl From<PARITYE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYE_A) -> Self {
        match variant {
            PARITYE_A::_0 => false,
            PARITYE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PARITYE`"]
pub type PARITYE_R = crate::R<bool, PARITYE_A>;
impl PARITYE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYE_A {
        match self.bits {
            false => PARITYE_A::_0,
            true => PARITYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARITYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARITYE_A::_1
    }
}
#[doc = "The current received dataword contained in D and C3\\[R8\\] was received with noise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOISY_A {
    #[doc = "0: The dataword was received without noise."]
    _0,
    #[doc = "1: The data was received with noise."]
    _1,
}
impl From<NOISY_A> for bool {
    #[inline(always)]
    fn from(variant: NOISY_A) -> Self {
        match variant {
            NOISY_A::_0 => false,
            NOISY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NOISY`"]
pub type NOISY_R = crate::R<bool, NOISY_A>;
impl NOISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISY_A {
        match self.bits {
            false => NOISY_A::_0,
            true => NOISY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOISY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOISY_A::_1
    }
}
impl R {
    #[doc = "Bit 6 - The current received dataword contained in D and C3\\[R8\\] was received with a parity error."]
    #[inline(always)]
    pub fn paritye(&self) -> PARITYE_R {
        PARITYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The current received dataword contained in D and C3\\[R8\\] was received with noise."]
    #[inline(always)]
    pub fn noisy(&self) -> NOISY_R {
        NOISY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
