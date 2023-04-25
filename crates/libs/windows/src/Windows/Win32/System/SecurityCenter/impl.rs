#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCDefaultProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetDefaultProduct(&self, etype: SECURITY_PRODUCT_TYPE, pguid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IWSCDefaultProduct {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCDefaultProduct_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: isize>() -> IWSCDefaultProduct_Vtbl {
        unsafe extern "system" fn SetDefaultProduct<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultProduct(::core::mem::transmute_copy(&etype), ::core::mem::transmute(&pguid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), SetDefaultProduct: SetDefaultProduct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSCDefaultProduct as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCProductList_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> ::windows_core::Result<()>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Item(&self, index: u32) -> ::windows_core::Result<IWscProduct>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IWSCProductList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCProductList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>() -> IWSCProductList_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&provider)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSCProductList as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn ProductName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductState(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_STATE>;
    fn SignatureStatus(&self) -> ::windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS>;
    fn RemediationPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductStateTimestamp(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductGuid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductIsDefault(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IWscProduct {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>() -> IWscProduct_Vtbl {
        unsafe extern "system" fn ProductName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SignatureStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemediationPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemediationPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductStateTimestamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductStateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductIsDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductIsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProductName: ProductName::<Identity, Impl, OFFSET>,
            ProductState: ProductState::<Identity, Impl, OFFSET>,
            SignatureStatus: SignatureStatus::<Identity, Impl, OFFSET>,
            RemediationPath: RemediationPath::<Identity, Impl, OFFSET>,
            ProductStateTimestamp: ProductStateTimestamp::<Identity, Impl, OFFSET>,
            ProductGuid: ProductGuid::<Identity, Impl, OFFSET>,
            ProductIsDefault: ProductIsDefault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWscProduct as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct2_Impl: Sized + IWscProduct_Impl {
    fn AntivirusScanSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusSettingsSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallDomainProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPrivateProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPublicProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IWscProduct2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>() -> IWscProduct2_Vtbl {
        unsafe extern "system" fn AntivirusScanSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusScanSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusSettingsSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusProtectionUpdateSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallDomainProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallPrivateProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallPublicProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWscProduct_Vtbl::new::<Identity, Impl, OFFSET>(),
            AntivirusScanSubstatus: AntivirusScanSubstatus::<Identity, Impl, OFFSET>,
            AntivirusSettingsSubstatus: AntivirusSettingsSubstatus::<Identity, Impl, OFFSET>,
            AntivirusProtectionUpdateSubstatus: AntivirusProtectionUpdateSubstatus::<Identity, Impl, OFFSET>,
            FirewallDomainProfileSubstatus: FirewallDomainProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPrivateProfileSubstatus: FirewallPrivateProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPublicProfileSubstatus: FirewallPublicProfileSubstatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWscProduct2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IWscProduct as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct3_Impl: Sized + IWscProduct2_Impl {
    fn AntivirusDaysUntilExpired(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IWscProduct3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: isize>() -> IWscProduct3_Vtbl {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusDaysUntilExpired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdays, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWscProduct2_Vtbl::new::<Identity, Impl, OFFSET>(), AntivirusDaysUntilExpired: AntivirusDaysUntilExpired::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWscProduct3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IWscProduct as ::windows_core::ComInterface>::IID || iid == &<IWscProduct2 as ::windows_core::ComInterface>::IID
    }
}
