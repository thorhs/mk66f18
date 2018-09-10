#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RANDOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANDOUTR {
    #[doc = "Invalid data (if you read this field when it is 0 and SR\\[OREG_LVL\\] is 0, RNGA then writes 1 to SR\\[ERRI\\], SR\\[ORU\\], and SR\\[LRS\\]; when the error interrupt is not masked (CR\\[INTM\\]=0), RNGA also asserts an error interrupt request to the interrupt controller)."]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl RANDOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            RANDOUTR::_0 => 0,
            RANDOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> RANDOUTR {
        match value {
            0 => RANDOUTR::_0,
            i => RANDOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RANDOUTR::_0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Random Output"]
    #[inline]
    pub fn randout(&self) -> RANDOUTR {
        RANDOUTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
