#[doc = "Reader of register HCCPARAMS"]
pub type R = crate::R<u32, super::HCCPARAMS>;
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFL`"]
pub type PFL_R = crate::R<bool, bool>;
#[doc = "Asynchronous Schedule Park capability\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASP_A {
    #[doc = "0: Park not supported."]
    _0,
    #[doc = "1: Park supported."]
    _1,
}
impl From<ASP_A> for bool {
    #[inline(always)]
    fn from(variant: ASP_A) -> Self {
        match variant {
            ASP_A::_0 => false,
            ASP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ASP`"]
pub type ASP_R = crate::R<bool, ASP_A>;
impl ASP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASP_A {
        match self.bits {
            false => ASP_A::_0,
            true => ASP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASP_A::_1
    }
}
#[doc = "Isochronous Scheduling Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IST_A {
    #[doc = "0: The value of the least significant 3 bits indicates the number of microframes a host controller can hold a set of isochronous data structures (one or more) before flushing the state"]
    _0,
}
impl From<IST_A> for u8 {
    #[inline(always)]
    fn from(variant: IST_A) -> Self {
        match variant {
            IST_A::_0 => 0,
        }
    }
}
#[doc = "Reader of field `IST`"]
pub type IST_R = crate::R<u8, IST_A>;
impl IST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IST_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IST_A::_0
    }
}
#[doc = "EHCI Extended Capabilities Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EECP_A {
    #[doc = "0: No extended capabilities are implemented"]
    _0,
}
impl From<EECP_A> for u8 {
    #[inline(always)]
    fn from(variant: EECP_A) -> Self {
        match variant {
            EECP_A::_0 => 0,
        }
    }
}
#[doc = "Reader of field `EECP`"]
pub type EECP_R = crate::R<u8, EECP_A>;
impl EECP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EECP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EECP_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EECP_A::_0
    }
}
impl R {
    #[doc = "Bit 0 - 64-bit addressing capability."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programmable Frame List flag"]
    #[inline(always)]
    pub fn pfl(&self) -> PFL_R {
        PFL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park capability"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer"]
    #[inline(always)]
    pub fn eecp(&self) -> EECP_R {
        EECP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
