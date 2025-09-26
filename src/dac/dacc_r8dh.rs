#[doc = "Register `DACC_R8DH` reader"]
pub type R = crate::R<DACC_R8DH_SPEC>;
#[doc = "Register `DACC_R8DH` writer"]
pub type W = crate::W<DACC_R8DH_SPEC>;
#[doc = "Field `DAC0_DH` reader - DAC0 8-bit right-aligned data"]
pub type DAC0_DH_R = crate::FieldReader;
#[doc = "Field `DAC0_DH` writer - DAC0 8-bit right-aligned data"]
pub type DAC0_DH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAC1_DH` reader - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_R = crate::FieldReader;
#[doc = "Field `DAC1_DH` writer - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W<DACC_R8DH_SPEC> {
        DAC0_DH_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<DACC_R8DH_SPEC> {
        DAC1_DH_W::new(self, 8)
    }
}
#[doc = "DAC concurrent mode 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r8dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r8dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACC_R8DH_SPEC;
impl crate::RegisterSpec for DACC_R8DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacc_r8dh::R`](R) reader structure"]
impl crate::Readable for DACC_R8DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacc_r8dh::W`](W) writer structure"]
impl crate::Writable for DACC_R8DH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACC_R8DH to value 0"]
impl crate::Resettable for DACC_R8DH_SPEC {
    const RESET_VALUE: u32 = 0;
}
