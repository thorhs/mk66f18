#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0xd890_000f"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xd890_000f
    }
}
#[doc = "Reader of field `MAXMB`"]
pub type MAXMB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXMB`"]
pub struct MAXMB_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDAM_A {
    #[doc = "0: Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01,
    #[doc = "2: Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10,
    #[doc = "3: Format D: All frames rejected."]
    _11,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        match variant {
            IDAM_A::_00 => 0,
            IDAM_A::_01 => 1,
            IDAM_A::_10 => 2,
            IDAM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `IDAM`"]
pub type IDAM_R = crate::R<u8, IDAM_A>;
impl IDAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::_00,
            1 => IDAM_A::_01,
            2 => IDAM_A::_10,
            3 => IDAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDAM_A::_11
    }
}
#[doc = "Write proxy for field `IDAM`"]
pub struct IDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAM_A::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAM_A::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAM_A::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled."]
    _0,
    #[doc = "1: Abort enabled."]
    _1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        match variant {
            AEN_A::_0 => false,
            AEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `AEN`"]
pub type AEN_R = crate::R<bool, AEN_A>;
impl AEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::_0,
            true => AEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEN_A::_1
    }
}
#[doc = "Write proxy for field `AEN`"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Abort disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEN_A::_0)
    }
    #[doc = "Abort enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled."]
    _0,
    #[doc = "1: Local Priority enabled."]
    _1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        match variant {
            LPRIOEN_A::_0 => false,
            LPRIOEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LPRIOEN`"]
pub type LPRIOEN_R = crate::R<bool, LPRIOEN_A>;
impl LPRIOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::_0,
            true => LPRIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPRIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPRIOEN_A::_1
    }
}
#[doc = "Write proxy for field `LPRIOEN`"]
pub struct LPRIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPRIOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Local Priority disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_0)
    }
    #[doc = "Local Priority enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Individual Rx Masking And Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    _1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        match variant {
            IRMQ_A::_0 => false,
            IRMQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IRMQ`"]
pub type IRMQ_R = crate::R<bool, IRMQ_A>;
impl IRMQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::_0,
            true => IRMQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRMQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRMQ_A::_1
    }
}
#[doc = "Write proxy for field `IRMQ`"]
pub struct IRMQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRMQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRMQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRMQ_A::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRMQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDIS_A {
    #[doc = "0: Self reception enabled."]
    _0,
    #[doc = "1: Self reception disabled."]
    _1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        match variant {
            SRXDIS_A::_0 => false,
            SRXDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SRXDIS`"]
