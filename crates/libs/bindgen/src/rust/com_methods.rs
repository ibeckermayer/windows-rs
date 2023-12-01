use super::*;

pub fn writer(writer: &Writer, def: TypeDef, kind: InterfaceKind, method: MethodDef, method_names: &mut MethodNames, virtual_names: &mut MethodNames, base_count: usize) -> TokenStream {
    let signature = method_def_signature(def.namespace(), method, &[]);

    let name = method_names.add(method);
    let vname = virtual_names.add(method);
    let generics = writer.constraint_generics(&signature.params);
    let where_clause = writer.where_clause(&signature.params);
    let mut cfg = signature_cfg(method);
    cfg.add_feature(def.namespace());
    let doc = writer.cfg_method_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    if kind == InterfaceKind::None {
        return quote! {};
    }

    let mut bases = quote! {};

    for _ in 0..base_count {
        bases.combine(&quote! { base__. });
    }

    let kind = signature.kind();
    let args = writer.win32_args(&signature.params, kind);
    let params = writer.win32_params(&signature.params, kind);

    match kind {
        SignatureKind::Query(_) => {
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows_core::Result<T> #where_clause {
                    let mut result__ = ::std::ptr::null_mut();
                    ::windows_core::vcall!(self. #bases #vname(#args)).from_abi(result__)
                }
            }
        }
        SignatureKind::QueryOptional(_) => {
            let generics = expand_generics(generics, quote!(T));
            let where_clause = expand_where_clause(where_clause, quote!(T: ::windows_core::ComInterface));

            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows_core::Result<()> #where_clause {
                    ::windows_core::vcall!(self. #bases #vname(#args)).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type = writer.type_name(&return_type);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows_core::Result<#return_type> #where_clause {
                    let mut result__ = ::std::mem::zeroed();
                    ::windows_core::vcall!(self. #bases #vname(#args)).from_abi(result__)
                }
            }
        }
        SignatureKind::ResultVoid => {
            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> ::windows_core::Result<()> #where_clause {
                    ::windows_core::vcall!(self. #bases #vname(#args)).ok()
                }
            }
        }
        SignatureKind::ReturnValue => {
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let is_nullable = type_is_nullable(&return_type);
            let return_type = writer.type_name(&return_type);

            if is_nullable {
                quote! {
                    #doc
                    #features
                    pub unsafe fn #name<#generics>(&self, #params) -> ::windows_core::Result<#return_type> #where_clause {
                        let mut result__ = ::std::mem::zeroed();
                        ::windows_core::vcall!(self. #bases #vname(#args));
                        ::windows_core::from_abi(result__)
                    }
                }
            } else {
                quote! {
                    #doc
                    #features
                    pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                        let mut result__ = ::std::mem::zeroed();
                        ::windows_core::vcall!(self. #bases #vname(#args));
                        ::std::mem::transmute(result__)
                    }
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let return_type = writer.type_name(&signature.return_type);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                    let mut result__: #return_type = ::core::mem::zeroed();
                    ::windows_core::vcall!(self. #bases #vname(&mut result__, #args));
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let return_type = writer.return_sig(&signature);

            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) #return_type #where_clause {
                    ::windows_core::vcall!(self. #bases #vname(#args))
                }
            }
        }
        SignatureKind::ReturnVoid => {
            quote! {
                #doc
                #features
                pub unsafe fn #name<#generics>(&self, #params) #where_clause {
                    ::windows_core::vcall!(self. #bases #vname(#args))
                }
            }
        }
    }
}

pub fn gen_upcall(writer: &Writer, sig: &Signature, inner: TokenStream) -> TokenStream {
    match sig.kind() {
        SignatureKind::ResultValue => {
            let invoke_args = sig.params[..sig.params.len() - 1].iter().map(|param| gen_win32_invoke_arg(writer, param));

            let result = writer.param_name(sig.params[sig.params.len() - 1].def);

            quote! {
                match #inner(#(#invoke_args,)*) {
                    ::core::result::Result::Ok(ok__) => {
                        // use `core::ptr::write` since the result could be uninitialized
                        ::core::ptr::write(#result, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into()
                }
            }
        }
        SignatureKind::Query(_) | SignatureKind::QueryOptional(_) | SignatureKind::ResultVoid => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        }
        SignatureKind::ReturnStruct => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                *result__ = #inner(#(#invoke_args,)*)
            }
        }
        _ => {
            let invoke_args = sig.params.iter().map(|param| gen_win32_invoke_arg(writer, param));

            quote! {
                #inner(#(#invoke_args,)*)
            }
        }
    }
}

fn gen_win32_invoke_arg(writer: &Writer, param: &SignatureParam) -> TokenStream {
    let name = writer.param_name(param.def);

    if param.def.flags().contains(ParamAttributes::In) && type_is_nullable(&param.ty) {
        quote! { ::windows_core::from_raw_borrowed(&#name) }
    } else if (!param.ty.is_pointer() && type_is_nullable(&param.ty)) || (param.def.flags().contains(ParamAttributes::In) && !type_is_primitive(&param.ty)) {
        quote! { ::core::mem::transmute(&#name) }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}
