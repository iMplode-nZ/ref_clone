use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn;
use syn::Data::Struct;
use syn::DataStruct;
use syn::Field;
use syn::Fields::Named;

#[proc_macro_derive(RefAccessors)]
pub fn ref_accessors_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ref_accessors(&ast)
}

fn impl_ref_accessors(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let Struct(DataStruct {
        fields: Named(data),
        ..
    }) = &ast.data
    {
        let data = data.named.iter();
        let interior = data.map(|x| {
            let Field { vis, ident, ty, .. } = x;
            let fn_ident = format_ident!("get_{}", ident.as_ref().unwrap());
            let tr_ident = format_ident!("Ref{}Fn{}", name, ident.as_ref().unwrap());
            let quote = quote! {
                #vis trait #tr_ident<'a, T: RefType> {
                    fn #fn_ident(self) -> Ref<'a, #ty, T>;
                }
                impl<'a, T: RefType> #tr_ident<'a, T> for Ref<'a, #name, T> {
                    fn #fn_ident(self) -> Ref<'a, #ty, T> {
                        let value = &self.value.#ident;
                        Ref {
                            ty: self.ty,
                            value,
                        }
                    }
                }
            };
            quote
        });
        let gen = quote! {
            #(#interior)*
        };
        gen.into()
    } else {
        panic!("Can not use RefAccessors on a non-strict type.");
    }
}
