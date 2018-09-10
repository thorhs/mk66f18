#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FCNFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EEERDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEERDYR {
    #[doc = "For devices with FlexNVM: FlexRAM is not available for EEPROM operation For devices without FlexNVM: See RAMRDY for availability of programming acceleration RAM"]
    _0,
    #[doc = "For devices with FlexNVM: FlexRAM is available for EEPROM operations where: reads from the FlexRAM return data previously written to the FlexRAM in EEPROM mode and writes launch an EEPROM operation to store the written data in the FlexRAM and EEPROM backup For devices without FlexNVM: Reserved"]
    _1,
}
impl EEERDYR {
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
            EEERDYR::_0 => false,
            EEERDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEERDYR {
        match value {
            false => EEERDYR::_0,
            true => EEERDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEERDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEERDYR::_1
    }
}
#[doc = "Possible values of the field `RAMRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRDYR {
    #[doc = "For devices with FlexNVM: FlexRAM is not available for traditional RAM access For devices without FlexNVM: Programming acceleration RAM is not available"]
    _0,
    #[doc = "For devices with FlexNVM: FlexRAM is available as traditional RAM only; writes to the FlexRAM do not trigger EEPROM operations For devices without FlexNVM: Programming acceleration RAM is available"]
    _1,
}
impl RAMRDYR {
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
            RAMRDYR::_0 => false,
            RAMRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMRDYR {
        match value {
            false => RAMRDYR::_0,
            true => RAMRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAMRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAMRDYR::_1
    }
}
#[doc = "Possible values of the field `PFLSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLSHR {
    #[doc = "For devices with FlexNVM: FTFE configuration supports two or three program flash blocks and two FlexNVM blocks For devices with program flash only: Reserved"]
    _0,
    #[doc = "For devices with FlexNVM: Reserved For devices with program flash only: FTFE configuration supports four program flash blocks"]
    _1,
}
impl PFLSHR {
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
            PFLSHR::_0 => false,
            PFLSHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFLSHR {
        match value {
            false => PFLSHR::_0,
            true => PFLSHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PFLSHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PFLSHR::_1
    }
}
#[doc = "Possible values of the field `SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPR {
    #[doc = "For devices with FlexNVM: Program flash 0 block is located at relative address 0x0000 For devices with program flash only: Program flash 0/1 blocks are located at relative address 0x0000"]
    _0,
    #[doc = "For devices with FlexNVM: Reserved For devices with program flash only: Program flash 2/3 blocks are located at relative address 0x0000"]
    _1,
}
impl SWAPR {
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
            SWAPR::_0 => false,
            SWAPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAPR {
        match value {
            false => SWAPR::_0,
            true => SWAPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWAPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWAPR::_1
    }
}
#[doc = "Possible values of the field `ERSSUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSPR {
    #[doc = "No suspend requested"]
    _0,
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    _1,
}
impl ERSSUSPR {
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
            ERSSUSPR::_0 => false,
            ERSSUSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERSSUSPR {
        match value {
            false => ERSSUSPR::_0,
            true => ERSSUSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERSSUSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERSSUSPR::_1
    }
}
#[doc = "Possible values of the field `ERSAREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQR {
    #[doc = "No request or request complete"]
    _0,
    #[doc = "Request to: run the Erase All Blocks command, verify the erased state, program the security byte in the Flash Configuration Field to the unsecure state, and release MCU security by setting the FSEC\\[SEC\\] field to the unsecure state"]
    _1,
}
impl ERSAREQR {
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
            ERSAREQR::_0 => false,
            ERSAREQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERSAREQR {
        match value {
            false => ERSAREQR::_0,
            true => ERSAREQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERSAREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERSAREQR::_1
    }
}
#[doc = "Possible values of the field `RDCOLLIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIER {
    #[doc = "Read collision error interrupt disabled"]
    _0,
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1,
}
impl RDCOLLIER {
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
            RDCOLLIER::_0 => false,
            RDCOLLIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDCOLLIER {
        match value {
            false => RDCOLLIER::_0,
            true => RDCOLLIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDCOLLIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDCOLLIER::_1
    }
}
#[doc = "Possible values of the field `CCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIER {
    #[doc = "Command complete interrupt disabled"]
    _0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    _1,
}
impl CCIER {
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
            CCIER::_0 => false,
            CCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIER {
        match value {
            false => CCIER::_0,
            true => CCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCIER::_1
    }
}
#[doc = "Values that can be written to the field `ERSSUSP`"]
pub enum ERSSUSPW {
    #[doc = "No suspend requested"]
    _0,
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    _1,
}
impl ERSSUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERSSUSPW::_0 => false,
            ERSSUSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERSSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _ERSSUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERSSUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No suspend requested"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSPW::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSPW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDCOLLIE`"]
pub enum RDCOLLIEW {
    #[doc = "Read collision error interrupt disabled"]
    _0,
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1,
}
impl RDCOLLIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDCOLLIEW::_0 => false,
            RDCOLLIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDCOLLIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDCOLLIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDCOLLIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIEW::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIEW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCIE`"]
pub enum CCIEW {
    #[doc = "Command complete interrupt disabled"]
    _0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    _1,
}
impl CCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIEW::_0 => false,
            CCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIEW::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIEW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access"]
    #[inline]
    pub fn eeerdy(&self) -> EEERDYR {
        EEERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline]
    pub fn ramrdy(&self) -> RAMRDYR {
        RAMRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - FTFE configuration"]
    #[inline]
    pub fn pflsh(&self) -> PFLSHR {
        PFLSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Swap"]
    #[inline]
    pub fn swap(&self) -> SWAPR {
        SWAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline]
    pub fn erssusp(&self) -> ERSSUSPR {
        ERSSUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline]
    pub fn ersareq(&self) -> ERSAREQR {
        ERSAREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline]
    pub fn rdcollie(&self) -> RDCOLLIER {
        RDCOLLIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccie(&self) -> CCIER {
        CCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline]
    pub fn erssusp(&mut self) -> _ERSSUSPW {
        _ERSSUSPW { w: self }
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline]
    pub fn rdcollie(&mut self) -> _RDCOLLIEW {
        _RDCOLLIEW { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccie(&mut self) -> _CCIEW {
        _CCIEW { w: self }
    }
}
