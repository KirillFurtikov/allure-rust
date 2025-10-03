extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemMod, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn allure_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let block = &input_fn.block;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;

    let title = if attr.is_empty() {
        quote! { #fn_name_str }
    } else {
        let title_lit = parse_macro_input!(attr as LitStr);
        let title_str = title_lit.value();
        quote! { #title_str }
    };

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            allure_rust::start_test_with_context(#title, None, Some(module_path!()));

            let result = std::panic::catch_unwind(|| {
                #block
            });

            let is_err = result.is_err();
            allure_rust::end_test(#title, result);

            if is_err {
                panic!("Test failed");
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn allure_suite(attr: TokenStream, item: TokenStream) -> TokenStream {
    let suite_name = if attr.is_empty() {
        return TokenStream::from(quote! {
            compile_error!("allure_suite requires a suite name");
        });
    } else {
        let suite_lit = parse_macro_input!(attr as LitStr);
        suite_lit.value()
    };

    let input_mod = parse_macro_input!(item as ItemMod);
    let mod_name = &input_mod.ident;
    let vis = &input_mod.vis;
    let attrs = &input_mod.attrs;
    let content = &input_mod.content;

    if let Some((_, items)) = content {
        let modified_items: Vec<_> = items
            .iter()
            .map(|item| {
                if let syn::Item::Fn(func) = item {
                    let has_allure_test = func.attrs.iter().any(|attr| {
                        attr.path()
                            .segments
                            .last()
                            .map(|seg| seg.ident == "allure_test")
                            .unwrap_or(false)
                    });

                    if has_allure_test {
                        let fn_name = &func.sig.ident;
                        let fn_vis = &func.vis;
                        let fn_attrs = &func.attrs;
                        let fn_sig = &func.sig;
                        let fn_block = &func.block;

                        let allure_test_attr = fn_attrs.iter().find(|attr| {
                            attr.path()
                                .segments
                                .last()
                                .map(|seg| seg.ident == "allure_test")
                                .unwrap_or(false)
                        });

                        let test_title = if let Some(attr) = allure_test_attr {
                            if let Ok(lit) = attr.parse_args::<LitStr>() {
                                lit.value()
                            } else {
                                fn_name.to_string()
                            }
                        } else {
                            fn_name.to_string()
                        };

                        let other_attrs: Vec<_> = fn_attrs
                            .iter()
                            .filter(|attr| {
                                let path = attr.path();
                                let last_seg = path.segments.last().map(|seg| &seg.ident);
                                !matches!(
                                    last_seg.map(|i| i.to_string()).as_deref(),
                                    Some("allure_test") | Some("test")
                                )
                            })
                            .collect();

                        return quote! {
                            #(#other_attrs)*
                            #[test]
                            #fn_vis #fn_sig {
                                allure_rust::start_test_with_suite(#test_title, Some(#suite_name));

                                let result = std::panic::catch_unwind(|| {
                                    #fn_block
                                });

                                let is_err = result.is_err();
                                allure_rust::end_test(#test_title, result);

                                if is_err {
                                    panic!("Test failed");
                                }
                            }
                        };
                    }
                }
                quote! { #item }
            })
            .collect();

        let output = quote! {
            #(#attrs)*
            #vis mod #mod_name {
                #(#modified_items)*
            }
        };

        TokenStream::from(output)
    } else {
        TokenStream::from(quote! {
            compile_error!("allure_suite can only be applied to modules with content");
        })
    }
}

#[proc_macro_attribute]
pub fn step(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let block = &input_fn.block;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;

    let title = if attr.is_empty() {
        quote! { #fn_name_str }
    } else {
        let title_lit = parse_macro_input!(attr as LitStr);
        let title_str = title_lit.value();
        quote! { #title_str }
    };

    let params: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    let param_name_str = param_name.to_string();
                    return Some(quote! {
                        allure_rust::models::Parameter {
                            name: #param_name_str.to_string(),
                            value: format!("{:?}", #param_name),
                        }
                    });
                }
            }
            None
        })
        .collect();

    let params_vec = if params.is_empty() {
        quote! { Vec::new() }
    } else {
        quote! { vec![#(#params),*] }
    };

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            let __params = #params_vec;
            allure_rust::start_step_with_params(#title, __params);

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #block
            }));

            let is_err = result.is_err();
            let step_result: std::thread::Result<()> = match &result {
                Ok(_) => Ok(()),
                Err(e) => {
                    let cloned: Box<dyn std::any::Any + Send> = if let Some(s) = e.downcast_ref::<&'static str>() {
                        Box::new(*s)
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        Box::new(s.clone())
                    } else {
                        Box::new("Unknown error")
                    };
                    Err(cloned)
                }
            };
            allure_rust::end_step(&step_result);

            if is_err {
                std::panic::resume_unwind(result.unwrap_err());
            } else {
                result.unwrap()
            }
        }
    };

    TokenStream::from(output)
}
