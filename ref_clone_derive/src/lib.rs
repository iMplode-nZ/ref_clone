// TODO: MAKE A SPECIAL IMPORT ATTRIBUTE TO IMPORT THE TRAIT WHICH IS NECESSARY TO CALL REF METHODS ON IT
// TODO: MAKE RefAccessors A TRAIT WITHIN REF_CLONE SO IT CAN BE IMPORTED

use proc_macro2::Span;
use syn::parse::Parser;
use syn::token::Gt;
use syn::token::Lt;

use proc_macro::TokenStream;
use punctuated::Punctuated;
use quote::format_ident;
use quote::quote;
use syn::*;
use Data::Struct;
use Fields::Named;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn RefAccessors(_attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let ast = syn::parse(input.clone()).unwrap();
    let out = impl_ref_accessors(&ast);
    println!("{}", out);
    input.extend::<TokenStream>(out.into());
    input
}

/// First TokenStream is the Struct definition (without the outside wrapper). Second TokenStream is the generator of it.
fn gen_named(
    ast: &FieldsNamed,
    struct_path: &Ident,
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
                unsafe { ::ref_clone::Ref::new(&value.#ident) }
            },
        }
    });
    let struct_gen = quote! {
        let value = self.value;
        #struct_path {
            #(#interior_gen)*
        }
    };
    (struct_def, struct_gen)
}

fn impl_ref_accessors(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Struct(DataStruct {
            fields: Named(data),
            ..
        }) => {
            // Names are inverted. Names which are uppercase are identifiers while lowercase names are types.
            // (Yes, I know this is bad, but it's to avoid name collisions by disregarding every single type rule.)
            let lt = Lifetime::new(&format!("'__Ref__Access__{}", name)[..], Span::call_site());
            let ref_type = format_ident!("__{}__ref_type", name);
            let struct_path = format_ident!("{}Ref", name);
            let generics = Punctuated::<syn::GenericParam, Token!(,)>::parse_terminated
                .parse2(quote! {
                    #lt, #ref_type : ::ref_clone::RefType
                })
                .unwrap();
            let mut clone = ast.generics.params.clone();
            clone.extend(generics);
            let generics = Generics {
                lt_token: Some(Lt {
                    spans: [Span::call_site()],
                }),
                gt_token: Some(Gt {
                    spans: [Span::call_site()],
                }),
                params: clone,
                where_clause: ast.generics.where_clause.clone(),
            };
            let (implgen, typegen, where_clause) = generics.split_for_impl();
            let ref_types = ast.generics.split_for_impl().1;

            let (def, gen) = gen_named(data, &struct_path, &lt, &ref_type);
            quote! {
                #[allow(non_camel_case_types, non_snake_case)]
                struct #struct_path #implgen #def
                #[allow(non_camel_case_types, non_snake_case)]
                impl #implgen ::ref_clone::RefAccessors<#struct_path #typegen> for Ref<#lt, #name #ref_types, #ref_type> #where_clause {
                    #[inline(always)]
                    fn to_ref(self) -> #struct_path #typegen {
                        #gen
                    }
                }
            }
        }
        /*Enum(DataEnum {
            variants,
            ..
        }) => {
            let data = variants.iter();
        },*/
        _ => {
            panic!("Can not use RefAccessors with a union.");
        }
    }
}
