#[doc = "Register `ALRML` writer"]
pub struct W(crate::W<ALRML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ALRML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRM` writer - alarm value low"]
pub type ALRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRML_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - alarm value low"]
    #[inline(always)]
    #[must_use]
    pub fn alrm(&mut self) -> ALRM_W<0> {
        ALRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm low register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrml](index.html) module"]
pub struct ALRML_SPEC;
impl crate::RegisterSpec for ALRML_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alrml::W](W) writer structure"]
impl crate::Writable for ALRML_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRML to value 0xffff"]
impl crate::Resettable for ALRML_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
