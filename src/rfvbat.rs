#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VBAT register file register"]
    pub reg: [REG; 8],
}
#[doc = "VBAT register file register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reg](reg) module"]
pub type REG = crate::Reg<u32, _REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG;
#[doc = "`read()` method returns [reg::R](reg::R) reader structure"]
impl crate::Readable for REG {}
#[doc = "`write(|w| ..)` method takes [reg::W](reg::W) writer structure"]
impl crate::Writable for REG {}
#[doc = "VBAT register file register"]
pub mod reg;
