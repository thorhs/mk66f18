#[doc = "Reader of register MF5"]
pub type R = crate::R<u8, super::MF5>;
#[doc = "Wakeup flag For module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF0_A {
    #[doc = "0: Module 0 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 0 input was a wakeup source"]
    _1,
}
impl From<MWUF0_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF0_A) -> Self {
        match variant {
            MWUF0_A::_0 => false,
            MWUF0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF0`"]
pub type MWUF0_R = crate::R<bool, MWUF0_A>;
impl MWUF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF0_A {
        match self.bits {
            false => MWUF0_A::_0,
            true => MWUF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF0_A::_1
    }
}
#[doc = "Wakeup flag For module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF1_A {
    #[doc = "0: Module 1 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 1 input was a wakeup source"]
    _1,
}
impl From<MWUF1_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF1_A) -> Self {
        match variant {
            MWUF1_A::_0 => false,
            MWUF1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF1`"]
pub type MWUF1_R = crate::R<bool, MWUF1_A>;
impl MWUF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF1_A {
        match self.bits {
            false => MWUF1_A::_0,
            true => MWUF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF1_A::_1
    }
}
#[doc = "Wakeup flag For module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF2_A {
    #[doc = "0: Module 2 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 2 input was a wakeup source"]
    _1,
}
impl From<MWUF2_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF2_A) -> Self {
        match variant {
            MWUF2_A::_0 => false,
            MWUF2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF2`"]
pub type MWUF2_R = crate::R<bool, MWUF2_A>;
impl MWUF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF2_A {
        match self.bits {
            false => MWUF2_A::_0,
            true => MWUF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF2_A::_1
    }
}
#[doc = "Wakeup flag For module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF3_A {
    #[doc = "0: Module 3 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 3 input was a wakeup source"]
    _1,
}
impl From<MWUF3_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF3_A) -> Self {
        match variant {
            MWUF3_A::_0 => false,
            MWUF3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF3`"]
pub type MWUF3_R = crate::R<bool, MWUF3_A>;
impl MWUF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF3_A {
        match self.bits {
            false => MWUF3_A::_0,
            true => MWUF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF3_A::_1
    }
}
#[doc = "Wakeup flag For module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF4_A {
    #[doc = "0: Module 4 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 4 input was a wakeup source"]
    _1,
}
impl From<MWUF4_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF4_A) -> Self {
        match variant {
            MWUF4_A::_0 => false,
            MWUF4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF4`"]
pub type MWUF4_R = crate::R<bool, MWUF4_A>;
impl MWUF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF4_A {
        match self.bits {
            false => MWUF4_A::_0,
            true => MWUF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF4_A::_1
    }
}
#[doc = "Wakeup flag For module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF5_A {
    #[doc = "0: Module 5 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 5 input was a wakeup source"]
    _1,
}
impl From<MWUF5_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF5_A) -> Self {
        match variant {
            MWUF5_A::_0 => false,
            MWUF5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF5`"]
pub type MWUF5_R = crate::R<bool, MWUF5_A>;
impl MWUF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF5_A {
        match self.bits {
            false => MWUF5_A::_0,
            true => MWUF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF5_A::_1
    }
}
#[doc = "Wakeup flag For module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF6_A {
    #[doc = "0: Module 6 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 6 input was a wakeup source"]
    _1,
}
impl From<MWUF6_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF6_A) -> Self {
        match variant {
            MWUF6_A::_0 => false,
            MWUF6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF6`"]
pub type MWUF6_R = crate::R<bool, MWUF6_A>;
impl MWUF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF6_A {
        match self.bits {
            false => MWUF6_A::_0,
            true => MWUF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF6_A::_1
    }
}
#[doc = "Wakeup flag For module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF7_A {
    #[doc = "0: Module 7 input was not a wakeup source"]
    _0,
    #[doc = "1: Module 7 input was a wakeup source"]
    _1,
}
impl From<MWUF7_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF7_A) -> Self {
        match variant {
            MWUF7_A::_0 => false,
            MWUF7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MWUF7`"]
pub type MWUF7_R = crate::R<bool, MWUF7_A>;
impl MWUF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF7_A {
        match self.bits {
            false => MWUF7_A::_0,
            true => MWUF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF7_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag For module 0"]
    #[inline(always)]
    pub fn mwuf0(&self) -> MWUF0_R {
        MWUF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag For module 1"]
    #[inline(always)]
    pub fn mwuf1(&self) -> MWUF1_R {
        MWUF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag For module 2"]
    #[inline(always)]
    pub fn mwuf2(&self) -> MWUF2_R {
        MWUF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag For module 3"]
    #[inline(always)]
    pub fn mwuf3(&self) -> MWUF3_R {
        MWUF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag For module 4"]
    #[inline(always)]
    pub fn mwuf4(&self) -> MWUF4_R {
        MWUF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag For module 5"]
    #[inline(always)]
    pub fn mwuf5(&self) -> MWUF5_R {
        MWUF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup flag For module 6"]
    #[inline(always)]
    pub fn mwuf6(&self) -> MWUF6_R {
        MWUF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup flag For module 7"]
    #[inline(always)]
    pub fn mwuf7(&self) -> MWUF7_R {
        MWUF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
