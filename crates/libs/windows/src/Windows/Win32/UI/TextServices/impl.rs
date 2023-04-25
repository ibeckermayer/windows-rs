#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccClientDocMgr_Impl: Sized {
    fn GetDocuments(&self) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetFocused(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IAccClientDocMgr {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccClientDocMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: isize>() -> IAccClientDocMgr_Vtbl {
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupByPoint(::core::mem::transmute(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocused(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, Impl, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, Impl, OFFSET>,
            GetFocused: GetFocused::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccClientDocMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccDictionary_Impl: Sized {
    fn GetLocalizedString(&self, term: *const ::windows_core::GUID, lcid: u32, presult: *mut ::windows_core::BSTR, plcid: *mut u32) -> ::windows_core::Result<()>;
    fn GetParentTerm(&self, term: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMnemonicString(&self, term: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LookupMnemonicTerm(&self, bstrmnemonic: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::GUID>;
    fn ConvertValueToString(&self, term: *const ::windows_core::GUID, lcid: u32, varvalue: &super::super::System::Com::VARIANT, pbstrresult: *mut ::windows_core::BSTR, plcid: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IAccDictionary {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccDictionary_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>() -> IAccDictionary_Vtbl {
        unsafe extern "system" fn GetLocalizedString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, lcid: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plcid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocalizedString(::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&presult), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetParentTerm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, pparentterm: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentTerm(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparentterm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMnemonicString(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmnemonic: ::std::mem::MaybeUninit<::windows_core::BSTR>, pterm: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupMnemonicTerm(::core::mem::transmute(&bstrmnemonic)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, lcid: u32, varvalue: super::super::System::Com::VARIANT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plcid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConvertValueToString(::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute(&varvalue), ::core::mem::transmute_copy(&pbstrresult), ::core::mem::transmute_copy(&plcid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalizedString: GetLocalizedString::<Identity, Impl, OFFSET>,
            GetParentTerm: GetParentTerm::<Identity, Impl, OFFSET>,
            GetMnemonicString: GetMnemonicString::<Identity, Impl, OFFSET>,
            LookupMnemonicTerm: LookupMnemonicTerm::<Identity, Impl, OFFSET>,
            ConvertValueToString: ConvertValueToString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccDictionary as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IAccServerDocMgr_Impl: Sized {
    fn NewDocument(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RevokeDocument(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnDocumentFocus(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAccServerDocMgr {}
impl IAccServerDocMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: isize>() -> IAccServerDocMgr_Vtbl {
        unsafe extern "system" fn NewDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewDocument(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RevokeDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeDocument(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDocumentFocus(::windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewDocument: NewDocument::<Identity, Impl, OFFSET>,
            RevokeDocument: RevokeDocument::<Identity, Impl, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccServerDocMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccStore_Impl: Sized {
    fn Register(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Unregister(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetDocuments(&self) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnDocumentFocus(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetFocused(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IAccStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>() -> IAccStore_Vtbl {
        unsafe extern "system" fn Register<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Register(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unregister(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupByPoint(::core::mem::transmute(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDocumentFocus(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetFocused<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocused(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, Impl, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, Impl, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, Impl, OFFSET>,
            GetFocused: GetFocused::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAnchor_Impl: Sized {
    fn SetGravity(&self, gravity: TsGravity) -> ::windows_core::Result<()>;
    fn GetGravity(&self) -> ::windows_core::Result<TsGravity>;
    fn IsEqual(&self, pawith: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Compare(&self, pawith: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<i32>;
    fn Shift(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn ShiftTo(&self, pasite: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetChangeHistoryMask(&self, dwmask: u32) -> ::windows_core::Result<()>;
    fn GetChangeHistory(&self) -> ::windows_core::Result<ANCHOR_CHANGE_HISTORY_FLAGS>;
    fn ClearChangeHistory(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IAnchor>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IAnchor {}
#[cfg(feature = "Win32_Foundation")]
impl IAnchor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>() -> IAnchor_Vtbl {
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGravity(::core::mem::transmute_copy(&gravity)).into()
        }
        unsafe extern "system" fn GetGravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGravity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgravity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::windows_core::from_raw_borrowed(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, plresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Compare(::windows_core::from_raw_borrowed(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shift(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::windows_core::from_raw_borrowed(&pahaltanchor)).into()
        }
        unsafe extern "system" fn ShiftTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftTo(::windows_core::from_raw_borrowed(&pasite)).into()
        }
        unsafe extern "system" fn ShiftRegion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShiftRegion(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChangeHistoryMask(::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetChangeHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChangeHistory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhistory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearChangeHistory().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaclone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            Shift: Shift::<Identity, Impl, OFFSET>,
            ShiftTo: ShiftTo::<Identity, Impl, OFFSET>,
            ShiftRegion: ShiftRegion::<Identity, Impl, OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Identity, Impl, OFFSET>,
            GetChangeHistory: GetChangeHistory::<Identity, Impl, OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAnchor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IClonableWrapper_Impl: Sized {
    fn CloneNewWrapper(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IClonableWrapper {}
impl IClonableWrapper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClonableWrapper_Impl, const OFFSET: isize>() -> IClonableWrapper_Vtbl {
        unsafe extern "system" fn CloneNewWrapper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IClonableWrapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloneNewWrapper(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CloneNewWrapper: CloneNewWrapper::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IClonableWrapper as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICoCreateLocally_Impl: Sized {
    fn CoCreateLocally(&self, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, punk: *mut ::core::option::Option<::windows_core::IUnknown>, riidparam: *const ::windows_core::GUID, punkparam: ::core::option::Option<&::windows_core::IUnknown>, varparam: &super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICoCreateLocally {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreateLocally_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoCreateLocally_Impl, const OFFSET: isize>() -> ICoCreateLocally_Vtbl {
        unsafe extern "system" fn CoCreateLocally<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoCreateLocally_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows_core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CoCreateLocally(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&punk), ::core::mem::transmute_copy(&riidparam), ::windows_core::from_raw_borrowed(&punkparam), ::core::mem::transmute(&varparam)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CoCreateLocally: CoCreateLocally::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICoCreateLocally as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICoCreatedLocally_Impl: Sized {
    fn LocalInit(&self, punklocalobject: ::core::option::Option<&::windows_core::IUnknown>, riidparam: *const ::windows_core::GUID, punkparam: ::core::option::Option<&::windows_core::IUnknown>, varparam: &super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ICoCreatedLocally {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreatedLocally_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoCreatedLocally_Impl, const OFFSET: isize>() -> ICoCreatedLocally_Vtbl {
        unsafe extern "system" fn LocalInit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoCreatedLocally_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows_core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LocalInit(::windows_core::from_raw_borrowed(&punklocalobject), ::core::mem::transmute_copy(&riidparam), ::windows_core::from_raw_borrowed(&punkparam), ::core::mem::transmute(&varparam)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LocalInit: LocalInit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICoCreatedLocally as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IDocWrap_Impl: Sized {
    fn SetDoc(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetWrappedDoc(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IDocWrap {}
impl IDocWrap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: isize>() -> IDocWrap_Vtbl {
        unsafe extern "system" fn SetDoc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDoc(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetWrappedDoc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWrappedDoc(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDoc: SetDoc::<Identity, Impl, OFFSET>,
            GetWrappedDoc: GetWrappedDoc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDocWrap as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumITfCompositionView_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn Next(&self, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumITfCompositionView {}
impl IEnumITfCompositionView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>() -> IEnumITfCompositionView_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcompositionview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumITfCompositionView as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumSpeechCommands_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumSpeechCommands>;
    fn Next(&self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumSpeechCommands {}
impl IEnumSpeechCommands_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>() -> IEnumSpeechCommands_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pspcmds), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumSpeechCommands as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfCandidates_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfCandidates>;
    fn Next(&self, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfCandidates {}
impl IEnumTfCandidates_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: isize>() -> IEnumTfCandidates_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcand), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfCandidates as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfContextViews_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfContextViews>;
    fn Next(&self, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfContextViews {}
impl IEnumTfContextViews_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: isize>() -> IEnumTfContextViews_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgviews), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfContextViews as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfContexts_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfContexts>;
    fn Next(&self, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfContexts {}
impl IEnumTfContexts_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: isize>() -> IEnumTfContexts_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcontext), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfContexts as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfDisplayAttributeInfo_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&self, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfDisplayAttributeInfo {}
impl IEnumTfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> IEnumTfDisplayAttributeInfo_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfDisplayAttributeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfDocumentMgrs_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn Next(&self, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfDocumentMgrs {}
impl IEnumTfDocumentMgrs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>() -> IEnumTfDocumentMgrs_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgdocumentmgr), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfDocumentMgrs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfFunctionProviders_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn Next(&self, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfFunctionProviders {}
impl IEnumTfFunctionProviders_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>() -> IEnumTfFunctionProviders_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut ::core::ffi::c_void, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcmdobj), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfFunctionProviders as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfInputProcessorProfiles_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfInputProcessorProfiles {}
impl IEnumTfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>() -> IEnumTfInputProcessorProfiles_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfInputProcessorProfiles as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfLangBarItems_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfLangBarItems>;
    fn Next(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfLangBarItems {}
impl IEnumTfLangBarItems_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>() -> IEnumTfLangBarItems_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfLangBarItems as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTfLanguageProfiles_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfLanguageProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnumTfLanguageProfiles {}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTfLanguageProfiles_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>() -> IEnumTfLanguageProfiles_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfLanguageProfiles as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfLatticeElements_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfLatticeElements>;
    fn Next(&self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfLatticeElements {}
impl IEnumTfLatticeElements_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>() -> IEnumTfLatticeElements_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgselements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfLatticeElements as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfProperties_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfProperties>;
    fn Next(&self, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfProperties {}
impl IEnumTfProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: isize>() -> IEnumTfProperties_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppprop), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumTfPropertyValue_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfPropertyValue>;
    fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IEnumTfPropertyValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumTfPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>() -> IEnumTfPropertyValue_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgvalues), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfPropertyValue as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfRanges_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfRanges>;
    fn Next(&self, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfRanges {}
impl IEnumTfRanges_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: isize>() -> IEnumTfRanges_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfRanges as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IEnumTfUIElements_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<IEnumTfUIElements>;
    fn Next(&self, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumTfUIElements {}
impl IEnumTfUIElements_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: isize>() -> IEnumTfUIElements_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppelement), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumTfUIElements as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IInternalDocWrap_Impl: Sized {
    fn NotifyRevoke(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IInternalDocWrap {}
impl IInternalDocWrap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternalDocWrap_Impl, const OFFSET: isize>() -> IInternalDocWrap_Vtbl {
        unsafe extern "system" fn NotifyRevoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInternalDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyRevoke().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyRevoke: NotifyRevoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInternalDocWrap as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ISpeechCommandProvider_Impl: Sized {
    fn EnumSpeechCommands(&self, langid: u16) -> ::windows_core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(&self, pszcommand: &::windows_core::PCWSTR, cch: u32, langid: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpeechCommandProvider {}
impl ISpeechCommandProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: isize>() -> ISpeechCommandProvider_Vtbl {
        unsafe extern "system" fn EnumSpeechCommands<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumSpeechCommands(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcommand: ::windows_core::PCWSTR, cch: u32, langid: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessCommand(::core::mem::transmute(&pszcommand), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&langid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumSpeechCommands: EnumSpeechCommands::<Identity, Impl, OFFSET>,
            ProcessCommand: ProcessCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechCommandProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreACP_Impl: Sized {
    fn AdviseSink(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(&self) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetEndACP(&self) -> ::windows_core::Result<i32>;
    fn GetActiveView(&self) -> ::windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> ::windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITextStoreACP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>() -> ITextStoreACP_Vtbl {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseSink(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseSink(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWnd(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetEndACP: GetEndACP::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACP as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreACP2_Impl: Sized {
    fn AdviseSink(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(&self) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetEndACP(&self) -> ::windows_core::Result<i32>;
    fn GetActiveView(&self) -> ::windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITextStoreACP2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>() -> ITextStoreACP2_Vtbl {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseSink(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseSink(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetEndACP: GetEndACP::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACP2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreACPEx_Impl: Sized {
    fn ScrollToRect(&self, acpstart: i32, acpend: i32, rc: &super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITextStoreACPEx {}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreACPEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPEx_Impl, const OFFSET: isize>() -> ITextStoreACPEx_Vtbl {
        unsafe extern "system" fn ScrollToRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollToRect(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&rc), ::core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACPEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACPServices_Impl: Sized {
    fn Serialize(&self, pprop: ::core::option::Option<&ITfProperty>, prange: ::core::option::Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Unserialize(&self, pprop: ::core::option::Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ploader: ::core::option::Option<&ITfPersistentPropertyLoaderACP>) -> ::windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: ::core::option::Option<&ITfProperty>) -> ::windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITextStoreACPServices {}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACPServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: isize>() -> ITextStoreACPServices_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::windows_core::from_raw_borrowed(&pprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unserialize(::windows_core::from_raw_borrowed(&pprop), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceLoadProperty(::windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Unserialize: Unserialize::<Identity, Impl, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, Impl, OFFSET>,
            CreateRange: CreateRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACPServices as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITextStoreACPSink_Impl: Sized {
    fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn OnSelectionChange(&self) -> ::windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextStoreACPSink {}
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>() -> ITextStoreACPSink_Vtbl {
        unsafe extern "system" fn OnTextChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTextChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAttrsChange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndEditTransaction().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, Impl, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, Impl, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITextStoreACPSinkEx_Impl: Sized + ITextStoreACPSink_Impl {
    fn OnDisconnect(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextStoreACPSinkEx {}
impl ITextStoreACPSinkEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSinkEx_Impl, const OFFSET: isize>() -> ITextStoreACPSinkEx_Vtbl {
        unsafe extern "system" fn OnDisconnect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreACPSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnect().into()
        }
        Self { base__: ITextStoreACPSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSinkEx as ::windows_core::ComInterface>::IID || iid == &<ITextStoreACPSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreAnchor_Impl: Sized {
    fn AdviseSink(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(&self) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, pateststart: ::core::option::Option<&IAnchor>, patestend: ::core::option::Option<&IAnchor>, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows_core::Result<()>;
    fn GetText(&self, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pchtext: ::windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn GetFormattedText(&self, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, dwflags: u32, papos: ::core::option::Option<&IAnchor>, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InsertEmbedded(&self, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, papos: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, papos: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(&self, pastart: ::core::option::Option<&IAnchor>, pahalt: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetStart(&self) -> ::windows_core::Result<IAnchor>;
    fn GetEnd(&self) -> ::windows_core::Result<IAnchor>;
    fn GetActiveView(&self) -> ::windows_core::Result<u32>;
    fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<IAnchor>;
    fn GetTextExt(&self, vcview: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn QueryInsertEmbedded(&self, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITextStoreAnchor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>() -> ITextStoreAnchor_Vtbl {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseSink(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseSink(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pateststart: *mut ::core::ffi::c_void, patestend: *mut ::core::ffi::c_void, cch: u32, pparesultstart: *mut *mut ::core::ffi::c_void, pparesultend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInsert(::windows_core::from_raw_borrowed(&pateststart), ::windows_core::from_raw_borrowed(&patestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pparesultstart), ::core::mem::transmute_copy(&pparesultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&fupdateanchor)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormattedText(::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, papos: *mut ::core::ffi::c_void, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbedded(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsAtPosition(::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAttrsTransitioningAtPosition(::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, pahalt: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindNextAttrTransition(::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&pahalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppastart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnchorFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextExt(::core::mem::transmute_copy(&vcview), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWnd(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreAnchorEx_Impl: Sized {
    fn ScrollToRect(&self, pstart: ::core::option::Option<&IAnchor>, pend: ::core::option::Option<&IAnchor>, rc: &super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITextStoreAnchorEx {}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreAnchorEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorEx_Impl, const OFFSET: isize>() -> ITextStoreAnchorEx_Vtbl {
        unsafe extern "system" fn ScrollToRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut ::core::ffi::c_void, pend: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollToRect(::windows_core::from_raw_borrowed(&pstart), ::windows_core::from_raw_borrowed(&pend), ::core::mem::transmute(&rc), ::core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITextStoreAnchorSink_Impl: Sized {
    fn OnTextChange(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn OnSelectionChange(&self) -> ::windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttrsChange(&self, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextStoreAnchorSink {}
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>() -> ITextStoreAnchorSink_Vtbl {
        unsafe extern "system" fn OnTextChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTextChange(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAttrsChange(::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndEditTransaction().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, Impl, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, Impl, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITextStoreSinkAnchorEx_Impl: Sized + ITextStoreAnchorSink_Impl {
    fn OnDisconnect(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextStoreSinkAnchorEx {}
impl ITextStoreSinkAnchorEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreSinkAnchorEx_Impl, const OFFSET: isize>() -> ITextStoreSinkAnchorEx_Vtbl {
        unsafe extern "system" fn OnDisconnect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextStoreSinkAnchorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnect().into()
        }
        Self { base__: ITextStoreAnchorSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextStoreSinkAnchorEx as ::windows_core::ComInterface>::IID || iid == &<ITextStoreAnchorSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfActiveLanguageProfileNotifySink_Impl: Sized {
    fn OnActivated(&self, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfActiveLanguageProfileNotifySink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfActiveLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> ITfActiveLanguageProfileNotifySink_Vtbl {
        unsafe extern "system" fn OnActivated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivated(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&factivated)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCandidateList_Impl: Sized {
    fn EnumCandidates(&self) -> ::windows_core::Result<IEnumTfCandidates>;
    fn GetCandidate(&self, nindex: u32) -> ::windows_core::Result<ITfCandidateString>;
    fn GetCandidateNum(&self) -> ::windows_core::Result<u32>;
    fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfCandidateList {}
impl ITfCandidateList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: isize>() -> ITfCandidateList_Vtbl {
        unsafe extern "system" fn EnumCandidates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCandidates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCandidate(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCandidateNum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncnt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResult(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&imcr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCandidates: EnumCandidates::<Identity, Impl, OFFSET>,
            GetCandidate: GetCandidate::<Identity, Impl, OFFSET>,
            GetCandidateNum: GetCandidateNum::<Identity, Impl, OFFSET>,
            SetResult: SetResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCandidateList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> ::windows_core::Result<u32>;
    fn GetDocumentMgr(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetSelection(&self) -> ::windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows_core::Result<()>;
    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> ::windows_core::Result<()>;
    fn GetCurrentPage(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfCandidateListUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>() -> ITfCandidateListUIElement_Vtbl {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&usize), ::core::mem::transmute_copy(&pupagecnt)).into()
        }
        unsafe extern "system" fn SetPageIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&upagecnt)).into()
        }
        unsafe extern "system" fn GetCurrentPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pupage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, Impl, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetPageIndex: GetPageIndex::<Identity, Impl, OFFSET>,
            SetPageIndex: SetPageIndex::<Identity, Impl, OFFSET>,
            GetCurrentPage: GetCurrentPage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElement as ::windows_core::ComInterface>::IID || iid == &<ITfUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElementBehavior_Impl: Sized + ITfCandidateListUIElement_Impl {
    fn SetSelection(&self, nindex: u32) -> ::windows_core::Result<()>;
    fn Finalize(&self) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfCandidateListUIElementBehavior {}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateListUIElementBehavior_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>() -> ITfCandidateListUIElementBehavior_Vtbl {
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn Finalize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finalize().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: ITfCandidateListUIElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            Finalize: Finalize::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElementBehavior as ::windows_core::ComInterface>::IID || iid == &<ITfUIElement as ::windows_core::ComInterface>::IID || iid == &<ITfCandidateListUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCandidateString_Impl: Sized {
    fn GetString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIndex(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for ITfCandidateString {}
impl ITfCandidateString_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: isize>() -> ITfCandidateString_Vtbl {
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCandidateString as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCategoryMgr_Impl: Sized {
    fn RegisterCategory(&self, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterCategory(&self, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumCategoriesInItem(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn EnumItemsInCategory(&self, rcatid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn FindClosestCategory(&self, rguid: *const ::windows_core::GUID, pcatid: *mut ::windows_core::GUID, ppcatidlist: *const *const ::windows_core::GUID, ulcount: u32) -> ::windows_core::Result<()>;
    fn RegisterGUIDDescription(&self, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn UnregisterGUIDDescription(&self, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGUIDDescription(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterGUIDDWORD(&self, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, dw: u32) -> ::windows_core::Result<()>;
    fn UnregisterGUIDDWORD(&self, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGUIDDWORD(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn RegisterGUID(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn GetGUID(&self, guidatom: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfCategoryMgr {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfCategoryMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>() -> ITfCategoryMgr_Vtbl {
        unsafe extern "system" fn RegisterCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn UnregisterCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCategoriesInItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCategoriesInItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumItemsInCategory(::core::mem::transmute_copy(&rcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pcatid: *mut ::windows_core::GUID, ppcatidlist: *const *const ::windows_core::GUID, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindClosestCategory(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pcatid), ::core::mem::transmute_copy(&ppcatidlist), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn RegisterGUIDDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUIDDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, dw: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDWORD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pdw: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUIDDWORD(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pguidatom: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterGUID(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidatom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUID(::core::mem::transmute_copy(&guidatom)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows_core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqualTfGuidAtom(::core::mem::transmute_copy(&guidatom), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategory: RegisterCategory::<Identity, Impl, OFFSET>,
            UnregisterCategory: UnregisterCategory::<Identity, Impl, OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Identity, Impl, OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Identity, Impl, OFFSET>,
            FindClosestCategory: FindClosestCategory::<Identity, Impl, OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Identity, Impl, OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Identity, Impl, OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Identity, Impl, OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Identity, Impl, OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Identity, Impl, OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Identity, Impl, OFFSET>,
            RegisterGUID: RegisterGUID::<Identity, Impl, OFFSET>,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCategoryMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCleanupContextDurationSink_Impl: Sized {
    fn OnStartCleanupContext(&self) -> ::windows_core::Result<()>;
    fn OnEndCleanupContext(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfCleanupContextDurationSink {}
impl ITfCleanupContextDurationSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>() -> ITfCleanupContextDurationSink_Vtbl {
        unsafe extern "system" fn OnStartCleanupContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStartCleanupContext().into()
        }
        unsafe extern "system" fn OnEndCleanupContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndCleanupContext().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartCleanupContext: OnStartCleanupContext::<Identity, Impl, OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextDurationSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCleanupContextSink_Impl: Sized {
    fn OnCleanupContext(&self, ecwrite: u32, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfCleanupContextSink {}
impl ITfCleanupContextSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCleanupContextSink_Impl, const OFFSET: isize>() -> ITfCleanupContextSink_Vtbl {
        unsafe extern "system" fn OnCleanupContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCleanupContextSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCleanupContext(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pic)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCleanupContext: OnCleanupContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfClientId_Impl: Sized {
    fn GetClientId(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for ITfClientId {}
impl ITfClientId_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfClientId_Impl, const OFFSET: isize>() -> ITfClientId_Vtbl {
        unsafe extern "system" fn GetClientId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfClientId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ptid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientId(::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClientId: GetClientId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfClientId as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfCompartment_Impl: Sized {
    fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn GetValue(&self) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITfCompartment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfCompartment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: isize>() -> ITfCompartment_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCompartment as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCompartmentEventSink_Impl: Sized {
    fn OnChange(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfCompartmentEventSink {}
impl ITfCompartmentEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentEventSink_Impl, const OFFSET: isize>() -> ITfCompartmentEventSink_Vtbl {
        unsafe extern "system" fn OnChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChange(::core::mem::transmute_copy(&rguid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCompartmentEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCompartmentMgr_Impl: Sized {
    fn GetCompartment(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfCompartment>;
    fn ClearCompartment(&self, tid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumCompartments(&self) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITfCompartmentMgr {}
#[cfg(feature = "Win32_System_Com")]
impl ITfCompartmentMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>() -> ITfCompartmentMgr_Vtbl {
        unsafe extern "system" fn GetCompartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppcomp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCompartment(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearCompartment(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCompartments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCompartments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompartment: GetCompartment::<Identity, Impl, OFFSET>,
            ClearCompartment: ClearCompartment::<Identity, Impl, OFFSET>,
            EnumCompartments: EnumCompartments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCompartmentMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfComposition_Impl: Sized {
    fn GetRange(&self) -> ::windows_core::Result<ITfRange>;
    fn ShiftStart(&self, ecwrite: u32, pnewstart: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn ShiftEnd(&self, ecwrite: u32, pnewend: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn EndComposition(&self, ecwrite: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfComposition {}
impl ITfComposition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: isize>() -> ITfComposition_Vtbl {
        unsafe extern "system" fn GetRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftStart(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pnewstart)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftEnd(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pnewend)).into()
        }
        unsafe extern "system" fn EndComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndComposition(::core::mem::transmute_copy(&ecwrite)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            ShiftStart: ShiftStart::<Identity, Impl, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, Impl, OFFSET>,
            EndComposition: EndComposition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfComposition as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCompositionSink_Impl: Sized {
    fn OnCompositionTerminated(&self, ecwrite: u32, pcomposition: ::core::option::Option<&ITfComposition>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfCompositionSink {}
impl ITfCompositionSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompositionSink_Impl, const OFFSET: isize>() -> ITfCompositionSink_Vtbl {
        unsafe extern "system" fn OnCompositionTerminated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCompositionTerminated(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCompositionSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfCompositionView_Impl: Sized {
    fn GetOwnerClsid(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetRange(&self) -> ::windows_core::Result<ITfRange>;
}
impl ::windows_core::RuntimeName for ITfCompositionView {}
impl ITfCompositionView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: isize>() -> ITfCompositionView_Vtbl {
        unsafe extern "system" fn GetOwnerClsid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwnerClsid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerClsid: GetOwnerClsid::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCompositionView as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfConfigureSystemKeystrokeFeed_Impl: Sized {
    fn DisableSystemKeystrokeFeed(&self) -> ::windows_core::Result<()>;
    fn EnableSystemKeystrokeFeed(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfConfigureSystemKeystrokeFeed {}
impl ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>() -> ITfConfigureSystemKeystrokeFeed_Vtbl {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableSystemKeystrokeFeed().into()
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableSystemKeystrokeFeed().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Identity, Impl, OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfConfigureSystemKeystrokeFeed as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContext_Impl: Sized {
    fn RequestEditSession(&self, tid: u32, pes: ::core::option::Option<&ITfEditSession>, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn InWriteSession(&self, tid: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSelection(&self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(&self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows_core::Result<()>;
    fn GetStart(&self, ec: u32) -> ::windows_core::Result<ITfRange>;
    fn GetEnd(&self, ec: u32) -> ::windows_core::Result<ITfRange>;
    fn GetActiveView(&self) -> ::windows_core::Result<ITfContextView>;
    fn EnumViews(&self) -> ::windows_core::Result<IEnumTfContextViews>;
    fn GetStatus(&self) -> ::windows_core::Result<TS_STATUS>;
    fn GetProperty(&self, guidprop: *const ::windows_core::GUID) -> ::windows_core::Result<ITfProperty>;
    fn GetAppProperty(&self, guidprop: *const ::windows_core::GUID) -> ::windows_core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(&self, prgprop: *const *const ::windows_core::GUID, cprop: u32, prgappprop: *const *const ::windows_core::GUID, cappprop: u32) -> ::windows_core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(&self) -> ::windows_core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(&self, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfRangeBackup>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfContext {}
#[cfg(feature = "Win32_Foundation")]
impl ITfContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>() -> ITfContext_Vtbl {
        unsafe extern "system" fn RequestEditSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pes: *mut ::core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestEditSession(::core::mem::transmute_copy(&tid), ::windows_core::from_raw_borrowed(&pes), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InWriteSession(::core::mem::transmute_copy(&tid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwritesession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStart(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnd(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumViews() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAppProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows_core::GUID, cprop: u32, prgappprop: *const *const ::windows_core::GUID, cappprop: u32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrackProperties(::core::mem::transmute_copy(&prgprop), ::core::mem::transmute_copy(&cprop), ::core::mem::transmute_copy(&prgappprop), ::core::mem::transmute_copy(&cappprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRangeBackup(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestEditSession: RequestEditSession::<Identity, Impl, OFFSET>,
            InWriteSession: InWriteSession::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            EnumViews: EnumViews::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetAppProperty: GetAppProperty::<Identity, Impl, OFFSET>,
            TrackProperties: TrackProperties::<Identity, Impl, OFFSET>,
            EnumProperties: EnumProperties::<Identity, Impl, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfContextComposition_Impl: Sized {
    fn StartComposition(&self, ecwrite: u32, pcompositionrange: ::core::option::Option<&ITfRange>, psink: ::core::option::Option<&ITfCompositionSink>) -> ::windows_core::Result<ITfComposition>;
    fn EnumCompositions(&self) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn FindComposition(&self, ecread: u32, ptestrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&self, ecwrite: u32, pcomposition: ::core::option::Option<&ITfCompositionView>, psink: ::core::option::Option<&ITfCompositionSink>) -> ::windows_core::Result<ITfComposition>;
}
impl ::windows_core::RuntimeName for ITfContextComposition {}
impl ITfContextComposition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: isize>() -> ITfContextComposition_Vtbl {
        unsafe extern "system" fn StartComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartComposition(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcompositionrange), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCompositions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindComposition(::core::mem::transmute_copy(&ecread), ::windows_core::from_raw_borrowed(&ptestrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TakeOwnership(::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcomposition), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartComposition: StartComposition::<Identity, Impl, OFFSET>,
            EnumCompositions: EnumCompositions::<Identity, Impl, OFFSET>,
            FindComposition: FindComposition::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextComposition as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextKeyEventSink_Impl: Sized {
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfContextKeyEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextKeyEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>() -> ITfContextKeyEventSink_Vtbl {
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTestKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, Impl, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextKeyEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfContextOwner_Impl: Sized {
    fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetStatus(&self) -> ::windows_core::Result<TS_STATUS>;
    fn GetWnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(&self, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITfContextOwner {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfContextOwner_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>() -> ITfContextOwner_Vtbl {
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetACPFromPoint(::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextExt(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows_core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttribute(::core::mem::transmute_copy(&rguidattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextOwner as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfContextOwnerCompositionServices_Impl: Sized + ITfContextComposition_Impl {
    fn TerminateComposition(&self, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfContextOwnerCompositionServices {}
impl ITfContextOwnerCompositionServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>() -> ITfContextOwnerCompositionServices_Vtbl {
        unsafe extern "system" fn TerminateComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TerminateComposition(::windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: ITfContextComposition_Vtbl::new::<Identity, Impl, OFFSET>(), TerminateComposition: TerminateComposition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as ::windows_core::ComInterface>::IID || iid == &<ITfContextComposition as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextOwnerCompositionSink_Impl: Sized {
    fn OnStartComposition(&self, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(&self, pcomposition: ::core::option::Option<&ITfCompositionView>, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn OnEndComposition(&self, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfContextOwnerCompositionSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextOwnerCompositionSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>() -> ITfContextOwnerCompositionSink_Vtbl {
        unsafe extern "system" fn OnStartComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnStartComposition(::windows_core::from_raw_borrowed(&pcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfok, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdateComposition(::windows_core::from_raw_borrowed(&pcomposition), ::windows_core::from_raw_borrowed(&prangenew)).into()
        }
        unsafe extern "system" fn OnEndComposition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndComposition(::windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartComposition: OnStartComposition::<Identity, Impl, OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Identity, Impl, OFFSET>,
            OnEndComposition: OnEndComposition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfContextOwnerServices_Impl: Sized {
    fn OnLayoutChange(&self) -> ::windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttributeChange(&self, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Serialize(&self, pprop: ::core::option::Option<&ITfProperty>, prange: ::core::option::Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Unserialize(&self, pprop: ::core::option::Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ploader: ::core::option::Option<&ITfPersistentPropertyLoaderACP>) -> ::windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: ::core::option::Option<&ITfProperty>) -> ::windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITfContextOwnerServices {}
#[cfg(feature = "Win32_System_Com")]
impl ITfContextOwnerServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>() -> ITfContextOwnerServices_Vtbl {
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLayoutChange().into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttributeChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAttributeChange(::core::mem::transmute_copy(&rguidattribute)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::windows_core::from_raw_borrowed(&pprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unserialize(::windows_core::from_raw_borrowed(&pprop), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceLoadProperty(::windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttributeChange: OnAttributeChange::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Unserialize: Unserialize::<Identity, Impl, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, Impl, OFFSET>,
            CreateRange: CreateRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerServices as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextView_Impl: Sized {
    fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<ITfRange>;
    fn GetTextExt(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(&self) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfContextView {}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextView_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: isize>() -> ITfContextView_Vtbl {
        unsafe extern "system" fn GetRangeFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRangeFromPoint(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextExt(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRangeFromPoint: GetRangeFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfContextView as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCreatePropertyStore_Impl: Sized {
    fn IsStoreSerializable(&self, guidprop: *const ::windows_core::GUID, prange: ::core::option::Option<&ITfRange>, ppropstore: ::core::option::Option<&ITfPropertyStore>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(&self, guidprop: *const ::windows_core::GUID, prange: ::core::option::Option<&ITfRange>, cb: u32, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<ITfPropertyStore>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfCreatePropertyStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfCreatePropertyStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: isize>() -> ITfCreatePropertyStore_Vtbl {
        unsafe extern "system" fn IsStoreSerializable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsStoreSerializable(::core::mem::transmute_copy(&guidprop), ::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&ppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfserializable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, prange: *mut ::core::ffi::c_void, cb: u32, pstream: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyStore(::core::mem::transmute_copy(&guidprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&cb), ::windows_core::from_raw_borrowed(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStoreSerializable: IsStoreSerializable::<Identity, Impl, OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfCreatePropertyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfDisplayAttributeInfo_Impl: Sized {
    fn GetGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows_core::Result<()>;
    fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfDisplayAttributeInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ITfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> ITfDisplayAttributeInfo_Vtbl {
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeInfo(::core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn SetAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributeInfo(::core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfDisplayAttributeMgr_Impl: Sized {
    fn OnUpdateInfo(&self) -> ::windows_core::Result<()>;
    fn EnumDisplayAttributeInfo(&self) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const ::windows_core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfDisplayAttributeMgr {}
impl ITfDisplayAttributeMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>() -> ITfDisplayAttributeMgr_Vtbl {
        unsafe extern "system" fn OnUpdateInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdateInfo().into()
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, ppinfo: *mut *mut ::core::ffi::c_void, pclsidowner: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayAttributeInfo(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&ppinfo), ::core::mem::transmute_copy(&pclsidowner)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdateInfo: OnUpdateInfo::<Identity, Impl, OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, Impl, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfDisplayAttributeNotifySink_Impl: Sized {
    fn OnUpdateInfo(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfDisplayAttributeNotifySink {}
impl ITfDisplayAttributeNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>() -> ITfDisplayAttributeNotifySink_Vtbl {
        unsafe extern "system" fn OnUpdateInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdateInfo().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfDisplayAttributeProvider_Impl: Sized {
    fn EnumDisplayAttributeInfo(&self) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfDisplayAttributeInfo>;
}
impl ::windows_core::RuntimeName for ITfDisplayAttributeProvider {}
impl ITfDisplayAttributeProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>() -> ITfDisplayAttributeProvider_Vtbl {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayAttributeInfo(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, Impl, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfDocumentMgr_Impl: Sized {
    fn CreateContext(&self, tidowner: u32, dwflags: u32, punk: ::core::option::Option<&::windows_core::IUnknown>, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows_core::Result<()>;
    fn Push(&self, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn Pop(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetTop(&self) -> ::windows_core::Result<ITfContext>;
    fn GetBase(&self) -> ::windows_core::Result<ITfContext>;
    fn EnumContexts(&self) -> ::windows_core::Result<IEnumTfContexts>;
}
impl ::windows_core::RuntimeName for ITfDocumentMgr {}
impl ITfDocumentMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>() -> ITfDocumentMgr_Vtbl {
        unsafe extern "system" fn CreateContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void, pectextstore: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateContext(::core::mem::transmute_copy(&tidowner), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ppic), ::core::mem::transmute_copy(&pectextstore)).into()
        }
        unsafe extern "system" fn Push<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Push(::windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn Pop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pop(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumContexts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            Push: Push::<Identity, Impl, OFFSET>,
            Pop: Pop::<Identity, Impl, OFFSET>,
            GetTop: GetTop::<Identity, Impl, OFFSET>,
            GetBase: GetBase::<Identity, Impl, OFFSET>,
            EnumContexts: EnumContexts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfDocumentMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfEditRecord_Impl: Sized {
    fn GetSelectionStatus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows_core::GUID, cproperties: u32) -> ::windows_core::Result<IEnumTfRanges>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfEditRecord {}
#[cfg(feature = "Win32_Foundation")]
impl ITfEditRecord_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: isize>() -> ITfEditRecord_Vtbl {
        unsafe extern "system" fn GetSelectionStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchanged, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows_core::GUID, cproperties: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTextAndPropertyUpdates(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&prgproperties), ::core::mem::transmute_copy(&cproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelectionStatus: GetSelectionStatus::<Identity, Impl, OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfEditRecord as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfEditSession_Impl: Sized {
    fn DoEditSession(&self, ec: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfEditSession {}
impl ITfEditSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditSession_Impl, const OFFSET: isize>() -> ITfEditSession_Vtbl {
        unsafe extern "system" fn DoEditSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoEditSession(::core::mem::transmute_copy(&ec)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoEditSession: DoEditSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfEditSession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfEditTransactionSink_Impl: Sized {
    fn OnStartEditTransaction(&self, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(&self, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfEditTransactionSink {}
impl ITfEditTransactionSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: isize>() -> ITfEditTransactionSink_Vtbl {
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStartEditTransaction(::windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndEditTransaction(::windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfEditTransactionSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnAdviseText_Impl: Sized + ITfFunction_Impl {
    fn OnTextUpdate(&self, prange: ::core::option::Option<&ITfRange>, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<()>;
    fn OnLatticeUpdate(&self, prange: ::core::option::Option<&ITfRange>, plattice: ::core::option::Option<&ITfLMLattice>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnAdviseText {}
impl ITfFnAdviseText_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: isize>() -> ITfFnAdviseText_Vtbl {
        unsafe extern "system" fn OnTextUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pchtext: ::windows_core::PCWSTR, cch: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTextUpdate(::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn OnLatticeUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, plattice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLatticeUpdate(::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&plattice)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnTextUpdate: OnTextUpdate::<Identity, Impl, OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnAdviseText as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnBalloon_Impl: Sized {
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnBalloon {}
impl ITfFnBalloon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnBalloon_Impl, const OFFSET: isize>() -> ITfFnBalloon_Vtbl {
        unsafe extern "system" fn UpdateBalloon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateBalloon(::core::mem::transmute_copy(&style), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdateBalloon: UpdateBalloon::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnBalloon as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigure_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnConfigure {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigure_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigure_Impl, const OFFSET: isize>() -> ITfFnConfigure_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), Show: Show::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnConfigure as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterEudc_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnConfigureRegisterEudc {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterEudc_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigureRegisterEudc_Impl, const OFFSET: isize>() -> ITfFnConfigureRegisterEudc_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigureRegisterEudc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), Show: Show::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterEudc as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterWord_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnConfigureRegisterWord {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterWord_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigureRegisterWord_Impl, const OFFSET: isize>() -> ITfFnConfigureRegisterWord_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnConfigureRegisterWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), Show: Show::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterWord as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnCustomSpeechCommand_Impl: Sized + ITfFunction_Impl {
    fn SetSpeechCommandProvider(&self, pspcmdprovider: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnCustomSpeechCommand {}
impl ITfFnCustomSpeechCommand_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnCustomSpeechCommand_Impl, const OFFSET: isize>() -> ITfFnCustomSpeechCommand_Vtbl {
        unsafe extern "system" fn SetSpeechCommandProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnCustomSpeechCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpeechCommandProvider(::windows_core::from_raw_borrowed(&pspcmdprovider)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), SetSpeechCommandProvider: SetSpeechCommandProvider::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnCustomSpeechCommand as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnGetLinguisticAlternates_Impl: Sized + ITfFunction_Impl {
    fn GetAlternates(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
}
impl ::windows_core::RuntimeName for ITfFnGetLinguisticAlternates {}
impl ITfFnGetLinguisticAlternates_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetLinguisticAlternates_Impl, const OFFSET: isize>() -> ITfFnGetLinguisticAlternates_Vtbl {
        unsafe extern "system" fn GetAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetLinguisticAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandidatelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAlternates(::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandidatelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), GetAlternates: GetAlternates::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnGetLinguisticAlternates as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnGetPreferredTouchKeyboardLayout_Impl: Sized + ITfFunction_Impl {
    fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnGetPreferredTouchKeyboardLayout {}
impl ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: isize>() -> ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
        unsafe extern "system" fn GetLayout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLayout(::core::mem::transmute_copy(&ptkblayouttype), ::core::mem::transmute_copy(&pwpreferredlayoutid)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), GetLayout: GetLayout::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnGetPreferredTouchKeyboardLayout as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnGetSAPIObject_Impl: Sized + ITfFunction_Impl {
    fn Get(&self, sobj: TfSapiObject) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for ITfFnGetSAPIObject {}
impl ITfFnGetSAPIObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetSAPIObject_Impl, const OFFSET: isize>() -> ITfFnGetSAPIObject_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnGetSAPIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute_copy(&sobj)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), Get: Get::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnGetSAPIObject as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMInternal_Impl: Sized + ITfFnLMProcessor_Impl {
    fn ProcessLattice(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnLMInternal {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMInternal_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMInternal_Impl, const OFFSET: isize>() -> ITfFnLMInternal_Vtbl {
        unsafe extern "system" fn ProcessLattice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessLattice(::windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ITfFnLMProcessor_Vtbl::new::<Identity, Impl, OFFSET>(), ProcessLattice: ProcessLattice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnLMInternal as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID || iid == &<ITfFnLMProcessor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMProcessor_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn QueryLangID(&self, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn QueryKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InvokeFunc(&self, pic: ::core::option::Option<&ITfContext>, refguidfunc: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnLMProcessor {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMProcessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>() -> ITfFnLMProcessor_Vtbl {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryRange(::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfaccepted)).into()
        }
        unsafe extern "system" fn QueryLangID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryLangID(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccepted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReconversion(::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reconvert(::windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn QueryKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinterested, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)).into()
        }
        unsafe extern "system" fn InvokeFunc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, refguidfunc: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeFunc(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&refguidfunc)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            QueryLangID: QueryLangID::<Identity, Impl, OFFSET>,
            GetReconversion: GetReconversion::<Identity, Impl, OFFSET>,
            Reconvert: Reconvert::<Identity, Impl, OFFSET>,
            QueryKey: QueryKey::<Identity, Impl, OFFSET>,
            InvokeKey: InvokeKey::<Identity, Impl, OFFSET>,
            InvokeFunc: InvokeFunc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnLMProcessor as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLangProfileUtil_Impl: Sized + ITfFunction_Impl {
    fn RegisterActiveProfiles(&self) -> ::windows_core::Result<()>;
    fn IsProfileAvailableForLang(&self, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnLangProfileUtil {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLangProfileUtil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: isize>() -> ITfFnLangProfileUtil_Vtbl {
        unsafe extern "system" fn RegisterActiveProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterActiveProfiles().into()
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsProfileAvailableForLang(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterActiveProfiles: RegisterActiveProfiles::<Identity, Impl, OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnLangProfileUtil as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPlayBack_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Play(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnPlayBack {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnPlayBack_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: isize>() -> ITfFnPlayBack_Vtbl {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryRange(::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfplayable)).into()
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Play(::windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnPlayBack as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnPropertyUIStatus_Impl: Sized + ITfFunction_Impl {
    fn GetStatus(&self, refguidprop: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn SetStatus(&self, refguidprop: *const ::windows_core::GUID, dw: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnPropertyUIStatus {}
impl ITfFnPropertyUIStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>() -> ITfFnPropertyUIStatus_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows_core::GUID, pdw: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus(::core::mem::transmute_copy(&refguidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows_core::GUID, dw: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&refguidprop), ::core::mem::transmute_copy(&dw)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnPropertyUIStatus as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnReconversion_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&self, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReconversion(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnReconversion {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnReconversion_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: isize>() -> ITfFnReconversion_Vtbl {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryRange(::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfconvertable)).into()
        }
        unsafe extern "system" fn GetReconversion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReconversion(::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reconvert(::windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            GetReconversion: GetReconversion::<Identity, Impl, OFFSET>,
            Reconvert: Reconvert::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnReconversion as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFnSearchCandidateProvider_Impl: Sized + ITfFunction_Impl {
    fn GetSearchCandidates(&self, bstrquery: &::windows_core::BSTR, bstrapplicationid: &::windows_core::BSTR) -> ::windows_core::Result<ITfCandidateList>;
    fn SetResult(&self, bstrquery: &::windows_core::BSTR, bstrapplicationid: &::windows_core::BSTR, bstrresult: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfFnSearchCandidateProvider {}
impl ITfFnSearchCandidateProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>() -> ITfFnSearchCandidateProvider_Vtbl {
        unsafe extern "system" fn GetSearchCandidates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSearchCandidates(::core::mem::transmute(&bstrquery), ::core::mem::transmute(&bstrapplicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResult(::core::mem::transmute(&bstrquery), ::core::mem::transmute(&bstrapplicationid), ::core::mem::transmute(&bstrresult)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSearchCandidates: GetSearchCandidates::<Identity, Impl, OFFSET>,
            SetResult: SetResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnSearchCandidateProvider as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnShowHelp_Impl: Sized + ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfFnShowHelp {}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnShowHelp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnShowHelp_Impl, const OFFSET: isize>() -> ITfFnShowHelp_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFnShowHelp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, Impl, OFFSET>(), Show: Show::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFnShowHelp as ::windows_core::ComInterface>::IID || iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFunction_Impl: Sized {
    fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for ITfFunction {}
impl ITfFunction_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunction_Impl, const OFFSET: isize>() -> ITfFunction_Vtbl {
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFunction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfFunctionProvider_Impl: Sized {
    fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFunction(&self, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for ITfFunctionProvider {}
impl ITfFunctionProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: isize>() -> ITfFunctionProvider_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFunction(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfFunctionProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfInputProcessorProfileActivationSink_Impl: Sized {
    fn OnActivated(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, catid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfInputProcessorProfileActivationSink {}
impl ITfInputProcessorProfileActivationSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>() -> ITfInputProcessorProfileActivationSink_Vtbl {
        unsafe extern "system" fn OnActivated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, catid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivated(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileActivationSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfInputProcessorProfileMgr_Impl: Sized {
    fn ActivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeactivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::Result<()>;
    fn EnumProfiles(&self, langid: u16) -> ::windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(&self, rclsid: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RegisterProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32, pchiconfile: &::windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::Result<()>;
    fn UnregisterProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveProfile(&self, catid: *const ::windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfInputProcessorProfileMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfInputProcessorProfileMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>() -> ITfInputProcessorProfileMgr_Vtbl {
        unsafe extern "system" fn ActivateProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeactivateProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeactivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&pprofile)).into()
        }
        unsafe extern "system" fn EnumProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseInputProcessor(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RegisterProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterProfile(
                ::core::mem::transmute_copy(&rclsid),
                ::core::mem::transmute_copy(&langid),
                ::core::mem::transmute_copy(&guidprofile),
                ::core::mem::transmute(&pchdesc),
                ::core::mem::transmute_copy(&cchdesc),
                ::core::mem::transmute(&pchiconfile),
                ::core::mem::transmute_copy(&cchfile),
                ::core::mem::transmute_copy(&uiconindex),
                ::core::mem::transmute_copy(&hklsubstitute),
                ::core::mem::transmute_copy(&dwpreferredlayout),
                ::core::mem::transmute_copy(&benabledbydefault),
                ::core::mem::transmute_copy(&dwflags),
            )
            .into()
        }
        unsafe extern "system" fn UnregisterProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, catid: *const ::windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActiveProfile(::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pprofile)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateProfile: ActivateProfile::<Identity, Impl, OFFSET>,
            DeactivateProfile: DeactivateProfile::<Identity, Impl, OFFSET>,
            GetProfile: GetProfile::<Identity, Impl, OFFSET>,
            EnumProfiles: EnumProfiles::<Identity, Impl, OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Identity, Impl, OFFSET>,
            RegisterProfile: RegisterProfile::<Identity, Impl, OFFSET>,
            UnregisterProfile: UnregisterProfile::<Identity, Impl, OFFSET>,
            GetActiveProfile: GetActiveProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: Sized {
    fn GetSubstituteKeyboardLayout(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<HKL>;
}
impl ::windows_core::RuntimeName for ITfInputProcessorProfileSubstituteLayout {}
impl ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>() -> ITfInputProcessorProfileSubstituteLayout_Vtbl {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, phkl: *mut HKL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubstituteKeyboardLayout(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileSubstituteLayout as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfiles_Impl: Sized {
    fn Register(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Unregister(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AddLanguageProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32, pchiconfile: &::windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows_core::Result<()>;
    fn RemoveLanguageProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumInputProcessorInfo(&self) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows_core::GUID, pclsid: *mut ::windows_core::GUID, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows_core::GUID, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ActivateLanguageProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetActiveLanguageProfile(&self, rclsid: *const ::windows_core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetLanguageProfileDescription(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentLanguage(&self) -> ::windows_core::Result<u16>;
    fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows_core::Result<()>;
    fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows_core::Result<()>;
    fn EnumLanguageProfiles(&self, langid: u16) -> ::windows_core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EnableLanguageProfileByDefault(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SubstituteKeyboardLayout(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, hkl: HKL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfInputProcessorProfiles {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>() -> ITfInputProcessorProfiles_Vtbl {
        unsafe extern "system" fn Register<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Register(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unregister(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn AddLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc), ::core::mem::transmute(&pchiconfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uiconindex)).into()
        }
        unsafe extern "system" fn RemoveLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumInputProcessorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows_core::GUID, pclsid: *mut ::windows_core::GUID, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows_core::GUID, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn ActivateLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActiveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&plangid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pbstrprofile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageProfileDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plangid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeCurrentLanguage(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetLanguageList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLanguageList(::core::mem::transmute_copy(&pplangid), ::core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn EnumLanguageProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumLanguageProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableLanguageProfileByDefault(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, hkl: HKL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubstituteKeyboardLayout(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Identity, Impl, OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Identity, Impl, OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Identity, Impl, OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Identity, Impl, OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Identity, Impl, OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Identity, Impl, OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Identity, Impl, OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Identity, Impl, OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Identity, Impl, OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Identity, Impl, OFFSET>,
            GetLanguageList: GetLanguageList::<Identity, Impl, OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Identity, Impl, OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Identity, Impl, OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Identity, Impl, OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Identity, Impl, OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfiles as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfilesEx_Impl: Sized + ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(&self, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchfile: &::windows_core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfInputProcessorProfilesEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputProcessorProfilesEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>() -> ITfInputProcessorProfilesEx_Vtbl {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchfile: ::windows_core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguageProfileDisplayName(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute(&pchfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uresid)).into()
        }
        Self {
            base__: ITfInputProcessorProfiles_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as ::windows_core::ComInterface>::IID || iid == &<ITfInputProcessorProfiles as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfInputScope_Impl: Sized {
    fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows_core::Result<()>;
    fn GetPhrase(&self, ppbstrphrases: *mut *mut ::windows_core::BSTR, pccount: *mut u32) -> ::windows_core::Result<()>;
    fn GetRegularExpression(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSRGS(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXML(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for ITfInputScope {}
impl ITfInputScope_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>() -> ITfInputScope_Vtbl {
        unsafe extern "system" fn GetInputScopes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputScopes(::core::mem::transmute_copy(&pprginputscopes), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetPhrase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut ::windows_core::BSTR, pccount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPhrase(::core::mem::transmute_copy(&ppbstrphrases), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetRegularExpression<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrregexp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegularExpression() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrregexp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSRGS() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsrgs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputScopes: GetInputScopes::<Identity, Impl, OFFSET>,
            GetPhrase: GetPhrase::<Identity, Impl, OFFSET>,
            GetRegularExpression: GetRegularExpression::<Identity, Impl, OFFSET>,
            GetSRGS: GetSRGS::<Identity, Impl, OFFSET>,
            GetXML: GetXML::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputScope as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInputScope2_Impl: Sized + ITfInputScope_Impl {
    fn EnumWordList(&self) -> ::windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITfInputScope2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITfInputScope2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope2_Impl, const OFFSET: isize>() -> ITfInputScope2_Vtbl {
        unsafe extern "system" fn EnumWordList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInputScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstring: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumWordList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITfInputScope_Vtbl::new::<Identity, Impl, OFFSET>(), EnumWordList: EnumWordList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInputScope2 as ::windows_core::ComInterface>::IID || iid == &<ITfInputScope as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInsertAtSelection_Impl: Sized {
    fn InsertTextAtSelection(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&self, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<ITfRange>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITfInsertAtSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ITfInsertAtSelection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: isize>() -> ITfInsertAtSelection_Vtbl {
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: ::windows_core::PCWSTR, cch: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertTextAtSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertEmbeddedAtSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfInsertAtSelection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfIntegratableCandidateListUIElement_Impl: Sized {
    fn SetIntegrationStyle(&self, guidintegrationstyle: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetSelectionStyle(&self) -> ::windows_core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfIntegratableCandidateListUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfIntegratableCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>() -> ITfIntegratableCandidateListUIElement_Vtbl {
        unsafe extern "system" fn SetIntegrationStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIntegrationStyle(::core::mem::transmute(&guidintegrationstyle)).into()
        }
        unsafe extern "system" fn GetSelectionStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectionStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptfselectionstyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShowCandidateNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalizeExactCompositionString().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIntegrationStyle: SetIntegrationStyle::<Identity, Impl, OFFSET>,
            GetSelectionStyle: GetSelectionStyle::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            ShowCandidateNumbers: ShowCandidateNumbers::<Identity, Impl, OFFSET>,
            FinalizeExactCompositionString: FinalizeExactCompositionString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfIntegratableCandidateListUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyEventSink_Impl: Sized {
    fn OnSetFocus(&self, fforeground: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTestKeyDown(&self, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(&self, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(&self, pic: ::core::option::Option<&ITfContext>, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfKeyEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>() -> ITfKeyEventSink_Vtbl {
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetFocus(::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTestKeyDown(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTestKeyUp(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnKeyDown(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnKeyUp(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnPreservedKey(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, Impl, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnPreservedKey: OnPreservedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfKeyEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyTraceEventSink_Impl: Sized {
    fn OnKeyTraceDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn OnKeyTraceUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfKeyTraceEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyTraceEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: isize>() -> ITfKeyTraceEventSink_Vtbl {
        unsafe extern "system" fn OnKeyTraceDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKeyTraceDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn OnKeyTraceUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKeyTraceUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyTraceDown: OnKeyTraceDown::<Identity, Impl, OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfKeyTraceEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeystrokeMgr_Impl: Sized {
    fn AdviseKeyEventSink(&self, tid: u32, psink: ::core::option::Option<&ITfKeyEventSink>, fforeground: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UnadviseKeyEventSink(&self, tid: u32) -> ::windows_core::Result<()>;
    fn GetForeground(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn TestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn TestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPreservedKey(&self, pic: ::core::option::Option<&ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsPreservedKey(&self, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn PreserveKey(&self, tid: u32, rguid: *const ::windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::Result<()>;
    fn UnpreserveKey(&self, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<()>;
    fn SetPreservedKeyDescription(&self, rguid: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::Result<()>;
    fn GetPreservedKeyDescription(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SimulatePreservedKey(&self, pic: ::core::option::Option<&ITfContext>, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfKeystrokeMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeystrokeMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>() -> ITfKeystrokeMgr_Vtbl {
        unsafe extern "system" fn AdviseKeyEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, psink: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseKeyEventSink(::core::mem::transmute_copy(&tid), ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseKeyEventSink(::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn GetForeground<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetForeground() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TestKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreservedKey(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPreservedKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfregistered, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: ::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreserveKey(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&prekey), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn UnpreserveKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnpreserveKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)).into()
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreservedKeyDescription(::core::mem::transmute_copy(&rguid), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreservedKeyDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SimulatePreservedKey(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseKeyEventSink: AdviseKeyEventSink::<Identity, Impl, OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Identity, Impl, OFFSET>,
            GetForeground: GetForeground::<Identity, Impl, OFFSET>,
            TestKeyDown: TestKeyDown::<Identity, Impl, OFFSET>,
            TestKeyUp: TestKeyUp::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            GetPreservedKey: GetPreservedKey::<Identity, Impl, OFFSET>,
            IsPreservedKey: IsPreservedKey::<Identity, Impl, OFFSET>,
            PreserveKey: PreserveKey::<Identity, Impl, OFFSET>,
            UnpreserveKey: UnpreserveKey::<Identity, Impl, OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Identity, Impl, OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Identity, Impl, OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfKeystrokeMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLMLattice_Impl: Sized {
    fn QueryType(&self, rguidtype: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumTfLatticeElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLMLattice {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLMLattice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: isize>() -> ITfLMLattice_Vtbl {
        unsafe extern "system" fn QueryType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows_core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryType(::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumLatticeElements(::core::mem::transmute_copy(&dwframestart), ::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryType: QueryType::<Identity, Impl, OFFSET>,
            EnumLatticeElements: EnumLatticeElements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLMLattice as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarEventSink_Impl: Sized {
    fn OnSetFocus(&self, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnThreadTerminate(&self, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnThreadItemChange(&self, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnModalInput(&self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLangBarEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>() -> ITfLangBarEventSink_Vtbl {
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetFocus(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadTerminate(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadItemChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadItemChange(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnModalInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnModalInput(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFloatingRect(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, Impl, OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Identity, Impl, OFFSET>,
            OnModalInput: OnModalInput::<Identity, Impl, OFFSET>,
            ShowFloating: ShowFloating::<Identity, Impl, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItem_Impl: Sized {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows_core::Result<()>;
    fn GetStatus(&self) -> ::windows_core::Result<u32>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetTooltipString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLangBarItem {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: isize>() -> ITfLangBarItem_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetTooltipString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTooltipString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtooltip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            GetTooltipString: GetTooltipString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemBalloon_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(&self) -> ::windows_core::Result<TF_LBBALLOONINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLangBarItemBalloon {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemBalloon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>() -> ITfLangBarItemBalloon_Vtbl {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBalloonInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBalloon as ::windows_core::ComInterface>::IID || iid == &<ITfLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmap_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for ITfLangBarItemBitmap {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>() -> ITfLangBarItemBitmap_Vtbl {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmap as ::windows_core::ComInterface>::IID || iid == &<ITfLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmapButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn InitMenu(&self, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> ::windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
    fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::RuntimeName for ITfLangBarItemBitmapButton {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmapButton_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>() -> ITfLangBarItemBitmapButton_Vtbl {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitMenu(::windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmapButton as ::windows_core::ComInterface>::IID || iid == &<ITfLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfLangBarItemButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn InitMenu(&self, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> ::windows_core::Result<()>;
    fn GetIcon(&self) -> ::windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for ITfLangBarItemButton {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfLangBarItemButton_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>() -> ITfLangBarItemButton_Vtbl {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitMenu(::windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemButton as ::windows_core::ComInterface>::IID || iid == &<ITfLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemMgr_Impl: Sized {
    fn EnumItems(&self) -> ::windows_core::Result<IEnumTfLangBarItems>;
    fn GetItem(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfLangBarItem>;
    fn AddItem(&self, punk: ::core::option::Option<&ITfLangBarItem>) -> ::windows_core::Result<()>;
    fn RemoveItem(&self, punk: ::core::option::Option<&ITfLangBarItem>) -> ::windows_core::Result<()>;
    fn AdviseItemSink(&self, punk: ::core::option::Option<&ITfLangBarItemSink>, pdwcookie: *mut u32, rguiditem: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnadviseItemSink(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetItemsStatus(&self, ulcount: u32, prgguid: *const ::windows_core::GUID, pdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetItemNum(&self) -> ::windows_core::Result<u32>;
    fn GetItems(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::Result<()>;
    fn UnadviseItemsSink(&self, ulcount: u32, pdwcookie: *const u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLangBarItemMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>() -> ITfLangBarItemMgr_Vtbl {
        unsafe extern "system" fn EnumItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveItem(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn AdviseItemSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseItemSink(::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&rguiditem)).into()
        }
        unsafe extern "system" fn UnadviseItemSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseItemSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFloatingRect(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows_core::GUID, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItemsStatus(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&prgguid), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetItemNum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemNum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItems(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn AdviseItemsSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const *mut ::core::ffi::c_void, pguiditem: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseItemsSink(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&pguiditem), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseItemsSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseItemsSink(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            AdviseItemSink: AdviseItemSink::<Identity, Impl, OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Identity, Impl, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, Impl, OFFSET>,
            GetItemsStatus: GetItemsStatus::<Identity, Impl, OFFSET>,
            GetItemNum: GetItemNum::<Identity, Impl, OFFSET>,
            GetItems: GetItems::<Identity, Impl, OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Identity, Impl, OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfLangBarItemSink_Impl: Sized {
    fn OnUpdate(&self, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfLangBarItemSink {}
impl ITfLangBarItemSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemSink_Impl, const OFFSET: isize>() -> ITfLangBarItemSink_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdate(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdate: OnUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarMgr_Impl: Sized {
    fn AdviseEventSink(&self, psink: ::core::option::Option<&ITfLangBarEventSink>, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows_core::Result<()>;
    fn UnadviseEventSink(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows_core::Result<()>;
    fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows_core::Result<()>;
    fn RestoreLastFocus(&self, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetModalInput(&self, psink: ::core::option::Option<&ITfLangBarEventSink>, dwthreadid: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetShowFloatingStatus(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLangBarMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>() -> ITfLangBarMgr_Vtbl {
        unsafe extern "system" fn AdviseEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseEventSink(::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseEventSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThreadMarshalInterface(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadLangBarItemMgr(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&pplbi), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputProcessorProfiles(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&ppaip), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn RestoreLastFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreLastFocus(::core::mem::transmute_copy(&pdwthreadid), ::core::mem::transmute_copy(&fprev)).into()
        }
        unsafe extern "system" fn SetModalInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModalInput(::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetShowFloatingStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetShowFloatingStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventSink: AdviseEventSink::<Identity, Impl, OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Identity, Impl, OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Identity, Impl, OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Identity, Impl, OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Identity, Impl, OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Identity, Impl, OFFSET>,
            SetModalInput: SetModalInput::<Identity, Impl, OFFSET>,
            ShowFloating: ShowFloating::<Identity, Impl, OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLangBarMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLanguageProfileNotifySink_Impl: Sized {
    fn OnLanguageChange(&self, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfLanguageProfileNotifySink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> ITfLanguageProfileNotifySink_Vtbl {
        unsafe extern "system" fn OnLanguageChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnLanguageChange(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccept, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLanguageChanged().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLanguageChange: OnLanguageChange::<Identity, Impl, OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfLanguageProfileNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfMSAAControl_Impl: Sized {
    fn SystemEnableMSAA(&self) -> ::windows_core::Result<()>;
    fn SystemDisableMSAA(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfMSAAControl {}
impl ITfMSAAControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: isize>() -> ITfMSAAControl_Vtbl {
        unsafe extern "system" fn SystemEnableMSAA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SystemEnableMSAA().into()
        }
        unsafe extern "system" fn SystemDisableMSAA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SystemDisableMSAA().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SystemEnableMSAA: SystemEnableMSAA::<Identity, Impl, OFFSET>,
            SystemDisableMSAA: SystemDisableMSAA::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMSAAControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfMenu_Impl: Sized {
    fn AddMenuItem(&self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: &::windows_core::PCWSTR, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::RuntimeName for ITfMenu {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfMenu_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMenu_Impl, const OFFSET: isize>() -> ITfMenu_Vtbl {
        unsafe extern "system" fn AddMenuItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: ::windows_core::PCWSTR, cch: u32, ppmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMenuItem(::core::mem::transmute_copy(&uid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&hbmpmask), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppmenu)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddMenuItem: AddMenuItem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMenu as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfMessagePump_Impl: Sized {
    fn PeekMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PeekMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for ITfMessagePump {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfMessagePump_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: isize>() -> ITfMessagePump_Vtbl {
        unsafe extern "system" fn PeekMessageA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PeekMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn PeekMessageW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PeekMessageW(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessageW(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PeekMessageA: PeekMessageA::<Identity, Impl, OFFSET>,
            GetMessageA: GetMessageA::<Identity, Impl, OFFSET>,
            PeekMessageW: PeekMessageW::<Identity, Impl, OFFSET>,
            GetMessageW: GetMessageW::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMessagePump as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfMouseSink_Impl: Sized {
    fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfMouseSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfMouseSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseSink_Impl, const OFFSET: isize>() -> ITfMouseSink_Vtbl {
        unsafe extern "system" fn OnMouseEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnMouseEvent(::core::mem::transmute_copy(&uedge), ::core::mem::transmute_copy(&uquadrant), ::core::mem::transmute_copy(&dwbtnstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMouseEvent: OnMouseEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMouseSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfMouseTracker_Impl: Sized {
    fn AdviseMouseSink(&self, range: ::core::option::Option<&ITfRange>, psink: ::core::option::Option<&ITfMouseSink>) -> ::windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfMouseTracker {}
impl ITfMouseTracker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: isize>() -> ITfMouseTracker_Vtbl {
        unsafe extern "system" fn AdviseMouseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdviseMouseSink(::windows_core::from_raw_borrowed(&range), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseMouseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, Impl, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMouseTracker as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfMouseTrackerACP_Impl: Sized {
    fn AdviseMouseSink(&self, range: ::core::option::Option<&ITfRangeACP>, psink: ::core::option::Option<&ITfMouseSink>) -> ::windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfMouseTrackerACP {}
impl ITfMouseTrackerACP_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: isize>() -> ITfMouseTrackerACP_Vtbl {
        unsafe extern "system" fn AdviseMouseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdviseMouseSink(::windows_core::from_raw_borrowed(&range), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseMouseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, Impl, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfMouseTrackerACP as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPersistentPropertyLoaderACP_Impl: Sized {
    fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ITfPersistentPropertyLoaderACP {}
#[cfg(feature = "Win32_System_Com")]
impl ITfPersistentPropertyLoaderACP_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>() -> ITfPersistentPropertyLoaderACP_Vtbl {
        unsafe extern "system" fn LoadProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadProperty(::core::mem::transmute_copy(&phdr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadProperty: LoadProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfPersistentPropertyLoaderACP as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfPreservedKeyNotifySink_Impl: Sized {
    fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfPreservedKeyNotifySink {}
impl ITfPreservedKeyNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>() -> ITfPreservedKeyNotifySink_Vtbl {
        unsafe extern "system" fn OnUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUpdated(::core::mem::transmute_copy(&pprekey)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdated: OnUpdated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfPreservedKeyNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfProperty_Impl: Sized + ITfReadOnlyProperty_Impl {
    fn FindRange(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn SetValueStore(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, ppropstore: ::core::option::Option<&ITfPropertyStore>) -> ::windows_core::Result<()>;
    fn SetValue(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::Result<()>;
    fn Clear(&self, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITfProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: isize>() -> ITfProperty_Vtbl {
        unsafe extern "system" fn FindRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindRange(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn SetValueStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueStore(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&ppropstore)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfReadOnlyProperty_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindRange: FindRange::<Identity, Impl, OFFSET>,
            SetValueStore: SetValueStore::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfProperty as ::windows_core::ComInterface>::IID || iid == &<ITfReadOnlyProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfPropertyStore_Impl: Sized {
    fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDataType(&self) -> ::windows_core::Result<u32>;
    fn GetData(&self) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn OnTextUpdated(&self, dwflags: u32, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Shrink(&self, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Divide(&self, prangethis: ::core::option::Option<&ITfRange>, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfPropertyStore>;
    fn Clone(&self) -> ::windows_core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Serialize(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITfPropertyStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>() -> ITfPropertyStore_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwreserved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: *mut ::core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTextUpdated(::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccept, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Shrink(::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffree, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangethis: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Divide(::windows_core::from_raw_borrowed(&prangethis), ::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyRangeCreator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Serialize(::windows_core::from_raw_borrowed(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataType: GetDataType::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            OnTextUpdated: OnTextUpdated::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            Divide: Divide::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfPropertyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfQueryEmbedded_Impl: Sized {
    fn QueryInsertEmbedded(&self, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfQueryEmbedded {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfQueryEmbedded_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfQueryEmbedded_Impl, const OFFSET: isize>() -> ITfQueryEmbedded_Vtbl {
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfQueryEmbedded_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfQueryEmbedded as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRange_Impl: Sized {
    fn GetText(&self, ec: u32, dwflags: u32, pchtext: ::windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows_core::Result<()>;
    fn SetText(&self, ec: u32, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<()>;
    fn GetFormattedText(&self, ec: u32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InsertEmbedded(&self, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::Result<()>;
    fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::Result<()>;
    fn ShiftStartToRange(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn ShiftEndToRange(&self, ec: u32, prange: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEmpty(&self, ec: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn IsEqualStart(&self, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEqualEnd(&self, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareStart(&self, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<i32>;
    fn CompareEnd(&self, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<i32>;
    fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows_core::Result<()>;
    fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<ITfRange>;
    fn GetContext(&self) -> ::windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfRange {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfRange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>() -> ITfRange_Vtbl {
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchmax), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormattedText(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn ShiftStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftStartToRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftStartToRange(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftEndToRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShiftEndToRange(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftStartRegion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShiftStartRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShiftEndRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEmpty(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfempty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collapse(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn IsEqualStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqualStart(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqualEnd(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareStart(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareEnd(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdjustForInsert(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertok, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGravity(::core::mem::transmute_copy(&pgstart), ::core::mem::transmute_copy(&pgend)).into()
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGravity(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&gstart), ::core::mem::transmute_copy(&gend)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            ShiftStart: ShiftStart::<Identity, Impl, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, Impl, OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Identity, Impl, OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Identity, Impl, OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Identity, Impl, OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Identity, Impl, OFFSET>,
            IsEmpty: IsEmpty::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            IsEqualStart: IsEqualStart::<Identity, Impl, OFFSET>,
            IsEqualEnd: IsEqualEnd::<Identity, Impl, OFFSET>,
            CompareStart: CompareStart::<Identity, Impl, OFFSET>,
            CompareEnd: CompareEnd::<Identity, Impl, OFFSET>,
            AdjustForInsert: AdjustForInsert::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfRange as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRangeACP_Impl: Sized + ITfRange_Impl {
    fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows_core::Result<()>;
    fn SetExtent(&self, acpanchor: i32, cch: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ITfRangeACP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfRangeACP_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: isize>() -> ITfRangeACP_Vtbl {
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExtent(::core::mem::transmute_copy(&pacpanchor), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetExtent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtent(::core::mem::transmute_copy(&acpanchor), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: ITfRange_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetExtent: GetExtent::<Identity, Impl, OFFSET>,
            SetExtent: SetExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfRangeACP as ::windows_core::ComInterface>::IID || iid == &<ITfRange as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfRangeBackup_Impl: Sized {
    fn Restore(&self, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfRangeBackup {}
impl ITfRangeBackup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRangeBackup_Impl, const OFFSET: isize>() -> ITfRangeBackup_Vtbl {
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfRangeBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Restore: Restore::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfRangeBackup as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfReadOnlyProperty_Impl: Sized {
    fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn EnumRanges(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn GetValue(&self, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::System::Com::VARIANT>;
    fn GetContext(&self) -> ::windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for ITfReadOnlyProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfReadOnlyProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>() -> ITfReadOnlyProperty_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut *mut ::core::ffi::c_void, ptargetrange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumRanges(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppenum), ::windows_core::from_raw_borrowed(&ptargetrange)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            EnumRanges: EnumRanges::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfReadOnlyProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfReadingInformationUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> ::windows_core::Result<u32>;
    fn GetContext(&self) -> ::windows_core::Result<ITfContext>;
    fn GetString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetMaxReadingStringLength(&self) -> ::windows_core::Result<u32>;
    fn GetErrorIndex(&self) -> ::windows_core::Result<u32>;
    fn IsVerticalOrderPreferred(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfReadingInformationUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfReadingInformationUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>() -> ITfReadingInformationUIElement_Vtbl {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxReadingStringLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVerticalOrderPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvertical, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Identity, Impl, OFFSET>,
            GetErrorIndex: GetErrorIndex::<Identity, Impl, OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfReadingInformationUIElement as ::windows_core::ComInterface>::IID || iid == &<ITfUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfReverseConversion_Impl: Sized {
    fn DoReverseConversion(&self, lpstr: &::windows_core::PCWSTR) -> ::windows_core::Result<ITfReverseConversionList>;
}
impl ::windows_core::RuntimeName for ITfReverseConversion {}
impl ITfReverseConversion_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversion_Impl, const OFFSET: isize>() -> ITfReverseConversion_Vtbl {
        unsafe extern "system" fn DoReverseConversion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstr: ::windows_core::PCWSTR, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoReverseConversion(::core::mem::transmute(&lpstr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoReverseConversion: DoReverseConversion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfReverseConversion as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfReverseConversionList_Impl: Sized {
    fn GetLength(&self) -> ::windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for ITfReverseConversionList {}
impl ITfReverseConversionList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: isize>() -> ITfReverseConversionList_Vtbl {
        unsafe extern "system" fn GetLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfReverseConversionMgr_Impl: Sized {
    fn GetReverseConversion(&self, langid: u16, guidprofile: *const ::windows_core::GUID, dwflag: u32) -> ::windows_core::Result<ITfReverseConversion>;
}
impl ::windows_core::RuntimeName for ITfReverseConversionMgr {}
impl ITfReverseConversionMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversionMgr_Impl, const OFFSET: isize>() -> ITfReverseConversionMgr_Vtbl {
        unsafe extern "system" fn GetReverseConversion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfReverseConversionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows_core::GUID, dwflag: u32, ppreverseconversion: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReverseConversion(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreverseconversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetReverseConversion: GetReverseConversion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfSource_Impl: Sized {
    fn AdviseSink(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
    fn UnadviseSink(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfSource {}
impl ITfSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: isize>() -> ITfSource_Vtbl {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdviseSink(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfSourceSingle_Impl: Sized {
    fn AdviseSingleSink(&self, tid: u32, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn UnadviseSingleSink(&self, tid: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfSourceSingle {}
impl ITfSourceSingle_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: isize>() -> ITfSourceSingle_Vtbl {
        unsafe extern "system" fn AdviseSingleSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AdviseSingleSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn UnadviseSingleSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnadviseSingleSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSingleSink: AdviseSingleSink::<Identity, Impl, OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSourceSingle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfSpeechUIServer_Impl: Sized {
    fn Initialize(&self) -> ::windows_core::Result<()>;
    fn ShowUI(&self, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfSpeechUIServer {}
#[cfg(feature = "Win32_Foundation")]
impl ITfSpeechUIServer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>() -> ITfSpeechUIServer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize().into()
        }
        unsafe extern "system" fn ShowUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowUI(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UpdateBalloon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateBalloon(::core::mem::transmute_copy(&style), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ShowUI: ShowUI::<Identity, Impl, OFFSET>,
            UpdateBalloon: UpdateBalloon::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSpeechUIServer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfStatusSink_Impl: Sized {
    fn OnStatusChange(&self, pic: ::core::option::Option<&ITfContext>, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfStatusSink {}
impl ITfStatusSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfStatusSink_Impl, const OFFSET: isize>() -> ITfStatusSink_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfStatusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfStatusSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfSystemDeviceTypeLangBarItem_Impl: Sized {
    fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows_core::Result<()>;
    fn GetIconMode(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for ITfSystemDeviceTypeLangBarItem {}
impl ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>() -> ITfSystemDeviceTypeLangBarItem_Vtbl {
        unsafe extern "system" fn SetIconMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIconMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetIconMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIconMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIconMode: SetIconMode::<Identity, Impl, OFFSET>,
            GetIconMode: GetIconMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSystemDeviceTypeLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfSystemLangBarItem_Impl: Sized {
    fn SetIcon(&self, hicon: super::WindowsAndMessaging::HICON) -> ::windows_core::Result<()>;
    fn SetTooltipString(&self, pchtooltip: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::RuntimeName for ITfSystemLangBarItem {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfSystemLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: isize>() -> ITfSystemLangBarItem_Vtbl {
        unsafe extern "system" fn SetIcon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIcon(::core::mem::transmute_copy(&hicon)).into()
        }
        unsafe extern "system" fn SetTooltipString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchtooltip: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTooltipString(::core::mem::transmute(&pchtooltip), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIcon: SetIcon::<Identity, Impl, OFFSET>,
            SetTooltipString: SetTooltipString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfSystemLangBarItemSink_Impl: Sized {
    fn InitMenu(&self, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfSystemLangBarItemSink {}
impl ITfSystemLangBarItemSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>() -> ITfSystemLangBarItemSink_Vtbl {
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitMenu(::windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfSystemLangBarItemText_Impl: Sized {
    fn SetItemText(&self, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn GetItemText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for ITfSystemLangBarItemText {}
impl ITfSystemLangBarItemText_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: isize>() -> ITfSystemLangBarItemText_Vtbl {
        unsafe extern "system" fn SetItemText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItemText(::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetItemText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetItemText: SetItemText::<Identity, Impl, OFFSET>,
            GetItemText: GetItemText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemText as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfTextEditSink_Impl: Sized {
    fn OnEndEdit(&self, pic: ::core::option::Option<&ITfContext>, ecreadonly: u32, peditrecord: ::core::option::Option<&ITfEditRecord>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfTextEditSink {}
impl ITfTextEditSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextEditSink_Impl, const OFFSET: isize>() -> ITfTextEditSink_Vtbl {
        unsafe extern "system" fn OnEndEdit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextEditSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, peditrecord: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndEdit(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&ecreadonly), ::windows_core::from_raw_borrowed(&peditrecord)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEndEdit: OnEndEdit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTextEditSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfTextInputProcessor_Impl: Sized {
    fn Activate(&self, ptim: ::core::option::Option<&ITfThreadMgr>, tid: u32) -> ::windows_core::Result<()>;
    fn Deactivate(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfTextInputProcessor {}
impl ITfTextInputProcessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: isize>() -> ITfTextInputProcessor_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::windows_core::from_raw_borrowed(&ptim), ::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfTextInputProcessorEx_Impl: Sized + ITfTextInputProcessor_Impl {
    fn ActivateEx(&self, ptim: ::core::option::Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfTextInputProcessorEx {}
impl ITfTextInputProcessorEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextInputProcessorEx_Impl, const OFFSET: isize>() -> ITfTextInputProcessorEx_Vtbl {
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextInputProcessorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateEx(::windows_core::from_raw_borrowed(&ptim), ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ITfTextInputProcessor_Vtbl::new::<Identity, Impl, OFFSET>(), ActivateEx: ActivateEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as ::windows_core::ComInterface>::IID || iid == &<ITfTextInputProcessor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfTextLayoutSink_Impl: Sized {
    fn OnLayoutChange(&self, pic: ::core::option::Option<&ITfContext>, lcode: TfLayoutCode, pview: ::core::option::Option<&ITfContextView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfTextLayoutSink {}
impl ITfTextLayoutSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextLayoutSink_Impl, const OFFSET: isize>() -> ITfTextLayoutSink_Vtbl {
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTextLayoutSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, lcode: TfLayoutCode, pview: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLayoutChange(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&lcode), ::windows_core::from_raw_borrowed(&pview)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfThreadFocusSink_Impl: Sized {
    fn OnSetThreadFocus(&self) -> ::windows_core::Result<()>;
    fn OnKillThreadFocus(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfThreadFocusSink {}
impl ITfThreadFocusSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: isize>() -> ITfThreadFocusSink_Vtbl {
        unsafe extern "system" fn OnSetThreadFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetThreadFocus().into()
        }
        unsafe extern "system" fn OnKillThreadFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKillThreadFocus().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetThreadFocus: OnSetThreadFocus::<Identity, Impl, OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfThreadFocusSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgr_Impl: Sized {
    fn Activate(&self) -> ::windows_core::Result<u32>;
    fn Deactivate(&self) -> ::windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn AssociateFocus(&self, hwnd: super::super::Foundation::HWND, pdimnew: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> ::windows_core::Result<ITfCompartmentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfThreadMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>() -> ITfThreadMgr_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocus(::windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn AssociateFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: *mut ::core::ffi::c_void, ppdimprev: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssociateFocus(::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&pdimnew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimprev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfthreadfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncprov, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlobalCompartment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcompmgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, Impl, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            AssociateFocus: AssociateFocus::<Identity, Impl, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, Impl, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, Impl, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, Impl, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgr2_Impl: Sized {
    fn Activate(&self) -> ::windows_core::Result<u32>;
    fn Deactivate(&self) -> ::windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> ::windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn IsThreadFocus(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> ::windows_core::Result<ITfCompartmentMgr>;
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveFlags(&self) -> ::windows_core::Result<u32>;
    fn SuspendKeystrokeHandling(&self) -> ::windows_core::Result<()>;
    fn ResumeKeystrokeHandling(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfThreadMgr2 {}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgr2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>() -> ITfThreadMgr2_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocus(::windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfthreadfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncprov, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlobalCompartment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcompmgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspendKeystrokeHandling().into()
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeKeystrokeHandling().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, Impl, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, Impl, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, Impl, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, Impl, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, Impl, OFFSET>,
            ActivateEx: ActivateEx::<Identity, Impl, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, Impl, OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Identity, Impl, OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait ITfThreadMgrEventSink_Impl: Sized {
    fn OnInitDocumentMgr(&self, pdim: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnUninitDocumentMgr(&self, pdim: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnSetFocus(&self, pdimfocus: ::core::option::Option<&ITfDocumentMgr>, pdimprevfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnPushContext(&self, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn OnPopContext(&self, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITfThreadMgrEventSink {}
impl ITfThreadMgrEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>() -> ITfThreadMgrEventSink_Vtbl {
        unsafe extern "system" fn OnInitDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInitDocumentMgr(::windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUninitDocumentMgr(::windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void, pdimprevfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetFocus(::windows_core::from_raw_borrowed(&pdimfocus), ::windows_core::from_raw_borrowed(&pdimprevfocus)).into()
        }
        unsafe extern "system" fn OnPushContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPushContext(::windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnPopContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPopContext(::windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInitDocumentMgr: OnInitDocumentMgr::<Identity, Impl, OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Identity, Impl, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnPushContext: OnPushContext::<Identity, Impl, OFFSET>,
            OnPopContext: OnPopContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgrEx_Impl: Sized + ITfThreadMgr_Impl {
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveFlags(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfThreadMgrEx {}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgrEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: isize>() -> ITfThreadMgrEx_Vtbl {
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfThreadMgr_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActivateEx: ActivateEx::<Identity, Impl, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as ::windows_core::ComInterface>::IID || iid == &<ITfThreadMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfToolTipUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfToolTipUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfToolTipUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfToolTipUIElement_Impl, const OFFSET: isize>() -> ITfToolTipUIElement_Vtbl {
        unsafe extern "system" fn GetString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfToolTipUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, Impl, OFFSET>(), GetString: GetString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as ::windows_core::ComInterface>::IID || iid == &<ITfUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionSink_Impl: Sized {
    fn OnTransitoryExtensionUpdated(&self, pic: ::core::option::Option<&ITfContext>, ecreadonly: u32, presultrange: ::core::option::Option<&ITfRange>, pcompositionrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfTransitoryExtensionSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>() -> ITfTransitoryExtensionSink_Vtbl {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, presultrange: *mut ::core::ffi::c_void, pcompositionrange: *mut ::core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnTransitoryExtensionUpdated(::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&ecreadonly), ::windows_core::from_raw_borrowed(&presultrange), ::windows_core::from_raw_borrowed(&pcompositionrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdeleteresultrange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetDocumentMgr(&self) -> ::windows_core::Result<ITfDocumentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfTransitoryExtensionUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>() -> ITfTransitoryExtensionUIElement_Vtbl {
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, Impl, OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as ::windows_core::ComInterface>::IID || iid == &<ITfUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElement_Impl: Sized {
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Show(&self, bshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsShown(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfUIElement {}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: isize>() -> ITfUIElement_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show(::core::mem::transmute_copy(&bshow)).into()
        }
        unsafe extern "system" fn IsShown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsShown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            IsShown: IsShown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfUIElement as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementMgr_Impl: Sized {
    fn BeginUIElement(&self, pelement: ::core::option::Option<&ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn GetUIElement(&self, dwuielementid: u32) -> ::windows_core::Result<ITfUIElement>;
    fn EnumUIElements(&self) -> ::windows_core::Result<IEnumTfUIElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfUIElementMgr {}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>() -> ITfUIElementMgr_Vtbl {
        unsafe extern "system" fn BeginUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUIElement(::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&pbshow), ::core::mem::transmute_copy(&pdwuielementid)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn GetUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUIElement(::core::mem::transmute_copy(&dwuielementid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumUIElements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, Impl, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, Impl, OFFSET>,
            EndUIElement: EndUIElement::<Identity, Impl, OFFSET>,
            GetUIElement: GetUIElement::<Identity, Impl, OFFSET>,
            EnumUIElements: EnumUIElements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfUIElementMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementSink_Impl: Sized {
    fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITfUIElementSink {}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: isize>() -> ITfUIElementSink_Vtbl {
        unsafe extern "system" fn BeginUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUIElement(::core::mem::transmute_copy(&dwuielementid), ::core::mem::transmute_copy(&pbshow)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, Impl, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, Impl, OFFSET>,
            EndUIElement: EndUIElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITfUIElementSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIManagerEventSink_Impl: Sized {
    fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowClosing(&self) -> ::windows_core::Result<()>;
    fn OnWindowClosed(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IUIManagerEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl IUIManagerEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>() -> IUIManagerEventSink_Vtbl {
        unsafe extern "system" fn OnWindowOpening<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowOpening(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowOpened<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowOpened(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdating<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowUpdating(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowUpdated(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowClosing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowClosing().into()
        }
        unsafe extern "system" fn OnWindowClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWindowClosed().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWindowOpening: OnWindowOpening::<Identity, Impl, OFFSET>,
            OnWindowOpened: OnWindowOpened::<Identity, Impl, OFFSET>,
            OnWindowUpdating: OnWindowUpdating::<Identity, Impl, OFFSET>,
            OnWindowUpdated: OnWindowUpdated::<Identity, Impl, OFFSET>,
            OnWindowClosing: OnWindowClosing::<Identity, Impl, OFFSET>,
            OnWindowClosed: OnWindowClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUIManagerEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"implement\"`*"]
pub trait IVersionInfo_Impl: Sized {
    fn GetSubcomponentCount(&self, ulsub: u32) -> ::windows_core::Result<u32>;
    fn GetImplementationID(&self, ulsub: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::Result<()>;
    fn GetComponentDescription(&self, ulsub: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInstanceDescription(&self, ulsub: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IVersionInfo {}
impl IVersionInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>() -> IVersionInfo_Vtbl {
        unsafe extern "system" fn GetSubcomponentCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubcomponentCount(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImplementationID(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(implid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuildVersion(::core::mem::transmute_copy(&ulsub), ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetComponentDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComponentDescription(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimplstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstanceDescription(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimplstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubcomponentCount: GetSubcomponentCount::<Identity, Impl, OFFSET>,
            GetImplementationID: GetImplementationID::<Identity, Impl, OFFSET>,
            GetBuildVersion: GetBuildVersion::<Identity, Impl, OFFSET>,
            GetComponentDescription: GetComponentDescription::<Identity, Impl, OFFSET>,
            GetInstanceDescription: GetInstanceDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVersionInfo as ::windows_core::ComInterface>::IID
    }
}