pub type SRXDIS_R = crate::R<bool, SRXDIS_A>;
impl SRXDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::_0,
            true => SRXDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRXDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRXDIS_A::_1
    }
}
#[doc = "Write proxy for field `SRXDIS`"]
pub struct SRXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRXDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Self reception enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRXDIS_A::_0)
    }
    #[doc = "Self reception disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRXDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Wake Up Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKSRC_A {
    #[doc = "0: FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    _0,
    #[doc = "1: FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    _1,
}
impl From<WAKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: WAKSRC_A) -> Self {
        match variant {
            WAKSRC_A::_0 => false,
            WAKSRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAKSRC`"]
pub type WAKSRC_R = crate::R<bool, WAKSRC_A>;
impl WAKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKSRC_A {
        match self.bits {
            false => WAKSRC_A::_0,
            true => WAKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKSRC_A::_1
    }
}
#[doc = "Write proxy for field `WAKSRC`"]
pub struct WAKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKSRC_A::_0)
    }
    #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Low-Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    #[doc = "0: FlexCAN is not in a low-power mode."]
    _0,
    #[doc = "1: FlexCAN is in a low-power mode."]
    _1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        match variant {
            LPMACK_A::_0 => false,
            LPMACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LPMACK`"]
pub type LPMACK_R = crate::R<bool, LPMACK_A>;
impl LPMACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::_0,
            true => LPMACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPMACK_A::_1
    }
}
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNEN_A {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        match variant {
            WRNEN_A::_0 => false,
            WRNEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WRNEN`"]
pub type WRNEN_R = crate::R<bool, WRNEN_A>;
impl WRNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::_0,
            true => WRNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRNEN_A::_1
    }
}
#[doc = "Write proxy for field `WRNEN`"]
pub struct WRNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRNEN_A::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRNEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Self Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLFWAK_A {
    #[doc = "0: FlexCAN Self Wake Up feature is disabled."]
    _0,
    #[doc = "1: FlexCAN Self Wake Up feature is enabled."]
    _1,
}
impl From<SLFWAK_A> for bool {
    #[inline(always)]
    fn from(variant: SLFWAK_A) -> Self {
        match variant {
            SLFWAK_A::_0 => false,
            SLFWAK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLFWAK`"]
pub type SLFWAK_R = crate::R<bool, SLFWAK_A>;
impl SLFWAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLFWAK_A {
        match self.bits {
            false => SLFWAK_A::_0,
            true => SLFWAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLFWAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLFWAK_A::_1
    }
}
#[doc = "Write proxy for field `SLFWAK`"]
pub struct SLFWAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLFWAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLFWAK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLFWAK_A::_0)
    }
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLFWAK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Supervisor Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPV_A {
    #[doc = "0: FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    _0,
    #[doc = "1: FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    _1,
}
impl From<SUPV_A> for bool {
    #[inline(always)]
    fn from(variant: SUPV_A) -> Self {
        match variant {
            SUPV_A::_0 => false,
            SUPV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SUPV`"]
pub type SUPV_R = crate::R<bool, SUPV_A>;
impl SUPV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPV_A {
        match self.bits {
            false => SUPV_A::_0,
            true => SUPV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUPV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUPV_A::_1
    }
}
#[doc = "Write proxy for field `SUPV`"]
pub struct SUPV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUPV_A::_0)
    }
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUPV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACK_A {
    #[doc = "0: FlexCAN not in Freeze mode, prescaler running."]
    _0,
    #[doc = "1: FlexCAN in Freeze mode, prescaler stopped."]
    _1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        match variant {
            FRZACK_A::_0 => false,
            FRZACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRZACK`"]
pub type FRZACK_R = crate::R<bool, FRZACK_A>;
impl FRZACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::_0,
            true => FRZACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZACK_A::_1
    }
}
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request."]
    _0,
    #[doc = "1: Resets the registers affected by soft reset."]
    _1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        match variant {
            SOFTRST_A::_0 => false,
            SOFTRST_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SOFTRST`"]
pub type SOFTRST_R = crate::R<bool, SOFTRST_A>;
impl SOFTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::_0,
            true => SOFTRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFTRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFTRST_A::_1
    }
}
#[doc = "Write proxy for field `SOFTRST`"]
pub struct SOFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTRST_A::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Wake Up Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKMSK_A {
    #[doc = "0: Wake Up Interrupt is disabled."]
    _0,
    #[doc = "1: Wake Up Interrupt is enabled."]
    _1,
}
impl From<WAKMSK_A> for bool {
    #[inline(always)]
    fn from(variant: WAKMSK_A) -> Self {
        match variant {
            WAKMSK_A::_0 => false,
            WAKMSK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAKMSK`"]
pub type WAKMSK_R = crate::R<bool, WAKMSK_A>;
impl WAKMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKMSK_A {
        match self.bits {
            false => WAKMSK_A::_0,
            true => WAKMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKMSK_A::_1
    }
}
#[doc = "Write proxy for field `WAKMSK`"]
pub struct WAKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake Up Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKMSK_A::_0)
    }
    #[doc = "Wake Up Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKMSK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDY_A {
    #[doc = "0: FlexCAN module is either in Normal mode, Listen-Only mode or Loop-Back mode."]
    _0,
    #[doc = "1: FlexCAN module is either in Disable mode , Stop mode or Freeze mode."]
    _1,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        match variant {
            NOTRDY_A::_0 => false,
            NOTRDY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `NOTRDY`"]
pub type NOTRDY_R = crate::R<bool, NOTRDY_A>;
impl NOTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRDY_A {
        match self.bits {
            false => NOTRDY_A::_0,
            true => NOTRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOTRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOTRDY_A::_1
    }
}
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: No Freeze mode request."]
    _0,
    #[doc = "1: Enters Freeze mode if the FRZ bit is asserted."]
    _1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        match variant {
            HALT_A::_0 => false,
            HALT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, HALT_A>;
impl HALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Freeze mode request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: Rx FIFO not enabled."]
    _0,
    #[doc = "1: Rx FIFO enabled."]
    _1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        match variant {
            RFEN_A::_0 => false,
            RFEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RFEN`"]
pub type RFEN_R = crate::R<bool, RFEN_A>;
impl RFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEN_A::_1
    }
}
#[doc = "Write proxy for field `RFEN`"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFEN_A::_0)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze mode."]
    _0,
    #[doc = "1: Enabled to enter Freeze mode."]
    _1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        match variant {
            FRZ_A::_0 => false,
            FRZ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, FRZ_A>;
impl FRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZ_A::_1
    }
}
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FlexCAN module."]
    _0,
    #[doc = "1: Disable the FlexCAN module."]
    _1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        match variant {
            MDIS_A::_0 => false,
            MDIS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline(always)]
    pub fn waksrc(&self) -> WAKSRC_R {
        WAKSRC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    pub fn slfwak(&self) -> SLFWAK_R {
        SLFWAK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakmsk(&self) -> WAKMSK_R {
        WAKMSK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&mut self) -> MAXMB_W {
        MAXMB_W { w: self }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&mut self) -> IDAM_W {
        IDAM_W { w: self }
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&mut self) -> LPRIOEN_W {
        LPRIOEN_W { w: self }
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&mut self) -> IRMQ_W {
        IRMQ_W { w: self }
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&mut self) -> SRXDIS_W {
        SRXDIS_W { w: self }
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline(always)]
    pub fn waksrc(&mut self) -> WAKSRC_W {
        WAKSRC_W { w: self }
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&mut self) -> WRNEN_W {
        WRNEN_W { w: self }
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    pub fn slfwak(&mut self) -> SLFWAK_W {
        SLFWAK_W { w: self }
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&mut self) -> SUPV_W {
        SUPV_W { w: self }
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W {
        SOFTRST_W { w: self }
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakmsk(&mut self) -> WAKMSK_W {
        WAKMSK_W { w: self }
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
}
