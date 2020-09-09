//! This crate uses a macro to generate necessary impls for easy use of the `Ref` type within the `ref_clone` package.
//!
//! To use this, prepend a `#[RefAccessors]` to a struct or enum definition.
//! The macro automatically generates a wrapper type for the struct or enum which wraps all values in a `Ref`.
//! This wrapper type may be accessed using the `to_wrapped` method on the trait `RefAccessors`.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;

use quote::format_ident;
use quote::quote;

use syn::*;
use Data::Enum;
use Data::Struct;
use Fields::Named;
use Fields::Unnamed;

use parse::Parser;
use punctuated::Punctuated;
use token::Gt;
use token::Lt;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn RefAccessors(_attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let ast = syn::parse(input.clone()).unwrap();
    let out = impl_ref_accessors(&ast);
    input.extend::<TokenStream>(out.into());
    input
}

/// First TokenStream is the Struct definition (without the outside wrapper). Second TokenStream is the generator of it.
fn gen_named(
    ast: &FieldsNamed,
    ref_path: &impl ToTokens,
    lt: &Lifetime,
    ref_type: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let interior = ast.named.iter().map(|x| {
        let Field { vis, ident, ty, .. } = x;
        let ident = ident.as_ref().unwrap();
        quote! {
            #vis #ident : Ref<#lt, #ty, #ref_type>,
        }
    });
    let struct_def = quote! {
        {
            #(#interior)*
        }
    };
    let interior_gen = ast.named.iter().map(|x| {
        let Field { ident, .. } = x;
        let ident = ident.as_ref().unwrap();

        quote! {
            #ident : {
                unsafe { ::ref_clone::Ref::new(#ident) }
            },
        }
    });
    let match_gen = ast.named.iter().map(|x| {
        let Field { ident, .. } = x;
        let ident = ident.as_ref().unwrap();

        quote! {
            #ident,
        }
    });
    let struct_gen = quote! {
        { #(#match_gen)* } => #ref_path {
            #(#interior_gen)*
        }
    };
    (struct_def, struct_gen)
}

fn gen_unnamed(
    ast: &FieldsUnnamed,
    ref_path: &impl ToTokens,
    lt: &Lifetime,
    ref_type: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let interior = ast.unnamed.iter().map(|x| {
        let Field { vis, ty, .. } = x;
        quote! {
            #vis Ref<#lt, #ty, #ref_type>,
        }
    });
    let struct_def = quote! {
        ( #(#interior)* )
    };
    let interior_gen = ast.unnamed.iter().enumerate().map(|(i, _)| {
        let ident = format_ident!("_{}", i);
        quote! {
            unsafe { ::ref_clone::Ref::new(#ident) }
        }
    });
    let match_gen = ast.unnamed.iter().enumerate().map(|(i, _)| {
        let ident = format_ident!("_{}", i);
        quote! {
            #ident,
        }
    });
    let struct_gen = quote! {
        (#(#match_gen)*) => #ref_path(#(#interior_gen)*)
    };
    (struct_def, struct_gen)
}

fn gen(
    ast: &Fields,
    ref_path: &impl ToTokens,
    lt: &Lifetime,
    ref_type: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match ast {
        Named(data) => gen_named(data, ref_path, lt, ref_type),
        Unnamed(data) => gen_unnamed(data, ref_path, lt, ref_type),
        _ => panic!("Panic in function Gen: Unit types are not supported for RefAccessors."),
    }
}

struct RefGenerics<'a> {
    ref_path: Ident,
    ref_type: Ident,
    lt: Lifetime,
    generics: Generics,
    ref_types: TypeGenerics<'a>,
}

fn compute_generics<'a>(name: &'a Ident, start: &'a Generics) -> RefGenerics<'a> {
    // Names are inverted. Names which are uppercase are identifiers while lowercase names are types.
    // (Yes, I know this is bad, but it's to avoid name collisions by disregarding every single naming convention.)
    let lt = Lifetime::new(&format!("'__Ref__Access__{}", name)[..], Span::call_site());
    let ref_type = format_ident!("__{}__ref_type", name);
    let ref_path = format_ident!("{}Ref", name);
    let generics = Punctuated::<syn::GenericParam, Token!(,)>::parse_terminated
        .parse2(quote! {
            #lt, #ref_type : ::ref_clone::RefType
        })
        .unwrap();
    let mut clone = start.params.clone();
    clone.extend(generics);
    let generics = Generics {
        lt_token: Some(Lt {
            spans: [Span::call_site()],
        }),
        gt_token: Some(Gt {
            spans: [Span::call_site()],
        }),
        params: clone,
        where_clause: start.where_clause.clone(),
    };
    let ref_types = start.split_for_impl().1;
    RefGenerics {
        ref_path,
        ref_type,
        lt,
        generics,
        ref_types,
    }
}

fn impl_ref_accessors(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let RefGenerics {
        ref_path,
        ref_type,
        lt,
        generics,
        ref_types,
    } = compute_generics(name, &ast.generics);

    let (implgen, typegen, where_clause) = generics.split_for_impl();

    match &ast.data {
        Struct(DataStruct { fields, .. }) => {
            let (def, gen) = gen(fields, &ref_path, &lt, &ref_type);
            quote! {
                #[allow(non_camel_case_types, non_snake_case)]
                #vis struct #ref_path #implgen #def
                #[allow(non_camel_case_types, non_snake_case)]
                impl #implgen ::ref_clone::RefAccessors<#ref_path #typegen> for Ref<#lt, #name #ref_types, #ref_type> #where_clause {
                    #[inline(always)]
                    fn to_wrapped(self) -> #ref_path #typegen {
                        match self.value {
                            #name #gen
                        }
                    }
                }
            }
        }
        Enum(DataEnum { variants, .. }) => {
            let variants = variants.iter();
            let (def, gen) = variants
                .map(|x| {
                    let Variant { fields, ident, .. } = x;
                    let (def, gen) = gen(
                        fields,
                        &quote! {
                            #ref_path :: #ident
                        },
                        &lt,
                        &ref_type,
                    );
                    (
                        quote! {
                            #ident #def ,
                        },
                        quote! {
                            #name :: #ident #gen,
                        },
                    )
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();
            let def = def.iter();
            let gen = gen.iter();
            quote! {
                #[allow(non_camel_case_types, non_snake_case)]
                #vis enum #ref_path #implgen {
                    #(#def)*
                }
                #[allow(non_camel_case_types, non_snake_case)]
                impl #implgen ::ref_clone::RefAccessors<#ref_path #typegen> for Ref<#lt, #name #ref_types, #ref_type> #where_clause {
                    #[inline(always)]
                    fn to_wrapped(self) -> #ref_path #typegen {
                        match self.value {
                            #(#gen)*
                        }
                    }
                }
            }
        }
        _ => {
            panic!("Can not use RefAccessors with a union.");
        }
    }
}
