#[doc = "Reader of register AC12ERR"]
pub type R = crate::R<u32, super::AC12ERR>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12NE_A {
    #[doc = "0: Executed."]
    _0,
    #[doc = "1: Not executed."]
    _1,
}
impl From<AC12NE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12NE_A) -> Self {
        match variant {
            AC12NE_A::_0 => false,
            AC12NE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12NE`"]
pub type AC12NE_R = crate::R<bool, AC12NE_A>;
impl AC12NE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12NE_A {
        match self.bits {
            false => AC12NE_A::_0,
            true => AC12NE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12NE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12NE_A::_1
    }
}
#[doc = "Auto CMD12 Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12TOE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Time out."]
    _1,
}
impl From<AC12TOE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12TOE_A) -> Self {
        match variant {
            AC12TOE_A::_0 => false,
            AC12TOE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12TOE`"]
pub type AC12TOE_R = crate::R<bool, AC12TOE_A>;
impl AC12TOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12TOE_A {
        match self.bits {
            false => AC12TOE_A::_0,
            true => AC12TOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12TOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12TOE_A::_1
    }
}
#[doc = "Auto CMD12 End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EBE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: End bit error generated."]
    _1,
}
impl From<AC12EBE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EBE_A) -> Self {
        match variant {
            AC12EBE_A::_0 => false,
            AC12EBE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12EBE`"]
pub type AC12EBE_R = crate::R<bool, AC12EBE_A>;
impl AC12EBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EBE_A {
        match self.bits {
            false => AC12EBE_A::_0,
            true => AC12EBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EBE_A::_1
    }
}
#[doc = "Auto CMD12 CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12CE_A {
    #[doc = "0: No CRC error."]
    _0,
    #[doc = "1: CRC error met in Auto CMD12 response."]
    _1,
}
impl From<AC12CE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12CE_A) -> Self {
        match variant {
            AC12CE_A::_0 => false,
            AC12CE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12CE`"]
pub type AC12CE_R = crate::R<bool, AC12CE_A>;
impl AC12CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12CE_A {
        match self.bits {
            false => AC12CE_A::_0,
            true => AC12CE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12CE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12CE_A::_1
    }
}
#[doc = "Auto CMD12 Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12IE_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Error, the CMD index in response is not CMD12."]
    _1,
}
impl From<AC12IE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12IE_A) -> Self {
        match variant {
            AC12IE_A::_0 => false,
            AC12IE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AC12IE`"]
pub type AC12IE_R = crate::R<bool, AC12IE_A>;
impl AC12IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12IE_A {
        match self.bits {
            false => AC12IE_A::_0,
            true => AC12IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12IE_A::_1
    }
}
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNIBAC12E_A {
    #[doc = "0: No error."]
    _0,
    #[doc = "1: Not issued."]
    _1,
}
impl From<CNIBAC12E_A> for bool {
    #[inline(always)]
    fn from(variant: CNIBAC12E_A) -> Self {
        match variant {
            CNIBAC12E_A::_0 => false,
            CNIBAC12E_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CNIBAC12E`"]
pub type CNIBAC12E_R = crate::R<bool, CNIBAC12E_A>;
impl CNIBAC12E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNIBAC12E_A {
        match self.bits {
            false => CNIBAC12E_A::_0,
            true => CNIBAC12E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNIBAC12E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNIBAC12E_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12ne(&self) -> AC12NE_R {
        AC12NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> AC12EBE_R {
        AC12EBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto CMD12 CRC Error"]
    #[inline(always)]
    pub fn ac12ce(&self) -> AC12CE_R {
        AC12CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto CMD12 Index Error"]
    #[inline(always)]
    pub fn ac12ie(&self) -> AC12IE_R {
        AC12IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> CNIBAC12E_R {
        CNIBAC12E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
