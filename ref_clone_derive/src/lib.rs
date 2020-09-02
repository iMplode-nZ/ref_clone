use proc_macro2::Span;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::*;
use Data::Struct;
use Fields::Named;

#[proc_macro_derive(RefAccessors)]
pub fn ref_accessors_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ref_accessors(&ast)
}

/// First TokenStream is the Struct definition (without the outside wrapper). Second TokenStream is the generator of it.
fn gen_named(ast: &FieldsNamed, struct_path: &Ident, lt: &Lifetime, ref_type: &Ident) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let interior = ast.named.iter().map(|x| {
        let Field { vis, ident, ty, .. } = x;
        let ident = ident.as_ref().unwrap();
        quote! {
            #vis #ident : Ref<#lt, #ty, #ref_type>,
        }
    });
    let struct_def = quote! {
        #struct_path<#lt, #ref_type : ::ref_clone::RefType> {
            #(#interior)*
        }
    };
    let interior_gen = ast.named.iter().map(|x| {
        let Field { ident, .. } = x;
        let ident = ident.as_ref().unwrap();

        quote! {
            #ident : {
                unsafe { ::ref_clone::Ref::new(ty, &value.#ident) }
            },
        }
    });
    let struct_gen = quote! {
        let value = self.value;
        let ty = self.ty;
        #struct_path {
            #(#interior_gen)*
        }
    };
    (struct_def, struct_gen)
}

fn impl_ref_accessors(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Struct(DataStruct {
            fields: Named(data), ..
        }) => {
            // Names are inverted. Names which are uppercase are identifiers while lowercase names are types.
            // (Yes, I know this is bad, but it's to avoid name collisions by disregarding every single type rule.)
            let lt = Lifetime::new(&format!("'__Ref__Access__{}", name)[..], Span::call_site());
            let ref_type = format_ident!("__{}__ref_type", name);
            let struct_path = format_ident!("{}Ref", name);
            let convert = format_ident!("__{}__ref__conversion__def", name);
            let (def, gen) = gen_named(data, &struct_path, &lt, &ref_type);
            (quote! {
                #[allow(non_camel_case_types, non_snake_case)]
                struct #def
                #[allow(non_camel_case_types, non_snake_case)]
                trait #convert<#lt, #ref_type : ::ref_clone::RefType> {
                    fn to_ref(self) -> #struct_path<#lt, #ref_type>;
                }
                #[allow(non_camel_case_types, non_snake_case)]
                impl<#lt, #ref_type : ::ref_clone::RefType> #convert<#lt, #ref_type> for Ref<#lt, #name, #ref_type> {
                    #[inline(always)]
                    fn to_ref(self) -> #struct_path<#lt, #ref_type> {
                        #gen
                    }
                }
            }).into()
        },
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
