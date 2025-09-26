#[doc = "Register `BDCTL` reader"]
pub type R = crate::R<BDCTL_SPEC>;
#[doc = "Register `BDCTL` writer"]
pub type W = crate::W<BDCTL_SPEC>;
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LXTALEN_R = crate::BitReader;
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LXTALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LXTALSTB_R = crate::BitReader;
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LXTALBPS_R = crate::BitReader;
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LXTALBPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RTCSRC_R = crate::FieldReader;
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RTCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRST` reader - Backup domain reset"]
pub type BKPRST_R = crate::BitReader;
#[doc = "Field `BKPRST` writer - Backup domain reset"]
pub type BKPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&self) -> LXTALEN_R {
        LXTALEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator stabilization"]
    #[inline(always)]
    pub fn lxtalstb(&self) -> LXTALSTB_R {
        LXTALSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&self) -> LXTALBPS_R {
        LXTALBPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalen(&mut self) -> LXTALEN_W<BDCTL_SPEC> {
        LXTALEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalbps(&mut self) -> LXTALBPS_W<BDCTL_SPEC> {
        LXTALBPS_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsrc(&mut self) -> RTCSRC_W<BDCTL_SPEC> {
        RTCSRC_W::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCTL_SPEC> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BKPRST_W<BDCTL_SPEC> {
        BKPRST_W::new(self, 16)
    }
}
#[doc = "Backup domain control register (RCU_BDCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCTL_SPEC;
impl crate::RegisterSpec for BDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdctl::R`](R) reader structure"]
impl crate::Readable for BDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdctl::W`](W) writer structure"]
impl crate::Writable for BDCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCTL to value 0x18"]
impl crate::Resettable for BDCTL_SPEC {
    const RESET_VALUE: u32 = 0x18;
}
