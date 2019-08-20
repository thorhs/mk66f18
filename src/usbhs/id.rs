#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `NID`"]
pub type NID_R = crate::R<u8, u8>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `VERSIONID`"]
pub type VERSIONID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Configuration number"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Ones complement version of ID."]
    #[inline(always)]
    pub fn nid(&self) -> NID_R {
        NID_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Tag"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:28 - Version"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Version ID"]
    #[inline(always)]
    pub fn versionid(&self) -> VERSIONID_R {
        VERSIONID_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
