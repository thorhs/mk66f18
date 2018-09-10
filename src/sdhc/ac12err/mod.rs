#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AC12ERR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `AC12NE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12NER {
    #[doc = "Executed."]
    _0,
    #[doc = "Not executed."]
    _1,
}
impl AC12NER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AC12NER::_0 => false,
            AC12NER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12NER {
        match value {
            false => AC12NER::_0,
            true => AC12NER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12NER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12NER::_1
    }
}
#[doc = "Possible values of the field `AC12TOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12TOER {
    #[doc = "No error."]
    _0,
    #[doc = "Time out."]
    _1,
}
impl AC12TOER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AC12TOER::_0 => false,
            AC12TOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12TOER {
        match value {
            false => AC12TOER::_0,
            true => AC12TOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12TOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12TOER::_1
    }
}
#[doc = "Possible values of the field `AC12EBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EBER {
    #[doc = "No error."]
    _0,
    #[doc = "End bit error generated."]
    _1,
}
impl AC12EBER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AC12EBER::_0 => false,
            AC12EBER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12EBER {
        match value {
            false => AC12EBER::_0,
            true => AC12EBER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12EBER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12EBER::_1
    }
}
#[doc = "Possible values of the field `AC12CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12CER {
    #[doc = "No CRC error."]
    _0,
    #[doc = "CRC error met in Auto CMD12 response."]
    _1,
}
impl AC12CER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AC12CER::_0 => false,
            AC12CER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12CER {
        match value {
            false => AC12CER::_0,
            true => AC12CER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12CER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12CER::_1
    }
}
#[doc = "Possible values of the field `AC12IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12IER {
    #[doc = "No error."]
    _0,
    #[doc = "Error, the CMD index in response is not CMD12."]
    _1,
}
impl AC12IER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AC12IER::_0 => false,
            AC12IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12IER {
        match value {
            false => AC12IER::_0,
            true => AC12IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12IER::_1
    }
}
#[doc = "Possible values of the field `CNIBAC12E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNIBAC12ER {
    #[doc = "No error."]
    _0,
    #[doc = "Not issued."]
    _1,
}
impl CNIBAC12ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CNIBAC12ER::_0 => false,
            CNIBAC12ER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNIBAC12ER {
        match value {
            false => CNIBAC12ER::_0,
            true => CNIBAC12ER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CNIBAC12ER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CNIBAC12ER::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline]
    pub fn ac12ne(&self) -> AC12NER {
        AC12NER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline]
    pub fn ac12toe(&self) -> AC12TOER {
        AC12TOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD12 End Bit Error"]
    #[inline]
    pub fn ac12ebe(&self) -> AC12EBER {
        AC12EBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Auto CMD12 CRC Error"]
    #[inline]
    pub fn ac12ce(&self) -> AC12CER {
        AC12CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Auto CMD12 Index Error"]
    #[inline]
    pub fn ac12ie(&self) -> AC12IER {
        AC12IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cnibac12e(&self) -> CNIBAC12ER {
        CNIBAC12ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
