//!

#![feature(proc_macro, proc_macro_lib)]
#![allow(unused_imports, unused_variables)]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::collections::HashSet as Set;
use std::str::FromStr;
use syn::fold::{self, Fold};
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::LitStr;
use syn::{
    DeriveInput, Expr, FnArg, GenericArgument, Ident, ImplItem, ImplItemMethod, Item, ItemImpl,
    ItemStatic, Pat, PathArguments, PathSegment, Stmt, Type,
};

macro_rules! check_input {
    ($y:expr, $x:expr) => {
        match $x {
            Ok(s) => s,
            Err(e) => panic!(
                "Failed to parse type in global arg annotation '{}'. Specific error: {:?}",
                $y, e
            ),
        }
    };
}

/// Main macro that implements automated clap generation.
///
/// Tag an `impl` block with this attribute of a type. Then
/// call `start()` on the type to handle match parsing.
#[proc_macro_attribute]
pub fn thunderclap(args: TokenStream, input: TokenStream) -> TokenStream {
    let i: ItemImpl = match syn::parse(input.clone()) {
        Ok(input) => input,
        Err(e) => panic!("Error: '{}'", e),
    };

    /* Manually parse any argument pars given to us */
    let args: String = args.to_string();
    let global_args = if args.len() != 0 {
        args.split(',')
            .map(|i| i.trim())
            .map(|i| i.split(':').map(|x| x.trim()).collect::<Vec<&str>>())
            .map(|triple| (triple[0], triple[1], triple[2]))
            .map(|(x, y, z)| {
                (
                    check_input! { x, TokenStream::from_str(&x.replace("\"", "")) },
                    check_input! { y, TokenStream::from_str(y) },
                    z.replace("\"", ""),
                )
            })
            .map(|(x, y, z)| {
                (
                    check_input! { x, syn::parse(x.clone()) },
                    check_input! { y, syn::parse(y.clone()) },
                    z,
                )
            })
            .map(|(x, y, z)| (x, y, String::from(z)))
            .collect::<Vec<(Type, Type, String)>>()
    } else {
        Vec::new()
    };

    let (name, app_token) = match *i.self_ty {
        Type::Path(ref p) => {
            let meh = p.path.segments[0].ident;
            (format!("{}", p.path.segments[0].ident), quote!( #meh ))
        }
        _ => (format!("Unknown App"), quote!()),
    };

    let about = i.attrs
        .iter()
        .map(|x| (x, x.path.segments.first()))
        .filter(|(a, x)| x.is_some())
        .map(|(a, x)| (a, x.unwrap().value().clone()))
        .map(|(a, v)| match &v.ident.to_string().as_str() {
            &"doc" => String::from(
                format!("{}", a.tts)
                    .replace("/", "")
                    .replace("\\", "")
                    .replace("\"", "")
                    .replace("=", "")
                    .trim(),
            ),
            _ => String::from(""),
        })
        .collect::<String>();

    let mut matches: Vec<quote::Tokens> = Vec::new();
    let orignal = quote!(#i);
    let mut app = quote! {
        App::new(#name).about(#about).setting(AppSettings::SubcommandRequired)
    };

    let mut accessors = quote!{};
    let mut data_struct_fields = quote!{};
    let mut init_struct_fields = quote!{};
    let mut global_match_state_matcher = quote!{};

    global_args.iter().for_each(|(name, typed, about)| {
        let (name, name_token) = match name {
            Type::Path(ref p) => {
                let meh = p.path.segments[0].ident;
                (format!("{}", p.path.segments[0].ident), quote!( #meh ))
            }
            _ => (format!("Unknown App"), quote!()),
        };

        let name = format!("{}", name);
        let optional = match typed {
            Type::Path(ref p) => match p.path.segments.first() {
                Some(ps) => match &ps.value().ident.to_string().as_str() {
                    &"Option" => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        };

        let inner = if optional {
            match typed {
                Type::Path(ref p) => match p.path.segments.first() {
                    Some(ps) => match ps.value().arguments {
                        PathArguments::AngleBracketed(ref b) => match b.args.first() {
                            Some(pair) => match pair.value() {
                                GenericArgument::Type(Type::Path(pp)) => {
                                    Some(Type::from(pp.clone()))
                                }
                                _ => None,
                            },
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        };

        accessors = quote! {
            #accessors

            #[allow(unused)]
            fn #name_token ( /* No Parameters */ ) -> #typed {
                unsafe {
                    __THUNDER_DATA_STATIC.as_ref().unwrap().#name_token.as_ref().unwrap().clone()
                }
            }
        };

        data_struct_fields = quote! {
            #data_struct_fields
            #name_token : Option< #typed > ,
        };

        init_struct_fields = quote! {
            #init_struct_fields
            #name_token : None ,
        };

        global_match_state_matcher = if optional {
            let inner = inner.unwrap();
            quote! {
                #global_match_state_matcher
                global_match_states.#name_token = match args.value_of(#name) {
                    Some(v) => Some(Some(v.parse::<#inner>().expect("Failed to parse value. Double check!"))),
                    None => None,
                };
            }
        } else {
            quote! {
                #global_match_state_matcher
                global_match_states.#name_token = Some(args.value_of(#name).unwrap().parse::<#typed>().expect("Failed to parse value. Double check!"));
            }
        };

        app = if optional {
            let long = format!("--{}", name);
            let short = format!("-{}", &name[..1]);
            quote! {
                #app
                .arg(Arg::with_name(#name).long(#long).short(#short).takes_value(true).help(#about))
            }
        } else {
            quote! {
                #app
                .arg(Arg::with_name(#name).takes_value(true).required(true).help(#about))
            }
        };
    });

    for item in &i.items {
        match item {
            &ImplItem::Method(ref i) => {
                let name = LitStr::new(&i.sig.ident.to_string(), i.sig.ident.span);
                let func_id = &i.sig.ident;
                let about = match i.attrs.first() {
                    Some(a) => String::from(
                        format!("{}", a.tts)
                        /* Clean the tokens TODO: Make this not suck */
                        .replace("/", "")
                        .replace("\\", "")
                        .replace("\"", "")
                        .replace("=", "").trim(),
                    ),
                    _ => String::new(),
                };

                let mut arguments = quote!();

                let mut index: usize = 0;
                let args = i.sig
                    .decl
                    .inputs
                    .iter()
                    .fold(quote!{}, |acc, arg| match arg {
                        &FnArg::Captured(ref arg) => match &arg.pat {
                            &Pat::Ident(ref i) => {
                                let name = format!("{}", i.ident);
                                let optional = match arg.ty {
                                    Type::Path(ref p) => match p.path.segments.first() {
                                        Some(ps) => match &ps.value().ident.to_string().as_str() {
                                            &"Option" => true,
                                            _ => false,
                                        },
                                        _ => false,
                                    },
                                    _ => false,
                                };

                                let mmm = if let Some(typed) = match arg.ty {
                                    Type::Path(ref p) => match p.path.segments.first() {
                                        Some(ps) => match optional {
                                            false => Some(arg.ty.clone()),
                                            true => match ps.value().arguments {
                                                PathArguments::AngleBracketed(ref b) => {
                                                    match b.args.first() {
                                                        Some(pair) => match pair.value() {
                                                            GenericArgument::Type(Type::Path(
                                                                pp,
                                                            )) => Some(Type::from(pp.clone())),
                                                            _ => None,
                                                        },
                                                        _ => None,
                                                    }
                                                }
                                                _ => None,
                                            },
                                        },
                                        _ => None,
                                    },
                                    _ => None,
                                } {
                                    if optional {
                                        quote! {
                                            match m.value_of(#name) {
                                                Some(m) => Some(m.parse::<#typed>().unwrap()),
                                                None => None
                                            }
                                        }
                                    } else {
                                        quote! { m.value_of(#name).unwrap().parse::<#typed>().unwrap() }
                                    }
                                } else {
                                    if optional {
                                        quote! { m.value_of(#name) }
                                    } else {
                                        quote! { m.value_of(#name).unwrap() }
                                    }
                                };

                                index += 1;
                                if optional {
                                    arguments = quote! {
                                        #arguments
                                        #mmm
                                    };
                                    quote! { #acc.arg(Arg::with_name(#name)) }
                                } else {
                                    arguments = quote! {
                                        #arguments
                                        #mmm,
                                    };
                                    quote! { #acc.arg(Arg::with_name(#name).required(true)) }
                                }
                            }
                            _ => quote!{ #acc },
                        },
                        _ => quote!{ #acc },
                    });

                app = quote! {
                    #app.subcommand(
                        SubCommand::with_name(#name).about(#about)#args
                    )
                };

                matches.push(quote! { (#name, Some(m)) => #app_token :: #func_id ( #arguments ), });
            }
            _ => {}
        }
    }

    // let mut matchy = quote!{ match args.subcommand() { };
    let mut matchy = quote!{};

    for m in &matches {
        matchy = quote! {
            #matchy
            #m
        };
    }

    matchy = quote! {
        match args.subcommand() {
            #matchy
            _ => { /* We drop errors for now... */ },
        }
    };

    matchy = quote! {
        let mut global_match_states = __ThunderDataStaticStore::new_empty_store();
        #global_match_state_matcher

        unsafe {
            __THUNDER_DATA_STATIC = Some(global_match_states);
        }

        #matchy
    };

    let tokens = quote! {
        #orignal

        /// This block was generated by thunder v0.0.0
        #[allow(unused)]
        impl #app_token {

            /// Starts the CLI parsing and calls whichever function handles the input
            fn start() {
                use clap::{App, SubCommand, Arg, AppSettings};

                let app = #app;
                let args = app.get_matches();
                #matchy
            }

            #accessors
        }

        static mut __THUNDER_DATA_STATIC: Option<__ThunderDataStaticStore> = None;

        /// This block was generated by thunder v0.0.0
        #[allow(unused)]
        #[doc(hidden)]
        struct __ThunderDataStaticStore {
            #data_struct_fields
        }

        #[allow(unused)]
        #[doc(hidden)]
        impl __ThunderDataStaticStore {
            pub fn new_empty_store() -> __ThunderDataStaticStore {
                __ThunderDataStaticStore {
                    #init_struct_fields
                }
            }
        }
    };

    tokens.into()
}
