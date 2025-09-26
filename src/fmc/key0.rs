#[doc = "Register `KEY0` writer"]
pub type W = crate::W<KEY0_SPEC>;
#[doc = "Field `KEY` writer - FMC_CTL0 unlock key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_CTL0 unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY0_SPEC> {
        KEY_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unlock key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY0_SPEC;
impl crate::RegisterSpec for KEY0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key0::W`](W) writer structure"]
impl crate::Writable for KEY0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for KEY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
