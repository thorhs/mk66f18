#[doc = "Reader of register USBCMD"]
pub type R = crate::R<u32, super::USBCMD>;
#[doc = "Writer for register USBCMD"]
pub type W = crate::W<u32, super::USBCMD>;
#[doc = "Register USBCMD `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::USBCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Frame list Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_A {
    #[doc = "0: When FS2 = 0, the size is 1024 elements (4096 bytes). When FS2 = 1, the size is 64 elements (256 bytes)."]
    _00,
    #[doc = "1: When FS2 = 0, the size is 512 elements (2048 bytes). When FS2 = 1, the size is 32 elements (128 bytes)."]
    _01,
    #[doc = "2: When FS2 = 0, the size is 256 elements (1024 bytes). When FS2 = 1, the size is 16 elements (64 bytes)."]
    _10,
    #[doc = "3: When FS2 = 0, the size is 128 elements (512 bytes). When FS2 = 1, the size is 8 elements (32 bytes)."]
    _11,
}
impl From<FS_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        match variant {
            FS_A::_00 => 0,
            FS_A::_01 => 1,
            FS_A::_10 => 2,
            FS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `FS`"]
pub type FS_R = crate::R<u8, FS_A>;
impl FS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_A {
        match self.bits {
            0 => FS_A::_00,
            1 => FS_A::_01,
            2 => FS_A::_10,
            3 => FS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FS_A::_11
    }
}
#[doc = "Write proxy for field `FS`"]
pub struct FS_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "When FS2 = 0, the size is 1024 elements (4096 bytes). When FS2 = 1, the size is 64 elements (256 bytes)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FS_A::_00)
    }
    #[doc = "When FS2 = 0, the size is 512 elements (2048 bytes). When FS2 = 1, the size is 32 elements (128 bytes)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FS_A::_01)
    }
    #[doc = "When FS2 = 0, the size is 256 elements (1024 bytes). When FS2 = 1, the size is 16 elements (64 bytes)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FS_A::_10)
    }
    #[doc = "When FS2 = 0, the size is 128 elements (512 bytes). When FS2 = 1, the size is 8 elements (32 bytes)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Periodic Schedule Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSE_A {
    #[doc = "0: Do not process periodic schedule."]
    _0,
    #[doc = "1: Use the PERIODICLISTBASE register to access the periodic schedule."]
    _1,
}
impl From<PSE_A> for bool {
    #[inline(always)]
    fn from(variant: PSE_A) -> Self {
        match variant {
            PSE_A::_0 => false,
            PSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PSE`"]
pub type PSE_R = crate::R<bool, PSE_A>;
impl PSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSE_A {
        match self.bits {
            false => PSE_A::_0,
            true => PSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSE_A::_1
    }
}
#[doc = "Write proxy for field `PSE`"]
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not process periodic schedule."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSE_A::_0)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Asynchronous Schedule Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASE_A {
    #[doc = "0: Do not process asynchronous schedule."]
    _0,
    #[doc = "1: Use the ASYNCLISTADDR register to access asynchronous schedule."]
    _1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        match variant {
            ASE_A::_0 => false,
            ASE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<bool, ASE_A>;
impl ASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::_0,
            true => ASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASE_A::_1
    }
}
#[doc = "Write proxy for field `ASE`"]
pub struct ASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not process asynchronous schedule."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASE_A::_0)
    }
    #[doc = "Use the ASYNCLISTADDR register to access asynchronous schedule."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IAA`"]
pub type IAA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IAA`"]
pub struct IAA_W<'a> {
    w: &'a mut W,
}
impl<'a> IAA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ASP`"]
pub type ASP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASP`"]
pub struct ASP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Asynchronous Schedule Park mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPE_A {
    #[doc = "0: Park mode disabled"]
    _0,
    #[doc = "1: Park mode enabled"]
    _1,
}
impl From<ASPE_A> for bool {
    #[inline(always)]
    fn from(variant: ASPE_A) -> Self {
        match variant {
            ASPE_A::_0 => false,
            ASPE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ASPE`"]
pub type ASPE_R = crate::R<bool, ASPE_A>;
impl ASPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASPE_A {
        match self.bits {
            false => ASPE_A::_0,
            true => ASPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASPE_A::_1
    }
}
#[doc = "Write proxy for field `ASPE`"]
pub struct ASPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Park mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASPE_A::_0)
    }
    #[doc = "Park mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASPE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SUTW`"]
pub type SUTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUTW`"]
pub struct SUTW_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ATDTW`"]
pub type ATDTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATDTW`"]
pub struct ATDTW_W<'a> {
    w: &'a mut W,
}
impl<'a> ATDTW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FS2`"]
pub type FS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FS2`"]
pub struct FS2_W<'a> {
    w: &'a mut W,
}
impl<'a> FS2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Interrupt Threshold Control\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITC_A {
    #[doc = "0: Immediate (no threshold)"]
    _0,
    #[doc = "1: 1 microframe"]
    _1,
    #[doc = "2: 2 microframes"]
    _10,
    #[doc = "4: 4 microframes"]
    _100,
    #[doc = "8: 8 microframes"]
    _1000,
    #[doc = "16: 16 microframes"]
    _10000,
    #[doc = "32: 32 microframes"]
    _100000,
    #[doc = "64: 64 microframes"]
    _1000000,
}
impl From<ITC_A> for u8 {
    #[inline(always)]
    fn from(variant: ITC_A) -> Self {
        match variant {
            ITC_A::_0 => 0,
            ITC_A::_1 => 1,
            ITC_A::_10 => 2,
            ITC_A::_100 => 4,
            ITC_A::_1000 => 8,
            ITC_A::_10000 => 16,
            ITC_A::_100000 => 32,
            ITC_A::_1000000 => 64,
        }
    }
}
#[doc = "Reader of field `ITC`"]
pub type ITC_R = crate::R<u8, ITC_A>;
impl ITC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ITC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ITC_A::_0),
            1 => Val(ITC_A::_1),
            2 => Val(ITC_A::_10),
            4 => Val(ITC_A::_100),
            8 => Val(ITC_A::_1000),
            16 => Val(ITC_A::_10000),
            32 => Val(ITC_A::_100000),
            64 => Val(ITC_A::_1000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITC_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ITC_A::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ITC_A::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ITC_A::_1000
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == ITC_A::_10000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == ITC_A::_100000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == ITC_A::_1000000
    }
}
#[doc = "Write proxy for field `ITC`"]
pub struct ITC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate (no threshold)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITC_A::_0)
    }
    #[doc = "1 microframe"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITC_A::_1)
    }
    #[doc = "2 microframes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ITC_A::_10)
    }
    #[doc = "4 microframes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ITC_A::_100)
    }
    #[doc = "8 microframes"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ITC_A::_1000)
    }
    #[doc = "16 microframes"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(ITC_A::_10000)
    }
    #[doc = "32 microframes"]
    #[inline(always)]
    pub fn _100000(self) -> &'a mut W {
        self.variant(ITC_A::_100000)
    }
    #[doc = "64 microframes"]
    #[inline(always)]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(ITC_A::_1000000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controller Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Frame list Size"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance doorbell"]
    #[inline(always)]
    pub fn iaa(&self) -> IAA_R {
        IAA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park mode count"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park mode Enable"]
    #[inline(always)]
    pub fn aspe(&self) -> ASPE_R {
        ASPE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Setup TripWire"]
    #[inline(always)]
    pub fn sutw(&self) -> SUTW_R {
        SUTW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Add dTD TripWire"]
    #[inline(always)]
    pub fn atdtw(&self) -> ATDTW_R {
        ATDTW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Frame list Size 2"]
    #[inline(always)]
    pub fn fs2(&self) -> FS2_R {
        FS2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 1 - Controller Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bits 2:3 - Frame list Size"]
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W {
        FS_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn ase(&mut self) -> ASE_W {
        ASE_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt on Async Advance doorbell"]
    #[inline(always)]
    pub fn iaa(&mut self) -> IAA_W {
        IAA_W { w: self }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park mode count"]
    #[inline(always)]
    pub fn asp(&mut self) -> ASP_W {
        ASP_W { w: self }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park mode Enable"]
    #[inline(always)]
    pub fn aspe(&mut self) -> ASPE_W {
        ASPE_W { w: self }
    }
    #[doc = "Bit 13 - Setup TripWire"]
    #[inline(always)]
    pub fn sutw(&mut self) -> SUTW_W {
        SUTW_W { w: self }
    }
    #[doc = "Bit 14 - Add dTD TripWire"]
    #[inline(always)]
    pub fn atdtw(&mut self) -> ATDTW_W {
        ATDTW_W { w: self }
    }
    #[doc = "Bit 15 - Frame list Size 2"]
    #[inline(always)]
    pub fn fs2(&mut self) -> FS2_W {
        FS2_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn itc(&mut self) -> ITC_W {
        ITC_W { w: self }
    }
}
