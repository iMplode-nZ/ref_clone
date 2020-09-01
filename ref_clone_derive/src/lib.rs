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
            let fn_ident = format_ident!("{}", ident.as_ref().unwrap());
            let tr_ident = format_ident!("Ref{}Fn{}", name, ident.as_ref().unwrap());
            let quote = quote! {
                #vis trait #tr_ident<'a, T: ::ref_clone::RefType> {
                    fn #fn_ident(self) -> ::ref_clone::Ref<'a, #ty, T>;
                }
                impl<'a, T: ::ref_clone::RefType> #tr_ident<'a, T> for ::ref_clone::Ref<'a, #name, T> {
                    fn #fn_ident(self) -> ::ref_clone::Ref<'a, #ty, T> {
                        let value = &self.value.#ident;
                        unsafe { ::ref_clone::Ref::new(self.ty, value) }
                    }
                }
            };
            println!("{}", quote);
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
