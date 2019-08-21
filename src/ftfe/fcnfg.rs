#[doc = "Reader of register FCNFG"]
pub type R = crate::R<u8, super::FCNFG>;
#[doc = "Writer for register FCNFG"]
pub type W = crate::W<u8, super::FCNFG>;
#[doc = "Register FCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FCNFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEERDY_A {
    #[doc = "0: For devices with FlexNVM: FlexRAM is not available for EEPROM operation For devices without FlexNVM: See RAMRDY for availability of programming acceleration RAM"]
    _0,
    #[doc = "1: For devices with FlexNVM: FlexRAM is available for EEPROM operations where: reads from the FlexRAM return data previously written to the FlexRAM in EEPROM mode and writes launch an EEPROM operation to store the written data in the FlexRAM and EEPROM backup For devices without FlexNVM: Reserved"]
    _1,
}
impl From<EEERDY_A> for bool {
    #[inline(always)]
    fn from(variant: EEERDY_A) -> Self {
        match variant {
            EEERDY_A::_0 => false,
            EEERDY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EEERDY`"]
pub type EEERDY_R = crate::R<bool, EEERDY_A>;
impl EEERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEERDY_A {
        match self.bits {
            false => EEERDY_A::_0,
            true => EEERDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEERDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEERDY_A::_1
    }
}
#[doc = "RAM Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRDY_A {
    #[doc = "0: For devices with FlexNVM: FlexRAM is not available for traditional RAM access For devices without FlexNVM: Programming acceleration RAM is not available"]
    _0,
    #[doc = "1: For devices with FlexNVM: FlexRAM is available as traditional RAM only; writes to the FlexRAM do not trigger EEPROM operations For devices without FlexNVM: Programming acceleration RAM is available"]
    _1,
}
impl From<RAMRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRDY_A) -> Self {
        match variant {
            RAMRDY_A::_0 => false,
            RAMRDY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RAMRDY`"]
pub type RAMRDY_R = crate::R<bool, RAMRDY_A>;
impl RAMRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRDY_A {
        match self.bits {
            false => RAMRDY_A::_0,
            true => RAMRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAMRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAMRDY_A::_1
    }
}
#[doc = "FTFE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLSH_A {
    #[doc = "0: For devices with FlexNVM: FTFE configuration supports two or three program flash blocks and two FlexNVM blocks For devices with program flash only: Reserved"]
    _0,
    #[doc = "1: For devices with FlexNVM: Reserved For devices with program flash only: FTFE configuration supports four program flash blocks"]
    _1,
}
impl From<PFLSH_A> for bool {
    #[inline(always)]
    fn from(variant: PFLSH_A) -> Self {
        match variant {
            PFLSH_A::_0 => false,
            PFLSH_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PFLSH`"]
pub type PFLSH_R = crate::R<bool, PFLSH_A>;
impl PFLSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFLSH_A {
        match self.bits {
            false => PFLSH_A::_0,
            true => PFLSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFLSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFLSH_A::_1
    }
}
#[doc = "Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP_A {
    #[doc = "0: For devices with FlexNVM: Program flash 0 block is located at relative address 0x0000 For devices with program flash only: Program flash 0/1 blocks are located at relative address 0x0000"]
    _0,
    #[doc = "1: For devices with FlexNVM: Reserved For devices with program flash only: Program flash 2/3 blocks are located at relative address 0x0000"]
    _1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        match variant {
            SWAP_A::_0 => false,
            SWAP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, SWAP_A>;
impl SWAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::_0,
            true => SWAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAP_A::_1
    }
}
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSP_A {
    #[doc = "0: No suspend requested"]
    _0,
    #[doc = "1: Suspend the current Erase Flash Sector command execution"]
    _1,
}
impl From<ERSSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: ERSSUSP_A) -> Self {
        match variant {
            ERSSUSP_A::_0 => false,
            ERSSUSP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERSSUSP`"]
pub type ERSSUSP_R = crate::R<bool, ERSSUSP_A>;
impl ERSSUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSSUSP_A {
        match self.bits {
            false => ERSSUSP_A::_0,
            true => ERSSUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSSUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSSUSP_A::_1
    }
}
#[doc = "Write proxy for field `ERSSUSP`"]
pub struct ERSSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERSSUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQ_A {
    #[doc = "0: No request or request complete"]
    _0,
    #[doc = "1: Request to: run the Erase All Blocks command, verify the erased state, program the security byte in the Flash Configuration Field to the unsecure state, and release MCU security by setting the FSEC\\[SEC\\] field to the unsecure state"]
    _1,
}
impl From<ERSAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERSAREQ_A) -> Self {
        match variant {
            ERSAREQ_A::_0 => false,
            ERSAREQ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERSAREQ`"]
pub type ERSAREQ_R = crate::R<bool, ERSAREQ_A>;
impl ERSAREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSAREQ_A {
        match self.bits {
            false => ERSAREQ_A::_0,
            true => ERSAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSAREQ_A::_1
    }
}
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIE_A {
    #[doc = "0: Read collision error interrupt disabled"]
    _0,
    #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1,
}
impl From<RDCOLLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLLIE_A) -> Self {
        match variant {
            RDCOLLIE_A::_0 => false,
            RDCOLLIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RDCOLLIE`"]
pub type RDCOLLIE_R = crate::R<bool, RDCOLLIE_A>;
impl RDCOLLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLLIE_A {
        match self.bits {
            false => RDCOLLIE_A::_0,
            true => RDCOLLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCOLLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCOLLIE_A::_1
    }
}
#[doc = "Write proxy for field `RDCOLLIE`"]
pub struct RDCOLLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCOLLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDCOLLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt disabled"]
    _0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    _1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        match variant {
            CCIE_A::_0 => false,
            CCIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIE_A::_1
    }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access"]
    #[inline(always)]
    pub fn eeerdy(&self) -> EEERDY_R {
        EEERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline(always)]
    pub fn ramrdy(&self) -> RAMRDY_R {
        RAMRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTFE configuration"]
    #[inline(always)]
    pub fn pflsh(&self) -> PFLSH_R {
        PFLSH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&self) -> ERSSUSP_R {
        ERSSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ERSAREQ_R {
        ERSAREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&self) -> RDCOLLIE_R {
        RDCOLLIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&mut self) -> ERSSUSP_W {
        ERSSUSP_W { w: self }
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&mut self) -> RDCOLLIE_W {
        RDCOLLIE_W { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
}
